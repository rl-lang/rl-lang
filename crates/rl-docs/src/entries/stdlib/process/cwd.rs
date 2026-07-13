use crate::entry::FnEntry;

pub static CWD: FnEntry = FnEntry {
    signature: "cwd()",
    description: "returns the current working directory as a string",
    example: r#"
get std::process::cwd

cwd()?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some(
        r#"
Will return errors on the following:

- current directory does not exist
- user lacks permissions
        "#,
    ),
    see_also: &["set_cwd"],
    since: Some("v0.1.5"),
};
