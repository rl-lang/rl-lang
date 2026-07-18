use crate::entry::FnEntry;

pub static RESULT_MAP: FnEntry = FnEntry {
    signature: "result_map(r, fn)",
    description: "applies a function to the ok value and returns a new ok; passes err through unchanged",
    example: "get std::res::result_map\n\ndec result[int] r = ok(5)\ndec result[int] doubled = result_map(r, fn(int x) -> int { return x * 2 })\nresult_unwrap(doubled)",
    expected_output: Some("10"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `r` is not a result\n- calling `fn` fails (the failure message is wrapped as the err value,\n  rather than propagating as an interpreter error)",
    ),
    see_also: &["result_map_err", "result_unwrap"],
    since: Some("v0.1.5"),
};
