//! Typed signatures for `std::process`.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("process")
        .with_functions(&["env"])
        .with_typed_function(args())
        .with_typed_function(exit())
        .with_typed_function(cwd())
        .with_typed_function(set_cwd())
        .with_typed_function(pid())
        .with_typed_function(sleep())
        .with_typed_function(exec())
        .with_typed_function(exec_code())
        .with_typed_function(exec_lines())
}

fn args() -> StdFn {
    StdFn::typed(
        "args",
        vec![(params(vec![]), T::Array(Box::new(T::String)))],
    )
}

fn exit() -> StdFn {
    StdFn::typed("exit", vec![(params(vec![T::Int]), T::Null)])
}

fn cwd() -> StdFn {
    StdFn::typed("cwd", vec![(params(vec![]), result(T::String))])
}

fn set_cwd() -> StdFn {
    StdFn::typed("set_cwd", vec![(params(vec![T::String]), result(T::Null))])
}

fn pid() -> StdFn {
    StdFn::typed("pid", vec![(params(vec![]), T::Int)])
}

fn sleep() -> StdFn {
    StdFn::typed("sleep", vec![(params(vec![T::Int]), T::Null)])
}

fn exec() -> StdFn {
    StdFn::typed("exec", vec![(params(vec![T::String]), result(T::String))])
}

fn exec_code() -> StdFn {
    StdFn::typed("exec_code", vec![(params(vec![T::String]), result(T::Int))])
}

fn exec_lines() -> StdFn {
    StdFn::typed(
        "exec_lines",
        vec![(
            params(vec![T::String]),
            result(T::Array(Box::new(T::String))),
        )],
    )
}
