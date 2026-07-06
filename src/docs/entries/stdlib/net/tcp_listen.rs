use crate::docs::entry::FnEntry;

pub static TCP_LISTEN: FnEntry = FnEntry {
    signature: "tcp_listen(addr)",
    description: "binds a TCP listener to \"host:port\" and returns a listener handle",
    example: r#"
get std::net::tcp_listen

dec int listener = result_unwrap(tcp_listen("127.0.0.1:7878"))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) when the address can't be bound (e.g. already in use)"),
    see_also: &["tcp_accept", "tcp_close"],
    since: Some("v0.1.5"),
};
