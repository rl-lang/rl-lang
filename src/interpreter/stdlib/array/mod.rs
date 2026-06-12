use crate::interpreter::native::Module;

mod pop;
mod push;

pub const KEYWORDS: &[&str] = &["push", "pop"];

pub fn module() -> Module {
    Module::new("array")
        .with_function("push", push::std_push)
        .with_function("pop", pop::std_pop)
}
