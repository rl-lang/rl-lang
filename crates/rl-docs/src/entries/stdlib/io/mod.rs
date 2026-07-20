use crate::entry::{FnEntry, StdEntry};

mod append_file;
mod delete_file;
mod eprint;
mod print;
mod println;
mod read;
mod read_bytes;
mod read_file;
mod read_float;
mod read_int;
mod read_lines;
mod write_file;

pub static IO: StdEntry = StdEntry {
    name: "io",
    description: "functions for input and output",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &read::READ,
    &read::READ_PROMPT,
    &read_int::READ_INT,
    &read_int::READ_INT_PROMPT,
    &read_float::READ_FLOAT,
    &read_float::READ_FLOAT_PROMPT,
    &append_file::APPEND_FILE,
    &delete_file::DELETE_FILE,
    &read_file::READ_FILE,
    &read_lines::READ_LINES,
    &read_bytes::READ_BYTES,
    &write_file::WRITE_FILE,
    &print::PRINT,
    &println::PRINTLN,
    &eprint::EPRINT,
];
