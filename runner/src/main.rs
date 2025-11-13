//! Workshop runner behind the cargo-make tasks. Run `cargo make help` for the list.

mod docker;
mod exercises;
mod export;
mod proc;
mod setup;
mod wgc;

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use proc::Result;

fn main() -> ExitCode {
    if let Err(e) = env::set_current_dir(repository_root()) {
        eprintln!("❌ Cannot enter the repository root: {e}");
        return ExitCode::FAILURE;
    }

    let args: Vec<String> = env::args().skip(1).collect();
    let (command, rest) = match args.split_first() {
        Some((command, rest)) => (command.as_str(), rest),
        None => ("help", &[][..]),
    };

    match dispatch(command, rest) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("\n❌ {e}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch(command: &str, rest: &[String]) -> Result {
    match command {
        "check-setup" => setup::check_setup(),
        "start" => exercises::start(),
        "solution" => exercises::solution(),
        "servers" => exercises::servers(),
        "db" => docker::db(),
        "export-schemas" => export::schemas(),
        "wgc-build-router" => wgc::build_router(wgc::Router::Local),
        "wgc-build-router-feature-flag" => wgc::build_router(wgc::Router::FeatureFlag),
        "local" => docker::local(),
        "local-feature-flag" => docker::local_feature_flag(),
        "federation-down" => docker::federation_down(),
        "supergraph" => wgc::supergraph(),
        "wgc-install" => wgc::install(),
        "router-token-create" => wgc::router_token_create(),
        "control-plane" => docker::control_plane(),
        "feature-flag-create" => wgc::feature_flag_create(),
        "promote-customers" => wgc::promote_customers(),
        "federation-reset" => wgc::reset(),
        // Runs any wgc command, resolving the npm shim on Windows.
        "wgc" => wgc::run(&rest.iter().map(String::as_str).collect::<Vec<_>>()),
        "help" | "-h" | "--help" => {
            help();
            Ok(())
        }
        other => {
            help();
            Err(format!("Unknown command '{other}'"))
        }
    }
}

/// The directory with fso2f.json, looking up from the current directory first.
fn repository_root() -> PathBuf {
    let is_root = |dir: &Path| dir.join("fso2f.json").is_file();
    env::current_dir()
        .ok()
        .and_then(|cwd| {
            cwd.ancestors()
                .find(|dir| is_root(dir))
                .map(Path::to_path_buf)
        })
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("the runner lives inside the repository")
                .to_path_buf()
        })
}

fn help() {
    const TASKS: &[(&str, &[(&str, &str)])] = &[
        (
            "Development",
            &[
                ("clippy", "Run lints"),
                (
                    "sqlx-prepare",
                    "Regenerate the sqlx offline query cache (.sqlx)",
                ),
            ],
        ),
        (
            "Workshop Runner",
            &[
                (
                    "check-setup",
                    "Check tools and Docker, pull images, run exercise 00",
                ),
                (
                    "init",
                    "Run the exercise 00 GraphQL server (localhost:4000)",
                ),
                (
                    "start",
                    "Show current exercise instructions and ask to run its tests",
                ),
                (
                    "solution",
                    "Replace the current exercise code with its solution (from main)",
                ),
                (
                    "servers",
                    "Run the servers of the unlocked exercises for Postman (localhost:40NN)",
                ),
            ],
        ),
        (
            "Local Federation",
            &[
                ("export-schemas", "Export GraphQL schemas"),
                ("wgc-build-router", "Build router configuration"),
                (
                    "wgc-build-router-feature-flag",
                    "Build router configuration with the ex10 feature flag",
                ),
                ("local", "Run all subgraphs with local router"),
                (
                    "local-feature-flag",
                    "Run ex10 subgraphs with a local router + feature flag",
                ),
                (
                    "federation-down",
                    "Stop subgraphs and routers (keeps postgres)",
                ),
                ("supergraph", "Generate the supergraph schema used by ex13"),
            ],
        ),
        (
            "WunderGraph Federation",
            &[
                (
                    "wgc-install",
                    "Install wgc (automatic when a task needs it)",
                ),
                ("federated-graph-create", "Create the federated graph"),
                (
                    "router-token-create",
                    "Create router token and show export command",
                ),
                ("subgraph-publish-products", "Publish products subgraph"),
                (
                    "control-plane",
                    "Run all subgraphs with cloud control plane",
                ),
                ("subgraph-publish-orders", "Publish orders subgraph"),
                (
                    "subgraph-publish-fake-customers",
                    "Publish fake customers subgraph",
                ),
                (
                    "subgraph-publish-strangler-customers",
                    "Publish strangler customers subgraph",
                ),
                ("feature-flag-create", "Create customers feature flag"),
                ("feature-flag-disable", "Disable customers feature flag"),
                ("feature-flag-enable", "Enable customers feature flag"),
                ("promote-customers", "Promote feature subgraph to base"),
                ("federation-reset", "Delete all the resource created"),
            ],
        ),
        ("Help", &[("help", "Show this help")]),
    ];

    println!("Available tasks:");
    for (i, (section, tasks)) in TASKS.iter().enumerate() {
        if i > 0 {
            println!();
        }
        println!("{section}:");
        for (task, description) in *tasks {
            println!("  {:<52}- {description}", format!("cargo make {task}"));
        }
    }
}
