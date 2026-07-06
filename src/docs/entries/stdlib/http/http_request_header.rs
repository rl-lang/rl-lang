use crate::docs::entry::FnEntry;

pub static HTTP_REQUEST_HEADER: FnEntry = FnEntry {
    signature: "http_request_header(req, name)",
    description: "looks up a request header by name (case-insensitive)",
    example: r#"
get std::http::http_request_header

http_request_header(req, "Content-Type")"#,
    expected_output: None,
    returns: "Result[string]",
    errors: Some("Err(string) when the header isn't present"),
    see_also: &["http_request_body"],
    since: Some("v0.1.5"),
};
