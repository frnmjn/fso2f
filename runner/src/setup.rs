//! `cargo make check-setup`: tools, Docker images and the exercise 00 test.

use crate::docker;
use crate::proc::{self, Result};

struct Report {
    failed: bool,
}

impl Report {
    fn ok(&self, message: impl AsRef<str>) {
        println!("  ✅ {}", message.as_ref());
    }

    fn ko(&mut self, message: impl AsRef<str>) {
        println!("  ❌ {}", message.as_ref());
        self.failed = true;
    }

    fn warn(&self, message: impl AsRef<str>) {
        println!("  ⚠️  {}", message.as_ref());
    }

    /// `program` must be in the PATH; its version is read from `version_args`.
    fn tool(&mut self, program: &str, version_args: &[&str], hint: &str) {
        if proc::in_path(program) {
            let version = proc::version(version_args[0], &version_args[1..]).unwrap_or_default();
            self.ok(format!("{program} {version}"));
        } else {
            self.ko(format!("{program} not found: {hint}"));
        }
    }

    fn optional_tool(&self, program: &str, hint: &str) {
        if proc::in_path(program) {
            let version = proc::version(program, &["--version"]).unwrap_or_default();
            self.ok(format!("{program} {version}"));
        } else {
            self.warn(format!("{program} not found (optional): {hint}"));
        }
    }

    fn step(&mut self, ok: &str, ko: &str, result: Result) {
        match result {
            Ok(()) => self.ok(ok),
            Err(e) => self.ko(format!("{ko}: {e}")),
        }
    }
}

pub fn check_setup() -> Result {
    let mut report = Report { failed: false };

    println!("\n🔧 Tools");
    report.tool(
        "cargo",
        &["cargo", "--version"],
        "https://www.rust-lang.org/tools/install",
    );
    report.tool(
        "cargo-make",
        &["cargo", "make", "--version"],
        "cargo install cargo-make",
    );
    report.tool(
        "docker",
        &["docker", "--version"],
        "https://docs.docker.com/engine/install/",
    );
    report.tool(
        "wgc",
        &["wgc", "--version"],
        "cargo make wgc-install (needs Node.js: https://nodejs.org)",
    );
    report.optional_tool("cynic", "exercise 13, cargo install --locked cynic-cli");

    println!("\n🐳 Docker");
    if !proc::succeeds("docker", &["info"]) {
        report.ko(
            "Docker daemon not reachable: start Docker Desktop (or Rancher Desktop, OrbStack, colima...)",
        );
    } else {
        let compose = proc::version("docker", &["compose", "version"]).unwrap_or_default();
        report.ok(format!("daemon running, Docker Compose {compose}"));

        println!("  ⏳ Pulling postgres and router images...");
        let mut pull = docker::ALL_PROFILES.to_vec();
        pull.extend(["pull", "--quiet", "--ignore-buildable"]);
        report.step(
            "images pulled",
            "docker compose pull failed",
            docker::compose(&pull),
        );

        println!("  ⏳ Building the subgraph image...");
        let mut build = docker::ALL_PROFILES.to_vec();
        build.extend(["build", "--quiet"]);
        report.step(
            "subgraph image built",
            "docker compose build failed",
            docker::compose(&build),
        );

        let postgres = if proc::succeeds("docker", &["compose", "up", "-d", "--wait", "postgres"]) {
            Ok(())
        } else {
            Err("docker compose up -d --wait postgres".to_string())
        };
        report.step(
            "postgres running on localhost:5432",
            "postgres did not start",
            postgres,
        );
    }

    println!("\n🧪 Exercise 00 (compiles the crate, queries the schema and the database)");
    report.step(
        "exercise 00 test passed",
        "exercise 00 test failed",
        proc::run("cargo", &["test", "--test", "ex00_init"]),
    );

    println!();
    if report.failed {
        return Err(
            "Setup incomplete: fix the items above and run cargo make check-setup again."
                .to_string(),
        );
    }
    println!("🎉 All set! Start the workshop with cargo make start");
    Ok(())
}
