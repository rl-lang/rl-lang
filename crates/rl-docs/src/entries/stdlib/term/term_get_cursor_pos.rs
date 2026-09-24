use crate::entry::FnEntry;

pub static TERM_GET_CURSOR_POS: FnEntry = FnEntry {
    signature: "term_get_cursor_pos()",
    description: "returns the current cursor position as [column, row]",
    example: r#"get std::term::term_get_cursor_pos

term_get_cursor_pos()?"#,
    expected_output: Some("[0, 0]"),
    returns: "result[array[int]]",
    errors: Some("Will return error if the cursor position cannot be queried"),
    see_also: &["term_move", "term_save_cursor"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
