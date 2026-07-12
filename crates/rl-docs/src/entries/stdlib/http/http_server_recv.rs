use crate::entry::FnEntry;

pub static HTTP_SERVER_RECV: FnEntry = FnEntry {
    signature: "http_server_recv(server)",
    description: "blocks until the next request arrives, returning a request handle",
    example: r#"
get std::http::http_server_recv

dec int req = result_unwrap(http_server_recv(server))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) on a server error"),
    see_also: &["http_server_try_recv", "http_respond"],
    since: Some("v0.1.5"),
};
