use crate::entry::FnEntry;

pub static ENV: FnEntry = FnEntry {
    signature: "env(key)",
    description: "returns the value of an environment variable, or null if not set",
    example: r#"
get std::process::env

env("HOME")"#,
    expected_output: None,
    returns: "string",
    errors: Some(
        r#"
       Will return null on the following:

       - variable value is not set
       - variable name contains `=` or `\0`
               "#,
    ),
    see_also: &["cwd"],
    since: Some("v0.1.5"),
};
