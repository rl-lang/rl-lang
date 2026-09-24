use crate::entry::FnEntry;

pub static IS_C_HANDLE: FnEntry = FnEntry {
    signature: "is_c_handle(v)",
    description: "true if v is a C handle",
    example: "get std::types::is_c_handle\n\nis_c_handle(c_handle())",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_net_handle", "is_http_handle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
