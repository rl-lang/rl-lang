use std::io::Read;
use std::path::Path;

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use tar::Archive;

use crate::cache;
use crate::error::PmError;

pub fn download(url: &str, dest: &Path) -> Result<(), PmError> {
    let resp = ureq::get(url)
        .call()
        .map_err(|e| PmError::Http(format!("{}: {}", url, e)))?;

    let mut reader = resp.into_reader();
    let mut file = std::fs::File::create(dest)?;
    std::io::copy(&mut reader, &mut file)?;
    Ok(())
}

pub fn verify_sha256(file: &Path, expected: &str) -> Result<bool, PmError> {
    let mut file = std::fs::File::open(file)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let actual = format!("{:x}", hasher.finalize());
    Ok(actual == expected)
}

pub fn compute_sha256(file: &Path) -> Result<String, PmError> {
    let mut file = std::fs::File::open(file)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn extract_tarball(tarball: &Path, dest: &Path) -> Result<(), PmError> {
    let file = std::fs::File::open(tarball)
        .map_err(|e| PmError::ExtractionFailed(format!("cannot open tarball: {}", e)))?;
    let dec = GzDecoder::new(file);
    let mut archive = Archive::new(dec);

    archive
        .unpack(dest)
        .map_err(|e| PmError::ExtractionFailed(format!("unpack failed: {}", e)))?;

    Ok(())
}

pub fn fetch_and_extract(
    name: &str,
    url: &str,
    expected_sha256: Option<&str>,
) -> Result<String, PmError> {
    cache::ensure_dirs()?;

    let tarballs = cache::tarballs_dir();
    let extracted = cache::extracted_dir();

    let tarball_name = format!("{}.tar.gz", name);
    let tarball_path = tarballs.join(&tarball_name);

    if !tarball_path.exists() {
        download(url, &tarball_path)?;
    }

    if let Some(expected) = expected_sha256
        && !verify_sha256(&tarball_path, expected)?
    {
        let actual = compute_sha256(&tarball_path)?;
        std::fs::remove_file(&tarball_path).ok();
        return Err(PmError::Sha256Mismatch {
            expected: expected.to_string(),
            actual,
        });
    }

    let sha256_path = tarballs.join(format!("{}.sha256", tarball_name));
    if sha256_path.exists() {
        std::fs::remove_file(&sha256_path).ok();
    }
    if let Some(expected) = expected_sha256 {
        std::fs::write(&sha256_path, expected)?;
    }

    let dest = extracted.join(name);
    if dest.exists() {
        std::fs::remove_dir_all(&dest)?;
    }
    extract_tarball(&tarball_path, &extracted)?;

    let lib_path = dest.join("lib.rl");
    if !lib_path.exists() {
        return Err(PmError::MissingLib(name.to_string()));
    }

    Ok(name.to_string())
}
