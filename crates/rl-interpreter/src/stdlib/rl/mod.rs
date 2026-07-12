//! `std::rl`

mod check;
mod eval;
mod eval_isolated;
mod lex;
mod rl_version;
mod source_name;

use crate::native::Module;

pub use rl_commons::keywords::rl::KEYWORDS;

pub fn module() -> Module {
    Module::new("rl")
        .with_function("lex", lex::func)
        .with_function("eval", eval::func)
        .with_function("eval_isolated", eval_isolated::func)
        .with_function("check", check::func)
        .with_function("rl_version", rl_version::func)
        .with_function("source_name", source_name::func)
}
