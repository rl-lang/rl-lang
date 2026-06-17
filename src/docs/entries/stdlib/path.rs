use crate::docs::entry::{FnEntry, StdEntry};

pub static PATH: StdEntry = StdEntry {
    name: "path",
    description: "functions for working with filesystem paths",
    functions: FUNCTIONS,
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

static PATH_EXISTS: FnEntry = FnEntry {
    signature: "path_exists(path)",
    description: "returns true if the path exists on the filesystem",
    example: "get std::path::path_exists\n\npath_exists(\"./Cargo.toml\") // true",
};

static PATH_EXTENSION: FnEntry = FnEntry {
    signature: "path_extension(path)",
    description: "returns the file extension of the path",
    example: "get std::path::path_extension\n\npath_extension(\"main.rl\") // \"rl\"",
};

static PATH_FILENAME: FnEntry = FnEntry {
    signature: "path_filename(path)",
    description: "returns the final component of the path",
    example: "get std::path::path_filename\n\npath_filename(\"/usr/bin/rl\") // \"rl\"",
};

static PATH_IS_DIR: FnEntry = FnEntry {
    signature: "path_is_dir(path)",
    description: "returns true if the path is a directory",
    example: "get std::path::path_is_dir\n\npath_is_dir(\"./src\") // true",
};

static PATH_IS_FILE: FnEntry = FnEntry {
    signature: "path_is_file(path)",
    description: "returns true if the path is a file",
    example: "get std::path::path_is_file\n\npath_is_file(\"./Cargo.toml\") // true",
};

static PATH_JOIN: FnEntry = FnEntry {
    signature: "path_join(path, other)",
    description: "joins two paths together",
    example: "get std::path::path_join\n\npath_join(\"src\", \"main.rl\") // \"src/main.rl\"",
};

static PATH_PARENT: FnEntry = FnEntry {
    signature: "path_parent(path)",
    description: "returns the parent directory of the path",
    example: "get std::path::path_parent\n\npath_parent(\"/usr/bin/rl\") // \"/usr/bin\"",
};

static PATH_POP: FnEntry = FnEntry {
    signature: "path_pop(path)",
    description: "removes the last component of the path and returns the result",
    example: "get std::path::path_pop\n\npath_pop(\"/usr/bin/rl\") // \"/usr/bin\"",
};

static PATH_PUSH: FnEntry = FnEntry {
    signature: "path_push(path, target)",
    description: "appends a component to the path and returns the result",
    example: "get std::path::path_push\n\npath_push(\"/usr/bin\", \"rl\") // \"/usr/bin/rl\"",
};

static PATH_SET_EXTENSION: FnEntry = FnEntry {
    signature: "path_set_extension(path, extension)",
    description: "sets or replaces the extension of the path and returns the result",
    example: "get std::path::path_set_extension\n\npath_set_extension(\"main.rl\", \"txt\") // \"main.txt\"",
};

static PATH_STEM: FnEntry = FnEntry {
    signature: "path_stem(path)",
    description: "returns the filename without its extension",
    example: "get std::path::path_stem\n\npath_stem(\"main.rl\") // \"main\"",
};
