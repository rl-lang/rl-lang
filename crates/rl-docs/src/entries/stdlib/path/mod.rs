use crate::entry::{FnEntry, StdEntry};

pub static PATH: StdEntry = StdEntry {
    name: "path",
    description: "functions for working with filesystem paths",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &PATH_EXISTS,
    &PATH_EXTENSION,
    &PATH_FILENAME,
    &PATH_IS_DIR,
    &PATH_IS_FILE,
    &PATH_JOIN,
    &PATH_PARENT,
    &PATH_POP,
    &PATH_PUSH,
    &PATH_SET_EXTENSION,
    &PATH_STEM,
];

pub static PATH_EXISTS: FnEntry = FnEntry {
    signature: "path_exists(path)",
    description: "returns true if the path exists on the filesystem",
    example: r#"
get std::path::path_exists

path_exists("./Cargo.toml")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

pub static PATH_EXTENSION: FnEntry = FnEntry {
    signature: "path_extension(path)",
    description: "returns the file extension of the path",
    example: r#"
get std::path::path_extension

path_extension("main.rl")"#,
    expected_output: Some("rl"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

pub static PATH_FILENAME: FnEntry = FnEntry {
    signature: "path_filename(path)",
    description: "returns the final component of the path",
    example: r#"
get std::path::path_filename

path_filename("/usr/bin/rl")"#,
    expected_output: Some("rl"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

pub static PATH_IS_DIR: FnEntry = FnEntry {
    signature: "path_is_dir(path)",
    description: "returns true if the path is a directory",
    example: r#"
get std::path::path_is_dir

path_is_dir("./src")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

pub static PATH_IS_FILE: FnEntry = FnEntry {
    signature: "path_is_file(path)",
    description: "returns true if the path is a file",
    example: r#"
get std::path::path_is_file

path_is_file("./Cargo.toml")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static PATH_JOIN: FnEntry = FnEntry {
    signature: "path_join(path, other)",
    description: "joins two paths together",
    example: "get std::path::path_join\n\npath_join(\"src\", \"main.rl\") // \"src/main.rl\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PATH_PARENT: FnEntry = FnEntry {
    signature: "path_parent(path)",
    description: "returns the parent directory of the path",
    example: "get std::path::path_parent\n\npath_parent(\"/usr/bin/rl\") // \"/usr/bin\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PATH_POP: FnEntry = FnEntry {
    signature: "path_pop(path)",
    description: "removes the last component of the path and returns the result",
    example: "get std::path::path_pop\n\npath_pop(\"/usr/bin/rl\") // \"/usr/bin\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PATH_PUSH: FnEntry = FnEntry {
    signature: "path_push(path, target)",
    description: "appends a component to the path and returns the result",
    example: "get std::path::path_push\n\npath_push(\"/usr/bin\", \"rl\") // \"/usr/bin/rl\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PATH_SET_EXTENSION: FnEntry = FnEntry {
    signature: "path_set_extension(path, extension)",
    description: "sets or replaces the extension of the path and returns the result",
    example: "get std::path::path_set_extension\n\npath_set_extension(\"main.rl\", \"txt\") // \"main.txt\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PATH_STEM: FnEntry = FnEntry {
    signature: "path_stem(path)",
    description: "returns the filename without its extension",
    example: "get std::path::path_stem\n\npath_stem(\"main.rl\") // \"main\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
