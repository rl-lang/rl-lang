use crate::entry::FnEntry;

pub static SYSCALL6: FnEntry = FnEntry {
    signature: "__syscall6(nr, a1, a2, a3, a4, a5, a6)",
    description: "intrinsic: raw Linux syscall trap. returns the raw register (negative means -errno, interpreted by the caller). Linux-only by construction; portable code uses std::fs / std::io instead",
    example: r#"get __syscall6 from core
get os_name from std::process

dec int pid = 0
if os_name() == "linux" {
    pid = __syscall6(39, 0, 0, 0, 0, 0, 0)
}"#,
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["__abort"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
