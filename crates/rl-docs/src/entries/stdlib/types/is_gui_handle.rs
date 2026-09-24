use crate::entry::FnEntry;

pub static IS_GUI_HANDLE: FnEntry = FnEntry {
    signature: "is_gui_handle(v)",
    description: "true if v is a GUI handle",
    example: "get std::types::is_gui_handle\n\nis_gui_handle(gui_handle())",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_audio_handle", "is_file_handle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
