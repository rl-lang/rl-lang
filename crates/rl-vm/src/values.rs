use std::rc::Rc;

use crate::Chunk;

#[derive(Debug, Clone, PartialEq)]
pub enum VmValue {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    Byte(u8),
    Char(char),
    Str(Rc<str>),
    Function(Rc<VmFunction>),
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

