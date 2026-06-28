use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn func(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Integer(v) => format!("{:x}", v),
        Value::Byte(v) => format!("{:x}", v),
        Value::Char(v) => format!("{:x}", v as u32),
        Value::String(s) => s.bytes().map(|b| format!("{:x}", b)).collect::<String>(),

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as hexadecimal",
                other.type_name()
            )));
        }
    };
    vok!(vs!(result))
}
