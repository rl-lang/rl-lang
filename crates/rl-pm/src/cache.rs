use std::path::PathBuf;

fn cache_root() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rlpm")
}

pub fn tarballs_dir() -> PathBuf {
    cache_root().join("tarballs")
}

pub fn extracted_dir() -> PathBuf {
    cache_root().join("extracted")
}

pub fn ensure_dirs() -> std::io::Result<()> {
    std::fs::create_dir_all(tarballs_dir())?;
    std::fs::create_dir_all(extracted_dir())?;
    Ok(())
}

pub fn clean() -> std::io::Result<()> {
    let root = cache_root();
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    Ok(())
}
