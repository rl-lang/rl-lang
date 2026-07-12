use crate::docs::entry::FnEntry;

pub static TCP_CLOSE: FnEntry = FnEntry {
    signature: "tcp_close(handle)",
    description: "closes a TCP listener or stream handle and frees its slot; using the handle again afterward errors",
    example: r#"
get std::net::tcp_close

tcp_close(stream)"#,
    expected_output: None,
    returns: "Result[null]",
    errors: None,
    see_also: &["tcp_shutdown"],
    since: Some("v0.1.5"),
};
