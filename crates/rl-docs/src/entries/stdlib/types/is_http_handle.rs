use crate::entry::FnEntry;

pub static IS_HTTP_HANDLE: FnEntry = FnEntry {
    signature: "is_http_handle(v)",
    description: "true if v is an HTTP handle",
    example: "get std::types::is_http_handle\n\nis_http_handle(http_handle())",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_net_handle", "is_c_handle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
