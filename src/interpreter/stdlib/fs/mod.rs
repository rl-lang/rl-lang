mod copy_file;
mod file_modified;
mod file_size;
mod list_dir;
mod mkdir;
mod mkdir_all;
mod move_file;
mod rmdir;
mod temp_dir;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "mkdir",
    "mkdir_all",
    "rmdir",
    "list_dir",
    "copy_file",
    "move_file",
    "file_size",
    "file_modified",
    "temp_dir",
];

pub fn module() -> Module {
    Module::new("display")
        .with_function("mkdir", mkdir::std_mkdir)
        .with_function("mkdir_all", mkdir_all::std_mkdir_all)
        .with_function("rmdir", rmdir::std_rmdir)
        .with_function("list_dir", list_dir::std_lit_dir)
        .with_function("copy_file", copy_file::std_copy_file)
        .with_function("move_file", move_file::std_move_file)
        .with_function("file_size", file_size::std_file_size)
        .with_function("file_modified", file_modified::std_file_modified)
        .with_function("temp_dir", temp_dir::std_temp_dir)
}
