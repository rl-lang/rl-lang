use crate::entry::FnEntry;

pub static RAND_BYTES: FnEntry = FnEntry {
    signature: "rand_bytes(count)",
    description: "returns an array of count random values, each in 0 to 255",
    example: "get std::random::rand_bytes\n\nrand_bytes(4)?",
    expected_output: None,
    returns: "result[arr[int]]",
    errors: Some(
        "Will return error if `count` is exactly 0.\n\nNote: despite the name, this returns `arr[int]`, not `arr[byte]` -\neach element is an `int` in 0-255, not a `byte` value. Also, a negative\n`count` is not caught by the current validation and silently returns an\nempty array instead of an error.",
    ),
    see_also: &["rand_byte", "rand_string"],
    since: Some("v0.1.5"),
};
