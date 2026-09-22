use crate::entry::FnEntry;

pub static PASSWORD_VERIFY: FnEntry = FnEntry {
    signature: "password_verify(password, hash)",
    description: "checks a password against an Argon2 hash from password_hash. wrong password or malformed hash returns false",
    example: r#"get password_hash, password_verify from std::crypto

dec string hash = password_hash("hunter2")
dec bool good = password_verify("hunter2", hash)
dec bool bad = password_verify("hunter3", hash)"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["password_hash"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
