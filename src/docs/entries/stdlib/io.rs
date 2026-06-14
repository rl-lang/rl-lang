use crate::docs::entry::{FnEntry, StdEntry};

pub static IO: StdEntry = StdEntry {
    name: "io",
    description: "functions for input and output",
    functions: FUNCTIONS,
};

static FUNCTIONS: &'static [&'static FnEntry] = &[&INPUT, &INPUT_PROMPT];

static INPUT: FnEntry = FnEntry {
    signature: "input()",
    description: "read a line from stdin",
    example: "get std::io::input\n\ndec str name = input()",
};

static INPUT_PROMPT: FnEntry = FnEntry {
    signature: "input(prompt)",
    description: "prints prompt and reads a line from stdin",
    example: "get std::io::input\n\ndec str name = input(\"enter your name: \")",
};
