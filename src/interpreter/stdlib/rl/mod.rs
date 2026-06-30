//! `std::rl`

mod eval;
mod eval_isolated;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &["lex", "eval", "eval_isolated"];

pub fn module() -> Module {
    Module::new("rl")
        .with_function("eval", eval::func)
        .with_function("eval_isolated", eval_isolated::func)
}
