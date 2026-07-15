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

use std::collections::HashSet;
use std::io::Write;
use std::path::{Path, PathBuf};

const MAGIC_SOURCE: &[u8] = b"\x00RL_PACKAGE_V1\x00";
const MAGIC_VM: &[u8] = b"\x00RL_PACKAGE_V2\x00";
const MAGIC_LEN: usize = 15;
const FOOTER_LEN: usize = MAGIC_LEN + 8;

pub enum EmbeddedProgram {
    Source(String),
    Bytecode(Vec<u8>),
}

/// Searches the running binary's own bytes for an embedded rl program.
///
/// Returns Some(source) if this binary was produced by rl package,
/// or None if it is a plain rl binary.
pub fn find_embedded() -> Option<EmbeddedProgram> {
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
    let magic = &bytes[magic_start..len_start];

    // payload sits just before the magic
    let payload_start = magic_start.checked_sub(len)?;
    let payload_end = magic_start;
    let payload = &bytes[payload_start..payload_end];

    if magic == MAGIC_SOURCE {
        String::from_utf8(payload.to_vec())
            .ok()
            .map(EmbeddedProgram::Source)
    } else if magic == MAGIC_VM {
        Some(EmbeddedProgram::Bytecode(payload.to_vec()))
    } else {
        None
    }
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

pub fn package_vm(bytecode: &[u8], output_path: &str) {
    if let Err(e) = try_package_payload(bytecode, MAGIC_VM, output_path) {
        eprintln!("error: packaging failed: {}", e);
        std::process::exit(1);
    }
    println!("packaged (vm) -> '{}'", output_path);
}

fn try_package(source_path: &str, output_path: &str) -> std::io::Result<()> {
    let source = bundle(source_path)?;
    try_package_payload(source.as_bytes(), MAGIC_SOURCE, output_path)
}

fn try_package_payload(payload: &[u8], magic: &[u8], output_path: &str) -> std::io::Result<()> {
    let self_bytes = std::fs::read(std::env::current_exe()?)?;

    let mut out = std::fs::File::create(output_path)?;
    out.write_all(&self_bytes)?;
    out.write_all(payload)?;
    out.write_all(magic)?;
    out.write_all(&(payload.len() as u64).to_le_bytes())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(output_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(output_path, perms)?;
    }

    Ok(())
}

fn bundle(entry_path: &str) -> std::io::Result<String> {
    let mut visited = HashSet::new();
    bundle_inner(Path::new(entry_path), &mut visited)
}

fn bundle_inner(path: &Path, visited: &mut HashSet<PathBuf>) -> std::io::Result<String> {
    let canonical = path.canonicalize()?;
    if visited.contains(&canonical) {
        return Ok(String::new());
    }
    visited.insert(canonical.clone());

    let source = std::fs::read_to_string(path)?;
    let base_dir = path.parent().unwrap_or(Path::new("."));
    let mut output = String::new();

    for line in source.lines() {
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("get ") {
            // extract the file name from either form:
            // "get csv"  ->  file_part = "csv"
            // "get x, y from csv"  ->  file_part = "csv"
            let file_part = if let Some(from_idx) = rest.find(" from ") {
                rest[from_idx + 6..].trim()
            } else {
                rest.trim()
            };

            // only inline local files, not std::
            if !file_part.contains("::") {
                let rel: PathBuf = file_part.split('/').collect();
                let file_path = base_dir.join(rel).with_extension("rl");
                match bundle_inner(&file_path, visited) {
                    Ok(inlined) => {
                        output.push_str(&inlined);
                        output.push('\n');
                        continue;
                    }
                    Err(e) => {
                        eprintln!("warning: could not bundle '{}': {}", file_path.display(), e);
                    }
                }
            }
        }

        output.push_str(line);
        output.push('\n');
    }

    Ok(output)
}
