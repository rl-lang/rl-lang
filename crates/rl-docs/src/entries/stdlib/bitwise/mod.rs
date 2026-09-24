use crate::entry::{FnEntry, StdEntry};

mod bit_and;
mod bit_clear;
mod bit_is_set;
mod bit_not;
mod bit_or;
mod bit_set;
mod bit_shift_left;
mod bit_shift_right;
mod bit_toggle;
mod bit_xor;
mod count_bits;
mod leading_zeros;
mod rotate_left;
mod rotate_right;
mod trailing_zeros;

use bit_and::BIT_AND;
use bit_clear::BIT_CLEAR;
use bit_is_set::BIT_IS_SET;
use bit_not::BIT_NOT;
use bit_or::BIT_OR;
use bit_set::BIT_SET;
use bit_shift_left::BIT_SHIFT_LEFT;
use bit_shift_right::BIT_SHIFT_RIGHT;
use bit_toggle::BIT_TOGGLE;
use bit_xor::BIT_XOR;
use count_bits::COUNT_BITS;
use leading_zeros::LEADING_ZEROS;
use rotate_left::ROTATE_LEFT;
use rotate_right::ROTATE_RIGHT;
use trailing_zeros::TRAILING_ZEROS;

pub static BITWISE: StdEntry = StdEntry {
    name: "bitwise",
    description: "functions for bitwise operations on byte and int values",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &BIT_AND,
    &BIT_CLEAR,
    &BIT_IS_SET,
    &BIT_NOT,
    &BIT_OR,
    &BIT_SET,
    &BIT_SHIFT_LEFT,
    &BIT_SHIFT_RIGHT,
    &BIT_TOGGLE,
    &BIT_XOR,
    &COUNT_BITS,
    &LEADING_ZEROS,
    &ROTATE_LEFT,
    &ROTATE_RIGHT,
    &TRAILING_ZEROS,
];
