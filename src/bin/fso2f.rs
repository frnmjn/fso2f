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
        Some("demo") => demo(verbose),
        _ => print_help(),
    }
}

fn task(_verbose: bool) {
    let exercise_name = load_exercise();

    let exercises = load_exercises();
    match exercises.iter().find(|e| e.name == exercise_name) {
        Some(ex) => println!("\n🎯 Exercise {}\n\n\n{}\n\n", ex.name, ex.instructions),
        None => println!(
            "\n⚠️  No instructions found for exercise '{}'\n",
            exercise_name
        ),
    }
}

fn show_current_task() {
    task(false);
}

fn ask_and_advance_to_next(exercises: &[Exercise], current_name: &str) {
    let mut idx = match exercises.iter().position(|e| e.name == current_name) {
        Some(i) => i,
        None => return,
    };

    loop {
        if idx + 1 >= exercises.len() {
            println!("\n🎉 You completed all exercises!");
            break;
        }

        let next = &exercises[idx + 1];
        print!("\n👉 Proceed to next exercise '{}'? [Y/n] ", next.name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let answer = input.trim().to_lowercase();

        if answer.is_empty() || answer == "y" || answer == "yes" {
            let next_name = next.name.clone();
            let next_test = next.test.clone();
            fs::write(PROGRESS_FILE, &next_name).expect("❌ Failed to update progress file");
            idx += 1;
            println!();
            show_current_task();

            print!("\n🧪 Run tests for '{}'? [Y/n] ", next_name);
            io::stdout().flush().unwrap();
            let mut run_input = String::new();
            io::stdin().read_line(&mut run_input).unwrap();
            let run_answer = run_input.trim().to_lowercase();

            if run_answer.is_empty() || run_answer == "y" || run_answer == "yes" {
                ensure_docker_services(&next_name);
                let success = run_tests(&next_test);
                if success {
                    println!("\n✅ All tests passed!");
                } else {
                    println!("\n❌ Tests failed. Fix the code and try again!");
                    println!("   Run `cargo make test` to re-check.\n");
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn demo(_verbose: bool) {
    let exercise_name = load_exercise();
    let exercises = load_exercises();

    show_current_task();
    ask_and_advance_to_next(&exercises, &exercise_name);
}

fn load_exercise() -> String {
    if !std::path::Path::new(PROGRESS_FILE).exists() {
        fs::write(PROGRESS_FILE, "01").expect("❌ Failed to create .fso2f.json");
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

fn test(_verbose: bool) {
    let exercise_name = load_exercise();

    ensure_docker_services(&exercise_name);

    let exercises = load_exercises();
    let exercise = exercises.iter().find(|e| e.name == exercise_name);
    let test_filter = exercise.map(|e| e.test.as_str()).unwrap_or("");

    let success = run_tests(test_filter);

    if success {
        println!("\n✅ All tests passed!");
        ask_and_advance_to_next(&exercises, &exercise_name);
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
        println!("   Running: cargo test --test {}\n", filter);
    }

    let mut args = vec!["test".to_string()];
    if !filter.is_empty() {
        args.push("--test".to_string());
        args.push(filter.to_string());
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

enum DockerRequirement {
    None,
    Postgres,
    LocalFederation,
}

fn docker_requirement(exercise_name: &str) -> DockerRequirement {
    match exercise_name {
        "07" | "08" => DockerRequirement::Postgres,
        "09" => DockerRequirement::LocalFederation,
        _ => DockerRequirement::None,
    }
}

fn ensure_docker_services(exercise_name: &str) {
    match docker_requirement(exercise_name) {
        DockerRequirement::None => {}
        DockerRequirement::Postgres => {
            if !is_compose_service_running("postgres") {
                println!("🐳 Starting postgres...");
                run_docker_compose(&[], &["up", "-d", "--wait", "postgres"]);
            }
        }
        DockerRequirement::LocalFederation => {
            let all_running = [
                "postgres",
                "products-subgraph-local",
                "orders-subgraph-local",
                "wundergraph-router-local",
            ]
            .iter()
            .all(|s| is_compose_service_running(s));
            if !all_running {
                println!("🐳 Starting local federation services...");
                let router_json = "./schemas/federation/router.json";
                if !std::path::Path::new(router_json).exists() {
                    println!("   Generating schemas and router config...");
                    Command::new("cargo")
                        .args(["run", "--bin", "export-schemas"])
                        .status()
                        .ok();
                    Command::new("wgc")
                        .args([
                            "router",
                            "compose",
                            "-i",
                            "./schemas/federation/federation-compose.yaml",
                            "-o",
                            router_json,
                        ])
                        .status()
                        .ok();
                }
                run_docker_compose(&["--profile", "local"], &["up", "-d", "--wait"]);
            }
        }
    }
}

fn is_compose_service_running(service: &str) -> bool {
    let output = Command::new("docker")
        .args(["compose", "ps", "--services", "--filter", "status=running"])
        .output();
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.lines().any(|l| l.trim() == service)
        }
        _ => false,
    }
}

fn run_docker_compose(profile_args: &[&str], cmd_args: &[&str]) {
    let mut args: Vec<&str> = vec!["compose"];
    args.extend_from_slice(profile_args);
    args.extend_from_slice(cmd_args);
    if let Err(e) = Command::new("docker").args(&args).status() {
        eprintln!("Failed to run docker compose: {}", e);
    }
}

fn print_help() {
    println!(
        r#"
From Simple Object to Federation Workshop Runner

USAGE:
    fso2f task         Show instructions for the current exercise
    fso2f test         Run tests for the current exercise
    fso2f demo         Show current task, then ask to proceed to next
    fso2f --help       Show this help message

PROGRESS:
    Exercise name is saved in .fso2f.json.
"#
    );
}
