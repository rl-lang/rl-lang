use serde::Deserialize;
use std::collections::HashMap;

/// Represents the full `rl.toml` project manifest.
#[derive(Deserialize)]
pub struct RlToml {
    pub project: Project,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
}

/// The `[project]` section of `rl.toml`.
#[derive(Deserialize)]
pub struct Project {
    /// The project name.
    pub name: String,
    /// The project version string (e.g. `"0.1.0"`).
    pub version: String,
    /// Path to the entry point relative to the project root (e.g. `"src/main.rl"`).
    pub entry: String,
}

/// A dependency entry in `[dependencies]`.
#[derive(Deserialize, Clone)]
pub struct Dependency {
    pub url: String,
    pub sha256: Option<String>,
}

/// Returns `true` if the raw TOML content contains a `[dependencies]` section.
pub fn has_dependencies_section(raw: &str) -> bool {
    raw.contains("[dependencies]")
}

/// Reads and parses `rl.toml` from the current directory.
///
/// Exits with code `1` if:
/// - `rl.toml` is not found in the current directory
/// - the file content is not valid TOML or doesn't match [`RlToml`]
pub fn read_rl_toml() -> RlToml {
    try_read_rl_toml(std::path::Path::new("rl.toml")).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    })
}

/// Inner fallible implementation of [`read_rl_toml`].
///
/// Returns an error if reading the file or parsing TOML fails.
pub fn try_read_rl_toml(path: &std::path::Path) -> Result<RlToml, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)
        .map_err(|_| "error: no rl.toml found in current directory".to_string())?;

    let manifest =
        toml::from_str(&content).map_err(|e| format!("error: invalid rl.toml - {}", e))?;

    Ok(manifest)
}
