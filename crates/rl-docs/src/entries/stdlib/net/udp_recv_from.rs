use crate::entry::FnEntry;

pub static UDP_RECV_FROM: FnEntry = FnEntry {
    signature: "udp_recv_from(socket, max_bytes)",
    description: "receives up to `max_bytes` from any sender, returning `(data, sender_addr)`",
    example: r#"
get std::net::udp_recv_from

dec (string, string) result = result_unwrap(udp_recv_from(socket, 1024))"#,
    expected_output: None,
    returns: "Result[(string, string)]",
    errors: Some("Err(string) on receive failure"),
    see_also: &["udp_recv", "udp_send_to"],
    since: Some("v0.1.5"),
};
