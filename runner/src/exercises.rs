//! Exercise progression: instructions, tests and solutions.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::Deserialize;

use crate::docker;
use crate::proc::{self, Result};

const PROGRESS_FILE: &str = ".fso2f";
const EXERCISES_FILE: &str = "fso2f.json";
const SOLUTIONS_REMOTE: &str = "origin";
const SOLUTIONS_BRANCH: &str = "main";

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

/// Shows the instructions of the current exercise, then asks to run its test.
pub fn start() -> Result {
    let exercise_name = load_exercise()?;
    let exercises = load_exercises()?;
    show_instructions(&exercises, &exercise_name);

    let test_filter = exercises
        .iter()
        .find(|e| e.name == exercise_name)
        .map(|e| e.test.as_str())
        .unwrap_or("");

    if ask_and_run_tests(&exercise_name, test_filter)? {
        ask_and_advance_to_next(&exercises, &exercise_name)
    } else {
        Ok(())
    }
}

/// Asks to run the tests of an exercise and, while they fail, asks to run them again.
/// Returns whether they passed.
fn ask_and_run_tests(exercise_name: &str, test_filter: &str) -> Result<bool> {
    if !ask(&format!("\n🧪 Run tests for '{exercise_name}'?")) {
        println!("\n   Run `cargo make start` when you are ready.\n");
        return Ok(false);
    }
    ensure_docker_services(exercise_name)?;

    loop {
        if run_tests(test_filter) {
            println!("\n✅ All tests passed!");
            return Ok(true);
        }
        println!("\n❌ Tests failed. Fix the code and try again!");
        if !ask("\n🔁 Run the tests again?") {
            println!("\n   Run `cargo make start` to re-check.\n");
            return Ok(false);
        }
    }
}

fn show_instructions(exercises: &[Exercise], exercise_name: &str) {
    match exercises.iter().find(|e| e.name == exercise_name) {
        Some(ex) => {
            // Unlock the test and the binaries so the files referenced by the instructions exist.
            enable_test(&ex.test);
            println!("\n🎯 Exercise {}\n\n\n{}\n\n", ex.name, ex.instructions);
        }
        None => println!("\n⚠️  No instructions found for exercise '{exercise_name}'\n"),
    }
}

/// Replaces the code of the current exercise (src/<test name>) with the solution on `main`.
pub fn solution() -> Result {
    let exercise_name = load_exercise()?;
    let exercises = load_exercises()?;
    let ex = exercises
        .iter()
        .find(|e| e.name == exercise_name)
        .ok_or_else(|| format!("No exercise '{exercise_name}' in {EXERCISES_FILE}"))?;
    let dir = format!("src/{}", ex.test);

    if !ask(&format!(
        "\n⚠️  This overwrites {dir} with the solution of exercise {}. Commit your progress first. Continue?",
        ex.name
    )) {
        return Ok(());
    }

    // FETCH_HEAD works even when the repository was cloned with --single-branch.
    proc::run("git", &["fetch", SOLUTIONS_REMOTE, SOLUTIONS_BRANCH])
        .map_err(|e| format!("Failed to fetch the solutions: {e}"))?;
    proc::run(
        "git",
        &["restore", "--source=FETCH_HEAD", "--worktree", "--", &dir],
    )
    .map_err(|e| format!("Failed to restore {dir}: {e}"))?;
    enable_test(&ex.test);

    println!(
        "\n✅ {dir} now contains the solution of exercise {} (see `git diff`). Run `cargo make start` to check it.\n",
        ex.name
    );
    Ok(())
}

fn ask(question: &str) -> bool {
    print!("{question} [Y/n] ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    matches!(input.trim().to_lowercase().as_str(), "" | "y" | "yes")
}

fn ask_and_advance_to_next(exercises: &[Exercise], current_name: &str) -> Result {
    let Some(mut idx) = exercises.iter().position(|e| e.name == current_name) else {
        return Ok(());
    };

    loop {
        let Some(next) = exercises.get(idx + 1) else {
            println!("\n🎉 You completed all exercises!");
            return Ok(());
        };
        if !ask(&format!("\n👉 Proceed to next exercise '{}'?", next.name)) {
            return Ok(());
        }

        fs::write(PROGRESS_FILE, &next.name)
            .map_err(|e| format!("Failed to update {PROGRESS_FILE}: {e}"))?;
        enable_test(&next.test);
        idx += 1;
        println!();
        show_instructions(exercises, &next.name);

        if !ask_and_run_tests(&next.name, &next.test)? {
            return Ok(());
        }
    }
}

fn load_exercise() -> Result<String> {
    if !Path::new(PROGRESS_FILE).exists() {
        fs::write(PROGRESS_FILE, "01")
            .map_err(|e| format!("Failed to create {PROGRESS_FILE}: {e}"))?;
    }
    fs::read_to_string(PROGRESS_FILE)
        .map(|content| content.trim().to_string())
        .map_err(|e| format!("Failed to read {PROGRESS_FILE}: {e}"))
}

fn load_exercises() -> Result<Vec<Exercise>> {
    let content = fs::read_to_string(EXERCISES_FILE)
        .map_err(|e| format!("Failed to read {EXERCISES_FILE}: {e}"))?;
    let file: ExercisesFile = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {EXERCISES_FILE}: {e}"))?;
    Ok(file.exercises)
}

fn run_tests(filter: &str) -> bool {
    enable_test(filter);
    let mut args = vec!["test"];
    if !filter.is_empty() {
        args.extend(["--test", filter]);
    }
    match proc::run("cargo", &args) {
        Ok(()) => true,
        Err(e) => {
            eprintln!("{e}");
            false
        }
    }
}

/// Activates an exercise test by renaming tests/<name>.hold to tests/<name>.rs.
fn enable_test(name: &str) {
    if name.is_empty() {
        return;
    }
    enable_bins(name);
    let hold = format!("tests/{name}.hold");
    let rs = format!("tests/{name}.rs");
    if Path::new(&hold).exists()
        && !Path::new(&rs).exists()
        && let Err(e) = fs::rename(&hold, &rs)
    {
        eprintln!("Failed to rename {hold} to {rs}: {e}");
    }
}

/// Activates the binaries of an exercise by renaming src/bin/<NN>_*.rs.hold to .rs.
fn enable_bins(name: &str) {
    let num: String = name
        .trim_start_matches("ex")
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if num.is_empty() {
        return;
    }
    let Ok(entries) = fs::read_dir("src/bin") else {
        return;
    };
    for entry in entries.flatten() {
        let file = entry.file_name().to_string_lossy().to_string();
        let Some(rs) = file.strip_suffix(".hold") else {
            continue;
        };
        let target = entry.path().with_file_name(rs);
        if rs.starts_with(&format!("{num}_"))
            && !target.exists()
            && let Err(e) = fs::rename(entry.path(), &target)
        {
            eprintln!("Failed to enable {rs}: {e}");
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
        "09" | "13" => DockerRequirement::LocalFederation,
        _ => DockerRequirement::None,
    }
}

fn ensure_docker_services(exercise_name: &str) -> Result {
    enable_bins(exercise_name);
    match docker_requirement(exercise_name) {
        DockerRequirement::None => Ok(()),
        DockerRequirement::Postgres => docker::db(),
        DockerRequirement::LocalFederation => {
            let all_running = [
                "postgres",
                "products-subgraph-local",
                "orders-subgraph-local",
                "wundergraph-router-local",
            ]
            .iter()
            .all(|s| docker::is_running(s));
            if all_running {
                println!(
                    "ℹ️  Local federation already running. After changing a subgraph, run `cargo make local` to restart it with your code."
                );
                Ok(())
            } else {
                // Composes the router config from *your* exported schemas and waits until
                // the subgraphs have finished compiling inside their containers.
                println!("🐳 Starting local federation services (cargo make local)...");
                docker::local()
            }
        }
    }
}

/// Starts, each on http://localhost:40NN/graphql, exercise 00 and the server of every
/// unlocked exercise that compiles. src/bin/server_NN.rs.hold is unlocked only here,
/// so an unfinished exercise never breaks `cargo test`.
pub fn servers() -> Result {
    let exercises = load_exercises()?;
    // Compiles the dependencies with visible progress; the servers are checked quietly.
    proc::run("cargo", &["build", "--bin", "00-init"])?;

    let mut bins = vec!["00-init".to_string()];
    for ex in &exercises {
        let bin = format!("server_{}", ex.name);
        let rs = format!("src/bin/{bin}.rs");
        let hold = format!("{rs}.hold");
        let unlocked = Path::new(&format!("tests/{}.rs", ex.test)).exists();
        if !unlocked || !(Path::new(&rs).exists() || Path::new(&hold).exists()) {
            continue;
        }
        let was_held = !Path::new(&rs).exists();
        if was_held {
            fs::rename(&hold, &rs).map_err(|e| format!("Failed to enable {rs}: {e}"))?;
        }
        if proc::succeeds("cargo", &["build", "--bin", &bin]) {
            bins.push(bin);
        } else {
            if was_held {
                fs::rename(&rs, &hold).map_err(|e| format!("Failed to disable {rs}: {e}"))?;
            }
            println!(
                "⏭️  Skipping exercise {}: it does not compile yet (`cargo make start` shows the errors)",
                ex.name
            );
        }
    }

    let mut children = Vec::new();
    for bin in &bins {
        proc::announce("cargo", &["run", "--quiet", "--bin", bin]);
        let child = proc::command("cargo")
            .args(["run", "--quiet", "--bin", bin])
            .spawn()
            .map_err(|e| format!("Failed to start {bin}: {e}"))?;
        children.push((bin, child));
    }
    println!("\nPress Ctrl+C to stop the servers.\n");

    // The servers run until Ctrl+C: if one exits, it failed, so stop the others too.
    loop {
        for (bin, child) in children.iter_mut() {
            if let Ok(Some(status)) = child.try_wait() {
                let bin = bin.clone();
                for (_, other) in children.iter_mut() {
                    other.kill().ok();
                }
                return Err(format!("{bin} stopped ({status})"));
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
