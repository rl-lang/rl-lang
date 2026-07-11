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


    /// Helper functions that wraps the Vec::pop to return valid VmError or VmValue
    fn pop(&mut self) -> Result<VmValue, VmError> {
        self.stack
            .pop()
            .ok_or_else(|| VmError("stack underflow".into()))
    }
    fn pop_two(&mut self) -> Result<(VmValue, VmValue), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        Ok((a, b))
    }

}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
