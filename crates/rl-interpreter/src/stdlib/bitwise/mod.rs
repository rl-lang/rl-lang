//! `std::bitwise` - bitwise operations on `byte` and `int` values.
//!
//! Mixed `byte`/`int` operands widen to `int`. `bit_xor` requires matching types.

pub mod bit_and;
pub mod bit_not;
pub mod bit_or;
pub mod bit_shift_left;
pub mod bit_shift_right;
pub mod bit_xor;
pub mod count_bits;
pub mod leading_zeros;
pub mod trailing_zeros;

use crate::native::Module;

pub use rl_commons::keywords::bitwise::KEYWORDS;

pub fn module() -> Module {
    Module::new("bitwise")
        .with_function("bit_and", bit_and::std_bit_and)
        .with_function("bit_or", bit_or::std_bit_or)
        .with_function("bit_xor", bit_xor::std_bit_xor)
        .with_function("bit_not", bit_not::std_bit_not)
        .with_function("bit_shift_left", bit_shift_left::std_bit_shift_left)
        .with_function("bit_shift_right", bit_shift_right::std_bit_shift_right)
        .with_function("count_bits", count_bits::std_count_bits)
        .with_function("leading_zeros", leading_zeros::std_leading_zeros)
        .with_function("trailing_zeros", trailing_zeros::std_trailing_zeros)
}
