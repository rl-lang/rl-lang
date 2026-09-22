//! `std::crypto` - hashing, HMAC, tokens, passwords.
//!
//! Byte-oriented functions take and return `array[byte]` (no new value
//! type). Integer literals coerce element-wise when in range (`[104, 105]`
//! works; out-of-range is a runtime type error). Digests are raw bytes;
//! use `hex_encode` / `base64_encode` to display them. SHA-1 and MD5 exist
//! for legacy checksum verification only, never for security. There is
//! deliberately no symmetric encryption here (key management is out of
//! scope).
//!
//! ```rl
//! get sha256, hex_encode from std::crypto
//! dec string hex = hex_encode(sha256([104, 105]))
//! ```

#[cfg(feature = "impls")]
use rl_std_core::Runtime;
#[cfg(feature = "impls")]
use rl_std_core::Bytes;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use digest::Digest;
#[cfg(feature = "impls")]
use rl_ast::statements::TypeAnnotation;

// ---- hashes ---------------------------------------------------------------

#[native_fn(module = "crypto")]
pub fn sha256(data: Bytes) -> Bytes {
    Bytes(sha2::Sha256::digest(&data.0).to_vec())
}

#[native_fn(module = "crypto")]
pub fn sha512(data: Bytes) -> Bytes {
    Bytes(sha2::Sha512::digest(&data.0).to_vec())
}

#[native_fn(module = "crypto")]
pub fn sha1(data: Bytes) -> Bytes {
    // Leading `::`: the function name shadows the crate in paths.
    Bytes(::sha1::Sha1::digest(&data.0).to_vec())
}

#[native_fn(module = "crypto")]
pub fn md5(data: Bytes) -> Bytes {
    Bytes(::md5::Md5::digest(&data.0).to_vec())
}

// ---- hmac -----------------------------------------------------------------

#[native_fn(module = "crypto")]
pub fn hmac_sha256(key: Bytes, data: Bytes) -> Bytes {
    use digest::Mac;
    // HMAC accepts any key length (long keys hash down), so this only
    // fails on a broken build; abort loudly instead of forging a MAC.
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(&key.0)
        .expect("hmac_sha256: invalid key");
    mac.update(&data.0);
    Bytes(mac.finalize().into_bytes().to_vec())
}

#[native_fn(module = "crypto")]
pub fn hmac_sha512(key: Bytes, data: Bytes) -> Bytes {
    use digest::Mac;
    let mut mac = hmac::Hmac::<sha2::Sha512>::new_from_slice(&key.0)
        .expect("hmac_sha512: invalid key");
    mac.update(&data.0);
    Bytes(mac.finalize().into_bytes().to_vec())
}

// ---- constant-time compare ------------------------------------------------

#[native_fn(module = "crypto")]
pub fn constant_time_eq(a: Bytes, b: Bytes) -> bool {
    use subtle::ConstantTimeEq;
    // Slice ct_eq is safe on differing lengths (returns false).
    a.0.as_slice().ct_eq(b.0.as_slice()).into()
}

// ---- random -----------------------------------------------------------------

#[cfg(feature = "impls")]
fn random_bytes(count: i64) -> Vec<u8> {
    if count <= 0 {
        return Vec::new();
    }
    let mut buf = vec![0u8; count as usize];
    // Entropy failure means a broken platform; abort loudly rather than
    // handing out predictable bytes.
    getrandom::getrandom(&mut buf).expect("secure_random_bytes: OS entropy failure");
    buf
}

#[native_fn(module = "crypto")]
pub fn secure_random_bytes(count: i64) -> Bytes {
    Bytes(random_bytes(count))
}

#[native_fn(module = "crypto")]
pub fn secure_token(count: i64) -> Bytes {
    Bytes(random_bytes(count))
}

#[native_fn(module = "crypto")]
pub fn secure_token_hex(count: i64) -> String {
    hex::encode(random_bytes(count))
}

#[native_fn(module = "crypto")]
pub fn secure_token_urlsafe(count: i64) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(random_bytes(count))
}

// ---- base64 / hex -----------------------------------------------------------

#[native_fn(module = "crypto")]
pub fn base64_encode(data: Bytes) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(&data.0)
}

#[cfg(feature = "impls")]
fn ok_bytes<R: Runtime>(bytes: Vec<u8>) -> R::Value {
    R::ok(R::array(
        bytes.into_iter().map(R::from_u8).collect(),
        TypeAnnotation::Byte,
    ))
}

#[native_fn(module = "crypto", sig(string -> result[array[byte]]))]
pub fn base64_decode<R: Runtime>(s: String) -> R::Value {
    use base64::Engine;
    match base64::engine::general_purpose::STANDARD.decode(s.as_bytes()) {
        Ok(bytes) => ok_bytes::<R>(bytes),
        Err(e) => R::err(R::from_string(format!("base64_decode: {e}"))),
    }
}

#[native_fn(module = "crypto")]
pub fn base64_url_encode(data: Bytes) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&data.0)
}

#[native_fn(module = "crypto", sig(string -> result[array[byte]]))]
pub fn base64_url_decode<R: Runtime>(s: String) -> R::Value {
    use base64::Engine;
    match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(s.as_bytes()) {
        Ok(bytes) => ok_bytes::<R>(bytes),
        Err(e) => R::err(R::from_string(format!("base64_url_decode: {e}"))),
    }
}

#[native_fn(module = "crypto")]
pub fn hex_encode(data: Bytes) -> String {
    hex::encode(&data.0)
}

#[native_fn(module = "crypto", sig(string -> result[array[byte]]))]
pub fn hex_decode<R: Runtime>(s: String) -> R::Value {
    match hex::decode(&s) {
        Ok(bytes) => ok_bytes::<R>(bytes),
        Err(e) => R::err(R::from_string(format!("hex_decode: {e}"))),
    }
}

// ---- uuid -------------------------------------------------------------------

#[native_fn(module = "crypto")]
pub fn uuid_v4() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[native_fn(module = "crypto")]
pub fn uuid_v7() -> String {
    uuid::Uuid::now_v7().to_string()
}

#[native_fn(module = "crypto", sig(string -> result[string]))]
pub fn uuid_parse<R: Runtime>(s: String) -> R::Value {
    match uuid::Uuid::parse_str(&s) {
        Ok(id) => R::ok(R::from_string(id.to_string())),
        Err(e) => R::err(R::from_string(format!("uuid_parse: {e}"))),
    }
}

// ---- passwords (argon2) -------------------------------------------------------

#[native_fn(module = "crypto")]
pub fn password_hash(password: String) -> String {
    use argon2::password_hash::{PasswordHasher, SaltString};
    // Salt from the OS CSPRNG directly: password-hash 0.5's re-exported
    // OsRng lacks its getrandom feature in this graph.
    let salt = SaltString::encode_b64(&random_bytes(16))
        .expect("password_hash: salt encoding failure");
    argon2::Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("password_hash: hashing failure")
        .to_string()
}

#[native_fn(module = "crypto")]
pub fn password_verify(password: String, hash: String) -> bool {
    use argon2::password_hash::{PasswordHash, PasswordVerifier};
    let Ok(parsed) = PasswordHash::new(&hash) else {
        return false;
    };
    argon2::Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

// ---- module registration --------------------------------------------------

rl_std_core::native_module!("crypto";
    funcs: [
        sha256, sha512, sha1, md5,
        hmac_sha256, hmac_sha512,
        constant_time_eq,
        secure_random_bytes,
        secure_token, secure_token_hex, secure_token_urlsafe,
        base64_encode, base64_decode,
        base64_url_encode, base64_url_decode,
        hex_encode, hex_decode,
        uuid_v4, uuid_v7, uuid_parse,
        password_hash, password_verify,
    ],
);
