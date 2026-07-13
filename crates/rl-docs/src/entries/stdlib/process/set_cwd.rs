use crate::entry::FnEntry;

pub static SET_CWD: FnEntry = FnEntry {
    signature: "set_cwd(path)",
    description: "changes the current working directory",
    example: r#"
get std::process::set_cwd

set_cwd("/tmp")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"
    Will return errors on the following:

    - operation failed
            "#,
    ),
    see_also: &["cwd"],
    since: Some("v0.1.5"),
};
