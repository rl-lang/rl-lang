use crate::docs::entry::FnEntry;

pub static HTTP_REQUEST_URL: FnEntry = FnEntry {
    signature: "http_request_url(req)",
    description: "returns the requested path, including query string",
    example: r#"
get std::http::http_request_url

http_request_url(req)"#,
    expected_output: None,
    returns: "Result[string]",
    errors: None,
    see_also: &["http_request_method"],
    since: Some("v0.1.5"),
};
