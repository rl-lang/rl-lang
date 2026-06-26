//! rl package - bundle a .rl program into a self-contained binary.
//!
//! Copies the rl binary itself, then appends the source text and a
//! magic footer so the copy can detect and run the embedded program at
//! startup without any arguments.
//!
//! # Binary layout after packaging
//!
//! [ original rl binary bytes ]
//! [ rl source text (UTF-8)   ]
//! [ magic marker: \x00RL_PACKAGE_V1\x00 ]
//! [ source length: u64 little-endian    ]
//!
//!
//! The magic + length are appended *after* the source so detection only
//! needs to read the last few bytes — no full scan required.

use std::io::Write;

const MAGIC: &[u8] = b"\x00RL_PACKAGE_V1\x00";
const MAGIC_LEN: usize = 16; // b"\x00RL_PACKAGE_V1\x00".len()
const FOOTER_LEN: usize = MAGIC_LEN + 8; // magic + u64

/// Searches the running binary's own bytes for an embedded rl program.
///
/// Returns Some(source) if this binary was produced by rl package,
/// or None if it is a plain rl binary.
pub fn find_embedded() -> Option<String> {
    let path = std::env::current_exe().ok()?;
    let bytes = std::fs::read(path).ok()?;

    if bytes.len() < FOOTER_LEN {
        return None;
    }

    // read length from the final 8 bytes
    let len_start = bytes.len() - 8;
    let len = u64::from_le_bytes(bytes[len_start..].try_into().ok()?) as usize;

    // verify magic sits just before the length
    let magic_start = len_start.checked_sub(MAGIC_LEN)?;
    if &bytes[magic_start..len_start] != MAGIC {
        return None;
    }

    // source sits just before the magic
    let source_start = magic_start.checked_sub(len)?;
    let source_end = magic_start;

    String::from_utf8(bytes[source_start..source_end].to_vec()).ok()
}

/// Packages source_path into a self-contained binary at output_path.
///
/// Copies the running rl binary verbatim, then appends the rl source
/// and the magic footer. The output is made executable on Unix.
///
/// Prints an error and exits with code 1 on any failure.
pub fn package(source_path: &str, output_path: &str) {
    if let Err(e) = try_package(source_path, output_path) {
        eprintln!("error: packaging failed: {}", e);
        std::process::exit(1);
    }
    println!("packaged '{}' -> '{}'", source_path, output_path);
}

fn try_package(source_path: &str, output_path: &str) -> std::io::Result<()> {
    let source = std::fs::read_to_string(source_path)?;
    let self_bytes = std::fs::read(std::env::current_exe()?)?;

    let mut out = std::fs::File::create(output_path)?;
    out.write_all(&self_bytes)?;
    out.write_all(source.as_bytes())?;
    out.write_all(MAGIC)?;
    out.write_all(&(source.len() as u64).to_le_bytes())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(output_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(output_path, perms)?;
    }

    Ok(())
}
