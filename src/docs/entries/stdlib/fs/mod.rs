use crate::docs::entry::{FnEntry, StdEntry};

mod copy_file;
mod mkdir;
mod mkdir_all;
mod move_file;
mod rename_file;
mod rmdir;
mod rmdir_all;
mod temp_dir;

pub static FS: StdEntry = StdEntry {
    name: "fs",
    description: "functions for working with the filesystem",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &copy_file::COPY_FILE,
    &FILE_MODIFIED,
    &FILE_SIZE,
    &LIST_DIR,
    &mkdir::MKDIR,
    &mkdir_all::MKDIR_ALL,
    &move_file::MOVE_FILE,
    &rename_file::RENAME_FILE,
    &rmdir::RMDIR,
    &rmdir_all::RMDIR_ALL,
    &temp_dir::TEMP_DIR,
];

static FILE_MODIFIED: FnEntry = FnEntry {
    signature: "file_modified(path)",
    description: "returns the last modification time of the file as a unix timestamp (seconds since epoch)",
    example: "get std::fs::file_modified\n\nfile_modified(\"./Cargo.toml\") // 1750000000",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FILE_SIZE: FnEntry = FnEntry {
    signature: "file_size(path)",
    description: "returns the size of the file in bytes",
    example: "get std::fs::file_size\n\nfile_size(\"./Cargo.toml\") // 215",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static LIST_DIR: FnEntry = FnEntry {
    signature: "list_dir(path)",
    description: "returns an array of paths for the entries in the directory",
    example: "get std::fs::list_dir\n\nlist_dir(\"./src\") // [\"./src/main.rl\", \"./src/html_tags\"]",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
