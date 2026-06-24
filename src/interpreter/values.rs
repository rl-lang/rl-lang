//! Runtime value types for the rl interpreter.

use crate::{
    ast::statements::{Param, Statement, TypeAnnotation},
    interpreter::evaluator::EnvironmentItem,
};
use std::fmt;

/// A runtime value produced by evaluating an rl expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A 64-bit signed integer.
    Integer(i64),
    /// A 64-bit float.
    Float(f64),
    /// A UTF-8 string.
    String(String),
    /// A boolean.
    Bool(bool),
    /// A single unsigned byte (`u8`).
    Byte(u8),
    /// A single Unicode character.
    Char(char),
    /// A homogeneous array of values with a tracked element type.
    Values {
        /// The declared element type of this array.
        items_type: TypeAnnotation,
        items: Vec<Value>,
    },
    /// The absence of a value - equivalent to `null` in rl source.
    Null,
    /// A first-class function or lambda value, carrying its closure environment.
    Function {
        params: Vec<Param>,
        body: Vec<Statement>,
        /// Declared return type; `None` for lambdas without an annotation.
        return_type: Option<TypeAnnotation>,
        /// The captured environment frames at the point of lambda definition.
        captured_env: Vec<Vec<EnvironmentItem>>,
    },
    /// A heterogeneous tuple of values.
    Tuple(Vec<Value>),
    /// An error value wrapping any non-error value.
    Error(Box<Value>),
}

impl Value {
    /// Human-readable type name used in error labels (e.g. "int", "bool", "array").
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Byte(_) => "byte",
            Value::Char(_) => "char",
            Value::Values { .. } => "array",
            Value::Null => "null",
            Value::Function { .. } => "function",
            Value::Tuple(_) => "tuple",
            Value::Error(_) => "error",
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
            Value::Byte(b) => write!(f, "{}", b),
            Value::Char(c) => write!(f, "'{}'", c),
            Value::Values { items, .. } => {
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
            Value::Tuple(items) => {
                let formatted: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "({})", formatted.join(", "))
            }
            Value::Error(inner) => write!(f, "error({})", inner),
        }
    }
}
