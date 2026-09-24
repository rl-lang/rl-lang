//! `std::path` - path manipulation using [`std::path::PathBuf`].
//!
//! All functions take path strings and return strings or booleans.
//! `path_pop` removes the last component (returns the shortened path).
//! `path_push` appends a component (same as `path_join`).
//!
//! Ported once from the former per-runtime `stdlib/path/*.rs` copies. The
//! four "component" queries (`path_extension`/`path_filename`/`path_parent`/
//! `path_stem`) return either a string or `null`, so they build a raw
//! `R::Value` and carry an explicit `(string) -> string` signature.

use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- predicates: `(string) -> bool` ---------------------------------------

#[native_fn(module = "path")]
pub fn path_is_absolute(path: String) -> bool {
    std::path::Path::new(&path).is_absolute()
}

#[native_fn(module = "path")]
pub fn path_is_relative(path: String) -> bool {
    std::path::Path::new(&path).is_relative()
}

#[native_fn(module = "path")]
pub fn path_starts_with(path: String, base: String) -> bool {
    std::path::Path::new(&path).starts_with(&base)
}

#[native_fn(module = "path")]
pub fn path_ends_with(path: String, child: String) -> bool {
    std::path::Path::new(&path).ends_with(&child)
}

// ---- component queries: `(string) -> string` (or `null`) ------------------

#[native_fn(module = "path", sig(string -> string))]
pub fn path_extension<R: Runtime>(path: String) -> R::Value {
    match std::path::Path::new(&path).extension() {
        Some(ext) => R::from_string(ext.to_string_lossy().to_string()),
        None => R::null(),
    }
}

#[native_fn(module = "path", sig(string -> string))]
pub fn path_filename<R: Runtime>(path: String) -> R::Value {
    match std::path::Path::new(&path).file_name() {
        Some(name) => R::from_string(name.to_string_lossy().to_string()),
        None => R::null(),
    }
}

#[native_fn(module = "path", sig(string -> string))]
pub fn path_parent<R: Runtime>(path: String) -> R::Value {
    match std::path::Path::new(&path).parent() {
        Some(p) => R::from_string(p.to_string_lossy().to_string()),
        None => R::null(),
    }
}

#[native_fn(module = "path", sig(string -> string))]
pub fn path_stem<R: Runtime>(path: String) -> R::Value {
    match std::path::Path::new(&path).file_stem() {
        Some(stem) => R::from_string(stem.to_string_lossy().to_string()),
        None => R::null(),
    }
}

// ---- `(string) -> string` (always) ----------------------------------------

#[native_fn(module = "path")]
pub fn path_pop(path: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.pop();
    buf.to_string_lossy().to_string()
}

#[native_fn(module = "path")]
pub fn path_normalize(path: String) -> String {
    let p = std::path::Path::new(&path);
    let starts_with_dotdot = path.starts_with("..");
    let components: Vec<_> = p.components().collect();

    if components.is_empty() {
        return path;
    }

    let mut result = std::path::PathBuf::new();
    for comp in &components {
        result.push(comp);
    }

    let mut out = result.to_string_lossy().to_string();

    if starts_with_dotdot && !out.starts_with("..") {
        out = format!("..{}", std::path::MAIN_SEPARATOR_STR) + &out;
    }

    if path.starts_with('/') && !out.starts_with('/') {
        out = format!("/{}", out);
    }

    out
}

// ---- `(string) -> array[string]` ------------------------------------------

#[native_fn(module = "path")]
pub fn path_split(path: String) -> Vec<String> {
    let p = std::path::Path::new(&path);
    let parent = p.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
    let file = p.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    vec![parent, file]
}

#[native_fn(module = "path")]
pub fn path_split_extension(path: String) -> Vec<String> {
    let p = std::path::Path::new(&path);
    let stem = p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let ext = p.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    vec![stem, ext]
}

#[native_fn(module = "path")]
pub fn path_components(path: String) -> Vec<String> {
    std::path::Path::new(&path)
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect()
}

// ---- `(string, string) -> string` -----------------------------------------

#[native_fn(module = "path")]
pub fn path_join(path: String, target: String) -> String {
    std::path::PathBuf::from(&path)
        .join(&target)
        .to_string_lossy()
        .to_string()
}

#[native_fn(module = "path")]
pub fn path_push(path: String, target: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.push(&target);
    buf.to_string_lossy().to_string()
}

#[native_fn(module = "path")]
pub fn path_set_extension(path: String, target: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.set_extension(&target);
    buf.to_string_lossy().to_string()
}

#[native_fn(module = "path")]
pub fn path_with_file_name(path: String, name: String) -> String {
    std::path::PathBuf::from(&path)
        .with_file_name(&name)
        .to_string_lossy()
        .to_string()
}

// ---- `(string, ...string) -> string` --------------------------------------

#[native_fn(module = "path")]
pub fn path_join_many(parts: Vec<String>) -> String {
    let mut buf = std::path::PathBuf::new();
    for part in &parts {
        buf.push(part);
    }
    buf.to_string_lossy().to_string()
}

rl_std_core::native_module!("path";
    funcs: [
        path_is_absolute, path_is_relative,
        path_starts_with, path_ends_with,
        path_extension, path_filename, path_parent, path_stem,
        path_pop,
        path_normalize,
        path_split, path_split_extension, path_components,
        path_join, path_push, path_set_extension,
        path_with_file_name,
        path_join_many,
    ],
);
