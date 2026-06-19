use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{
        errors::{Error, ErrorReason, Reason},
        span::Span,
    },
};
use std::io::{self, Write};

fn input(prompt: Option<Value>, span: Span) -> Result<Value, Error> {
    match prompt {
        None => read_line(span),
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
            read_line(span)
        }
    }
}

fn read_line(span: Span) -> Result<Value, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| {
        Error::at(
            Reason::Runtime,
            format!("read(): failed to read line: {}", e),
            span,
        )
    })?;
    Ok(Value::String(input.trim().to_string()))
}

pub fn std_read(_: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    match len {
        0 => input(None, span),
        1 => input(args.next(), span),
        n => Err(Error::init(
            format!("read() expects 0 or 1 argument(s), got {}", n),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )),
    }
}

// reads input then parses to integer if possible otherwise error
pub fn std_read_int(_: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    let value = match len {
        0 => input(None, span),
        1 => input(args.next(), span),
        n => Err(Error::init(
            format!("read_int() expects 0 or 1 argument(s), got {}", n),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )),
    }?;

    match value {
        Value::String(s) => s.parse::<i64>().map(Value::Integer).map_err(|_| {
            Error::init(
                format!("read_int(): \"{}\" is not a valid integer", s),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )
        }),
        // those unreachable btw
        Value::Integer(i) => Ok(Value::Integer(i)),
        Value::Float(f) => Ok(Value::Integer(f as i64)),
        other => Err(Error::init(
            format!(
                "read_int(): found unsupported type from input, got {}",
                other.type_name()
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )),
    }
}

// reads the input then parses the string into float if possible or error
pub fn std_read_float(_: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    let value = match len {
        0 => input(None, span),
        1 => input(args.next(), span),
        n => Err(Error::init(
            format!("read_float() expects 0 or 1 argument(s), got {}", n),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )),
    }?;

    match value {
        Value::String(s) => s.parse::<f64>().map(Value::Float).map_err(|_| {
            Error::init(
                format!("read_float(): \"{}\" is not a valid float", s),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )
        }),
        // those are un----reachable!! might change read_line() or remove them
        Value::Integer(i) => Ok(Value::Float(i as f64)),
        Value::Float(f) => Ok(Value::Float(f)),
        other => Err(Error::init(
            format!(
                "read_float(): found unsupported type from input, got {}",
                other.type_name()
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )),
    }
}
