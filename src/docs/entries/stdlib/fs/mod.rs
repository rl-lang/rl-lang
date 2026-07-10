use crate::docs::entry::{FnEntry, StdEntry};

mod copy_file;
mod move_file;
mod rename_file;
mod rmdir;
mod rmdir_all;

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
    &MKDIR,
    &MKDIR_ALL,
    &move_file::MOVE_FILE,
    &rename_file::RENAME_FILE,
    &rmdir::RMDIR,
    &rmdir_all::RMDIR_ALL,
    &TEMP_DIR,
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

static MKDIR: FnEntry = FnEntry {
    signature: "mkdir(path)",
    description: "creates a directory, fails if the parent directory does not exist",
    example: "get std::fs::mkdir\n\nmkdir(\"./build\")",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static MKDIR_ALL: FnEntry = FnEntry {
    signature: "mkdir_all(path)",
    description: "creates a directory along with any missing parent directories",
    example: "get std::fs::mkdir_all\n\nmkdir_all(\"./build/assets/css\")",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static TEMP_DIR: FnEntry = FnEntry {
    signature: "temp_dir()",
    description: "returns the path of the system's temporary directory",
    example: "get std::fs::temp_dir\n\ntemp_dir() // \"/tmp\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
