use crate::docs::entry::FnEntry;

pub static HTTP_SERVER_STOP: FnEntry = FnEntry {
    signature: "http_server_stop(server)",
    description: "stops and closes an HTTP server handle",
    example: r#"
get std::http::http_server_stop

http_server_stop(server)"#,
    expected_output: None,
    returns: "Result[null]",
    errors: None,
    see_also: &["http_server_start"],
    since: Some("v0.1.5"),
};
