use crate::entry::{FnEntry, StdEntry};

mod path_exists;
mod path_extension;
mod path_filename;
mod path_is_dir;
mod path_is_file;
mod path_join;
mod path_parent;
mod path_pop;
mod path_push;
mod path_set_extension;
mod path_stem;

pub static PATH: StdEntry = StdEntry {
    name: "path",
    description: "functions for working with filesystem paths",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &path_exists::PATH_EXISTS,
    &path_extension::PATH_EXTENSION,
    &path_filename::PATH_FILENAME,
    &path_is_dir::PATH_IS_DIR,
    &path_is_file::PATH_IS_FILE,
    &path_join::PATH_JOIN,
    &path_parent::PATH_PARENT,
    &path_pop::PATH_POP,
    &path_push::PATH_PUSH,
    &path_set_extension::PATH_SET_EXTENSION,
    &path_stem::PATH_STEM,
];
