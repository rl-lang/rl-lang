use crate::docs::entry::FnEntry;

pub static TCP_PEER_ADDR: FnEntry = FnEntry {
    signature: "tcp_peer_addr(stream)",
    description: "returns the remote address of a connected TCP stream",
    example: r#"
get std::net::tcp_peer_addr

tcp_peer_addr(stream)"#,
    expected_output: None,
    returns: "Result[string]",
    errors: Some("Err(string) if the stream has no peer address available"),
    see_also: &["tcp_local_addr"],
    since: Some("v0.1.5"),
};
