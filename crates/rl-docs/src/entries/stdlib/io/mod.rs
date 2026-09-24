use crate::entry::{FnEntry, StdEntry};

mod decode_utf8;
mod eprint;
mod eprintln;
mod encode_utf8;
mod isatty;
mod print;
mod println;
mod read;
mod read_all_stdin;
mod read_float;
mod read_int;

pub static IO: StdEntry = StdEntry {
    name: "io",
    description: "functions for input and output",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &decode_utf8::DECODE_UTF8,
    &encode_utf8::ENCODE_UTF8,
    &eprint::EPRINT,
    &eprintln::EPRINTLN,
    &isatty::ISATTY,
    &print::PRINT,
    &println::PRINTLN,
    &read::READ,
    &read_all_stdin::READ_ALL_STDIN,
    &read_float::READ_FLOAT,
    &read_float::READ_FLOAT_PROMPT,
    &read_int::READ_INT,
    &read_int::READ_INT_PROMPT,
    &read::READ_PROMPT,
];
