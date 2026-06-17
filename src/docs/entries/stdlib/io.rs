use crate::docs::entry::{FnEntry, StdEntry};

pub static IO: StdEntry = StdEntry {
    name: "io",
    description: "functions for input and output",
    functions: FUNCTIONS,
};

static FUNCTIONS: &[&FnEntry] = &[
    &READ,
    &READ_PROMPT,
    &READ_INT,
    &READ_INT_PROMPT,
    &READ_FLOAT,
    &READ_FLOAT_PROMPT,
    &APPEND_FILE,
    &DELETE_FILE,
    &READ_FILE,
    &READ_LINES,
    &WRITE_FILE,
];

static READ: FnEntry = FnEntry {
    signature: "read()",
    description: "read a line from stdin",
    example: "get std::io::read\n\ndec str name = read()",
};

static READ_PROMPT: FnEntry = FnEntry {
    signature: "read(prompt)",
    description: "prints prompt and reads a line from stdin",
    example: "get std::io::read\n\ndec str name = read(\"enter your name: \")",
};

static READ_INT: FnEntry = FnEntry {
    signature: "read_int()",
    description: "read a line from stdin then parses to integer",
    example: "get std::io::read_int\n\ndec int age = read()",
};

static READ_INT_PROMPT: FnEntry = FnEntry {
    signature: "read_int(prompt)",
    description: "prints prompt and reads a line from stdin then parses to integer",
    example: "get std::io::read_int\n\ndec int age = read_float(\"enter your age: \")",
};

static READ_FLOAT: FnEntry = FnEntry {
    signature: "read_float()",
    description: "read a line from stdin then parses to float",
    example: "get std::io::read_float\n\ndec float pi = read_float()",
};

static READ_FLOAT_PROMPT: FnEntry = FnEntry {
    signature: "read_float(prompt)",
    description: "prints prompt and reads a line from stdin then parses to float",
    example: "get std::io::read_float\n\ndec float pi = read_float(\"enter your pi: \")",
};

static APPEND_FILE: FnEntry = FnEntry {
    signature: "append_file(path, content)",
    description: "appends content to a file creating it if it does not exist",
    example: "get std::io::append_file\n\nappend_file(\"info.txt\", \"name: Mohamed\")",
};

static DELETE_FILE: FnEntry = FnEntry {
    signature: "delete_file(path)",
    description: "deletes a file at the given path",
    example: "get std::io::delete_file\n\ndelete_file(\"info.txt\")",
};

static READ_FILE: FnEntry = FnEntry {
    signature: "read_file(path)",
    description: "reads the entire contents of a file as a string",
    example: "get std::io::read_file\n\ndec string data = read_file(\"backup_info.txt\")",
};

static READ_LINES: FnEntry = FnEntry {
    signature: "read_lines(path)",
    description: "reads a file and returns its lines as an array of strings",
    example: "get std::io::read_lines\n\ndec arr[string] data = read_lines(\"index.html\")",
};

static WRITE_FILE: FnEntry = FnEntry {
    signature: "write_file(path, contents)",
    description: "writes content to a file overwriting it if it already exists",
    example: "get std::io::write_file\n\nwrite_file(\"index.html\", \"<p>hello \\\"Mohamed\\\"</p>\")",
};
