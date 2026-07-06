use crate::docs::entry::FnEntry;

pub static TCP_ACCEPT: FnEntry = FnEntry {
    signature: "tcp_accept(listener)",
    description: "blocks until a client connects, then returns a stream handle for that connection",
    example: r#"
get std::net::tcp_accept

dec int stream = result_unwrap(tcp_accept(listener))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) on accept failure"),
    see_also: &["tcp_listen", "tcp_read", "tcp_write"],
    since: Some("v0.1.5"),
};
