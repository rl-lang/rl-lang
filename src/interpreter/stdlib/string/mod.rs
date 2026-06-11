mod to_lower;
mod to_upper;
mod trim;
mod trim_end;
mod trim_start;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &["to_lower", "to_upper", "trim", "trim_end", "trim_start"];

pub fn module() -> Module {
    Module::new("str")
        .with_function("to_upper", to_upper::std_to_upper)
        .with_function("to_lower", to_lower::std_to_lower)
        .with_function("trim", trim::std_trim)
        .with_function("trim_end", trim_end::std_trim_end)
        .with_function("trim_start", trim_start::std_trim_start)
}
