use serde::Deserialize;

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
