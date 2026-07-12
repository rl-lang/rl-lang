use crate::entry::FnEntry;

pub static TCP_SHUTDOWN: FnEntry = FnEntry {
    signature: "tcp_shutdown(stream, mode)",
    description: "shuts down the read side, write side, or both, of a TCP stream; `mode` is \"read\", \"write\", or \"both\"",
    example: r#"
get std::net::tcp_shutdown

result_unwrap(tcp_shutdown(stream, "both"))"#,
    expected_output: None,
    returns: "Result[null]",
    errors: Some("Err(string) on shutdown failure"),
    see_also: &["tcp_close"],
    since: Some("v0.1.5"),
};
