use crate::docs::entry::{FnEntry, StdEntry};

pub static DISPLAY: StdEntry = StdEntry {
    name: "display",
    description: "functions for displaying output",
    functions: FUNCTIONS,
};

static FUNCTIONS: &[&FnEntry] = &[&LEN];

static LEN: FnEntry = FnEntry {
    signature: "len(x)",
    description: "length of string or array",
    example: "get std::display::len\n\nlen(\"hello\") // 5",
};
