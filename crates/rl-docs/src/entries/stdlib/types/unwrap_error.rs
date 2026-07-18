use crate::entry::FnEntry;

pub static UNWRAP_ERROR: FnEntry = FnEntry {
    signature: "unwrap_error(x)",
    description: "extracts the inner value from x if it is error type",
    example: "get std::types::unwrap_error\n\nunwrap_error(error(1))",
    expected_output: Some("1"),
    returns: "result[T] where T can be any valid type",
    errors: Some("Will return err when x is not error type"),
    see_also: &["is_error"],
    since: Some("v0.1.5"),
};
