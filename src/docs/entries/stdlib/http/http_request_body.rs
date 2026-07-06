use crate::docs::entry::FnEntry;

pub static HTTP_REQUEST_BODY: FnEntry = FnEntry {
    signature: "http_request_body(req)",
    description: "reads the request body as a string; the body can only be read once",
    example: r#"
get std::http::http_request_body

dec string body = result_unwrap(http_request_body(req))"#,
    expected_output: None,
    returns: "Result[string]",
    errors: Some("Err(string) on a read error"),
    see_also: &["http_request_header"],
    since: Some("v0.1.5"),
};
