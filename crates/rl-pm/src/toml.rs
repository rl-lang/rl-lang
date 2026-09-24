use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::PmError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RlToml {
    project: ProjectSection,
    #[serde(default)]
    dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectSection {
    name: String,
    version: String,
    entry: String,
}

#[derive(Debug, Serialize)]
struct RlTomlOut {
    project: ProjectSection,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    dependencies: HashMap<String, Dependency>,
}

fn rl_toml_path(project_root: &Path) -> PathBuf {
    project_root.join("rl.toml")
}

pub fn read_deps(project_root: &Path) -> Result<HashMap<String, Dependency>, PmError> {
    let path = rl_toml_path(project_root);
    let content = std::fs::read_to_string(&path).map_err(|_| PmError::NoProject)?;
    let manifest: RlToml =
        toml::from_str(&content).map_err(|e| PmError::TomlParse(e.to_string()))?;
    Ok(manifest.dependencies)
}

pub fn has_dependencies_section(project_root: &Path) -> bool {
    let path = rl_toml_path(project_root);
    let Ok(content) = std::fs::read_to_string(&path) else {
        return false;
    };
    content.contains("[dependencies]")
}

pub fn add_dep(
    project_root: &Path,
    name: &str,
    dep: &Dependency,
) -> Result<(), PmError> {
    let path = rl_toml_path(project_root);
    let content = std::fs::read_to_string(&path).map_err(|_| PmError::NoProject)?;

    let mut manifest: RlToml =
        toml::from_str(&content).map_err(|e| PmError::TomlParse(e.to_string()))?;

    manifest
        .dependencies
        .insert(name.to_string(), dep.clone());

    let output = RlTomlOut {
        project: manifest.project,
        dependencies: manifest.dependencies,
    };

    let new_content =
        toml::to_string_pretty(&output).map_err(|e| PmError::TomlParse(e.to_string()))?;
    std::fs::write(&path, new_content)?;
    Ok(())
}

pub fn remove_dep(project_root: &Path, name: &str) -> Result<(), PmError> {
    let path = rl_toml_path(project_root);
    let content = std::fs::read_to_string(&path).map_err(|_| PmError::NoProject)?;

    let mut manifest: RlToml =
        toml::from_str(&content).map_err(|e| PmError::TomlParse(e.to_string()))?;

    manifest.dependencies.remove(name);

    let output = RlTomlOut {
        project: manifest.project,
        dependencies: manifest.dependencies,
    };

    let new_content =
        toml::to_string_pretty(&output).map_err(|e| PmError::TomlParse(e.to_string()))?;
    std::fs::write(&path, new_content)?;
    Ok(())
}
