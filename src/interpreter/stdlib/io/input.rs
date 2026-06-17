use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};
use std::io::{self, Write};

pub fn std_read(_: &mut Evaluator, args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        0 => read_line(),
        1 => {
            let prompt = match args.into_iter().next().unwrap_or(Value::Null) {
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
        n => Err(Error::init(
            format!("read() expects 0 or 1 argument(s), got {}", n),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )),
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
