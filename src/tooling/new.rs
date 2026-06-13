pub fn create_project(name: &str) {
    // the project coniguration file
    let toml = format!(
        r#"[project]
name = "{}"
version = "0.0.1"
entry = "main.rl"
"#,
        name
    );

    // the demo function of project
    let main = r#"get println from std::display

fn main() {
    println("hello world")
}

main()
"#;

    // create project folder
    std::fs::create_dir(name).unwrap();

    // write files
    std::fs::write(format!("{}/rl.toml", name), toml).unwrap();
    std::fs::write(format!("{}/main.rl", name), main).unwrap();
    std::fs::write(format!("{}/.gitignore", name), "").unwrap();

    // git init
    std::process::Command::new("git")
        .args(["init", name])
        .output()
        .unwrap();

    println!("created project '{}'", name);
}
