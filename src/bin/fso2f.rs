use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

const PROGRESS_FILE: &str = ".fso2f";
const EXERCISES_FILE: &str = "fso2f.json";

#[derive(Debug, Deserialize)]
struct Exercise {
    name: String,
    instructions: String,
    test: String,
    solution: String,
}

#[derive(Debug, Deserialize)]
struct ExercisesFile {
    exercises: Vec<Exercise>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let verbose = args.iter().any(|a| a == "-v" || a == "--verbose");
    let cmd = args.iter().find(|a| !a.starts_with('-') && *a != &args[0]);

    match cmd.map(|s| s.as_str()) {
        Some("task") => task(verbose),
        Some("test") => test(verbose),
        Some("solution") => solution(verbose),
        _ => print_help(),
    }
}

fn task(verbose: bool) {
    let exercise_name = load_exercise();
    checkout_branch(&exercise_name, verbose);

    let exercises = load_exercises();
    match exercises.iter().find(|e| e.name == exercise_name) {
        Some(ex) => println!("\n📖 {}\n", ex.instructions),
        None => println!(
            "\n⚠️  No instructions found for exercise '{}'\n",
            exercise_name
        ),
    }
}

fn load_exercise() -> String {
    if !std::path::Path::new(PROGRESS_FILE).exists() {
        fs::write(PROGRESS_FILE, "workshop").expect("❌ Failed to create .fso2f.json");
    }
    let content = fs::read_to_string(PROGRESS_FILE)
        .unwrap_or_else(|e| panic!("❌ Failed to read {}: {}", PROGRESS_FILE, e));
    content.trim().to_string()
}

fn load_exercises() -> Vec<Exercise> {
    let content = fs::read_to_string(EXERCISES_FILE)
        .unwrap_or_else(|e| panic!("❌ Failed to read {}: {}", EXERCISES_FILE, e));
    let file: ExercisesFile = serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("❌ Failed to parse {}: {}", EXERCISES_FILE, e));
    file.exercises
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

fn test(_verbose: bool) {
    let exercise_name = load_exercise();

    let exercises = load_exercises();
    let exercise = exercises.iter().find(|e| e.name == exercise_name);
    let test_filter = exercise.map(|e| e.test.as_str()).unwrap_or("");

    let success = run_tests(test_filter);

    if success {
        println!("\n✅ All tests passed!");

        let current_idx = exercises.iter().position(|e| e.name == exercise_name);

        if let Some(idx) = current_idx {
            if idx + 1 < exercises.len() {
                let next = &exercises[idx + 1];
                print!("\n👉 Proceed to next exercise '{}'? [Y/n] ", next.name);
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let answer = input.trim().to_lowercase();

                if answer.is_empty() || answer == "y" || answer == "yes" {
                    fs::write(PROGRESS_FILE, &next.name)
                        .expect("❌ Failed to update progress file");
                    println!(
                        "\n📝 Moved to exercise '{}'. Run `fso2f task` to see instructions.\n",
                        next.name
                    );
                }
            } else {
                println!("\n🎉 You completed all exercises!");
            }
        }
    } else {
        println!("\n❌ Tests failed. Fix the code and try again!");
        println!("   Run `fso2f test` to re-check.\n");
        std::process::exit(1);
    }
}

fn run_tests(filter: &str) -> bool {
    if filter.is_empty() {
        println!("   Running: cargo test\n");
    } else {
        println!("   Running: cargo test {}\n", filter);
    }

    let mut args = vec!["test"];
    if !filter.is_empty() {
        args.push(filter);
    }

    let status = Command::new("cargo").args(&args).status();

    match status {
        Ok(s) => s.success(),
        Err(e) => {
            eprintln!("Failed to run cargo test: {}", e);
            false
        }
    }
}

fn solution(verbose: bool) {
    let exercise_name = load_exercise();
    let exercises = load_exercises();

    match exercises.iter().find(|e| e.name == exercise_name) {
        Some(ex) => {
            checkout_branch(&ex.solution, verbose);
            println!("\n✅ Switched to solution branch '{}'\n", ex.solution);
        }
        None => {
            eprintln!("❌ No solution found for exercise '{}'", exercise_name);
            std::process::exit(1);
        }
    }
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
    Exercise name is saved in .fso2f.json.
"#
    );
}
