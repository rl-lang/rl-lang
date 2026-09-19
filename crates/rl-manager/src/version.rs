use crate::error::{RlmError, Result};
use crate::variants::REPO;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Version {
    Latest,
    Nightly,
    Specific(String),
}

impl Version {
    pub fn as_tag(&self) -> String {
        match self {
            Version::Latest => "latest".to_string(),
            Version::Nightly => "nightly".to_string(),
            Version::Specific(v) => v.clone(),
        }
    }
}

pub fn resolve_version(requested: &str) -> Result<String> {
    match requested {
        "latest" => resolve_latest(),
        "nightly" => {
            if release_exists("nightly") {
                Ok("nightly".to_string())
            } else {
                Err(RlmError::VersionNotFound("nightly".to_string()))
            }
        }
        v if v.starts_with('v') && v.len() > 1 => {
            if release_exists(v) {
                Ok(v.to_string())
            } else {
                Err(RlmError::VersionNotFound(v.to_string()))
            }
        }
        v if v.chars().next().is_some_and(|c| c.is_ascii_digit()) => {
            let normalized = format!("v{}", v);
            if release_exists(&normalized) {
                Ok(normalized)
            } else {
                Err(RlmError::VersionNotFound(v.to_string()))
            }
        }
        other => Err(RlmError::VersionNotFound(other.to_string())),
    }
}

fn resolve_latest() -> Result<String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);
    let body = ureq::get(&url)
        .call()
        .map_err(|e| RlmError::Http(e.to_string()))?
        .into_string()
        .map_err(|e| RlmError::Http(e.to_string()))?;

    // Simple JSON parse for tag_name
    if let Some(start) = body.find("\"tag_name\"") {
        let slice = &body[start..];
        if let Some(q1) = slice.find('"') {
            let slice = &slice[q1 + 1..];
            if let Some(q2) = slice.find('"') {
                return Ok(slice[..q2].to_string());
            }
        }
    }
    Err(RlmError::VersionNotFound("latest".to_string()))
}

pub fn release_exists(tag: &str) -> bool {
    let url = format!("https://api.github.com/repos/{}/releases/tags/{}", REPO, tag);
    ureq::head(&url).call().is_ok()
}
