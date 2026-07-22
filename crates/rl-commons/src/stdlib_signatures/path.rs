//! Typed signatures for `std::path`.

use super::params;
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("path")
        .with_typed_function(path_exists())
        .with_typed_function(path_extension())
        .with_typed_function(path_filename())
        .with_typed_function(path_is_dir())
        .with_typed_function(path_is_file())
        .with_typed_function(path_join())
        .with_typed_function(path_parent())
        .with_typed_function(path_pop())
        .with_typed_function(path_push())
        .with_typed_function(path_set_extension())
        .with_typed_function(path_stem())
}

fn path_predicate(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String]), T::Bool)])
}
fn path_exists() -> StdFn {
    path_predicate("path_exists")
}
fn path_is_dir() -> StdFn {
    path_predicate("path_is_dir")
}
fn path_is_file() -> StdFn {
    path_predicate("path_is_file")
}

fn path_to_string(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String]), T::String)])
}
fn path_extension() -> StdFn {
    path_to_string("path_extension")
}
fn path_filename() -> StdFn {
    path_to_string("path_filename")
}
fn path_parent() -> StdFn {
    path_to_string("path_parent")
}
fn path_pop() -> StdFn {
    path_to_string("path_pop")
}
fn path_stem() -> StdFn {
    path_to_string("path_stem")
}

fn path_string_string(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String, T::String]), T::String)])
}
fn path_join() -> StdFn {
    path_string_string("path_join")
}
fn path_push() -> StdFn {
    path_string_string("path_push")
}
fn path_set_extension() -> StdFn {
    path_string_string("path_set_extension")
}
