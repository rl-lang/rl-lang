use crate::docs::entry::FnEntry;

pub static HTTP_SERVER_START: FnEntry = FnEntry {
    signature: "http_server_start(addr)",
    description: "starts an HTTP server bound to \"host:port\" and returns a server handle",
    example: r#"
get std::http::http_server_start

dec int server = result_unwrap(http_server_start("0.0.0.0:8080"))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) if the address can't be bound"),
    see_also: &["http_server_recv", "http_server_stop"],
    since: Some("v0.1.5"),
};
