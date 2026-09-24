use crate::entry::FnEntry;

pub static IS_FILE_HANDLE: FnEntry = FnEntry {
    signature: "is_file_handle(v)",
    description: "true if v is a file handle",
    example: "get std::types::is_file_handle\n\nis_file_handle(file_handle())",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_gui_handle", "is_audio_handle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
