use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

const PROGRESS_FILE: &str = ".fso2f.json";

#[derive(Debug, Serialize, Deserialize)]
struct Progress {
    current_branch: String,
    instructions: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let verbose = args.iter().any(|a| a == "-v" || a == "--verbose");
    let cmd = args.iter().find(|a| !a.starts_with('-') && *a != &args[0]);

    match cmd.map(|s| s.as_str()) {
        Some("task") => task(verbose),
        Some("test") => test(verbose),
        _ => print_help(),
    }
}

fn task(verbose: bool) {
    let progress = load_progress();
    checkout_branch(&progress.current_branch, verbose);

    if progress.instructions.is_empty() {
        println!("📋 No instructions for this exercise.");
    } else {
        println!("\n📖 {}\n", progress.instructions);
    }
}

fn test(verbose: bool) {
    let progress = load_progress();
    checkout_branch(&progress.current_branch, verbose);

    let success = run_tests();

    if success {
        println!("\n✅ All tests passed!");
    } else {
        println!("\n❌ Tests failed. Fix the code and try again!");
        println!("   Run `fso2f test` to re-check.\n");
        std::process::exit(1);
    }
}

fn checkout_branch(branch_name: &str, verbose: bool) {
    let output = Command::new("git").args(["checkout", branch_name]).output();

    match output {
        Ok(o) if o.status.success() => {
            if verbose {
                let stdout = String::from_utf8_lossy(&o.stdout);
                let stderr = String::from_utf8_lossy(&o.stderr);
                if !stdout.is_empty() {
                    print!("{}", stdout);
                }
                if !stderr.is_empty() {
                    eprint!("{}", stderr);
                }
            }
        }
        Ok(o) => {
            eprintln!(
                "❌ Failed to checkout branch: {}\n{}",
                branch_name,
                String::from_utf8_lossy(&o.stderr)
            );
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
    let content = fs::read_to_string(PROGRESS_FILE)
        .unwrap_or_else(|e| panic!("❌ Failed to read {}: {}", PROGRESS_FILE, e));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("❌ Failed to parse {}: {}", PROGRESS_FILE, e))
}

fn print_help() {
    println!(
        r#"
From Simple Object to Federation Workshop Runner

USAGE:
    fso2f task         Show instructions for the current exercise
    fso2f test         Run tests for the current exercise
    fso2f --help       Show this help message

PROGRESS:
    Progress is saved in .fso2f.json.
"#
    );
}
