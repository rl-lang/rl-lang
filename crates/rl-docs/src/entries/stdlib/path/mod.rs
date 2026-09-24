use crate::entry::{FnEntry, StdEntry};

mod path_components;
mod path_ends_with;
mod path_extension;
mod path_filename;
mod path_is_absolute;
mod path_is_relative;
mod path_join;
mod path_join_many;
mod path_normalize;
mod path_parent;
mod path_pop;
mod path_push;
mod path_set_extension;
mod path_split;
mod path_split_extension;
mod path_stem;
mod path_starts_with;
mod path_with_file_name;

pub static PATH: StdEntry = StdEntry {
    name: "path",
    description: "functions for working with filesystem paths",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &path_components::PATH_COMPONENTS,
    &path_ends_with::PATH_ENDS_WITH,
    &path_extension::PATH_EXTENSION,
    &path_filename::PATH_FILENAME,
    &path_is_absolute::PATH_IS_ABSOLUTE,
    &path_is_relative::PATH_IS_RELATIVE,
    &path_join::PATH_JOIN,
    &path_join_many::PATH_JOIN_MANY,
    &path_normalize::PATH_NORMALIZE,
    &path_parent::PATH_PARENT,
    &path_pop::PATH_POP,
    &path_push::PATH_PUSH,
    &path_set_extension::PATH_SET_EXTENSION,
    &path_split::PATH_SPLIT,
    &path_split_extension::PATH_SPLIT_EXTENSION,
    &path_starts_with::PATH_STARTS_WITH,
    &path_stem::PATH_STEM,
    &path_with_file_name::PATH_WITH_FILE_NAME,
];
