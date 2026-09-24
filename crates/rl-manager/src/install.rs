use std::io::Read;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};

use crate::error::{RlmError, Result};
use crate::platform::{Arch, Platform};
use crate::variants::{Variant, REPO};

pub fn download_url(variant: &Variant, version: &str, platform: Platform, arch: Arch) -> String {
    let ext = platform.archive_ext();
    let asset = format!("{}-{}-{}.{}", variant.actual, platform.as_str(), arch.as_str(), ext);
    format!("https://github.com/{}/releases/download/{}/{}", REPO, version, asset)
}

pub fn download_file(url: &str, dest: &Path) -> Result<()> {
    let resp = ureq::get(url)
        .call()
        .map_err(|e| RlmError::Http(format!("{}: {}", url, e)))?;
    let mut reader = resp.into_reader();
    let mut file = std::fs::File::create(dest)?;
    std::io::copy(&mut reader, &mut file)?;
    Ok(())
}

pub fn compute_sha256(path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn verify_sha256(path: &Path, expected: &str) -> Result<()> {
    let actual = compute_sha256(path)?;
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
    download_file(url, dest)?;

    // Try to download checksum file
    let sha_url = format!("{}.sha256", url);
    let sha_path = dest.with_file_name(format!("{}.sha256", dest.file_name().unwrap().to_string_lossy()));
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
            verify_sha256(dest, &expected)?;
        }
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
    mut progress: Option<&mut dyn FnMut(&str)>,
) -> Result<bool> {
    let actual = variant.actual;
    let binary_name = if platform == Platform::Windows {
        format!("{}.exe", actual)
    } else {
        actual.to_string()
    };

    let target = install_dir.join(&binary_name);
    if target.exists() && !force {
        if let Some(ref mut cb) = progress {
            cb(&format!("{} already exists, skipping (use --force to overwrite)", actual));
        }
        return Ok(true);
    }

    let url = download_url(variant, version, platform, arch);
    let tmp_dir = tempfile::tempdir()?;
    let ext = platform.archive_ext();
    let archive_path = tmp_dir.path().join(format!("{}.{}", actual, ext));

    if let Some(ref mut cb) = progress {
        cb(&format!("downloading {} {}...", variant.name, version));
    }

    download_and_verify(&url, &archive_path)?;

    if let Some(ref mut cb) = progress {
        cb("extracting...");
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

    if let Some(ref mut cb) = progress {
        cb(&format!("installed: {}", target.display()));
    }

    Ok(true)
}
