use crate::docs::entry::FnEntry;

pub static UDP_RECV: FnEntry = FnEntry {
    signature: "udp_recv(socket, max_bytes)",
    description: "receives up to `max_bytes` from the socket's connected peer",
    example: r#"
get std::net::udp_recv

dec string data = result_unwrap(udp_recv(socket, 1024))"#,
    expected_output: None,
    returns: "Result[string]",
    errors: Some("Err(string) on receive failure"),
    see_also: &["udp_recv_from", "udp_send"],
    since: Some("v0.1.5"),
};
