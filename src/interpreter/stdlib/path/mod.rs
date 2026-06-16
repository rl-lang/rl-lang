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

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "path_exists",
    "path_extension",
    "path_filename",
    "path_is_dir",
    "path_is_file",
    "path_join",
    "path_parent",
    "path_pop",
    "path_push",
    "path_set_extension",
    "path_stem",
];

pub fn module() -> Module {
    Module::new("path")
        .with_function("path_exists", path_exists::std_path_exists)
        .with_function("path_extension", path_extension::std_path_extension)
        .with_function("path_filename", path_filename::std_path_filename)
        .with_function("path_is_dir", path_is_dir::std_path_is_dir)
        .with_function("path_is_file", path_is_file::std_path_is_file)
        .with_function("path_join", path_join::std_path_join)
        .with_function("path_parent", path_parent::std_path_parent)
        .with_function("path_pop", path_pop::std_path_pop)
        .with_function("path_push", path_push::std_path_push)
        .with_function(
            "path_set_extension",
            path_set_extension::std_path_set_extension,
        )
        .with_function("path_stem", path_stem::std_path_stem)
}
