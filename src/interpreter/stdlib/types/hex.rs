use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_to_hex(eval: &mut Evaluator, value: Value, span: Span) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{:x}", v)),
        Value::Byte(v) => Ok(format!("{:x}", v)),
        Value::Char(v) => Ok(format!("{:x}", v as u32)),
        Value::String(s) => Ok(s.bytes().map(|b| format!("{:x}", b)).collect::<String>()),

        other => Err(eval.err(
            format!("cannot parse \"{}\" as hexadecimal", other.type_name()),
            span,
        )),
    }
}
