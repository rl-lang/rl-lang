pub mod bit_and;
pub mod bit_not;
pub mod bit_or;
pub mod bit_xor;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &["bit_and", "bit_or", "bit_xor", "bit_not"];

pub fn module() -> Module {
    Module::new("bitwise")
        .with_raw_function("bit_and", bit_and::std_bit_and)
        .with_raw_function("bit_or", bit_or::std_bit_or)
        .with_raw_function("bit_xor", bit_xor::std_bit_xor)
        .with_raw_function("bit_not", bit_not::std_bit_not)
}
