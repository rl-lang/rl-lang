use crate::entry::FnEntry;

pub static UDP_BIND: FnEntry = FnEntry {
    signature: "udp_bind(addr)",
    description: "binds a UDP socket to \"host:port\" and returns a socket handle",
    example: r#"
get std::net::udp_bind

dec int socket = result_unwrap(udp_bind("127.0.0.1:9000"))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) if the address can't be bound"),
    see_also: &["udp_recv_from", "udp_close"],
    since: Some("v0.1.5"),
};
