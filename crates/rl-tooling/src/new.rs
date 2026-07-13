use std::io;

/// Creates a new rl project directory at `name`.
///
/// Scaffolds the following structure:
/// ```text
/// <name>/
/// |-- .gitignore
/// |-- rl.toml
/// |-- src/
///     |-- main.rl
/// ```
///
/// `rl.toml` is pre-filled with the project name and current rl version.
/// `src/main.rl` contains a hello world entry point.
/// A git repository is initialized automatically.
///
/// Prints an error and exits with code `1` on any IO failure.
pub fn create_project(name: &str, no_git: bool) {
    if let Err(e) = try_create_project(name, no_git) {
        eprintln!("error: failed to create project '{}': {}", name, e);
        std::process::exit(1);
    }
    println!("created project '{}'", name);
}

/// Inner fallible implementation of [`create_project`].
///
/// Returns [`io::Error`] if any filesystem operation or `git init` fails.
fn try_create_project(name: &str, no_git: bool) -> io::Result<()> {
    let toml = format!(
        r#"[project]
name = "{}"
rl-version = "{}"
version = "0.0.1"
entry = "src/main.rl"
"#,
        name,
        env!("CARGO_PKG_VERSION"),
    );
    let main = r#"get println from std::io
fn main() {
    println("hello world")
}
main()
"#;
    std::fs::create_dir(name)?;
    std::fs::create_dir(format!("{}/src", name))?;
    std::fs::write(format!("{}/rl.toml", name), toml)?;
    std::fs::write(format!("{}/src/main.rl", name), main)?;
    std::fs::write(format!("{}/.gitignore", name), "")?;
    if !no_git {
        std::process::Command::new("git")
            .args(["init", name])
            .output()?;
    }
    Ok(())
}
