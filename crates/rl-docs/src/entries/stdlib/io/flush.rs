use crate::entry::FnEntry;

pub static FLUSH: FnEntry = FnEntry {
    signature: "flush(handle)",
    description: "flushes buffered writes to disk",
    example: r#"get std::io::open
get std::io::write
get std::io::flush
get std::io::close

dec file = open("out.txt", "w")?
write(file, "data")?
flush(file)?
close(file)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the handle is invalid or not open for writing"),
    see_also: &["write_handle", "close"],
    since: Some("v2.1.0"),
    deprecated: Some("moved to std::fs::flush"),
    updated: Some("v2.1.0"),
};
