use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};
use std::io::{self, Write};

fn input(prompt: Option<Value>) -> Result<Value, Error> {
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

fn read_line() -> Result<Value, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| {
        Error::init(
            format!("read(): failed to read line: {}", e),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;
    Ok(Value::String(input.trim().to_string()))
}

pub fn std_read(_: &mut Evaluator, args: Vec<Value>) -> Result<Value, Error> {
    let len = args.len();
    let mut args = args.into_iter();

    match len {
        0 => input(None),
        1 => input(args.next()),
        n => Err(Error::init(
            format!("read() expects 0 or 1 argument(s), got {}", n),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )),
    }
}
