use crate::entry::{FnEntry, StdEntry};

mod append_file;
mod copy_dir;
mod copy_file;
mod delete_file;
mod dir_size;
mod file_accessed;
mod file_created;
mod file_modified;
mod file_permissions;
mod file_size;
mod glob;
mod hardlink;
mod is_symlink;
mod list_dir;
mod list_dir_names;
mod lock_file;
mod mkdir;
mod mkdir_all;
mod move_file;
mod path_absolute;
mod path_canonicalize;
mod path_expand_home;
mod path_exists;
mod path_is_dir;
mod path_is_file;
mod read_bytes;
mod read_file;
mod read_lines;
mod readlink;
mod realpath;
mod rename_file;
mod rmdir;
mod rmdir_all;
mod set_permissions;
mod symlink;
mod temp_dir;
mod temp_file;
mod temp_file_in;
mod touch;
mod truncate_file;
mod unlock_file;
mod walk_dir;
mod write_file;

pub static FS: StdEntry = StdEntry {
    name: "fs",
    description: "functions for working with the filesystem",
    functions: FUNCTIONS,
    since: Some("v2.1.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &append_file::APPEND_FILE,
    &copy_dir::COPY_DIR,
    &copy_file::COPY_FILE,
    &delete_file::DELETE_FILE,
    &dir_size::DIR_SIZE,
    &file_accessed::FILE_ACCESSED,
    &file_created::FILE_CREATED,
    &file_modified::FILE_MODIFIED,
    &file_permissions::FILE_PERMISSIONS,
    &file_size::FILE_SIZE,
    &glob::GLOB,
    &hardlink::HARDLINK,
    &is_symlink::IS_SYMLINK,
    &list_dir::LIST_DIR,
    &list_dir_names::LIST_DIR_NAMES,
    &lock_file::LOCK_FILE,
    &mkdir::MKDIR,
    &mkdir_all::MKDIR_ALL,
    &move_file::MOVE_FILE,
    &path_absolute::PATH_ABSOLUTE,
    &path_canonicalize::PATH_CANONICALIZE,
    &path_expand_home::PATH_EXPAND_HOME,
    &path_exists::PATH_EXISTS,
    &path_is_dir::PATH_IS_DIR,
    &path_is_file::PATH_IS_FILE,
    &read_bytes::READ_BYTES,
    &read_file::READ_FILE,
    &read_lines::READ_LINES,
    &readlink::READLINK,
    &realpath::REALPATH,
    &rename_file::RENAME_FILE,
    &rmdir::RMDIR,
    &rmdir_all::RMDIR_ALL,
    &set_permissions::SET_PERMISSIONS,
    &symlink::SYMLINK,
    &temp_dir::TEMP_DIR,
    &temp_file::TEMP_FILE,
    &temp_file_in::TEMP_FILE_IN,
    &touch::TOUCH,
    &truncate_file::TRUNCATE_FILE,
    &unlock_file::UNLOCK_FILE,
    &walk_dir::WALK_DIR,
    &write_file::WRITE_FILE,
];
