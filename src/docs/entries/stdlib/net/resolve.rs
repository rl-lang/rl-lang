use crate::docs::entry::FnEntry;

pub static RESOLVE: FnEntry = FnEntry {
    signature: "resolve(host_port)",
    description: "resolves \"host:port\" via DNS and returns the resolved IP addresses as strings",
    example: r#"
get std::net::resolve

dec array[string] ips = result_unwrap(resolve("example.com:80"))"#,
    expected_output: None,
    returns: "Result[array[string]]",
    errors: Some("Err(string) when the host can't be resolved"),
    see_also: &["tcp_connect"],
    since: Some("v0.1.5"),
};
