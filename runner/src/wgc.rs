//! WunderGraph CLI (wgc): local router configs and Cosmo cloud commands.

use std::fs;
use std::path::Path;

use crate::proc::{self, Result};

const SUPERGRAPH: &str = "schemas/13_supergraph.graphql";

pub enum Router {
    Local,
    FeatureFlag,
}

impl Router {
    fn input(&self) -> &'static str {
        match self {
            Router::Local => "./schemas/federation/federation-compose.yaml",
            Router::FeatureFlag => "./schemas/federation/federation-compose-feature-flag.yaml",
        }
    }

    fn output(&self) -> &'static str {
        match self {
            Router::Local => "./schemas/federation/router.json",
            Router::FeatureFlag => "./schemas/federation/router-feature-flag.json",
        }
    }
}

/// Installs wgc with npm if it is missing.
pub fn install() -> Result {
    if proc::in_path("wgc") {
        return Ok(());
    }
    if !proc::in_path("npm") {
        return Err("wgc requires Node.js (npm): https://nodejs.org".to_string());
    }
    println!("📦 Installing the WunderGraph CLI (wgc)...");
    proc::run("npm", &["-g", "install", "wgc"])
}

/// Runs `wgc <args>`, installing it first if needed.
pub fn run(args: &[&str]) -> Result {
    install()?;
    proc::run("wgc", args)
}

/// Like [`run`], but a failure is only reported: used by the clean-up commands,
/// where the resource may already be gone.
fn run_ignoring_failure(args: &[&str]) {
    if let Err(e) = run(args) {
        eprintln!("⚠️  {e}");
    }
}

pub fn build_router(router: Router) -> Result {
    run(&[
        "router",
        "compose",
        "-i",
        router.input(),
        "-o",
        router.output(),
    ])
}

/// Generates the supergraph used by the cynic client of exercise 13, if missing.
pub fn supergraph() -> Result {
    if Path::new(SUPERGRAPH).exists() {
        println!("{SUPERGRAPH} already exists");
        return Ok(());
    }
    build_router(Router::Local)?;

    let router = Router::Local.output();
    let config = fs::read_to_string(router).map_err(|e| format!("cannot read {router}: {e}"))?;
    let config: serde_json::Value =
        serde_json::from_str(&config).map_err(|e| format!("invalid {router}: {e}"))?;
    let schema = config["engineConfig"]["graphqlSchema"]
        .as_str()
        .ok_or_else(|| format!("{router} has no engineConfig.graphqlSchema"))?;

    // A partial file would be skipped on rerun and break build.rs for the whole crate.
    let tmp = format!("{SUPERGRAPH}.tmp");
    fs::write(&tmp, format!("{schema}\n")).map_err(|e| format!("cannot write {tmp}: {e}"))?;
    fs::rename(&tmp, SUPERGRAPH).map_err(|e| format!("cannot write {SUPERGRAPH}: {e}"))?;
    println!("✅ {SUPERGRAPH} generated");
    Ok(())
}

pub fn router_token_create() -> Result {
    install()?;
    let args = [
        "router",
        "token",
        "create",
        "fso2f",
        "-g",
        "fso2f",
        "-r",
    ];
    proc::announce("wgc", &args);
    let output = proc::command("wgc")
        .args(args)
        .stderr(std::process::Stdio::inherit())
        .output()
        .map_err(|e| format!("cannot run wgc: {e}"))?;
    if !output.status.success() {
        return Err("Failed to create router token".to_string());
    }
    let token = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let export = if cfg!(windows) {
        format!("$env:GRAPH_API_TOKEN = '{token}'    # PowerShell")
    } else {
        format!("export GRAPH_API_TOKEN='{token}'")
    };
    println!("\n✅ Token created! Run the following command to export it:\n\n\n\n");
    println!("  {export}");
    println!("\n\n\n");
    Ok(())
}

pub fn feature_flag_create() -> Result {
    run(&[
        "feature-flag",
        "create",
        "flag-customers",
        "--feature-subgraphs",
        "strangler-customers",
        "--enabled",
    ])
}

pub fn promote_customers() -> Result {
    install()?;
    run_ignoring_failure(&[
        "subgraph",
        "update",
        "customers",
        "--routing-url",
        "http://customers-subgraph:3003/graphql",
    ]);
    run_ignoring_failure(&[
        "subgraph",
        "publish",
        "customers",
        "--schema",
        "schemas/10_customers.graphql",
    ]);
    run_ignoring_failure(&[
        "subgraph",
        "delete",
        "strangler-customers",
        "-f",
    ]);
    run_ignoring_failure(&[
        "feature-flag",
        "delete",
        "flag-customers",
        "-f",
    ]);
    Ok(())
}

/// Deletes every resource created on Cosmo.
pub fn reset() -> Result {
    install()?;
    run_ignoring_failure(&[
        "router",
        "token",
        "delete",
        "fso2f",
        "-g",
        "fso2f",
        "-f",
    ]);
    run_ignoring_failure(&[
        "federated-graph",
        "delete",
        "fso2f",
        "-f",
    ]);
    for subgraph in ["products", "orders", "customers"] {
        run_ignoring_failure(&["subgraph", "delete", subgraph, "-f"]);
    }
    run_ignoring_failure(&[
        "feature-subgraph",
        "delete",
        "strangler-customers",
        "-f",
    ]);
    run_ignoring_failure(&[
        "feature-flag",
        "delete",
        "flag-customers",
        "-f",
    ]);
    Ok(())
}
