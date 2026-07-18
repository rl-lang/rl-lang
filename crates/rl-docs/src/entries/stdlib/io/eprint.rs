use crate::entry::FnEntry;

pub static EPRINT: FnEntry = FnEntry {
    signature: "eprint(message)",
    description: "halts evaluation, raising message as a runtime error",
    example: "get std::io::eprint\n\neprint(\"something went wrong\") // error: something went wrong",
    expected_output: None,
    returns: "never returns",
    errors: Some(
        "Always raises `message` as an interpreter-level runtime error - this is\nnot a catchable `result[..]` err, and cannot be caught with `?`. Program\nevaluation stops here.",
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};
