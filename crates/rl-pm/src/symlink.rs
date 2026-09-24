use std::path::Path;

use crate::cache;
use crate::error::PmError;

pub fn create_link(name: &str) -> Result<(), PmError> {
    let deps_dir = std::env::current_dir()?.join("deps");
    std::fs::create_dir_all(&deps_dir)?;

    let link_path = deps_dir.join(name);
    let target = cache::extracted_dir().join(name);

    if link_path.exists() || link_path.symlink_metadata().is_ok() {
        std::fs::remove_file(&link_path).or_else(|_| std::fs::remove_dir_all(&link_path))?;
    }

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&target, &link_path)?;
        Ok(())
    }

    #[cfg(windows)]
    {
        match std::os::windows::fs::symlink_dir(&target, &link_path) {
            Ok(()) => return Ok(()),
            Err(_) => {
                eprintln!(
                    "warning: symlink failed (enable developer mode or run as admin), copying instead"
                );
                copy_dir_recursive(&target, &link_path)?;
                return Ok(());
            }
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        copy_dir_recursive(&target, &link_path)?;
        Ok(())
    }
}

pub fn remove_link(name: &str) -> Result<(), PmError> {
    let deps_dir = std::env::current_dir()?.join("deps");
    let link_path = deps_dir.join(name);

    if !link_path.exists() {
        return Err(PmError::NotFound(format!(
            "dependency '{}' not found in deps/",
            name
        )));
    }

    if link_path.symlink_metadata().is_ok() {
        let is_dir = link_path.is_dir();
        if is_dir {
            std::fs::remove_dir_all(&link_path)?;
        } else {
            std::fs::remove_file(&link_path)?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dest)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            std::fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}
