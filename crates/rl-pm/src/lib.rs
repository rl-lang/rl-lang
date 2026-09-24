pub mod cache;
pub mod download;
pub mod error;
pub mod resolve;
pub mod symlink;
pub mod toml;

use std::path::Path;

pub use error::PmError;
pub use toml::Dependency;

pub struct DepInfo {
    pub name: String,
    pub url: String,
    pub installed: bool,
}

pub fn install_all(project_root: &Path) -> Result<(), PmError> {
    let deps = toml::read_deps(project_root)?;
    for (name, dep) in &deps {
        install_one(project_root, name, dep)?;
    }
    Ok(())
}

pub fn install_one(
    _project_root: &Path,
    name: &str,
    dep: &Dependency,
) -> Result<(), PmError> {
    eprintln!("installing {} from {}", name, dep.url);
    download::fetch_and_extract(name, &dep.url, dep.sha256.as_deref())?;
    symlink::create_link(name)?;
    eprintln!("  {} installed", name);
    Ok(())
}

pub fn add_dep(
    project_root: &Path,
    name: &str,
    url: &str,
    sha256: Option<&str>,
) -> Result<(), PmError> {
    let dep = Dependency {
        url: url.to_string(),
        sha256: sha256.map(|s| s.to_string()),
    };

    download::fetch_and_extract(name, url, sha256)?;
    symlink::create_link(name)?;
    toml::add_dep(project_root, name, &dep)?;

    eprintln!("added {} ({})", name, url);
    Ok(())
}

pub fn remove_dep(project_root: &Path, name: &str) -> Result<(), PmError> {
    symlink::remove_link(name)?;
    toml::remove_dep(project_root, name)?;
    eprintln!("removed {}", name);
    Ok(())
}

pub fn list_deps(project_root: &Path) -> Result<Vec<DepInfo>, PmError> {
    let deps = toml::read_deps(project_root)?;
    let deps_dir = project_root.join("deps");

    let mut result = Vec::new();
    for (name, dep) in &deps {
        let installed = deps_dir.join(name).exists();
        result.push(DepInfo {
            name: name.clone(),
            url: dep.url.clone(),
            installed,
        });
    }
    Ok(result)
}

pub fn update_deps(project_root: &Path) -> Result<(), PmError> {
    let deps = toml::read_deps(project_root)?;
    for (name, dep) in &deps {
        eprintln!("updating {}...", name);

        let extracted = cache::extracted_dir().join(name);
        if extracted.exists() {
            std::fs::remove_dir_all(&extracted)?;
        }

        install_one(project_root, name, dep)?;
    }
    Ok(())
}

pub fn cache_clean() -> Result<(), PmError> {
    cache::clean()?;
    eprintln!("cache cleared");
    Ok(())
}
