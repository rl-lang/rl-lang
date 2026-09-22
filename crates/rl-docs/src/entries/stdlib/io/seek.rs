use crate::entry::FnEntry;

pub static SEEK: FnEntry = FnEntry {
    signature: "seek(handle, offset, whence)",
    description: "repositions the read/write offset. whence: 0=start, 1=current, 2=end",
    example: r#"get std::io::open
get std::io::seek
get std::io::read
get std::io::close

dec file = open("data.txt", "r")?
seek(file, 10, 0)?
dec string chunk = read(file, 5)?
close(file)?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if the handle is invalid or whence is not 0, 1, or 2"),
    see_also: &["open", "read_handle"],
    since: Some("v2.1.0"),
    deprecated: Some("moved to std::fs::seek"),
    updated: Some("v2.1.0"),
};
