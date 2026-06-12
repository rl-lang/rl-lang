use crate::interpreter::native::Module;

mod insert;
mod pop;
mod push;
mod remove;

pub const KEYWORDS: &[&str] = &["push", "pop", "insert", "remove"];

pub fn module() -> Module {
    Module::new("array")
        .with_function("push", push::std_push)
        .with_function("pop", pop::std_pop)
        .with_function("insert", insert::std_insert)
        .with_function("remove", remove::std_remove)
}
