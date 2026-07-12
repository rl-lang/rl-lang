use crate::entry::{FnEntry, StdEntry};

mod copy_file;
mod file_modified;
mod file_size;
mod list_dir;
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
    &file_modified::FILE_MODIFIED,
    &file_size::FILE_SIZE,
    &list_dir::LIST_DIR,
    &mkdir::MKDIR,
    &mkdir_all::MKDIR_ALL,
    &move_file::MOVE_FILE,
    &rename_file::RENAME_FILE,
    &rmdir::RMDIR,
    &rmdir_all::RMDIR_ALL,
    &temp_dir::TEMP_DIR,
];
