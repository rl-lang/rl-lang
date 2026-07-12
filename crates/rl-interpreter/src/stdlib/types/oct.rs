use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn func(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Integer(v) => format!("{:o}", v),
        Value::Byte(v) => format!("{:o}", v),
        Value::Char(v) => format!("{:o}", v as u32),
        Value::String(s) => s.bytes().map(|b| format!("{:o}", b)).collect::<String>(),

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as octal",
                other.type_name()
            )));
        }
    };
    vok!(vs!(result))
}
