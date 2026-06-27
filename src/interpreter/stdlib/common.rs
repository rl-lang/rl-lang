use crate::{
    interpreter::values::Value,
    utils::{
        errors::{Error, Reason},
        span::Span,
    },
};

pub fn check_arity(args: &[Value], expected: usize, name: &str, span: Span) -> Result<(), Error> {
    if args.len() != expected {
        let message = if expected == 0 {
            format!("{}: function takes no arguments, got {}", name, args.len())
        } else {
            format!("{}: expected {} arg(s), got {}", name, expected, args.len())
        };

        return Err(Error::at(Reason::Runtime, message, span));
    }
    Ok(())
}

pub fn check_type(value: &Value, expected: &str, name: &str, span: Span) -> Result<(), Error> {
    match (value, expected) {
        (Value::Bool(_), "bool")
        | (Value::String(_), "string")
        | (Value::Integer(_), "int")
        | (Value::Byte(_), "byte")
        | (Value::Char(_), "char")
        | (Value::Error(_), "error")
        | (Value::Float(_), "float")
        | (Value::Function { .. }, "fn")
        | (Value::Null, "none" | "null")
        | (Value::Ok(_), "ok")
        | (Value::Tuple(_), "tuple" | "()")
        | (Value::Values { .. }, "arr") => Ok(()),

        (other_val, other_exp) => {
            let message = format!(
                "{}: expected {} type, got {}",
                name,
                other_exp,
                other_val.type_name()
            );

            return Err(Error::at(Reason::Runtime, message, span));
        }
    }
}

pub fn extract_string(value: Value, name: &str, span: Span) -> Result<String, Error> {
    check_type(&value, "string", name, span)?;
    match value {
        Value::String(s) => Ok(s),
        _ => {
            unreachable!()
        }
    }
}

pub fn extract_int(value: Value, name: &str, span: Span) -> Result<i64, Error> {
    check_type(&value, "int", name, span)?;
    match value {
        Value::Integer(i) => Ok(i),
        _ => {
            unreachable!()
        }
    }
}

pub fn extract_byte(value: Value, name: &str, span: Span) -> Result<u8, Error> {
    check_type(&value, "byte", name, span)?;
    match value {
        Value::Byte(b) => Ok(b),
        _ => {
            unreachable!()
        }
    }
}

pub fn extract_number(value: Value, name: &str, span: Span) -> Result<u64, Error> {
    if check_type(&value, "int", name, span).is_err()
        && check_type(&value, "byte", name, span).is_err()
    {
        let message = format!(
            "{}: expected int or byte type, got {}",
            name,
            value.type_name()
        );

        return Err(Error::at(Reason::Runtime, message, span));
    }

    match value {
        Value::Integer(i) => Ok(i as u64),
        Value::Byte(b) => Ok(b as u64),
        _ => {
            unreachable!()
        }
    }
}
