use std::fmt;

use crate::{
    ast::statements::{Param, Statement, TypeAnnotation},
    interpreter::evaluator::EnvironmentItem,
};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    Values(Vec<Value>),
    Null,
    Function {
        params: Vec<Param>,
        body: Vec<Statement>,
        return_type: Option<TypeAnnotation>,
        captured_env: Vec<std::collections::HashMap<String, EnvironmentItem>>,
    },
}

impl Value {
    /// Human-readable type name used in error labels (e.g. "int", "bool", "array").
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Char(_) => "char",
            Value::Values(_) => "array",
            Value::Null => "null",
            Value::Function { .. } => "function",
        }
    }
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
            Value::Function { params, .. } => {
                let mut params_name = vec![];
                for param in params {
                    params_name.push(param.param_name.clone());
                }
                write!(f, "<fn({})>", params_name.join(", "))
            }
        }
    }
}
