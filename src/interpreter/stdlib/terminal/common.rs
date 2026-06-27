use crate::{
    interpreter::{
        evaluator::Evaluator,
        stdlib::common::{check_arity, extract_int, extract_number},
        values::Value,
    },
    utils::{errors::Error, span::Span},
};

pub fn extract_byte(v: Value, name: &str, span: Span) -> Result<u8, Error> {
    let byte = extract_number(v, name, span)?;
    Ok(byte as u8)
}

pub fn extract_u16(v: Value, name: &str, eval: &mut Evaluator, span: Span) -> Result<u16, Error> {
    let v = extract_int(v, name, span)?;
    match v {
        v if v >= 0 => Ok(v as u16),
        v => Err(eval.err(format!("{} must be >= 0, got {}", name, v), span)),
    }
}

pub fn extract_u16_one_arg(
    name: &str,
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<u16, Error> {
    check_arity(&args, 1, name, span)?;
    extract_u16(args.into_iter().next().unwrap(), "n", eval, span)
}
