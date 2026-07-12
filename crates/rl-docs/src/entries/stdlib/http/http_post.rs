use crate::entry::FnEntry;

pub static HTTP_POST: FnEntry = FnEntry {
    signature: "http_post(url, body, content_type?)",
    description: "performs an HTTP POST request with the given body, returning `(status, response_body)`",
    example: r#"
get std::http::http_post

dec (int, string) resp = result_unwrap(http_post("https://example.com/api", "{}", "application/json"))"#,
    expected_output: None,
    returns: "Result[(int, string)]",
    errors: Some("Err(string) on a transport failure - not on non-2xx status"),
    see_also: &["http_get", "http_request"],
    since: Some("v0.1.5"),
};
