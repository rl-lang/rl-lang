use crate::entry::FnEntry;

pub static RESULT_UNWRAP_ERR: FnEntry = FnEntry {
    signature: "result_unwrap_err(r)",
    description: "returns the inner value of an err result; panics at runtime if r is ok",
    example: "get std::res::result_unwrap_err\n\ndec result[int] r = err(\"oops\")\nresult_unwrap_err(r)",
    expected_output: Some("\"oops\""),
    returns: "T",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) on the following:\n\n- `r` is `ok(..)`\n- `r` is not a result at all",
    ),
    see_also: &["result_unwrap", "is_err"],
    since: Some("v0.1.5"),
};
