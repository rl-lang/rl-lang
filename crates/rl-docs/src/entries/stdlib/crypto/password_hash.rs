use crate::entry::FnEntry;

pub static PASSWORD_HASH: FnEntry = FnEntry {
    signature: "password_hash(password)",
    description: "Argon2 hash of a password with a fresh random salt, returned as an encoded string. store this, never the password. each call produces a different string for the same password",
    example: r#"get password_hash, password_verify from std::crypto

dec string hash = password_hash("hunter2")
dec bool good = password_verify("hunter2", hash)"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["password_verify", "secure_random_bytes"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
