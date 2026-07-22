//! Typed signatures for `std::fs`.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("fs")
        .with_typed_function(mkdir())
        .with_typed_function(mkdir_all())
        .with_typed_function(rmdir())
        .with_typed_function(rmdir_all())
        .with_typed_function(list_dir())
        .with_typed_function(copy_file())
        .with_typed_function(move_file())
        .with_typed_function(file_size())
        .with_typed_function(file_modified())
        .with_typed_function(temp_dir())
        .with_typed_function(rename_file())
}

fn path_to_null_result(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String]), result(T::Null))])
}
fn mkdir() -> StdFn {
    path_to_null_result("mkdir")
}
fn mkdir_all() -> StdFn {
    path_to_null_result("mkdir_all")
}
fn rmdir() -> StdFn {
    path_to_null_result("rmdir")
}
fn rmdir_all() -> StdFn {
    path_to_null_result("rmdir_all")
}

fn list_dir() -> StdFn {
    StdFn::typed(
        "list_dir",
        vec![(
            params(vec![T::String]),
            result(T::Array(Box::new(T::String))),
        )],
    )
}

fn copy_file() -> StdFn {
    StdFn::typed(
        "copy_file",
        vec![(params(vec![T::String, T::String]), result(T::Int))],
    )
}

fn move_file() -> StdFn {
    StdFn::typed(
        "move_file",
        vec![(params(vec![T::String, T::String]), result(T::Null))],
    )
}

fn path_to_int_result(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String]), result(T::Int))])
}
fn file_size() -> StdFn {
    path_to_int_result("file_size")
}
fn file_modified() -> StdFn {
    path_to_int_result("file_modified")
}

fn temp_dir() -> StdFn {
    StdFn::typed("temp_dir", vec![(params(vec![]), T::String)])
}

fn rename_file() -> StdFn {
    StdFn::typed(
        "rename_file",
        vec![(params(vec![T::String, T::String]), result(T::String))],
    )
}
