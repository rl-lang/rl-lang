use crate::{interpreter::values::Value, utils::errors::Error};

pub mod len;
pub mod print;
pub mod println;

const KEYWORDS: &[&str] = &["print", "println", "len"];

pub fn is_in_display(name: &str) -> bool {
    if KEYWORDS.contains(&name) {
        return true;
    }
    false
}

pub fn match_std_display(name: &str, args: Vec<Value>) -> Value {
    match name {
        "print" => print::std_print(args),
        "println" => println::std_println(args),
        "len" => match &args[0] {
            Value::Values(items) => Value::Integer(items.len() as i64),
            Value::String(s) => Value::Integer(s.len() as i64),
            _ => {
                Error::init("len() expects an array or string".to_string(), None, None)
                    .print_error();
                unreachable!()
            }
        },
        &_ => unreachable!(),
    }
}
