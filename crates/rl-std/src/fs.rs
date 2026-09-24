//! `std::fs` - filesystem operations: create, remove, list, copy, move,
//! metadata, file reading/writing, and handle-based streaming I/O.
//!
//! All errors include the path and the underlying OS error message. Every
//! fallible function returns a language `result[T]` (an `Ok`/`Err` value):
//! `mkdir`/`mkdir_all`/`rmdir`/`rmdir_all`/`move_file` yield `result[null]`,
//! `copy_file`/`file_size`/`file_modified` yield `result[int]`, `list_dir`
//! yields `result[array[string]]`, and `rename_file` yields `result[string]`.
//! `temp_dir` takes no arguments and returns a plain string. Ported once from
//! the former per-runtime `stdlib/fs/*.rs` copies.

#[cfg(feature = "impls")]
use std::io::{BufRead, Read, Seek, Write};
#[cfg(feature = "impls")]
use crate::io::{IoFileHandle, IoStore, insert_handle, extract_handle};
#[cfg(not(feature = "impls"))]
use crate::io::{IoFileHandle, IoStore};
use rl_std_macros::native_fn;

// ---- directory creation / removal (language `result[null]`) ---------------

#[native_fn(module = "fs")]
pub fn mkdir(path: String) -> Result<(), String> {
    if let Err(e) = std::fs::create_dir(&path) {
        return Err(format!("mkdir: failed to create \"{}\": {}", path, e));
    };
    Ok(())
}

#[native_fn(module = "fs")]
pub fn mkdir_all(path: String) -> Result<(), String> {
    if let Err(e) = std::fs::create_dir_all(&path) {
        return Err(format!("mkdir_all: failed to create \"{}\": {}", path, e));
    };
    Ok(())
}

#[native_fn(module = "fs")]
pub fn rmdir(path: String) -> Result<(), String> {
    if let Err(e) = std::fs::remove_dir(&path) {
        return Err(format!("rmdir: failed to delete \"{}\": {}", path, e));
    };
    Ok(())
}

#[native_fn(module = "fs")]
pub fn rmdir_all(path: String) -> Result<(), String> {
    if let Err(e) = std::fs::remove_dir_all(&path) {
        return Err(format!("rmdir_all: failed to delete \"{}\": {}", path, e));
    };
    Ok(())
}

// ---- listing (language `result[array[string]]`) ---------------------------

#[native_fn(module = "fs")]
pub fn list_dir(path: String) -> Result<Vec<String>, String> {
    match std::fs::read_dir(&path) {
        Err(e) => Err(format!("list_dir: failed to read \"{}\": {}", path, e)),
        Ok(d) => Ok(d
            .filter_map(|i| i.ok())
            .map(|i| i.path().to_string_lossy().to_string())
            .collect::<Vec<String>>()),
    }
}

#[native_fn(module = "fs")]
pub fn list_dir_names(path: String) -> Result<Vec<String>, String> {
    match std::fs::read_dir(&path) {
        Err(e) => Err(format!(
            "list_dir_names: failed to read \"{}\": {}",
            path, e
        )),
        Ok(d) => Ok(d
            .filter_map(|i| i.ok())
            .map(|i| {
                i.file_name()
                    .to_string_lossy()
                    .to_string()
            })
            .collect::<Vec<String>>()),
    }
}

// ---- copy / move ----------------------------------------------------------

#[native_fn(module = "fs")]
pub fn copy_file(src: String, dst: String) -> Result<i64, String> {
    let bytes = match std::fs::copy(&src, &dst) {
        Ok(b) => b,
        Err(e) => {
            return Err(format!(
                "copy_file: failed to copy \"{}\" to \"{}\": {}",
                src, dst, e
            ));
        }
    };
    Ok(bytes as i64)
}

#[native_fn(module = "fs")]
pub fn move_file(src: String, dst: String) -> Result<(), String> {
    if let Err(e) = std::fs::rename(&src, &dst) {
        return Err(format!(
            "move_file: failed to move \"{}\" to \"{}\": {}",
            src, dst, e
        ));
    };
    Ok(())
}

// ---- metadata (language `result[int]`) ------------------------------------

#[native_fn(module = "fs")]
pub fn file_size(path: String) -> Result<i64, String> {
    match std::fs::metadata(&path) {
        Err(e) => Err(format!("file_size: failed to read \"{}\": {}", path, e)),
        Ok(metadata) => Ok(metadata.len() as i64),
    }
}

#[native_fn(module = "fs")]
pub fn file_modified(path: String) -> Result<i64, String> {
    match std::fs::metadata(&path) {
        Err(e) => Err(format!("file_modified: failed to read \"{}\": {}", path, e)),

        Ok(metadata) => match metadata.modified() {
            Err(e) => Err(format!(
                "file_modified: could not get modification time for \"{}\": {}",
                path, e
            )),
            Ok(modified) => match modified.duration_since(std::time::UNIX_EPOCH) {
                Err(e) => Err(format!(
                    "file_modified: modification time before epoch for \"{}\": {}",
                    path, e
                )),
                Ok(t) => Ok(t.as_secs() as i64),
            },
        },
    }
}

#[native_fn(module = "fs")]
pub fn file_created(path: String) -> Result<i64, String> {
    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => return Err(format!("file_created: failed to read \"{}\": {}", path, e)),
    };
    match metadata.created() {
        Err(e) => Err(format!(
            "file_created: creation time not available for \"{}\": {}",
            path, e
        )),
        Ok(t) => match t.duration_since(std::time::UNIX_EPOCH) {
            Err(e) => Err(format!(
                "file_created: creation time before epoch for \"{}\": {}",
                path, e
            )),
            Ok(d) => Ok(d.as_secs() as i64),
        },
    }
}

#[native_fn(module = "fs")]
pub fn file_accessed(path: String) -> Result<i64, String> {
    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "file_accessed: failed to read \"{}\": {}",
                path, e
            ))
        }
    };
    match metadata.accessed() {
        Err(e) => Err(format!(
            "file_accessed: access time not available for \"{}\": {}",
            path, e
        )),
        Ok(t) => match t.duration_since(std::time::UNIX_EPOCH) {
            Err(e) => Err(format!(
                "file_accessed: access time before epoch for \"{}\": {}",
                path, e
            )),
            Ok(d) => Ok(d.as_secs() as i64),
        },
    }
}

// ---- permissions ----------------------------------------------------------

#[native_fn(module = "fs")]
pub fn file_permissions(path: String) -> Result<i64, String> {
    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "file_permissions: failed to read \"{}\": {}",
                path, e
            ))
        }
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        Ok(metadata.permissions().mode() as i64)
    }
    #[cfg(not(unix))]
    {
        Ok(if metadata.permissions().readonly() { 0o444 } else { 0o644 })
    }
}

#[native_fn(module = "fs")]
pub fn set_permissions(path: String, mode: i64) -> Result<(), String> {
    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "set_permissions: failed to read \"{}\": {}",
                path, e
            ))
        }
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = metadata.permissions();
        perms.set_mode(mode as u32);
        match std::fs::set_permissions(&path, perms) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "set_permissions: failed to set permissions on \"{}\": {}",
                path, e
            )),
        }
    }
    #[cfg(not(unix))]
    {
        let _ = (metadata, mode);
        Err("set_permissions: not supported on this platform".to_string())
    }
}

// ---- temp dir / file ------------------------------------------------------

#[native_fn(module = "fs")]
pub fn temp_dir() -> String {
    std::env::temp_dir().to_string_lossy().to_string()
}

#[native_fn(module = "fs")]
pub fn temp_file() -> Result<String, String> {
    match tempfile::NamedTempFile::new() {
        Ok(f) => Ok(f.into_temp_path().to_string_lossy().to_string()),
        Err(e) => Err(format!("temp_file: {}", e)),
    }
}

#[native_fn(module = "fs")]
pub fn temp_file_in(dir: String) -> Result<String, String> {
    match tempfile::Builder::new().tempfile_in(&dir) {
        Ok(f) => Ok(f.into_temp_path().to_string_lossy().to_string()),
        Err(e) => Err(format!("temp_file_in: {}", e)),
    }
}

// ---- rename (language `result[string]`) -----------------------------------

#[native_fn(module = "fs")]
pub fn rename_file(path: String, new_name: String) -> Result<String, String> {
    let old_path = std::path::Path::new(&path);
    let new_path = match old_path.parent() {
        Some(parent) => parent.join(&new_name),
        None => std::path::PathBuf::from(&new_name),
    };

    if let Err(e) = std::fs::rename(old_path, &new_path) {
        return Err(format!(
            "rename_file(): failed to rename \"{}\" to \"{}\": {}",
            path,
            new_path.to_string_lossy(),
            e
        ));
    };

    Ok(new_path.to_string_lossy().to_string())
}

// ---- touch / truncate -----------------------------------------------------

#[native_fn(module = "fs")]
pub fn touch(path: String) -> Result<(), String> {
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("touch: failed to create \"{}\": {}", path, e)),
    }
}

#[native_fn(module = "fs")]
pub fn truncate_file(path: String, len: i64) -> Result<(), String> {
    match std::fs::File::open(&path) {
        Ok(f) => match f.set_len(len.max(0) as u64) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "truncate_file: failed to truncate \"{}\": {}",
                path, e
            )),
        },
        Err(e) => Err(format!(
            "truncate_file: failed to open \"{}\": {}",
            path, e
        )),
    }
}

// ---- glob -----------------------------------------------------------------

#[native_fn(module = "fs")]
pub fn glob(pattern: String) -> Result<Vec<String>, String> {
    match ::glob::glob(&pattern) {
        Ok(paths) => Ok(paths
            .filter_map(|p| p.ok())
            .map(|p| p.to_string_lossy().to_string())
            .collect()),
        Err(e) => Err(format!("glob: invalid pattern \"{}\": {}", pattern, e)),
    }
}

// ---- walk -----------------------------------------------------------------

#[native_fn(module = "fs")]
pub fn walk_dir(path: String) -> Result<Vec<String>, String> {
    let mut results = Vec::new();
    let mut stack = vec![std::path::PathBuf::from(&path)];

    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                return Err(format!(
                    "walk_dir: failed to read \"{}\": {}",
                    dir.to_string_lossy(),
                    e
                ))
            }
        };

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path.clone());
            }
            results.push(path.to_string_lossy().to_string());
        }
    }

    Ok(results)
}

// ---- symlinks -------------------------------------------------------------

#[native_fn(module = "fs")]
pub fn symlink(src: String, dst: String) -> Result<(), String> {
    #[cfg(unix)]
    {
        match std::os::unix::fs::symlink(&src, &dst) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "symlink: failed to create symlink from \"{}\" to \"{}\": {}",
                src, dst, e
            )),
        }
    }
    #[cfg(not(unix))]
    {
        let _ = (src, dst);
        Err("symlink: not supported on this platform".to_string())
    }
}

#[native_fn(module = "fs")]
pub fn readlink(path: String) -> Result<String, String> {
    match std::fs::read_link(&path) {
        Ok(target) => Ok(target.to_string_lossy().to_string()),
        Err(e) => Err(format!(
            "readlink: failed to read symlink \"{}\": {}",
            path, e
        )),
    }
}

#[native_fn(module = "fs")]
pub fn hardlink(src: String, dst: String) -> Result<(), String> {
    match std::fs::hard_link(&src, &dst) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "hardlink: failed to create hard link from \"{}\" to \"{}\": {}",
            src, dst, e
        )),
    }
}

// ---- realpath -------------------------------------------------------------

#[native_fn(module = "fs")]
pub fn realpath(path: String) -> Result<String, String> {
    match std::fs::canonicalize(&path) {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => Err(format!("realpath: failed to resolve \"{}\": {}", path, e)),
    }
}

// ---- lock / unlock --------------------------------------------------------

#[native_fn(module = "fs")]
pub fn lock_file(path: String) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let file = match std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&path)
        {
            Ok(f) => f,
            Err(e) => return Err(format!("lock_file: failed to open \"{}\": {}", path, e)),
        };
        let ret = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) };
        if ret == 0 {
            std::mem::forget(file);
            Ok(())
        } else {
            Err(format!("lock_file: failed to lock \"{}\"", path))
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Err("lock_file: not supported on this platform".to_string())
    }
}

#[native_fn(module = "fs")]
pub fn unlock_file(path: String) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let file = match std::fs::OpenOptions::new()
            .write(true)
            .open(&path)
        {
            Ok(f) => f,
            Err(e) => {
                return Err(format!(
                    "unlock_file: failed to open \"{}\": {}",
                    path, e
                ))
            }
        };
        let ret = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
        if ret == 0 {
            Ok(())
        } else {
            Err(format!("unlock_file: failed to unlock \"{}\"", path))
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Err("unlock_file: not supported on this platform".to_string())
    }
}

// ---- file reading (moved from io) -----------------------------------------

#[native_fn(module = "fs")]
pub fn read_file(file: String) -> Result<String, String> {
    match std::fs::read_to_string(&file) {
        Ok(d) => Ok(d),
        Err(e) => Err(format!("read_file: failed to read \"{}\": {}", file, e)),
    }
}

#[native_fn(module = "fs")]
pub fn read_lines(file: String) -> Result<Vec<String>, String> {
    match std::fs::read_to_string(&file) {
        Ok(d) => Ok(d.lines().map(String::from).collect()),
        Err(e) => Err(format!("read_lines: failed to read \"{}\": {}", file, e)),
    }
}

#[native_fn(module = "fs")]
pub fn read_bytes(file: String) -> Result<Vec<u8>, String> {
    match std::fs::read(&file) {
        Ok(d) => Ok(d),
        Err(e) => Err(format!("read_bytes: failed to read \"{}\": {}", file, e)),
    }
}

// ---- file writing (moved from io) -----------------------------------------

#[native_fn(module = "fs")]
pub fn write_file(file: String, content: String) -> Result<(), String> {
    match std::fs::write(&file, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("write_file: failed to write \"{}\": {}", file, e)),
    }
}

#[native_fn(module = "fs")]
pub fn append_file(file: String, content: String) -> Result<(), String> {
    let mut file_data = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file)
    {
        Ok(fd) => fd,
        Err(e) => {
            return Err(format!("append_file: failed to open \"{}\": {}", file, e));
        }
    };

    match std::io::Write::write_all(&mut file_data, content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("append_file: failed to append \"{}\": {}", file, e)),
    }
}

#[native_fn(module = "fs")]
pub fn delete_file(file: String) -> Result<(), String> {
    match std::fs::remove_file(&file) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("delete_file: failed to read \"{}\": {}", file, e)),
    }
}

// ---- path predicates moved from path --------------------------------------

#[native_fn(module = "fs")]
pub fn path_exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
}

#[native_fn(module = "fs")]
pub fn path_is_dir(path: String) -> bool {
    std::path::Path::new(&path).is_dir()
}

#[native_fn(module = "fs")]
pub fn path_is_file(path: String) -> bool {
    std::path::Path::new(&path).is_file()
}

#[native_fn(module = "fs")]
pub fn path_canonicalize(path: String) -> Result<String, String> {
    match std::path::Path::new(&path).canonicalize() {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => Err(format!("path_canonicalize: {}: {}", path, e)),
    }
}

#[native_fn(module = "fs")]
pub fn path_absolute(path: String) -> Result<String, String> {
    if std::path::Path::new(&path).is_absolute() {
        return Ok(path);
    }
    match std::env::current_dir() {
        Ok(cwd) => Ok(cwd.join(&path).to_string_lossy().to_string()),
        Err(e) => Err(format!("path_absolute: {}", e)),
    }
}

#[native_fn(module = "fs")]
pub fn path_expand_home(path: String) -> String {
    if path.starts_with('~') {
        #[cfg(unix)]
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen('~', &home, 1);
        }
        #[cfg(windows)]
        if let Ok(home) = std::env::var("USERPROFILE") {
            return path.replacen('~', &home, 1);
        }
    }
    path
}

// ---- path ops with syscalls (moved from path) -----------------------------

#[native_fn(module = "fs")]
pub fn path_relative(from: String, to: String) -> Result<String, String> {
    let from_path = std::path::Path::new(&from);
    let to_path = std::path::Path::new(&to);

    let from_abs = if from_path.is_absolute() {
        from_path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(from_path),
            Err(e) => return Err(format!("path_relative: {}", e)),
        }
    };

    let to_abs = if to_path.is_absolute() {
        to_path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(to_path),
            Err(e) => return Err(format!("path_relative: {}", e)),
        }
    };

    let from_components: Vec<_> = from_abs.components().collect();
    let to_components: Vec<_> = to_abs.components().collect();

    let mut i = 0;
    while i < from_components.len() && i < to_components.len() && from_components[i] == to_components[i] {
        i += 1;
    }

    let mut result = std::path::PathBuf::new();
    for _ in i..from_components.len() {
        result.push("..");
    }
    for comp in &to_components[i..] {
        result.push(comp);
    }

    Ok(result.to_string_lossy().to_string())
}

// ---- handle-based I/O (moved from io) -------------------------------------

#[native_fn(module = "fs", bound = "IoStore",
    sig(string, string -> result[handle(File)]))]
pub fn open<R: IoStore>(cx: &mut R::Cx, file: R::Value, mode: R::Value) -> R::Value {
    let file_str = match R::as_str(&file) {
        Some(s) => s.to_owned(),
        None => {
            return R::err(R::from_string(format!(
                "open: expected string for file, got {}",
                R::type_name(&file)
            )))
        }
    };
    let mode_str = match R::as_str(&mode) {
        Some(s) => s.to_owned(),
        None => {
            return R::err(R::from_string(format!(
                "open: expected string for mode, got {}",
                R::type_name(&mode)
            )))
        }
    };

    let mut opts = std::fs::OpenOptions::new();
    match mode_str.as_str() {
        "r" => { opts.read(true); }
        "w" => { opts.write(true).create(true).truncate(true); }
        "a" => { opts.append(true).create(true); }
        "r+" => { opts.read(true).write(true).create(true); }
        "w+" => { opts.read(true).write(true).create(true).truncate(true); }
        "a+" => { opts.read(true).append(true).create(true); }
        _ => {
            return R::err(R::from_string(format!(
                "open: invalid mode \"{}\" (expected r, w, a, r+, w+, a+)",
                mode_str
            )));
        }
    }

    let file_handle = match opts.open(&file_str) {
        Ok(f) => f,
        Err(e) => {
            return R::err(R::from_string(format!(
                "open: failed to open \"{}\": {}", file_str, e
            )))
        }
    };

    let readable = mode_str.starts_with('r') || mode_str.contains('+');
    let writable = mode_str != "r" || mode_str.contains('+');

    let io_handle = match (readable, writable) {
        (true, true) => {
            let writer = match file_handle.try_clone() {
                Ok(f) => f,
                Err(e) => return R::err(R::from_string(format!("open: {}", e))),
            };
            let reader = std::io::BufReader::new(file_handle);
            IoFileHandle::ReadWrite(reader, writer)
        }
        (true, false) => IoFileHandle::Read(std::io::BufReader::new(file_handle)),
        (false, true) => IoFileHandle::Write(file_handle),
        (false, false) => {
            return R::err(R::from_string("open: mode must allow read or write".to_string()));
        }
    };

    R::ok(insert_handle::<R>(cx, io_handle))
}

#[native_fn(module = "fs", bound = "IoStore",
    sig(handle(File) -> result[null]))]
pub fn close<R: IoStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "close") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    match R::io_remove(cx, id) {
        Some(_) => R::ok(R::null()),
        None => R::err(R::from_string("close: invalid handle".to_string())),
    }
}

#[native_fn(module = "fs", bound = "IoStore",
    sig(handle(File), int -> result[string]))]
pub fn read_handle<R: IoStore>(cx: &mut R::Cx, handle: R::Value, n: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "read") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    let n = match R::as_i64(&n) {
        Some(v) => v,
        None => {
            return R::err(R::from_string(format!(
                "read: expected int for n, got {}",
                R::type_name(&n)
            )))
        }
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("read: invalid handle".to_string())),
    };

    let mut buf = vec![0u8; n.max(0) as usize];
    let bytes_read = match io_file {
        IoFileHandle::Read(r) => r.read(&mut buf),
        IoFileHandle::ReadWrite(r, _) => r.read(&mut buf),
        IoFileHandle::Write(_) => {
            return R::err(R::from_string("read: handle is not open for reading".to_string()))
        }
    };

    match bytes_read {
        Ok(bytes) => {
            buf.truncate(bytes);
            R::ok(R::from_string(String::from_utf8_lossy(&buf).into_owned()))
        }
        Err(e) => R::err(R::from_string(format!("read: {}", e))),
    }
}

#[native_fn(module = "fs", bound = "IoStore",
    sig(handle(File), string -> result[int]))]
pub fn write_handle<R: IoStore>(cx: &mut R::Cx, handle: R::Value, data: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "write") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    let bytes = match R::as_str(&data) {
        Some(s) => s.as_bytes().to_vec(),
        None => {
            return R::err(R::from_string(format!(
                "write: expected string for data, got {}",
                R::type_name(&data)
            )))
        }
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("write: invalid handle".to_string())),
    };

    let result = match io_file {
        IoFileHandle::Write(w) => w.write_all(&bytes),
        IoFileHandle::ReadWrite(_, w) => w.write_all(&bytes),
        IoFileHandle::Read(_) => {
            return R::err(R::from_string("write: handle is not open for writing".to_string()))
        }
    };

    match result {
        Ok(()) => R::ok(R::from_i64(bytes.len() as i64)),
        Err(e) => R::err(R::from_string(format!("write: {}", e))),
    }
}

#[native_fn(module = "fs", bound = "IoStore",
    sig(handle(File), int, int -> result[int]))]
pub fn seek<R: IoStore>(cx: &mut R::Cx, handle: R::Value, offset: R::Value, whence: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "seek") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    let offset = match R::as_i64(&offset) {
        Some(v) => v,
        None => {
            return R::err(R::from_string(format!(
                "seek: expected int for offset, got {}",
                R::type_name(&offset)
            )))
        }
    };
    let whence = match R::as_i64(&whence) {
        Some(v) => v,
        None => {
            return R::err(R::from_string(format!(
                "seek: expected int for whence, got {}",
                R::type_name(&whence)
            )))
        }
    };

    let seek_from = match whence {
        0 => std::io::SeekFrom::Start(offset.max(0) as u64),
        1 => std::io::SeekFrom::Current(offset),
        2 => std::io::SeekFrom::End(offset),
        _ => {
            return R::err(R::from_string(format!(
                "seek: invalid whence {} (expected 0, 1, or 2)",
                whence
            )))
        }
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("seek: invalid handle".to_string())),
    };

    let result = match io_file {
        IoFileHandle::Read(r) => r.seek(seek_from),
        IoFileHandle::ReadWrite(r, _) => r.seek(seek_from),
        IoFileHandle::Write(w) => w.seek(seek_from),
    };

    match result {
        Ok(pos) => R::ok(R::from_i64(pos as i64)),
        Err(e) => R::err(R::from_string(format!("seek: {}", e))),
    }
}

#[native_fn(module = "fs", bound = "IoStore",
    sig(handle(File) -> result[null]))]
pub fn flush<R: IoStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "flush") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("flush: invalid handle".to_string())),
    };

    let result = match io_file {
        IoFileHandle::Write(w) => w.flush(),
        IoFileHandle::ReadWrite(_, w) => w.flush(),
        IoFileHandle::Read(_) => {
            return R::err(R::from_string("flush: handle is not open for writing".to_string()))
        }
    };

    match result {
        Ok(()) => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!("flush: {}", e))),
    }
}

#[native_fn(module = "fs", bound = "IoStore",
    sig(handle(File) -> result[string]))]
pub fn read_all<R: IoStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "read_all") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("read_all: invalid handle".to_string())),
    };

    let mut buf = Vec::new();
    let result = match io_file {
        IoFileHandle::Read(r) => r.read_to_end(&mut buf),
        IoFileHandle::ReadWrite(r, _) => r.read_to_end(&mut buf),
        IoFileHandle::Write(_) => {
            return R::err(R::from_string("read_all: handle is not open for reading".to_string()))
        }
    };

    match result {
        Ok(_) => R::ok(R::from_string(String::from_utf8_lossy(&buf).into_owned())),
        Err(e) => R::err(R::from_string(format!("read_all: {}", e))),
    }
}

#[native_fn(module = "fs", bound = "IoStore",
    sig(handle(File) -> result[string]))]
pub fn readline<R: IoStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "readline") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("readline: invalid handle".to_string())),
    };

    let mut line = String::new();
    let result = match io_file {
        IoFileHandle::Read(r) => r.read_line(&mut line),
        IoFileHandle::ReadWrite(r, _) => r.read_line(&mut line),
        IoFileHandle::Write(_) => {
            return R::err(R::from_string("readline: handle is not open for reading".to_string()))
        }
    };

    match result {
        Ok(0) => R::ok(R::from_string(String::new())),
        Ok(_) => {
            line.truncate(line.trim_end().len());
            R::ok(R::from_string(line))
        }
        Err(e) => R::err(R::from_string(format!("readline: {}", e))),
    }
}

// ---- recursive dir operations ---------------------------------------------

#[native_fn(module = "fs")]
pub fn copy_dir(src: String, dest: String) -> Result<(), String> {
    let src_path = std::path::Path::new(&src);
    if !src_path.is_dir() {
        return Err(format!("copy_dir: source \"{}\" is not a directory", src));
    }
    let dest_path = std::path::Path::new(&dest);
    if let Err(e) = std::fs::create_dir_all(dest_path) {
        return Err(format!(
            "copy_dir: failed to create \"{}\": {}",
            dest, e
        ));
    }
    let mut stack = vec![src_path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                return Err(format!(
                    "copy_dir: failed to read \"{}\": {}",
                    dir.to_string_lossy(),
                    e
                ))
            }
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let rel = path.strip_prefix(src_path).unwrap_or(&path);
            let target = dest_path.join(rel);
            if path.is_dir() {
                if let Err(e) = std::fs::create_dir_all(&target) {
                    return Err(format!(
                        "copy_dir: failed to create \"{}\": {}",
                        target.to_string_lossy(),
                        e
                    ));
                }
                stack.push(path);
            } else {
                if let Some(parent) = target.parent()
                    && let Err(e) = std::fs::create_dir_all(parent)
                {
                    return Err(format!(
                        "copy_dir: failed to create \"{}\": {}",
                        parent.to_string_lossy(),
                        e
                    ));
                }
                if let Err(e) = std::fs::copy(&path, &target) {
                    return Err(format!(
                        "copy_dir: failed to copy \"{}\" to \"{}\": {}",
                        path.to_string_lossy(),
                        target.to_string_lossy(),
                        e
                    ));
                }
            }
        }
    }
    Ok(())
}

#[native_fn(module = "fs")]
pub fn dir_size(path: String) -> Result<i64, String> {
    let mut total: i64 = 0;
    let mut stack = vec![std::path::PathBuf::from(&path)];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                return Err(format!(
                    "dir_size: failed to read \"{}\": {}",
                    dir.to_string_lossy(),
                    e
                ))
            }
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match std::fs::metadata(&path) {
                    Ok(m) => total += m.len() as i64,
                    Err(e) => {
                        return Err(format!(
                            "dir_size: failed to stat \"{}\": {}",
                            path.to_string_lossy(),
                            e
                        ))
                    }
                }
            }
        }
    }
    Ok(total)
}

#[native_fn(module = "fs")]
pub fn is_symlink(path: String) -> Result<bool, String> {
    match std::fs::symlink_metadata(&path) {
        Ok(metadata) => Ok(metadata.file_type().is_symlink()),
        Err(e) => Err(format!(
            "is_symlink: failed to read \"{}\": {}",
            path, e
        )),
    }
}

rl_std_core::native_module!("fs";
    bound: IoStore;
    funcs: [
        mkdir, mkdir_all, rmdir, rmdir_all,
        list_dir, list_dir_names,
        copy_file, move_file,
        file_size, file_modified, file_created, file_accessed,
        file_permissions, set_permissions,
        temp_dir, temp_file, temp_file_in,
        rename_file,
        touch, truncate_file,
        glob, walk_dir,
        symlink, readlink, hardlink,
        realpath,
        lock_file, unlock_file,
        read_file, read_lines, read_bytes,
        write_file, append_file, delete_file,
        path_exists, path_is_dir, path_is_file,
        path_canonicalize, path_absolute, path_expand_home,
        path_relative,
        open, close,
        read_handle, write_handle, seek, flush, read_all, readline,
        copy_dir, dir_size, is_symlink,
    ],
);
