use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

// Emitters for `std::crypto`. Shapes mirror the VM exactly: digests and
// random bytes travel as int64-element arrays holding 0-255, text as
// `rl_string`, results as `rl_result`.

fn one_arg(cc: &mut CCodegen, name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(name);
    cc.writer.write("(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(")");
    Ok(())
}

fn two_args(cc: &mut CCodegen, name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(name);
    cc.writer.write("(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_sha256(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_sha256", args)
}

pub(super) fn compile_sha512(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_sha512", args)
}

pub(super) fn compile_sha1(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_sha1", args)
}

pub(super) fn compile_md5(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_md5", args)
}

pub(super) fn compile_hmac_sha256(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_crypto_hmac_sha256", args)
}

pub(super) fn compile_hmac_sha512(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_crypto_hmac_sha512", args)
}

pub(super) fn compile_constant_time_eq(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    two_args(cc, "rl_crypto_constant_time_eq", args)
}

pub(super) fn compile_secure_random_bytes(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_secure_random_bytes", args)
}

pub(super) fn compile_secure_token(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_secure_token", args)
}

pub(super) fn compile_secure_token_hex(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_secure_token_hex", args)
}

pub(super) fn compile_secure_token_urlsafe(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_secure_token_urlsafe", args)
}

pub(super) fn compile_base64_encode(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_base64_encode", args)
}

pub(super) fn compile_base64_decode(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_base64_decode", args)
}

pub(super) fn compile_base64_url_encode(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_base64_url_encode", args)
}

pub(super) fn compile_base64_url_decode(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_base64_url_decode", args)
}

pub(super) fn compile_hex_encode(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_hex_encode", args)
}

pub(super) fn compile_hex_decode(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_hex_decode", args)
}

pub(super) fn compile_uuid_v4(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_crypto_uuid_v4()");
    Ok(())
}

pub(super) fn compile_uuid_v7(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_crypto_uuid_v7()");
    Ok(())
}

pub(super) fn compile_uuid_parse(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_uuid_parse", args)
}

pub(super) fn compile_password_hash(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_crypto_password_hash", args)
}

pub(super) fn compile_password_verify(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    two_args(cc, "rl_crypto_password_verify", args)
}
