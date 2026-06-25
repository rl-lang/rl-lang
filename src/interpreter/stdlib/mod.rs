//! The rl standard library - all built-in modules registered under `std::*`.

use crate::{
    interpreter::values::Value,
    utils::{
        errors::{Error, Reason},
        span::Span,
    },
};

pub mod array;
pub mod bitwise;
pub mod fs;
pub mod io;
pub mod len;
pub mod math;
pub mod path;
pub mod process;
pub mod random;
pub mod result;
pub mod string;
pub mod time;
pub mod types;

// helper function for raw functions
pub fn check_arity(args: &[Value], expected: usize, name: &str, span: Span) -> Result<(), Error> {
    if args.len() != expected {
        return Err(Error::at(
            Reason::Runtime,
            format!("{}: expected {} arg(s), got {}", name, expected, args.len()),
            span,
        ));
    }
    Ok(())
}
