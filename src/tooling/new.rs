use std::io;

pub fn create_project(name: &str) {
    if let Err(e) = try_create_project(name) {
        eprintln!("error: failed to create project '{}': {}", name, e);
        std::process::exit(1);
    }
    println!("created project '{}'", name);
}

fn try_create_project(name: &str) -> io::Result<()> {
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
    let main = r#"get println from std::display
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
    std::process::Command::new("git")
        .args(["init", name])
        .output()?;
    Ok(())
}
