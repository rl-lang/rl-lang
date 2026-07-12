//! `std::http` - a minimal HTTP server (`tiny_http`) and client (`ureq`).

use crate::native::Module;

mod common;
mod http_get;
mod http_post;
mod http_request;
mod http_request_body;
mod http_request_header;
mod http_request_method;
mod http_request_url;
mod http_respond;
mod http_server_recv;
mod http_server_start;
mod http_server_stop;
mod http_server_try_recv;

pub const KEYWORDS: &[&str] = &[
    "http_server_start",
    "http_server_recv",
    "http_server_try_recv",
    "http_request_method",
    "http_request_url",
    "http_request_header",
    "http_request_body",
    "http_respond",
    "http_server_stop",
    "http_get",
    "http_post",
    "http_request",
];

/// A single native HTTP resource, stored behind an `int` handle.
pub enum HttpHandle {
    Server(tiny_http::Server),
    Request(tiny_http::Request),
}

pub fn module() -> Module {
    Module::new("http")
        .with_function("http_server_start", http_server_start::func)
        .with_function("http_server_recv", http_server_recv::func)
        .with_function("http_server_try_recv", http_server_try_recv::func)
        .with_function("http_request_method", http_request_method::func)
        .with_function("http_request_url", http_request_url::func)
        .with_function("http_request_header", http_request_header::func)
        .with_function("http_request_body", http_request_body::func)
        .with_raw_function("http_respond", http_respond::func)
        .with_function("http_server_stop", http_server_stop::func)
        .with_function("http_get", http_get::func)
        .with_raw_function("http_post", http_post::func)
        .with_raw_function("http_request", http_request::func)
}
