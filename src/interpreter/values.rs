use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    Values(Vec<Value>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Char(c) => write!(f, "'{}'", c),
            Value::Values(items) => {
                let formatted: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", formatted.join(", "))
            }
            Value::Null => write!(f, "null"),
        }
    }
}
