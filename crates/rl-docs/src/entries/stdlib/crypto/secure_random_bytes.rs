use crate::entry::FnEntry;

pub static SECURE_RANDOM_BYTES: FnEntry = FnEntry {
    signature: "secure_random_bytes(n)",
    description: "n cryptographically secure random bytes from the OS. aborts on entropy failure rather than returning predictable bytes. non-positive n returns an empty array. unlike rand_bytes, this is suitable for keys and tokens",
    example: r#"get secure_random_bytes from std::crypto
get len from std::array
get result_unwrap from std::res

dec int n = result_unwrap(len(secure_random_bytes(16)))"#,
    expected_output: None,
    returns: "array[byte]",
    errors: None,
    see_also: &["secure_token", "secure_token_hex", "secure_token_urlsafe"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
