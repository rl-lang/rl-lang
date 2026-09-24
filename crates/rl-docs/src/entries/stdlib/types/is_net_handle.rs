use crate::entry::FnEntry;

pub static IS_NET_HANDLE: FnEntry = FnEntry {
    signature: "is_net_handle(v)",
    description: "true if v is a network handle",
    example: "get std::types::is_net_handle\n\nis_net_handle(net_handle())",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_c_handle", "is_http_handle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
