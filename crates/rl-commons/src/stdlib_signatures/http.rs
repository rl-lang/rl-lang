//! Typed signatures for `std::http`.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;
use std::rc::Rc;

pub fn module() -> ModuleNames {
    ModuleNames::new("http")
        .with_functions(&["http_server_try_recv"])
        .with_typed_function(http_server_start())
        .with_typed_function(http_server_recv())
        .with_typed_function(http_request_method())
        .with_typed_function(http_request_url())
        .with_typed_function(http_request_header())
        .with_typed_function(http_request_body())
        .with_typed_function(http_respond())
        .with_typed_function(http_server_stop())
        .with_typed_function(http_get())
        .with_typed_function(http_post())
        .with_typed_function(http_request())
}

fn handle() -> Vec<T> {
    vec![T::Int, T::Byte]
}
fn fixed(t: T) -> Vec<T> {
    vec![t]
}

fn combos(parts: Vec<Vec<T>>) -> Vec<Vec<T>> {
    parts.into_iter().fold(vec![vec![]], |acc, options| {
        acc.into_iter()
            .flat_map(|prefix| {
                options.iter().map(move |o| {
                    let mut next = prefix.clone();
                    next.push(o.clone());
                    next
                })
            })
            .collect()
    })
}

fn overloads(parts: Vec<Vec<T>>, ret: T) -> Vec<(T, T)> {
    combos(parts)
        .into_iter()
        .map(|combo| (params(combo), ret.clone()))
        .collect()
}

fn status_and_body() -> T {
    T::Tuple(Rc::new(vec![T::Int, T::String]))
}

fn http_server_start() -> StdFn {
    StdFn::typed(
        "http_server_start",
        vec![(params(vec![T::String]), result(T::Int))],
    )
}

fn http_server_recv() -> StdFn {
    StdFn::typed(
        "http_server_recv",
        overloads(vec![handle()], result(T::Int)),
    )
}

fn handle_to_string(name: &'static str) -> StdFn {
    StdFn::typed(name, overloads(vec![handle()], result(T::String)))
}
fn http_request_method() -> StdFn {
    handle_to_string("http_request_method")
}
fn http_request_url() -> StdFn {
    handle_to_string("http_request_url")
}
fn http_request_body() -> StdFn {
    handle_to_string("http_request_body")
}

fn http_request_header() -> StdFn {
    StdFn::typed(
        "http_request_header",
        overloads(vec![handle(), fixed(T::String)], result(T::String)),
    )
}

fn http_respond() -> StdFn {
    let mut signatures = overloads(
        vec![handle(), fixed(T::Int), fixed(T::String)],
        result(T::Null),
    );
    signatures.extend(overloads(
        vec![handle(), fixed(T::Int), fixed(T::String), fixed(T::String)],
        result(T::Null),
    ));
    StdFn::typed("http_respond", signatures)
}

fn http_server_stop() -> StdFn {
    StdFn::typed(
        "http_server_stop",
        overloads(vec![handle()], result(T::Null)),
    )
}

fn http_get() -> StdFn {
    StdFn::typed(
        "http_get",
        vec![(params(vec![T::String]), result(status_and_body()))],
    )
}

fn http_post() -> StdFn {
    StdFn::typed(
        "http_post",
        vec![
            (
                params(vec![T::String, T::String]),
                result(status_and_body()),
            ),
            (
                params(vec![T::String, T::String, T::String]),
                result(status_and_body()),
            ),
        ],
    )
}

fn http_request() -> StdFn {
    let header_pair = T::Tuple(Rc::new(vec![T::String, T::String]));
    StdFn::typed(
        "http_request",
        vec![
            (
                params(vec![T::String, T::String]),
                result(status_and_body()),
            ),
            (
                params(vec![T::String, T::String, T::String]),
                result(status_and_body()),
            ),
            (
                params(vec![
                    T::String,
                    T::String,
                    T::String,
                    T::Array(Box::new(header_pair)),
                ]),
                result(status_and_body()),
            ),
        ],
    )
}
