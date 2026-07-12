//! `std::rl`

mod check;
mod eval;
mod eval_isolated;
mod lex;
mod rl_version;
mod source_name;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "lex",
    "eval",
    "eval_isolated",
    "check",
    "rl_version",
    "source_name",
];

pub fn module() -> Module {
    Module::new("rl")
        .with_function("lex", lex::func)
        .with_function("eval", eval::func)
        .with_function("eval_isolated", eval_isolated::func)
        .with_function("check", check::func)
        .with_function("rl_version", rl_version::func)
        .with_function("source_name", source_name::func)
}
