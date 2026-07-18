use crate::entry::FnEntry;

static PAD_RIGHT: FnEntry = FnEntry {
    signature: "pad_right(str, width, char)",
    description: "pads str on the right with char until the total length reaches width",
    example: "get std::str::pad_right\n\npad_right(\"hi\", 5, '.')",
    expected_output: Some("\"hi...\""),
    returns: "string",
    errors: None,
    see_also: &["pad_left"],
    since: Some("v0.1.5"),
};
