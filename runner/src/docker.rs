//! Docker Compose services: postgres, subgraphs and routers.

use crate::proc::{self, Result};
use crate::{export, wgc};

/// Every profile of docker-compose.yml, to reach all the services.
pub const ALL_PROFILES: [&str; 6] = [
    "--profile",
    "local",
    "--profile",
    "local-feature-flag",
    "--profile",
    "control-plane",
];

/// Subgraphs and routers of every profile: everything but postgres.
const FEDERATION_SERVICES: [&str; 8] = [
    "products-subgraph-local",
    "orders-subgraph-local",
    "wundergraph-router-local",
    "products-subgraph-control-plane",
    "orders-subgraph-control-plane",
    "customers-subgraph-control-plane",
    "wundergraph-router-control-plane",
    "wundergraph-router-local-feature-flag",
];

const WAIT_MESSAGE: &str =
    "⏳ Waiting for the subgraphs to compile and start (first run can take a few minutes)...";

/// Runs `docker compose <args>`.
pub fn compose(args: &[&str]) -> Result {
    let mut full = vec!["compose"];
    full.extend_from_slice(args);
    proc::run("docker", &full)
}

pub fn is_running(service: &str) -> bool {
    proc::capture(
        "docker",
        &["compose", "ps", "--services", "--filter", "status=running"],
    )
    .is_ok_and(|o| {
        String::from_utf8_lossy(&o.stdout)
            .lines()
            .any(|l| l.trim() == service)
    })
}

/// Starts postgres if it is not running.
pub fn db() -> Result {
    if !is_running("postgres") {
        println!("🐳 Starting postgres...");
        compose(&["up", "-d", "--wait", "postgres"])?;
    }
    Ok(())
}

/// Stops subgraphs and routers. Postgres stays up.
pub fn federation_down() -> Result {
    let mut stop = ALL_PROFILES.to_vec();
    stop.extend(["stop", "-t", "0"]);
    stop.extend(FEDERATION_SERVICES);
    compose(&stop)?;

    let mut rm = ALL_PROFILES.to_vec();
    rm.extend(["rm", "-f"]);
    rm.extend(FEDERATION_SERVICES);
    compose(&rm)
}

/// The subgraph containers share the crate registry volume, but cargo's package
/// lock lives outside of it: started together on an empty volume, they unpack
/// the same crates at once and fail with "File exists". Download them first,
/// in a single throwaway container.
fn fetch_crates(profile: &str, service: &str) -> Result {
    println!("📦 Downloading the crates for the subgraphs...");
    compose(&[
        "--profile",
        profile,
        "run",
        "--rm",
        "--no-deps",
        service,
        "cargo",
        "fetch",
        "--quiet",
    ])
}

/// Starts the services of a profile and waits until they are healthy.
fn up(profile: &str, fetch_service: &str) -> Result {
    federation_down()?;
    fetch_crates(profile, fetch_service)?;
    println!("{WAIT_MESSAGE}");
    compose(&["--profile", profile, "up", "-d", "--wait"])
}

/// Exercise 09: subgraphs + local router built from the exported schemas.
pub fn local() -> Result {
    db()?;
    export::schemas()?;
    wgc::build_router(wgc::Router::Local)?;
    up("local", "products-subgraph-local")
}

/// Exercise 10 without a Cosmo account: local router with the feature flag.
pub fn local_feature_flag() -> Result {
    db()?;
    export::schemas()?;
    wgc::build_router(wgc::Router::FeatureFlag)?;
    up("local-feature-flag", "products-subgraph-control-plane")
}

/// Exercise 10 with Cosmo: the router reads its config from the control plane.
pub fn control_plane() -> Result {
    db()?;
    up("control-plane", "products-subgraph-control-plane")
}
