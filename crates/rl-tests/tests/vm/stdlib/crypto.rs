use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

// All digest vectors verified against Python hashlib / hmac.

#[test]
fn sha256_empty_known_vector() {
    let result = compile_and_run(
        r#"
get sha256, hex_encode from std::crypto
hex_encode(sha256([]))
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".into())
    );
}

#[test]
fn sha256_hi_known_vector() {
    let result = compile_and_run(
        r#"
get sha256, hex_encode from std::crypto
hex_encode(sha256([104, 105]))
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str("8f434346648f6b96df89dda901c5176b10a6d83961dd3c1ac88b59b2dc327aa4".into())
    );
}

#[test]
fn sha512_empty_known_vector() {
    let result = compile_and_run(
        r#"
get sha512, hex_encode from std::crypto
hex_encode(sha512([]))
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e".into())
    );
}

#[test]
fn sha1_empty_known_vector() {
    let result = compile_and_run(
        r#"
get sha1, hex_encode from std::crypto
hex_encode(sha1([]))
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str("da39a3ee5e6b4b0d3255bfef95601890afd80709".into())
    );
}

#[test]
fn md5_empty_known_vector() {
    let result = compile_and_run(
        r#"
get md5, hex_encode from std::crypto
hex_encode(md5([]))
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str("d41d8cd98f00b204e9800998ecf8427e".into())
    );
}

#[test]
fn hmac_sha256_known_vector() {
    let result = compile_and_run(
        r#"
get hmac_sha256, hex_encode from std::crypto
hex_encode(hmac_sha256([107, 101, 121], [104, 105]))
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str("1c9dc82e5f8e5ed5a0180aad33b8204dea12fde2fb62ffb5e963035bf324a7a4".into())
    );
}

#[test]
fn hmac_sha512_known_vector() {
    let result = compile_and_run(
        r#"
get hmac_sha512, hex_encode from std::crypto
hex_encode(hmac_sha512([107, 101, 121], [104, 105]))
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str("5b27423cfcc1e84ba3968c0553ec1a6b9ba3515246a234dbb3ac5e620f9abfd0c74027f6139d017b80a12219ac527c52a44e1c39757952277ce14456d516d619".into())
    );
}

#[test]
fn constant_time_eq_self_and_other() {
    let result = compile_and_run(
        r#"
get constant_time_eq from std::crypto
constant_time_eq([1, 2, 3], [1, 2, 3])
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    let result = compile_and_run(
        r#"
get constant_time_eq from std::crypto
constant_time_eq([1, 2, 3], [1, 2, 4])
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
    let result = compile_and_run(
        r#"
get constant_time_eq from std::crypto
constant_time_eq([1, 2], [1, 2, 3])
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn base64_round_trip() {
    let result = compile_and_run(
        r#"
get base64_encode, base64_decode from std::crypto
get result_unwrap from std::res
dec string s = base64_encode([104, 105])
dec arr[byte] back = result_unwrap(base64_decode(s))
back
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![VmValue::Byte(104), VmValue::Byte(105)]))
    );
}

#[test]
fn base64_decode_rejects_garbage() {
    let result = compile_and_run(
        r#"
get base64_decode from std::crypto
get is_err from std::res
dec bool x = is_err(base64_decode("!!!"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn base64_url_round_trip() {
    let result = compile_and_run(
        r#"
get base64_url_encode, base64_url_decode from std::crypto
get result_unwrap from std::res
dec string s = base64_url_encode([251, 255])
dec arr[byte] back = result_unwrap(base64_url_decode(s))
back
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![VmValue::Byte(251), VmValue::Byte(255)]))
    );
}

#[test]
fn hex_round_trip() {
    let result = compile_and_run(
        r#"
get hex_encode, hex_decode from std::crypto
get result_unwrap from std::res
dec string s = hex_encode([222, 173])
dec arr[byte] back = result_unwrap(hex_decode(s))
back
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![VmValue::Byte(222), VmValue::Byte(173)]))
    );
}

#[test]
fn hex_decode_rejects_garbage() {
    let result = compile_and_run(
        r#"
get hex_decode from std::crypto
get is_err from std::res
dec bool x = is_err(hex_decode("zz"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn secure_random_bytes_length() {
    let result = compile_and_run(
        r#"
get secure_random_bytes from std::crypto
get result_unwrap from std::res
get len from std::array
dec arr[byte] b = secure_random_bytes(16)
result_unwrap(len(b))
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(16));
}

#[test]
fn secure_token_lengths() {
    let result = compile_and_run(
        r#"
get secure_token, secure_token_hex, secure_token_urlsafe from std::crypto
get result_unwrap from std::res
get len from std::array
dec arr[byte] raw = secure_token(16)
dec string hex = secure_token_hex(16)
dec string url = secure_token_urlsafe(16)
result_unwrap(len(raw)) + result_unwrap(len(hex)) + result_unwrap(len(url))
"#,
    )
    .unwrap();
    // 16 raw bytes, 32 hex chars, 22 unpadded base64url chars.
    assert_eq!(result, VmValue::Int(70));
}

#[test]
fn uuid_v4_shape() {
    let result = compile_and_run(
        r#"
get uuid_v4 from std::crypto
get result_unwrap from std::res
get len from std::array
result_unwrap(len(uuid_v4()))
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(36));
}

#[test]
fn uuid_parse_round_trip_and_reject() {
    let result = compile_and_run(
        r#"
get uuid_parse, uuid_v4 from std::crypto
get result_unwrap, is_err from std::res
dec string id = result_unwrap(uuid_parse(uuid_v4()))
dec bool bad = is_err(uuid_parse("not-a-uuid"))
bad
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn password_round_trip() {
    let result = compile_and_run(
        r#"
get password_hash, password_verify from std::crypto
dec string hash = password_hash("hunter2")
dec bool good = password_verify("hunter2", hash)
dec bool bad = password_verify("hunter3", hash)
good
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    let result = compile_and_run(
        r#"
get password_hash, password_verify from std::crypto
dec string hash = password_hash("hunter2")
password_verify("hunter3", hash)
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}
