use crate::entry::{FnEntry, StdEntry};

mod check;
mod eval;
mod eval_isolated;
mod lex;
mod rl_version;
mod source_name;

use check::CHECK;
use eval::EVAL;
use eval_isolated::EVAL_ISOLATED;
use lex::LEX;
use rl_version::RL_VERSION;
use source_name::SOURCE_NAME;

pub static RL: StdEntry = StdEntry {
    name: "rl",
    description: "functions for introspecting and running rl source code from within rl itself",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &LEX,
    &CHECK,
    &EVAL,
    &EVAL_ISOLATED,
    &RL_VERSION,
    &SOURCE_NAME,
];
