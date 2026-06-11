mod to_lower;
mod to_upper;
mod trim;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &["to_lower", "to_upper"];

pub fn module() -> Module {
    Module::new("str")
        .with_function("to_upper", to_upper::std_to_upper)
        .with_function("to_lower", to_lower::std_to_lower)
        .with_function("trim", trim::std_trim)
}
