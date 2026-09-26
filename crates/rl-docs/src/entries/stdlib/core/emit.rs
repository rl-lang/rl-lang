use crate::entry::FnEntry;

pub static EMIT: FnEntry = FnEntry {
    signature: "__emit(text)",
    description: "intrinsic: streams one progress message from inside a worker thread; the next `__poll` on the main thread picks it up. Calling it outside a worker is an err, not an abort",
    example: r#"get __spawn, __emit from core

__spawn("get __emit from core\n__emit(\"half\")\n\"done\"")"#,
    expected_output: None,
    returns: "null",
    errors: Some("err outside a worker thread"),
    see_also: &["__spawn", "__poll"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
