//! Runtime value types for the rl interpreter.

use crate::{
    ast::statements::{Param, Statement, TypeAnnotation},
    interpreter::evaluator::EnvironmentItem,
};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

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
    Function(Rc<FunctionData>),

    /// A heterogeneous tuple of values.
    Tuple(Vec<Value>),
    /// An error value wrapping any non-error value.
    Error(Box<Value>),
    Ok(Box<Value>),
    Err(Box<Value>),

    Struct {
        name: String,
        fields: Rc<RefCell<Vec<(String, Value)>>>,
    },
    Enum {
        name: String,
        variant: String,
    },

    Map {
        key_type: TypeAnnotation,
        value_type: TypeAnnotation,
        entries: Rc<RefCell<HashMap<MapKey, Value>>>,
    },
    Set {
        /// The declared element type of this set.
        items_type: TypeAnnotation,
        items: Vec<Value>,
    },
}

/// Payload for `Value::Function`
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionData {
    pub params: Rc<Vec<Param>>,
    pub body: Rc<Vec<Statement>>,
    /// Declared return type; `None` for lambdas without an annotation.
    pub return_type: Option<TypeAnnotation>,
    /// The captured environment frames at the point of lambda definition.
    pub captured_env: Vec<Vec<EnvironmentItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapKey {
    Integer(i64),
    String(String),
    Bool(bool),
    Byte(u8),
    Char(char),
}

impl MapKey {
    pub fn from_value(value: &Value) -> Option<MapKey> {
        match value {
            Value::Integer(i) => Some(MapKey::Integer(*i)),
            Value::String(s) => Some(MapKey::String(s.clone())),
            Value::Bool(b) => Some(MapKey::Bool(*b)),
            Value::Byte(b) => Some(MapKey::Byte(*b)),
            Value::Char(c) => Some(MapKey::Char(*c)),
            _ => None,
        }
    }

    pub fn into_value(self) -> Value {
        match self {
            MapKey::Integer(i) => Value::Integer(i),
            MapKey::String(s) => Value::String(s),
            MapKey::Bool(b) => Value::Bool(b),
            MapKey::Byte(b) => Value::Byte(b),
            MapKey::Char(c) => Value::Char(c),
        }
    }
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
            Value::Map { .. } => "map",
            Value::Null => "null",
            Value::Function { .. } => "function",
            Value::Tuple(_) => "tuple",
            Value::Error(_) => "error",
            Value::Ok(_) => "ok",
            Value::Err(_) => "err",
            Value::Struct { .. } => "record",
            Value::Enum { .. } => "tag",
            Value::Set { .. } => "set",
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Value::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, Value::Err(_))
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
            Value::Function(data) => {
                let mut params_name = vec![];
                for param in data.params.iter() {
                    params_name.push(param.param_name.clone());
                }
                write!(f, "<fn({})>", params_name.join(", "))
            }
            Value::Tuple(items) => {
                let formatted: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "({})", formatted.join(", "))
            }
            Value::Error(inner) => write!(f, "error({})", inner),
            Value::Ok(inner) => write!(f, "ok({})", inner),
            Value::Err(inner) => write!(f, "err({})", inner),
            Value::Struct { name, fields } => {
                let fields = fields.borrow();
                let formatted: Vec<String> = fields
                    .iter()
                    .map(|(field_name, value)| format!("{}: {}", field_name, value))
                    .collect();
                write!(f, "{} {{ {} }}", name, formatted.join(", "))
            }
            Value::Enum { name, variant } => write!(f, "{}.{}", name, variant),
            Value::Map { entries, .. } => {
                let entries = entries.borrow();
                let formatted: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k.clone().into_value(), v))
                    .collect();
                write!(f, "{{{}}}", formatted.join(", "))
            }
            Value::Set { items, .. } => {
                let formatted: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "{{{}}}", formatted.join(", "))
            }
        }
    }
}
