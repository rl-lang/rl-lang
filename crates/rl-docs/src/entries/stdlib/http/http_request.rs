use crate::docs::entry::FnEntry;

pub static HTTP_REQUEST: FnEntry = FnEntry {
    signature: "http_request(method, url, body?, headers?)",
    description: "generic HTTP request covering any verb (PUT/DELETE/PATCH/HEAD/etc.); `headers` is an array of (string, string) tuples",
    example: r#"
get std::http::http_request

dec (int, string) resp = result_unwrap(http_request("DELETE", "https://example.com/api/1", null, [("Authorization", "Bearer xyz")]))"#,
    expected_output: None,
    returns: "Result[(int, string)]",
    errors: Some("Err(string) on a transport failure - not on non-2xx status"),
    see_also: &["http_get", "http_post"],
    since: Some("v0.1.5"),
};
