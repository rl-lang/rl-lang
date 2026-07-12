use crate::docs::entry::{FnEntry, StdEntry};

mod bit_and;
mod bit_not;
mod bit_or;
mod bit_shift_left;
mod bit_shift_right;
mod bit_xor;
mod count_bits;
mod leading_zeros;
mod trailing_zeros;

use bit_and::BIT_AND;
use bit_not::BIT_NOT;
use bit_or::BIT_OR;
use bit_shift_left::BIT_SHIFT_LEFT;
use bit_shift_right::BIT_SHIFT_RIGHT;
use bit_xor::BIT_XOR;
use count_bits::COUNT_BITS;
use leading_zeros::LEADING_ZEROS;
use trailing_zeros::TRAILING_ZEROS;

pub static BITWISE: StdEntry = StdEntry {
    name: "bitwise",
    description: "functions for bitwise operations on byte and int values",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &BIT_AND,
    &BIT_OR,
    &BIT_XOR,
    &BIT_NOT,
    &BIT_SHIFT_LEFT,
    &BIT_SHIFT_RIGHT,
    &COUNT_BITS,
    &LEADING_ZEROS,
    &TRAILING_ZEROS,
];
