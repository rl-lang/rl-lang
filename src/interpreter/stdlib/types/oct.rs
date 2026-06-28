use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_to_oct(eval: &mut Evaluator, value: Value, span: Span) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{:o}", v)),
        Value::Byte(v) => Ok(format!("{:o}", v)),
        Value::Char(v) => Ok(format!("{:o}", v as u32)),
        Value::String(s) => Ok(s.bytes().map(|b| format!("{:o}", b)).collect::<String>()),

        other => Err(eval.err(
            format!("cannot parse \"{}\" as octal", other.type_name()),
            span,
        )),
    }
}
