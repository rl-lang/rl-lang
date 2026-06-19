use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_to_string(eval: &mut Evaluator, value: Value, span: Span) -> Result<String, Error> {
    match value {
        Value::Integer(v) => Ok(format!("{}", v)),
        Value::Float(v) => Ok(format!("{}", v)),
        Value::Bool(v) => Ok(format!("{}", v)),
        Value::Char(v) => Ok(v.to_string()),
        Value::String(s) => Ok(s),

        other => Err(eval.err(
            format!("cannot parse \"{}\" as bool", other.type_name()),
            span,
        )),
    }
}
