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
        }
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
