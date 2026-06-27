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

pub fn check_type(value: Value, expected: &str, name: &str, span: Span) -> Result<Value, Error> {
    match (&value, expected) {
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
        | (Value::Values { .. }, "arr") => Ok(value),

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
