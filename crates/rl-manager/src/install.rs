use std::io::Read;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};

use crate::error::{RlmError, Result};
use crate::platform::{Arch, Platform};
use crate::variants::{Variant, REPO};

/// Phase identifiers emitted with [`InstallEvent`]. UIs map these to
/// one progress bar each: download, sha256 verification, extraction.
pub const PHASE_DOWNLOAD: &str = "download";
pub const PHASE_CHECKSUM: &str = "sha256";
pub const PHASE_EXTRACT: &str = "extract";

/// Structured progress reported while installing one binary.
///
/// UIs (CLI, TUI) match on this to render progress bars; the plain
/// log line equivalent is available via [`InstallEvent::message`].
#[derive(Debug, Clone)]
pub enum InstallEvent {
    /// Human-readable log line (phase changes, completion, skips).
    Message(String),
    /// A phase started.
    PhaseStart { phase: &'static str },
    /// Byte progress within a phase. `total` is `None` when the size
    /// is unknown (e.g. missing `content-length` header).
    PhaseProgress { phase: &'static str, done: u64, total: Option<u64> },
    /// A phase finished.
    PhaseDone { phase: &'static str },
}

impl InstallEvent {
    /// Plain log-line equivalent, for UIs that only show text.
    pub fn message(&self) -> Option<&str> {
        match self {
            InstallEvent::Message(m) => Some(m),
            _ => None,
        }
    }

    /// Completion ratio 0.0..=1.0 for progress events, if computable.
    pub fn ratio(&self) -> Option<f64> {
        match self {
            InstallEvent::PhaseProgress { done, total, .. } => {
                (*total).filter(|t| *t > 0).map(|t| (*done as f64 / t as f64).clamp(0.0, 1.0))
            }
            InstallEvent::PhaseDone { .. } => Some(1.0),
            _ => None,
        }
    }
}

/// Reborrow an optional progress callback so it can be forwarded to a
/// helper and still be used afterwards.
fn reborrow<'a>(
    slot: &'a mut Option<&mut dyn FnMut(InstallEvent)>,
) -> Option<&'a mut dyn FnMut(InstallEvent)> {
    match slot {
        Some(cb) => Some(&mut **cb),
        None => None,
    }
}

pub fn download_url(variant: &Variant, version: &str, platform: Platform, arch: Arch) -> String {
    let ext = platform.archive_ext();
    let asset = format!("{}-{}-{}.{}", variant.actual, platform.as_str(), arch.as_str(), ext);
    format!("https://github.com/{}/releases/download/{}/{}", REPO, version, asset)
}

pub fn download_file(url: &str, dest: &Path) -> Result<()> {
    download_file_with_progress(url, dest, None)
}

pub fn download_file_with_progress(
    url: &str,
    dest: &Path,
    mut progress: Option<&mut dyn FnMut(InstallEvent)>,
) -> Result<()> {
    let emit = |ev: InstallEvent, progress: &mut Option<&mut dyn FnMut(InstallEvent)>| {
        if let Some(cb) = progress {
            cb(ev);
        }
    };

    let resp = ureq::get(url)
        .call()
        .map_err(|e| RlmError::Http(format!("{}: {}", url, e)))?;
    let total: Option<u64> = resp
        .header("Content-Length")
        .or_else(|| resp.header("content-length"))
        .and_then(|v| v.parse().ok());

    let mut reader = resp.into_reader();
    let mut file = std::fs::File::create(dest)?;
    let mut buf = [0u8; 32 * 1024];
    let mut done: u64 = 0;
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        std::io::Write::write_all(&mut file, &buf[..n])?;
        done += n as u64;
        emit(
            InstallEvent::PhaseProgress { phase: PHASE_DOWNLOAD, done, total },
            &mut progress,
        );
    }
    Ok(())
}

pub fn compute_sha256(path: &Path) -> Result<String> {
    compute_sha256_with_progress(path, None)
}

pub fn compute_sha256_with_progress(
    path: &Path,
    mut progress: Option<&mut dyn FnMut(InstallEvent)>,
) -> Result<String> {
    let total: Option<u64> = std::fs::metadata(path).ok().map(|m| m.len());
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    let mut done: u64 = 0;
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
        done += n as u64;
        if let Some(cb) = &mut progress {
            cb(InstallEvent::PhaseProgress { phase: PHASE_CHECKSUM, done, total });
        }
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn verify_sha256(path: &Path, expected: &str) -> Result<()> {
    verify_sha256_with_progress(path, expected, None)
}

pub fn verify_sha256_with_progress(
    path: &Path,
    expected: &str,
    progress: Option<&mut dyn FnMut(InstallEvent)>,
) -> Result<()> {
    let actual = compute_sha256_with_progress(path, progress)?;
    if actual == expected {
        Ok(())
    } else {
        Err(RlmError::ChecksumMismatch {
            expected: expected.to_string(),
            actual,
        })
    }
}

pub fn download_and_verify(url: &str, dest: &Path) -> Result<()> {
    download_and_verify_with_progress(url, dest, None)
}

pub fn download_and_verify_with_progress(
    url: &str,
    dest: &Path,
    mut progress: Option<&mut dyn FnMut(InstallEvent)>,
) -> Result<()> {
    if let Some(cb) = &mut progress {
        cb(InstallEvent::PhaseStart { phase: PHASE_DOWNLOAD });
    }
    download_file_with_progress(url, dest, reborrow(&mut progress))?;
    if let Some(cb) = &mut progress {
        cb(InstallEvent::PhaseDone { phase: PHASE_DOWNLOAD });
    }

    // Try to download checksum file (small; no bar needed)
    let sha_url = format!("{}.sha256", url);
    let sha_path = dest.with_file_name(format!("{}.sha256", dest.file_name().unwrap().to_string_lossy()));
    let mut did_verify = false;
    if download_file(&sha_url, &sha_path).is_ok() {
        let expected = std::fs::read_to_string(&sha_path)?
            .lines()
            .next()
            .unwrap_or("")
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();
        if !expected.is_empty() {
            if let Some(cb) = &mut progress {
                cb(InstallEvent::PhaseStart { phase: PHASE_CHECKSUM });
            }
            verify_sha256_with_progress(dest, &expected, reborrow(&mut progress))?;
            if let Some(cb) = &mut progress {
                cb(InstallEvent::PhaseDone { phase: PHASE_CHECKSUM });
            }
            did_verify = true;
        }
    }
    if !did_verify && let Some(cb) = progress.as_mut() {
        cb(InstallEvent::Message(
            "no checksum file, skipping verification".to_string(),
        ));
    }
    Ok(())
}

pub fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<Vec<PathBuf>> {
    let platform = Platform::detect();
    if platform == Platform::Windows {
        extract_zip(archive_path, dest_dir)
    } else {
        extract_tar_gz(archive_path, dest_dir)
    }
}

fn extract_tar_gz(archive_path: &Path, dest_dir: &Path) -> Result<Vec<PathBuf>> {
    let file = std::fs::File::open(archive_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive.set_overwrite(true);
    archive.unpack(dest_dir).map_err(|e| RlmError::ExtractionFailed(e.to_string()))?;

    // Collect extracted paths
    let mut files = Vec::new();
    let file = std::fs::File::open(archive_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    for entry in archive.entries().map_err(|e| RlmError::ExtractionFailed(e.to_string()))? {
        let entry = entry.map_err(|e| RlmError::ExtractionFailed(e.to_string()))?;
        let path = dest_dir.join(entry.path().map_err(|e| RlmError::ExtractionFailed(e.to_string()))?);
        if entry.header().entry_type() == tar::EntryType::Regular {
            files.push(path);
        }
    }
    Ok(files)
}

fn extract_zip(archive_path: &Path, dest_dir: &Path) -> Result<Vec<PathBuf>> {
    let file = std::fs::File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| RlmError::ExtractionFailed(e.to_string()))?;

    let mut files = Vec::new();
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| RlmError::ExtractionFailed(e.to_string()))?;
        let out_path = dest_dir.join(entry.mangled_name());
        if entry.is_file() {
            let mut out_file = std::fs::File::create(&out_path)?;
            std::io::copy(&mut entry, &mut out_file)?;
            files.push(out_path);
        }
    }
    Ok(files)
}

pub fn install_binary(
    variant: &Variant,
    version: &str,
    platform: Platform,
    arch: Arch,
    install_dir: &Path,
    force: bool,
    mut progress: Option<&mut dyn FnMut(InstallEvent)>,
) -> Result<bool> {
    let actual = variant.actual;
    let binary_name = if platform == Platform::Windows {
        format!("{}.exe", actual)
    } else {
        actual.to_string()
    };

    let msg = |text: String, progress: &mut Option<&mut dyn FnMut(InstallEvent)>| {
        if let Some(cb) = progress {
            cb(InstallEvent::Message(text));
        }
    };

    let target = install_dir.join(&binary_name);
    if target.exists() && !force {
        msg(
            format!("{} already exists, skipping (use --force to overwrite)", actual),
            &mut progress,
        );
        return Ok(true);
    }

    let url = download_url(variant, version, platform, arch);
    let tmp_dir = tempfile::tempdir()?;
    let ext = platform.archive_ext();
    let archive_path = tmp_dir.path().join(format!("{}.{}", actual, ext));

    msg(format!("downloading {} {}...", variant.name, version), &mut progress);

    download_and_verify_with_progress(&url, &archive_path, reborrow(&mut progress))?;

    msg("extracting...".to_string(), &mut progress);
    if let Some(cb) = &mut progress {
        cb(InstallEvent::PhaseStart { phase: PHASE_EXTRACT });
    }

    let extracted = extract_archive(&archive_path, tmp_dir.path())?;

    // Find the binary in extracted files
    let binary = extracted.iter().find(|p| {
        p.file_name().is_some_and(|n| {
            n.to_string_lossy() == binary_name || n.to_string_lossy() == actual
        })
    }).ok_or_else(|| RlmError::ExtractionFailed(format!("binary '{}' not found in archive", binary_name)))?;

    std::fs::create_dir_all(install_dir)?;
    std::fs::copy(binary, &target)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&target)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&target, perms)?;
    }

    if let Some(cb) = &mut progress {
        cb(InstallEvent::PhaseDone { phase: PHASE_EXTRACT });
    }
    msg(format!("installed: {}", target.display()), &mut progress);

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::{Arch, Platform};

    fn variant(actual: &'static str) -> Variant {
        Variant { name: actual, actual, group: "test" }
    }

    #[test]
    fn download_urls_match_release_asset_scheme() {
        // Scheme published by build-variants.sh / _build.yml, e.g.
        // https://github.com/rl-lang/rl-lang/releases/download/v2.2.1/rl-linux-x86_64.tar.gz
        let url = download_url(&variant("rl"), "v2.2.1", Platform::Linux, Arch::X86_64);
        assert_eq!(
            url,
            "https://github.com/rl-lang/rl-lang/releases/download/v2.2.1/rl-linux-x86_64.tar.gz"
        );
        let url = download_url(&variant("rlm"), "v2.2.1", Platform::Windows, Arch::Aarch64);
        assert_eq!(
            url,
            "https://github.com/rl-lang/rl-lang/releases/download/v2.2.1/rlm-windows-aarch64.zip"
        );
        let url = download_url(&variant("rlt"), "nightly", Platform::Macos, Arch::Aarch64);
        assert_eq!(
            url,
            "https://github.com/rl-lang/rl-lang/releases/download/nightly/rlt-macos-aarch64.tar.gz"
        );
    }

    #[test]
    fn all_variants_produce_known_release_assets() {
        for v in crate::variants::all_variants() {
            let url = download_url(&v, "v2.2.1", Platform::Linux, Arch::X86_64);
            let asset = url.rsplit('/').next().unwrap();
            assert!(
                asset.starts_with(v.actual) && asset.ends_with("-linux-x86_64.tar.gz"),
                "unexpected asset name: {}",
                asset
            );
        }
    }

    #[test]
    fn event_ratio_clamps_and_handles_unknown_total() {
        let ev = InstallEvent::PhaseProgress { phase: PHASE_DOWNLOAD, done: 50, total: Some(100) };
        assert_eq!(ev.ratio(), Some(0.5));
        let ev = InstallEvent::PhaseProgress { phase: PHASE_DOWNLOAD, done: 200, total: Some(100) };
        assert_eq!(ev.ratio(), Some(1.0));
        let ev = InstallEvent::PhaseProgress { phase: PHASE_DOWNLOAD, done: 10, total: None };
        assert_eq!(ev.ratio(), None);
        let ev = InstallEvent::PhaseProgress { phase: PHASE_DOWNLOAD, done: 10, total: Some(0) };
        assert_eq!(ev.ratio(), None);
        let ev = InstallEvent::PhaseDone { phase: PHASE_EXTRACT };
        assert_eq!(ev.ratio(), Some(1.0));
    }
}
