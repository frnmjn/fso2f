use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

const PROGRESS_FILE: &str = ".fso2f.json";
const INITIAL_BRANCH: &str = "00";

#[derive(Debug, Serialize, Deserialize)]
struct Progress {
    current_branch: String,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            current_branch: INITIAL_BRANCH.to_string(),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("--help" | "-h") => {
            print_help();
        }
        _ => run(),
    }
}

fn run() {
    let progress = load_progress();

    checkout_branch(&progress.current_branch);

    let success = run_tests();

    if success {
        save_progress(&progress);
        println!("\n✅ All tests passed!");
    } else {
        println!("\n❌ Tests failed. Fix the code and try again!");
        println!("   Run `fso2f` to re-check.\n");
        std::process::exit(1);
    }
}

fn checkout_branch(branch_name: &str) {
    let status = Command::new("git").args(["checkout", branch_name]).status();

    match status {
        Ok(s) if s.success() => {}
        Ok(_) => {
            eprintln!("❌ Failed to checkout branch: {}", branch_name);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ Failed to run git checkout: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_tests() -> bool {
    println!("   Running: cargo test\n");

    let status = Command::new("cargo").args(["test"]).status();

    match status {
        Ok(s) => s.success(),
        Err(e) => {
            eprintln!("Failed to run cargo test: {}", e);
            false
        }
    }
}

fn load_progress() -> Progress {
    fs::read_to_string(PROGRESS_FILE)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

fn save_progress(progress: &Progress) {
    let json = serde_json::to_string_pretty(progress).unwrap();
    fs::write(PROGRESS_FILE, json).unwrap();
}

fn print_help() {
    println!(
        r#"
From Simple Object to Federation Workshop Runner

USAGE:
    fso2f              Run tests for the current exercise
    fso2f --help       Show this help message

PROGRESS:
    Progress is saved in .fso2f.json (git-ignored).
"#
    );
}
