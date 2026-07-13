use crate::entry::FnEntry;

pub static ARGS: FnEntry = FnEntry {
    signature: "args()",
    description: "returns the command-line arguments passed to the script as an array of strings",
    example: r#"
get std::process::args

dec arr[string] a = args()"#,
    expected_output: None,
    returns: "arr[string]",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
