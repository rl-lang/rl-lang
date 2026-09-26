use crate::entry::FnEntry;

pub static FLUSH: FnEntry = FnEntry {
    signature: "flush(handle)",
    description: "flushes buffered writes to disk",
    example: r#"get std::fs::open
get std::fs::write
get std::fs::flush
get std::fs::close

dec file = open("out.txt", "w")?
write(file, "data")?
flush(file)?
close(file)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the handle is invalid or not open for writing"),
    see_also: &["write_handle", "close"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
