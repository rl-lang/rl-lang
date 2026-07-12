//! `std::io` - input/output: reading from stdin, reading/writing files, printing.
//!
//! `print` and `println` write to [`Evaluator::output_buffer`] when set (LSP/REPL),
//! otherwise they write directly to stdout.
//!
//! `eprint` raises a runtime error rather than writing to stderr, so errors
//! surface through rl's normal error reporting pipeline.

mod append_file;
mod delete_file;
mod eprint;
mod input;
mod print;
mod println;
mod read_bytes;
mod read_file;
mod read_lines;
mod write_file;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "read",
    "read_int",
    "read_float",
    "read_file",
    "read_lines",
    "delete_file",
    "write_file",
    "append_file",
    "print",
    "println",
    "eprint",
    "read_bytes",
];

pub fn module() -> Module {
    Module::new("io")
        .with_raw_function("read", input::std_read)
        .with_raw_function("read_int", input::std_read_int)
        .with_raw_function("read_float", input::std_read_float)
        .with_function("read_file", read_file::std_read_file)
        .with_function("read_lines", read_lines::std_read_lines)
        .with_function("delete_file", delete_file::std_delete_file)
        .with_function("write_file", write_file::std_write_file)
        .with_function("append_file", append_file::std_append_file)
        .with_raw_function("print", print::std_print)
        .with_raw_function("println", println::std_println)
        .with_function("eprint", eprint::std_eprint)
        .with_function("read_bytes", read_bytes::func)
}
