use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vi, vok, vs},
    values::Value,
};
use rl_utils::{errors::Error, span::Span};
use std::io::{self, Write};

fn input(prompt: Option<Value>) -> Value {
    match prompt {
        None => read_line(),
        Some(p) => {
            let prompt = match p {
                Value::Integer(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::String(s) => s,
                Value::Char(c) => c.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
                _ => "".to_string(),
            };
            print!("{}", prompt);
            io::stdout().flush().ok();
            read_line()
        }
    }
}

fn read_line() -> Value {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => vok!(vs!(input.trim().to_string())),
        Err(e) => verr!(vs!(format!("read: failed to read line: {}", e))),
    }
}

pub fn std_read(_: &mut Evaluator, args: Vec<Value>, _: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    match len {
        0 => Ok(input(None)),
        1 => Ok(input(args.next())),
        n => Ok(verr!(vs!(format!(
            "read: expects 0 or 1 argument(s), got {}",
            n
        )))),
    }
}

// reads input then parses to integer if possible otherwise error
pub fn std_read_int(_: &mut Evaluator, args: Vec<Value>, _: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    let value = match len {
        0 => input(None),
        1 => input(args.next()),
        n => {
            return Ok(verr!(vs!(format!(
                "read_int: expects 0 or 1 argument(s), got {}",
                n
            ))));
        }
    };

    match value {
        Value::Ok(inner) => match *inner {
            Value::String(s) => match s.parse::<i64>() {
                Ok(i) => Ok(vok!(vi!(i))),
                Err(_) => Ok(verr!(vs!(format!(
                    "read_int: \"{}\" is not a valid integer",
                    s
                )))),
            },
            other => Ok(verr!(vs!(format!(
                "read_int: found unsupported type from input, got {}",
                other.type_name()
            )))),
        },
        // propagate a failed read as-is (e.g. stdin read error)
        err @ Value::Err(_) => Ok(err),
        other => Ok(verr!(vs!(format!(
            "read_int: found unsupported type from input, got {}",
            other.type_name()
        )))),
    }
}

// reads the input then parses the string into float if possible or error
pub fn std_read_float(_: &mut Evaluator, args: Vec<Value>, _: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    let value = match len {
        0 => input(None),
        1 => input(args.next()),
        n => {
            return Ok(verr!(vs!(format!(
                "read_float: expects 0 or 1 argument(s), got {}",
                n
            ))));
        }
    };

    match value {
        Value::Ok(inner) => match *inner {
            Value::String(s) => match s.parse::<f64>() {
                Ok(f) => Ok(vok!(vf!(f))),
                Err(_) => Ok(verr!(vs!(format!(
                    "read_float: \"{}\" is not a valid float",
                    s
                )))),
            },
            other => Ok(verr!(vs!(format!(
                "read_float: found unsupported type from input, got {}",
                other.type_name()
            )))),
        },
        err @ Value::Err(_) => Ok(err),
        other => Ok(verr!(vs!(format!(
            "read_float: found unsupported type from input, got {}",
            other.type_name()
        )))),
    }
}
