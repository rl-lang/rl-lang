use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::Chunk;
use crate::native::NativeFn;

#[derive(Debug, Clone, PartialEq)]
pub enum VmValue {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    Byte(u8),
    Char(char),
    Str(Rc<str>),
    /// user defined function call
    Function(Rc<VmFunction>),
    /// std function call
    Native(Rc<VmNativeFn>),
    Ok(Box<VmValue>),
    Err(Box<VmValue>),
    Error(Box<VmValue>),
    Arr(Rc<Vec<VmValue>>),
    Tuple(Rc<Vec<VmValue>>),
    Set(Rc<Vec<VmValue>>),
    Map(Rc<RefCell<HashMap<VmMapKey, VmValue>>>),
    Record {
        name: Rc<str>,
        fields: RecordFields,
    },
    Tag {
        name: Rc<str>,
        variant: Rc<str>,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RecordFields(Rc<RefCell<Vec<(Rc<str>, VmValue)>>>);

impl RecordFields {
    pub fn new(fields: Vec<(Rc<str>, VmValue)>) -> Self {
        Self(Rc::new(RefCell::new(fields)))
    }

    pub fn get(&self, name: &str) -> Option<VmValue> {
        self.0
            .borrow()
            .iter()
            .find(|(n, _)| &**n == name)
            .map(|(_, v)| v.clone())
    }

    pub fn set(&self, name: &str, value: VmValue) {
        let mut fields = self.0.borrow_mut();
        if let Some(entry) = fields.iter_mut().find(|(n, _)| &**n == name) {
            entry.1 = value;
        } else {
            fields.push((Rc::from(name), value));
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Rc<str>, VmValue)> + '_ {
        self.0.borrow().clone().into_iter()
    }

    pub fn has(&self, name: &str) -> bool {
        self.0.borrow().iter().any(|(n, _)| &**n == name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VmMapKey {
    Int(i64),
    Str(Rc<str>),
    Bool(bool),
    Byte(u8),
    Char(char),
}

impl VmMapKey {
    pub fn from_value(v: &VmValue) -> Option<VmMapKey> {
        match v {
            VmValue::Int(i) => Some(VmMapKey::Int(*i)),
            VmValue::Str(s) => Some(VmMapKey::Str(s.clone())),
            VmValue::Bool(b) => Some(VmMapKey::Bool(*b)),
            VmValue::Byte(b) => Some(VmMapKey::Byte(*b)),
            VmValue::Char(c) => Some(VmMapKey::Char(*c)),
            _ => None,
        }
    }
    pub fn into_value(self) -> VmValue {
        match self {
            VmMapKey::Int(i) => VmValue::Int(i),
            VmMapKey::Str(s) => VmValue::Str(s),
            VmMapKey::Bool(b) => VmValue::Bool(b),
            VmMapKey::Byte(b) => VmValue::Byte(b),
            VmMapKey::Char(c) => VmValue::Char(c),
        }
    }
}

impl fmt::Display for VmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmValue::Null => write!(f, "null"),
            VmValue::Int(i) => write!(f, "{}", i),
            VmValue::Float(fl) => write!(f, "{}", fl),
            VmValue::Bool(b) => write!(f, "{}", b),
            VmValue::Byte(b) => write!(f, "{}", b),
            VmValue::Char(c) => write!(f, "'{}'", c),
            VmValue::Str(s) => write!(f, "{}", s),
            VmValue::Function(func) => write!(f, "<fn {}/{}>", func.name, func.arity),
            VmValue::Native(func) => write!(f, "<native fn {}>", func.name),
            VmValue::Ok(inner) => write!(f, "ok({})", inner),
            VmValue::Err(inner) => write!(f, "err({})", inner),
            VmValue::Error(inner) => write!(f, "error({})", inner),
            VmValue::Arr(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            VmValue::Tuple(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            VmValue::Set(items) => {
                write!(f, "{{")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "}}")
            }
            VmValue::Map(entries) => {
                write!(f, "{{")?;
                for (i, (k, v)) in entries.borrow().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k.clone().into_value(), v)?;
                }
                write!(f, "}}")
            }
            VmValue::Record { name, fields } => {
                write!(f, "{} {{", name)?;
                for (i, (fname, fval)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", fname, fval)?;
                }
                write!(f, "}}")
            }
            VmValue::Tag { name, variant } => write!(f, "{}.{}", name, variant),
        }
    }
}

impl VmValue {
    /// Human-readable type name used in error labels (e.g. "int", "bool").
    pub fn type_name(&self) -> &'static str {
        match self {
            VmValue::Null => "null",
            VmValue::Int(_) => "int",
            VmValue::Float(_) => "float",
            VmValue::Bool(_) => "bool",
            VmValue::Byte(_) => "byte",
            VmValue::Char(_) => "char",
            VmValue::Str(_) => "string",
            VmValue::Function(_) => "function",
            VmValue::Native(_) => "native function",
            VmValue::Ok(_) => "ok",
            VmValue::Err(_) => "err",
            VmValue::Error(_) => "error",
            VmValue::Arr(_) => "arr",
            VmValue::Tuple(_) => "tuple",
            VmValue::Set(_) => "set",
            VmValue::Map(_) => "map",
            VmValue::Record { .. } => "record",
            VmValue::Tag { .. } => "tag",
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, VmValue::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, VmValue::Err(_))
    }
}

#[derive(Debug)]
pub struct VmFunction {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
}

// VmValue derives PartialEq, so VmFunction needs it too - compare by
// identity (name+arity) rather than deep-comparing bytecode
impl PartialEq for VmFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }
}

/// A native (Rust-implemented) function bound into `VmValue::Native`.
/// Compiled call paths like `std::io::println` resolve to one of these
/// at compile time and get embedded as a constant, same as `VmFunction`.
pub struct VmNativeFn {
    pub name: String,
    pub func: NativeFn,
}

impl fmt::Debug for VmNativeFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VmNativeFn({})", self.name)
    }
}

impl PartialEq for VmNativeFn {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
