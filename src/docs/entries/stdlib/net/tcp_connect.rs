use crate::docs::entry::FnEntry;

pub static TCP_CONNECT: FnEntry = FnEntry {
    signature: "tcp_connect(addr)",
    description: "connects to \"host:port\" as a client and returns a stream handle",
    example: r#"
get std::net::tcp_connect

dec int stream = result_unwrap(tcp_connect("example.com:80"))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) when the connection fails (refused, timed out, unresolved host)"),
    see_also: &["tcp_read", "tcp_write", "resolve"],
    since: Some("v0.1.5"),
};
