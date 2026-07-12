use crate::entry::FnEntry;

pub static HTTP_SERVER_TRY_RECV: FnEntry = FnEntry {
    signature: "http_server_try_recv(server)",
    description: "non-blocking variant of http_server_recv; returns null immediately if no request is pending",
    example: r#"
get std::http::http_server_try_recv

dec int req_or_null = result_unwrap(http_server_try_recv(server))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) on a server error"),
    see_also: &["http_server_recv"],
    since: None,
};
