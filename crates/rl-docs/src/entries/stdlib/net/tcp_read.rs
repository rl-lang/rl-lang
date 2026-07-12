use crate::entry::FnEntry;

pub static TCP_READ: FnEntry = FnEntry {
    signature: "tcp_read(stream, max_bytes)",
    description: "reads up to `max_bytes` from a TCP stream; blocks until at least some data arrives unless a timeout/nonblocking mode is set",
    example: r#"
get std::net::tcp_read

dec string data = result_unwrap(tcp_read(stream, 1024))"#,
    expected_output: None,
    returns: "Result[string]",
    errors: Some("Err(string) on a read error"),
    see_also: &["tcp_write", "tcp_set_timeout", "tcp_set_nonblocking"],
    since: Some("v0.1.5"),
};
