mod path_absolute;
mod path_extension;
mod path_filename;
mod path_join;
mod path_parent;
mod path_stem;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "path_absolute",
    "path_extension",
    "path_filename",
    "path_join",
    "path_parent",
    "path_stem",
];

pub fn module() -> Module {
    Module::new("path")
        .with_function("path_absolute", path_absolute::std_path_absolute)
        .with_function("path_extension", path_extension::std_path_extension)
        .with_function("path_filename", path_filename::std_path_filename)
        .with_function("path_join", path_join::std_path_join)
        .with_function("path_parent", path_parent::std_path_parent)
        .with_function("path_stem", path_stem::std_path_stem)
}
