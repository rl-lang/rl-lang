use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{vc, verr, vok, vs},
    values::Value,
};

pub fn std_is_char(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Char(_))
}

pub fn std_to_char(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Char(c) => c,
        Value::Integer(i) => match char::from_u32(i as u32) {
            Some(c) => c,
            None => return verr!(vs!(format!("{} is not a valid unicode codepoint", i))),
        },
        Value::Byte(i) => match char::from_u32(i as u32) {
            Some(c) => c,
            None => return verr!(vs!(format!("{} is not a valid unicode codepoint", i))),
        },
        Value::String(s) => {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => c,
                _ => return verr!(vs!("string must be exactly one character".to_string())),
            }
        }

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as character",
                other.type_name()
            )));
        }
    };

    vok!(vc!(result))
}
