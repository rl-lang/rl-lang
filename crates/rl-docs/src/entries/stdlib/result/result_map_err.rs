use crate::entry::FnEntry;

pub static RESULT_MAP_ERR: FnEntry = FnEntry {
    signature: "result_map_err(r, fn)",
    description: "applies a function to the err value and returns a new err; passes ok through unchanged",
    example: "get std::res::result_map_err\n\ndec result[int] r = err(\"oops\")\ndec result[int] r2 = result_map_err(r, fn(string s) -> string { return concat(\"ERR: \", s) })\nresult_unwrap_err(r2)",
    expected_output: Some("\"ERR: oops\""),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `r` is not a result\n- calling `fn` fails (the failure message is wrapped as the err value,\n  rather than propagating as an interpreter error)",
    ),
    see_also: &["result_map", "result_unwrap_err"],
    since: Some("v0.1.5"),
};
