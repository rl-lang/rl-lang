use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_to_bin(eval: &mut Evaluator, value: Value, span: Span) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{:b}", v)),
        Value::Bool(v) => {
            if v {
                Ok("1".to_string())
            } else {
                Ok("0".to_string())
            }
        }
        Value::Char(v) => Ok(format!("{:b}", v as u32)),
        Value::String(s) => Ok(s.bytes().map(|b| format!("{:b}", b)).collect::<String>()),

        other => Err(eval.err(
            format!("cannot parse \"{}\" as binary", other.type_name()),
            span,
        )),
    }
}
