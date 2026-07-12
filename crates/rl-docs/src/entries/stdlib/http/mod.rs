use crate::docs::entry::{FnEntry, StdEntry};

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

pub static HTTP: StdEntry = StdEntry {
    name: "http",
    description: "a minimal HTTP server (tiny_http) and blocking HTTP client (ureq)",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &http_server_start::HTTP_SERVER_START,
    &http_server_recv::HTTP_SERVER_RECV,
    &http_server_try_recv::HTTP_SERVER_TRY_RECV,
    &http_request_method::HTTP_REQUEST_METHOD,
    &http_request_url::HTTP_REQUEST_URL,
    &http_request_header::HTTP_REQUEST_HEADER,
    &http_request_body::HTTP_REQUEST_BODY,
    &http_respond::HTTP_RESPOND,
    &http_server_stop::HTTP_SERVER_STOP,
    &http_get::HTTP_GET,
    &http_post::HTTP_POST,
    &http_request::HTTP_REQUEST,
];
