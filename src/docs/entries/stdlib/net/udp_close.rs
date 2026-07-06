use crate::docs::entry::FnEntry;

pub static UDP_CLOSE: FnEntry = FnEntry {
    signature: "udp_close(socket)",
    description: "closes a UDP socket handle and frees its slot",
    example: r#"
get std::net::udp_close

udp_close(socket)"#,
    expected_output: None,
    returns: "Result[null]",
    errors: None,
    see_also: &["udp_bind"],
    since: Some("v0.1.5"),
};
