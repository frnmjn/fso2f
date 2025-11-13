//! Export of the GraphQL SDL of the unlocked exercises.

use std::fs;

use crate::proc::{self, Result};

/// Runs every `src/bin/NN_export-schemas.rs` (the locked ones are still `.rs.hold`).
pub fn schemas() -> Result {
    let entries = fs::read_dir("src/bin").map_err(|e| format!("cannot read src/bin: {e}"))?;
    let mut bins: Vec<String> = entries
        .flatten()
        .filter_map(|entry| {
            let file = entry.file_name().to_string_lossy().to_string();
            file.strip_suffix(".rs")
                .filter(|name| name.ends_with("_export-schemas"))
                .map(str::to_string)
        })
        .collect();
    bins.sort();

    for bin in bins {
        proc::run("cargo", &["run", "--quiet", "--bin", &bin])?;
    }
    Ok(())
}
