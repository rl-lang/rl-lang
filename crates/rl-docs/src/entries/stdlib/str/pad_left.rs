use crate::entry::FnEntry;

pub static PAD_LEFT: FnEntry = FnEntry {
    signature: "pad_left(str, width, char)",
    description: "pads str on the left with char until the total length reaches width",
    example: "get std::str::pad_left\n\npad_left(\"5\", 3, '0')",
    expected_output: Some("\"005\""),
    returns: "string",
    errors: None,
    see_also: &["pad_right"],
    since: Some("v0.1.5"),
};
