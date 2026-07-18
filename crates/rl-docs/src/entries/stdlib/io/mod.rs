use crate::entry::{FnEntry, StdEntry};

pub static IO: StdEntry = StdEntry {
    name: "io",
    description: "functions for input and output",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
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
    &READ_BYTES,
    &WRITE_FILE,
    &PRINT,
    &PRINTLN,
    &EPRINT,
];

static READ: FnEntry = FnEntry {
    signature: "read()",
    description: "reads a line from stdin",
    example: "get std::io::read\n\ndec string name = read()?",
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if reading from stdin fails"),
    see_also: &["read_int", "read_float"],
    since: Some("v0.1.5"),
};

static READ_PROMPT: FnEntry = FnEntry {
    signature: "read(prompt)",
    description: "prints prompt and reads a line from stdin",
    example: "get std::io::read\n\ndec string name = read(\"enter your name: \")?",
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if reading from stdin fails"),
    see_also: &["read_int", "read_float"],
    since: Some("v0.1.5"),
};

static READ_INT: FnEntry = FnEntry {
    signature: "read_int()",
    description: "reads a line from stdin then parses it to an integer",
    example: "get std::io::read_int\n\ndec int age = read_int()?",
    expected_output: None,
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- reading from stdin fails\n- the input line is not a valid integer",
    ),
    see_also: &["read", "read_float"],
    since: Some("v0.1.5"),
};

static READ_INT_PROMPT: FnEntry = FnEntry {
    signature: "read_int(prompt)",
    description: "prints prompt and reads a line from stdin then parses it to an integer",
    example: "get std::io::read_int\n\ndec int age = read_int(\"enter your age: \")?",
    expected_output: None,
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- reading from stdin fails\n- the input line is not a valid integer",
    ),
    see_also: &["read", "read_float"],
    since: Some("v0.1.5"),
};

static READ_FLOAT: FnEntry = FnEntry {
    signature: "read_float()",
    description: "reads a line from stdin then parses it to a float",
    example: "get std::io::read_float\n\ndec float pi = read_float()?",
    expected_output: None,
    returns: "result[float]",
    errors: Some(
        "Will return error on the following:\n\n- reading from stdin fails\n- the input line is not a valid float",
    ),
    see_also: &["read", "read_int"],
    since: Some("v0.1.5"),
};

static READ_FLOAT_PROMPT: FnEntry = FnEntry {
    signature: "read_float(prompt)",
    description: "prints prompt and reads a line from stdin then parses it to a float",
    example: "get std::io::read_float\n\ndec float pi = read_float(\"enter pi: \")?",
    expected_output: None,
    returns: "result[float]",
    errors: Some(
        "Will return error on the following:\n\n- reading from stdin fails\n- the input line is not a valid float",
    ),
    see_also: &["read", "read_int"],
    since: Some("v0.1.5"),
};

static APPEND_FILE: FnEntry = FnEntry {
    signature: "append_file(path, content)",
    description: "appends content to a file, creating it if it does not exist",
    example: "get std::io::append_file\n\nappend_file(\"info.txt\", \"name: Mohamed\")?",
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "Will return error on the following:\n\n- `path`'s parent directory does not exist\n- the current process lacks permission to write to `path`",
    ),
    see_also: &["write_file", "read_file"],
    since: Some("v0.1.5"),
};

static DELETE_FILE: FnEntry = FnEntry {
    signature: "delete_file(path)",
    description: "deletes a file at the given path",
    example: "get std::io::delete_file\n\ndelete_file(\"info.txt\")?",
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "Will return error on the following:\n\n- `path` does not exist\n- `path` is a directory, not a file\n- the current process lacks permission to remove `path`",
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};

static READ_FILE: FnEntry = FnEntry {
    signature: "read_file(path)",
    description: "reads the entire contents of a file as a string",
    example: "get std::io::read_file\n\ndec string data = read_file(\"backup_info.txt\")?",
    expected_output: None,
    returns: "result[string]",
    errors: Some(
        "Will return error on the following:\n\n- `path` does not exist\n- the current process lacks permission to read `path`\n- `path`'s contents are not valid UTF-8",
    ),
    see_also: &["read_lines", "read_bytes"],
    since: Some("v0.1.5"),
};

static READ_LINES: FnEntry = FnEntry {
    signature: "read_lines(path)",
    description: "reads a file and returns its lines as an array of strings",
    example: "get std::io::read_lines\n\ndec arr[string] data = read_lines(\"index.html\")?",
    expected_output: None,
    returns: "result[array[string]]",
    errors: Some(
        "Will return error on the following:\n\n- `path` does not exist\n- the current process lacks permission to read `path`\n- `path`'s contents are not valid UTF-8",
    ),
    see_also: &["read_file", "read_bytes"],
    since: Some("v0.1.5"),
};

static READ_BYTES: FnEntry = FnEntry {
    signature: "read_bytes(path)",
    description: "reads the entire contents of a file as a byte array",
    example: "get std::io::read_bytes\n\ndec arr[byte] data = read_bytes(\"backup_info.txt\")?",
    expected_output: None,
    returns: "result[array[byte]]",
    errors: Some(
        "Will return error on the following:\n\n- `path` does not exist\n- the current process lacks permission to read `path`\n\nUnlike `read_file`/`read_lines`, this does not require valid UTF-8, since\nit reads raw bytes rather than a string.",
    ),
    see_also: &["read_file", "read_lines"],
    since: Some("v0.1.5"),
};

static WRITE_FILE: FnEntry = FnEntry {
    signature: "write_file(path, contents)",
    description: "writes content to a file, overwriting it if it already exists",
    example: "get std::io::write_file\n\nwrite_file(\"index.html\", \"<p>hello \\\"Mohamed\\\"</p>\")?",
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "Will return error on the following:\n\n- `path`'s parent directory does not exist\n- the current process lacks permission to write to `path`",
    ),
    see_also: &["append_file", "read_file"],
    since: Some("v0.1.5"),
};

static PRINT: FnEntry = FnEntry {
    signature: "print(x, ...)",
    description: "prints any number of values without a trailing newline",
    example: "get std::io::print\n\nprint(\"hello\")",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["println"],
    since: Some("v0.1.5"),
};

static PRINTLN: FnEntry = FnEntry {
    signature: "println(x, ...)",
    description: "prints any number of values followed by a newline",
    example: "get std::io::println\n\nprintln(\"hello\")",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["print"],
    since: Some("v0.1.5"),
};

static EPRINT: FnEntry = FnEntry {
    signature: "eprint(message)",
    description: "halts evaluation, raising message as a runtime error",
    example: "get std::io::eprint\n\neprint(\"something went wrong\") // error: something went wrong",
    expected_output: None,
    returns: "never returns",
    errors: Some(
        "Always raises `message` as an interpreter-level runtime error - this is\nnot a catchable `result[..]` err, and cannot be caught with `?`. Program\nevaluation stops here.",
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};
