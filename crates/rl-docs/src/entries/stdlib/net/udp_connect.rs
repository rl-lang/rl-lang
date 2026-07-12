use crate::entry::FnEntry;

pub static UDP_CONNECT: FnEntry = FnEntry {
    signature: "udp_connect(socket, addr)",
    description: "fixes a default peer address for `udp_send`/`udp_recv`, so you don't need to pass the address on every call",
    example: r#"
get std::net::udp_connect

result_unwrap(udp_connect(socket, "127.0.0.1:9001"))"#,
    expected_output: None,
    returns: "Result[null]",
    errors: Some("Err(string) on failure"),
    see_also: &["udp_send", "udp_recv"],
    since: Some("v0.1.5"),
};
