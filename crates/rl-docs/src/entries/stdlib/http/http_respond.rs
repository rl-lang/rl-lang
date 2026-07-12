use crate::docs::entry::FnEntry;

pub static HTTP_RESPOND: FnEntry = FnEntry {
    signature: "http_respond(req, status, body, content_type?)",
    description: "sends a response for `req` and consumes the request handle; using the handle again afterward errors",
    example: r#"
get std::http::http_respond

http_respond(req, 200, "hello world", "text/plain")"#,
    expected_output: None,
    returns: "Result[null]",
    errors: Some("Err(string) on a write/send failure"),
    see_also: &["http_server_recv"],
    since: Some("v0.1.5"),
};
