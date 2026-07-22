use crate::entry::FnEntry;

pub static TERM_READ_KEY: FnEntry = FnEntry {
    signature: "term_read_key()",
    description: "blocks until a key, mouse, or resize event occurs and returns it",
    example: r#"get std::term::term_read_key

term_read_key()?"#,
    expected_output: Some("[\"Char:a\"]"),
    returns: "result[arr[string]]",
    errors: Some(
        r#"Will return error if reading the input event fails.

Note: the array's length depends on the event. Key press and focus
events return a 1-element array (e.g. `["Char:a"]`, `["Enter"]`,
`["Ctrl:c"]`, `["FocusGained"]`); mouse and resize events return a
3-element array instead (e.g. `["MouseLeft", "10", "5"]`,
`["Resize", "80", "24"]`)."#,
    ),
    see_also: &["term_poll"],
    since: Some("v0.4.0"),
};
