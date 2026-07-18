use crate::entry::FnEntry;

pub static RESULT_UNWRAP: FnEntry = FnEntry {
    signature: "result_unwrap(r)",
    description: "returns the inner value of an ok result; panics at runtime if r is err",
    example: "get std::res::result_unwrap\n\ndec result[int] r = ok(10)\nresult_unwrap(r)",
    expected_output: Some("10"),
    returns: "T",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) on the following:\n\n- `r` is `err(..)`\n- `r` is not a result at all",
    ),
    see_also: &["result_unwrap_err", "result_unwrap_or", "is_ok"],
    since: Some("v0.1.5"),
};
