use crate::entry::{FnEntry, StdEntry};

mod base64_decode;
mod base64_encode;
mod base64_url_decode;
mod base64_url_encode;
mod constant_time_eq;
mod hex_decode;
mod hex_encode;
mod hmac_sha256;
mod hmac_sha512;
mod md5;
mod password_hash;
mod password_verify;
mod secure_random_bytes;
mod secure_token;
mod secure_token_hex;
mod secure_token_urlsafe;
mod sha1;
mod sha256;
mod sha512;
mod uuid_parse;
mod uuid_v4;
mod uuid_v7;

pub static CRYPTO: StdEntry = StdEntry {
    name: "crypto",
    description: "functions for hashing, keyed MACs, random tokens and password hashing",
    functions: FUNCTIONS,
    since: Some("v2.3.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &sha256::SHA256,
    &sha512::SHA512,
    &sha1::SHA1,
    &md5::MD5,
    &hmac_sha256::HMAC_SHA256,
    &hmac_sha512::HMAC_SHA512,
    &constant_time_eq::CONSTANT_TIME_EQ,
    &secure_random_bytes::SECURE_RANDOM_BYTES,
    &secure_token::SECURE_TOKEN,
    &secure_token_hex::SECURE_TOKEN_HEX,
    &secure_token_urlsafe::SECURE_TOKEN_URLSAFE,
    &base64_encode::BASE64_ENCODE,
    &base64_decode::BASE64_DECODE,
    &base64_url_encode::BASE64_URL_ENCODE,
    &base64_url_decode::BASE64_URL_DECODE,
    &hex_encode::HEX_ENCODE,
    &hex_decode::HEX_DECODE,
    &uuid_v4::UUID_V4,
    &uuid_v7::UUID_V7,
    &uuid_parse::UUID_PARSE,
    &password_hash::PASSWORD_HASH,
    &password_verify::PASSWORD_VERIFY,
];
