use std::io;
use std::path::PathBuf;

/// Creates a new rl project directory at `name`.
///
/// Scaffolds the following structure:
/// ```text
/// <name>/
/// |-- .gitignore
/// |-- rl.toml
/// |-- src/
///     |-- main.rl (or lib.rl if lib=true)
/// ```
///
/// `rl.toml` is pre-filled with the project name and current rl version.
/// `src/main.rl` contains a hello world entry point.
/// A git repository is initialized automatically.
///
/// Prints an error and exits with code `1` on any IO failure.
pub fn create_project(name: &str, no_git: bool, lib: bool) {
    if lib {
        if let Err(e) = try_create_lib_project(name, no_git) {
            eprintln!("error: failed to create project '{}': {}", name, e);
            std::process::exit(1);
        }
        println!("created library '{}'", name);
    } else {
        if let Err(e) = try_create_project(name, no_git) {
            eprintln!("error: failed to create project '{}': {}", name, e);
            std::process::exit(1);
        }
        println!("created project '{}'", name);
    }
}

/// Creates a standalone rl script file with a shebang header.
///
/// Writes `{name}.rl` in the current directory with:
/// - A `#!` shebang line pointing to the `rl` binary (or `rlc` fallback)
/// - A hello world program
/// - Executable permissions (`0o755`)
///
/// Prints an error and exits with code `1` on any IO failure.
pub fn create_script(name: &str) {
    if let Err(e) = try_create_script(name) {
        eprintln!("error: failed to create script '{}': {}", name, e);
        std::process::exit(1);
    }
    println!("created script '{}.rl'", name);
}

/// Inner fallible implementation of [`create_script`].
pub fn try_create_script(name: &str) -> io::Result<()> {
    let shebang = find_rl_binary()
        .map(|bin| format!("#!{} run", bin))
        .unwrap_or_else(|| "#!/usr/bin/env rl run".into());
    let content = format!(
        "{shebang}\nget println from std::io\nprintln(\"hello from {name}\")\n"
    );
    let path = format!("{}.rl", name);
    std::fs::write(&path, &content)?;
    // set executable permission (owner rwx, group rx, other rx)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

/// Finds the path to the `rl` binary.
///
/// 1. Try `current_exe()` - resolves the running binary's own path
/// 2. Scan `PATH` for an executable named `rl`
/// 3. Fall back to scanning for `rlc` (the transpiler-only variant)
/// 4. Return `None` if nothing found
pub fn find_rl_binary() -> Option<String> {
    // 1. current_exe (most reliable)
    if let Ok(exe) = std::env::current_exe() {
        return Some(exe.to_string_lossy().into_owned());
    }
    // 2. scan PATH for "rl"
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in path_var.split(':') {
            let candidate = PathBuf::from(dir).join("rl");
            if candidate.exists() {
                return Some(candidate.to_string_lossy().into_owned());
            }
        }
    }
    // 3. fall back to "rlc"
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in path_var.split(':') {
            let candidate = PathBuf::from(dir).join("rlc");
            if candidate.exists() {
                return Some(candidate.to_string_lossy().into_owned());
            }
        }
    }
    None
}

/// Inner fallible implementation of [`create_project`].
///
/// Returns [`io::Error`] if any filesystem operation or `git init` fails.
pub fn try_create_project(name: &str, no_git: bool) -> io::Result<()> {
    let toml = format!(
        r#"[project]
name = "{}"
rl-version = "{}"
version = "0.0.1"
entry = "src/main.rl"

[dependencies]
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
    std::fs::write(format!("{}/.gitignore", name), "deps/\n")?;
    if !no_git {
        std::process::Command::new("git")
            .args(["init", name])
            .output()?;
    }
    Ok(())
}

/// Inner fallible implementation of library project creation.
pub fn try_create_lib_project(name: &str, no_git: bool) -> io::Result<()> {
    let toml = format!(
        r#"[project]
name = "{}"
rl-version = "{}"
version = "0.0.1"
entry = "src/lib.rl"

[dependencies]
"#,
        name,
        env!("CARGO_PKG_VERSION"),
    );
    let lib_rl = r#"# library entry point
"#;
    std::fs::create_dir(name)?;
    std::fs::create_dir(format!("{}/src", name))?;
    std::fs::write(format!("{}/rl.toml", name), toml)?;
    std::fs::write(format!("{}/src/lib.rl", name), lib_rl)?;
    std::fs::write(format!("{}/.gitignore", name), "deps/\n")?;
    if !no_git {
        std::process::Command::new("git")
            .args(["init", name])
            .output()?;
    }
    Ok(())
}
