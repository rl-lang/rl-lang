use crate::docs::entry::FnEntry;

pub static UDP_SEND_TO: FnEntry = FnEntry {
    signature: "udp_send_to(socket, data, addr)",
    description: "sends `data` to `addr` directly, without needing a connected peer",
    example: r#"
get std::net::udp_send_to

result_unwrap(udp_send_to(socket, "ping", "127.0.0.1:9001"))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) on send failure"),
    see_also: &["udp_send", "udp_recv_from"],
    since: Some("v0.1.5"),
};
