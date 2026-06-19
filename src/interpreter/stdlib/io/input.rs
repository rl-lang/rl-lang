use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};
use std::io::{self, Write};

fn input(prompt: Option<Value>, eval: &mut Evaluator, span: Span) -> Result<Value, Error> {
    match prompt {
        None => read_line(eval, span),
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
            read_line(eval, span)
        }
    }
}

fn read_line(eval: &mut Evaluator, span: Span) -> Result<Value, Error> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| eval.err(format!("read(): failed to read line: {}", e), span))?;
    Ok(Value::String(input.trim().to_string()))
}

pub fn std_read(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    match len {
        0 => input(None, eval, span),
        1 => input(args.next(), eval, span),
        n => Err(eval.err(
            format!("read() expects 0 or 1 argument(s), got {}", n),
            span,
        )),
    }
}

// reads input then parses to integer if possible otherwise error
pub fn std_read_int(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    let value = match len {
        0 => input(None, eval, span),
        1 => input(args.next(), eval, span),
        n => Err(eval.err(
            format!("read_int() expects 0 or 1 argument(s), got {}", n),
            span,
        )),
    }?;

    match value {
        Value::String(s) => s.parse::<i64>().map(Value::Integer).map_err(|_| {
            eval.err(
                format!("read_int(): \"{}\" is not a valid integer", s),
                span,
            )
        }),
        // those unreachable btw
        Value::Integer(i) => Ok(Value::Integer(i)),
        Value::Float(f) => Ok(Value::Integer(f as i64)),
        other => Err(eval.err(
            format!(
                "read_int(): found unsupported type from input, got {}",
                other.type_name()
            ),
            span,
        )),
    }
}

// reads the input then parses the string into float if possible or error
pub fn std_read_float(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    let value = match len {
        0 => input(None, eval, span),
        1 => input(args.next(), eval, span),
        n => Err(eval.err(
            format!("read_float() expects 0 or 1 argument(s), got {}", n),
            span,
        )),
    }?;

    match value {
        Value::String(s) => s.parse::<f64>().map(Value::Float).map_err(|_| {
            eval.err(
                format!("read_float(): \"{}\" is not a valid float", s),
                span,
            )
        }),
        // those are un----reachable!! might change read_line() or remove them
        Value::Integer(i) => Ok(Value::Float(i as f64)),
        Value::Float(f) => Ok(Value::Float(f)),
        other => Err(eval.err(
            format!(
                "read_float(): found unsupported type from input, got {}",
                other.type_name()
            ),
            span,
        )),
    }
}
