use crate::entry::FnEntry;

pub static IS_AUDIO_HANDLE: FnEntry = FnEntry {
    signature: "is_audio_handle(v)",
    description: "true if v is an audio handle",
    example: "get std::types::is_audio_handle\n\nis_audio_handle(audio_handle())",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_gui_handle", "is_file_handle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
