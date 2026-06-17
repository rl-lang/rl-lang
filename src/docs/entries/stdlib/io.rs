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
