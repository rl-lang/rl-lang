use crate::docs::entry::{FnEntry, StdEntry};

pub static DISPLAY: StdEntry = StdEntry {
    name: "display",
    description: "functions for displaying output",
    functions: FUNCTIONS,
};

static FUNCTIONS: &[&FnEntry] = &[&PRINT, &PRINTLN, &LEN, &EPRINT];

static PRINT: FnEntry = FnEntry {
    signature: "print(x)",
    description: "print without newline",
    example: "get std::display::print\n\nprint(\"hello\")",
};

static PRINTLN: FnEntry = FnEntry {
    signature: "println(x)",
    description: "print with newline",
    example: "get std::display::println\n\nprintln(\"hello\")",
};

static LEN: FnEntry = FnEntry {
    signature: "len(x)",
    description: "length of string or array",
    example: "get std::display::len\n\nlen(\"hello\") // 5",
};

static EPRINT: FnEntry = FnEntry {
    signature: "eprint(string)",
    description: "halts evaluation with an error containing the given message",
    example: "get std::display::eprint\n\neprint(\"something went wrong\") // ✗ error: something went wrong",
};
