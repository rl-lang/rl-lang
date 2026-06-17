use crate::docs::entry::{FnEntry, StdEntry};

pub static IO: StdEntry = StdEntry {
    name: "io",
    description: "functions for input and output",
    functions: FUNCTIONS,
};

static FUNCTIONS: &[&FnEntry] = &[&READ, &READ_PROMPT];

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
