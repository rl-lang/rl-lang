use std::io;

use crate::{interpreter::evaluator::Evaluator, interpreter::values::Value, utils::errors::Error};

pub fn std_input(_: &mut Evaluator, args: Vec<Value>) -> Value {
    match args.len() {
        0 => read_line(),
        1 => {
            let prompt = match args.into_iter().next().unwrap() {
                Value::Integer(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::String(s) => s,
                Value::Char(c) => c.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
                _ => "".to_string(),
            };
            println!("{}", prompt);
            read_line()
        }
        _ => {
            Error::init("incorrect usage".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}

fn read_line() -> Value {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    Value::String(input.trim().to_string())
}
