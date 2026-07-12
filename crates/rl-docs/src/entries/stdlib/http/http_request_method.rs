use crate::docs::entry::FnEntry;

pub static HTTP_REQUEST_METHOD: FnEntry = FnEntry {
    signature: "http_request_method(req)",
    description: "returns the HTTP method of a request (e.g. \"GET\", \"POST\")",
    example: r#"
get std::http::http_request_method

http_request_method(req)"#,
    expected_output: None,
    returns: "Result[string]",
    errors: None,
    see_also: &["http_request_url"],
    since: Some("v0.1.5"),
};
