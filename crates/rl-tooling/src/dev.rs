use serde::Deserialize;

/// Represents the full `rl.toml` project manifest.
#[derive(Deserialize)]
pub struct RlToml {
    pub project: Project,
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

/// Reads and parses `rl.toml` from the current directory.
///
/// Exits with code `1` if:
/// - `rl.toml` is not found in the current directory
/// - the file content is not valid TOML or doesn't match [`RlToml`]
pub fn read_rl_toml() -> RlToml {
    let content = std::fs::read_to_string("rl.toml").unwrap_or_else(|_| {
        eprintln!("error: no rl.toml found in current directory");
        std::process::exit(1);
    });

    toml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("error: invalid rl.toml - {}", e);
        std::process::exit(1);
    })
}
