use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct RlToml {
    pub project: Project,
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub entry: String,
}

/// walks up from the current directory looking for `rl.toml`,
fn find_project_root() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_else(|_| {
        eprintln!("error: could not read current directory");
        std::process::exit(1);
    });
    loop {
        if dir.join("rl.toml").is_file() {
            return dir;
        }
        if !dir.pop() {
            eprintln!("error: no rl.toml found in this directory or any parent");
            std::process::exit(1);
        }
    }
}

/// reads `rl.toml`, returning the parsed config alongside the
/// project root directory it was found in (so callers can resolve
/// `entry` relative to the project, not the shell's cwd).
pub fn read_rl_toml() -> (RlToml, PathBuf) {
    let root = find_project_root();
    let content = std::fs::read_to_string(root.join("rl.toml")).unwrap_or_else(|_| {
        eprintln!("error: no rl.toml found in current directory");
        std::process::exit(1);
    });
    let parsed: RlToml = toml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("error: invalid rl.toml - {}", e);
        std::process::exit(1);
    });
    (parsed, root)
}
