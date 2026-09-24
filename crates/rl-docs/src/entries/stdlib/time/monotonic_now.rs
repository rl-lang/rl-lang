use crate::entry::FnEntry;

pub static MONOTONIC_NOW: FnEntry = FnEntry {
    signature: "monotonic_now()",
    description: "returns the monotonic clock time in seconds since an arbitrary point, useful for benchmarks and elapsed time",
    example: r#"get std::time::monotonic_now

dec float start = monotonic_now()
# ... do work ...
dec float elapsed = monotonic_now() - start"#,
    expected_output: Some("12345.678"),
    returns: "float",
    errors: None,
    see_also: &["now", "now_ms", "time_diff"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
