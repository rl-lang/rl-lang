use crate::docs::entry::FnEntry;

pub static HTTP_GET: FnEntry = FnEntry {
    signature: "http_get(url)",
    description: "performs an HTTP GET request, returning `(status, body)`; non-2xx statuses are still a successful result, not an error",
    example: r#"
get std::http::http_get

dec (int, string) resp = result_unwrap(http_get("https://example.com"))"#,
    expected_output: None,
    returns: "Result[(int, string)]",
    errors: Some(
        "Err(string) on a transport failure (DNS, connection, timeout) - not on non-2xx status",
    ),
    see_also: &["http_post", "http_request"],
    since: Some("v0.1.5"),
};
