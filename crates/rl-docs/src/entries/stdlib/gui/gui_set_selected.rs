use crate::entry::FnEntry;

pub static GUI_SET_SELECTED: FnEntry = FnEntry {
    signature: "gui_set_selected(selectable, selected)",
    description: "sets a selectable row's highlighted state. Selection is RL-owned: the widget never flips itself",
    example: r#"gui_set_selected(row, true)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_selectable", "gui_is_selected"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
