use crate::vm::chunk::{Chunk, OpCode, VmValue};

#[derive(Debug)]
pub struct VmError(pub String);

pub struct Vm {
    stack: Vec<VmValue>,
    locals: Vec<VmValue>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            locals: Vec::new(),
        }
    }

}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
