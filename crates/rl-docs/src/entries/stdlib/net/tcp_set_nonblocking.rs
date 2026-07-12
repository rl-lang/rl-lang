use crate::entry::FnEntry;

pub static TCP_SET_NONBLOCKING: FnEntry = FnEntry {
    signature: "tcp_set_nonblocking(stream, flag)",
    description: "toggles nonblocking mode; reads/writes return immediately with a `WouldBlock`-style error instead of blocking",
    example: r#"
get std::net::tcp_set_nonblocking

result_unwrap(tcp_set_nonblocking(stream, true))"#,
    expected_output: None,
    returns: "Result[null]",
    errors: Some("Err(string) if the mode can't be set"),
    see_also: &["tcp_set_timeout"],
    since: Some("v0.1.5"),
};
