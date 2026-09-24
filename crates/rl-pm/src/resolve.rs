use std::path::{Path, PathBuf};

/// Given a base directory and import path segments, resolve the file path.
///
/// For `get fn from csv` with path `["csv"]`:
///   - First checks `{base}/csv.rl` (local file, existing behavior)
///   - Then checks `{base}/deps/csv/lib.rl` (dependency)
///
/// For `get fn from csv::utils` with path `["csv", "utils"]`:
///   - First checks `{base}/csv/utils.rl` (local file)
///   - Then checks `{base}/deps/csv/utils.rl` (inside dependency)
pub fn resolve_import_path(base_dir: &Path, path: &[String]) -> Option<PathBuf> {
    if path.is_empty() {
        return None;
    }

    let direct = base_dir.join(path.join("/")).with_extension("rl");
    if direct.exists() {
        return Some(direct);
    }

    let first = &path[0];
    let dep_dir = base_dir.join("deps").join(first);
    if dep_dir.exists() {
        if path.len() == 1 {
            let lib = dep_dir.join("lib.rl");
            if lib.exists() {
                return Some(lib);
            }
        } else {
            let rest: PathBuf = path[1..].iter().collect();
            let inside = dep_dir.join(rest).with_extension("rl");
            if inside.exists() {
                return Some(inside);
            }
        }
    }

    None
}

/// Check if an import path refers to a dependency (first segment matches deps/).
pub fn is_dep_import(base_dir: &Path, path: &[String]) -> bool {
    if path.is_empty() {
        return false;
    }
    let first = &path[0];
    let dep_dir = base_dir.join("deps").join(first);
    dep_dir.exists()
}

/// Resolve the directory for a dependency's root (deps/{name}/).
pub fn dep_dir(base_dir: &Path, name: &str) -> PathBuf {
    base_dir.join("deps").join(name)
}
