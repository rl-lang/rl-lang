use crate::vm::chunk::{Chunk, OpCode, VmValue};

#[derive(Debug)]
pub struct VmError(pub String);

pub struct Vm {
    stack: Vec<VmValue>,
    locals: Vec<VmValue>,
}

