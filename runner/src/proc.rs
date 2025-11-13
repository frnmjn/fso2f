//! Helpers to run external tools in the same way on Linux, macOS and Windows.

use std::env;
use std::io;
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub type Result<T = ()> = std::result::Result<T, String>;

/// Builds a command for an external tool.
///
/// npm installs its tools as `.cmd` shims on Windows, which `Command` does not
/// find from the bare name.
pub fn command(program: &str) -> Command {
    if cfg!(windows) && matches!(program, "npm" | "wgc") {
        Command::new(format!("{program}.cmd"))
    } else {
        Command::new(program)
    }
}

/// Prints the command about to run, so it is clear what each task does.
pub fn announce(program: &str, args: &[&str]) {
    println!("\n▶ {}\n", display(program, args));
}

/// Runs a tool with the terminal attached and fails on a non-zero exit code.
pub fn run(program: &str, args: &[&str]) -> Result {
    announce(program, args);
    let status = command(program)
        .args(args)
        .status()
        .map_err(|e| spawn_error(program, e))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`{}` failed ({status})", display(program, args)))
    }
}

/// Runs a tool without printing anything and tells whether it succeeded.
pub fn succeeds(program: &str, args: &[&str]) -> bool {
    command(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Runs a tool and captures stdout and stderr.
pub fn capture(program: &str, args: &[&str]) -> Result<Output> {
    command(program)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| spawn_error(program, e))
}

/// The first `x.y` or `x.y.z` version printed by `program args`.
pub fn version(program: &str, args: &[&str]) -> Option<String> {
    let output = capture(program, args).ok()?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    text.split(|c: char| !(c.is_ascii_digit() || c == '.'))
        .map(|token| token.trim_matches('.'))
        .find(|token| token.contains('.') && token.split('.').all(|p| !p.is_empty()))
        .map(str::to_string)
}

/// Whether an executable named `program` is in the `PATH`.
pub fn in_path(program: &str) -> bool {
    let Some(paths) = env::var_os("PATH") else {
        return false;
    };
    let extensions: Vec<String> = if cfg!(windows) {
        env::var("PATHEXT")
            .unwrap_or_else(|_| ".EXE;.CMD;.BAT".to_string())
            .split(';')
            .map(str::to_lowercase)
            .collect()
    } else {
        vec![String::new()]
    };
    env::split_paths(&paths).any(|dir| {
        extensions
            .iter()
            .any(|ext| is_file(&dir.join(format!("{program}{ext}"))))
    })
}

fn is_file(path: &Path) -> bool {
    path.metadata().is_ok_and(|m| m.is_file())
}

fn spawn_error(program: &str, error: io::Error) -> String {
    if error.kind() == io::ErrorKind::NotFound {
        format!("`{program}` not found: is it installed and in the PATH?")
    } else {
        format!("cannot run `{program}`: {error}")
    }
}

fn display(program: &str, args: &[&str]) -> String {
    std::iter::once(program)
        .chain(args.iter().copied())
        .map(|arg| {
            if arg.is_empty() || arg.contains(char::is_whitespace) {
                format!("\"{arg}\"")
            } else {
                arg.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
