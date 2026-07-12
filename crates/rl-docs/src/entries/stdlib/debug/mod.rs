use crate::entry::{FnEntry, StdEntry};

mod assert;
mod assert_approx_eq;
mod assert_eq;
mod assert_ge;
mod assert_gt;
mod assert_le;
mod assert_lt;
mod assert_ne;
mod bench;
mod dbg;
mod panic;
mod todo;
mod type_of;
mod unreachable;

use assert::ASSERT;
use assert_approx_eq::ASSERT_APPROX_EQ;
use assert_eq::ASSERT_EQ;
use assert_ge::ASSERT_GE;
use assert_gt::ASSERT_GT;
use assert_le::ASSERT_LE;
use assert_lt::ASSERT_LT;
use assert_ne::ASSERT_NE;
use bench::BENCH;
use dbg::DBG;
use panic::PANIC;
use todo::TODO;
use type_of::TYPE_OF;
use unreachable::UNREACHABLE;

pub static DEBUG: StdEntry = StdEntry {
    name: "debug",
    description: "assertions, panics, and debug utilities for catching bugs and inspecting values at runtime",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &ASSERT,
    &ASSERT_EQ,
    &ASSERT_NE,
    &ASSERT_LT,
    &ASSERT_LE,
    &ASSERT_GT,
    &ASSERT_GE,
    &ASSERT_APPROX_EQ,
    &PANIC,
    &UNREACHABLE,
    &TODO,
    &DBG,
    &TYPE_OF,
    &BENCH,
];
