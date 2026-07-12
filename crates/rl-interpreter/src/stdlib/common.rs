use crate::values::Value;
use rl_utils::{
    errors::{Error, Reason},
    span::Span,
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

pub fn check_arity_range(
    args: &[Value],
    expected_from: usize,
    expected_to: usize,
    name: &str,
    span: Span,
) -> Result<(), Error> {
    let len = args.len();
    if !(expected_from..expected_to).contains(&len) {
        let message = format!(
            "{}: expected from {} to {} arg(s), got {}",
            name,
            expected_from,
            expected_to,
            args.len()
        );

        return Err(Error::at(Reason::Runtime, message, span));
    }
    Ok(())
}

pub fn check_type(value: &Value, expected: &str, name: &str) -> Result<(), String> {
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

        (other_val, other_exp) => Err(format!(
            "{}: expected {} type, got {}",
            name,
            other_exp,
            other_val.type_name()
        )),
    }
}

pub fn extract_string(value: Value, name: &str) -> Result<String, String> {
    check_type(&value, "string", name)?;
    let Value::String(v) = value else {
        unreachable!()
    };
    Ok(v)
}

pub fn extract_int(value: Value, name: &str) -> Result<i64, String> {
    check_type(&value, "int", name)?;
    let Value::Integer(v) = value else {
        unreachable!()
    };
    Ok(v)
}

pub fn extract_number(value: Value, name: &str) -> Result<u64, String> {
    if check_type(&value, "int", name).is_err() && check_type(&value, "byte", name).is_err() {
        return Err(format!(
            "{}: expected int or byte type, got {}",
            name,
            value.type_name()
        ));
    }

    match value {
        Value::Integer(i) => Ok(i as u64),
        Value::Byte(b) => Ok(b as u64),
        _ => {
            unreachable!()
        }
    }
}

macro_rules! vok {
    ($e:expr) => {
        Value::Ok(Box::new($e))
    };
}
pub(crate) use vok;

macro_rules! verr {
    ($e:expr) => {
        Value::Err(Box::new($e))
    };
}
pub(crate) use verr;

macro_rules! vb {
    ($e:expr) => {
        Value::Bool($e)
    };
}
pub(crate) use vb;

macro_rules! vi {
    ($e:expr) => {
        Value::Integer($e)
    };
}
pub(crate) use vi;

macro_rules! vf {
    ($e:expr) => {
        Value::Float($e)
    };
}
pub(crate) use vf;

macro_rules! vs {
    ($e:expr) => {
        Value::String($e)
    };
}
pub(crate) use vs;

macro_rules! vc {
    ($e:expr) => {
        Value::Char($e)
    };
}
pub(crate) use vc;

macro_rules! vby {
    ($e:expr) => {
        Value::Byte($e)
    };
}
pub(crate) use vby;

macro_rules! vnl {
    () => {
        Value::Null
    };
}
pub(crate) use vnl;

macro_rules! try_fn {
    ($s:expr, $e:expr) => {
        if let Err(e) = $e {
            return verr!(vs!(format!("{}: {}", $s, e)));
        }
    };
}
pub(crate) use try_fn;
