use crate::docs::entry::FnEntry;

pub static TCP_LOCAL_ADDR: FnEntry = FnEntry {
    signature: "tcp_local_addr(stream)",
    description: "returns the local address of a connected TCP stream",
    example: r#"
get std::net::tcp_local_addr

tcp_local_addr(stream)"#,
    expected_output: None,
    returns: "Result[string]",
    errors: None,
    see_also: &["tcp_peer_addr"],
    since: Some("v0.1.5"),
};
