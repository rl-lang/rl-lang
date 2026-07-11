use crate::vm::chunk::{Chunk, OpCode, VmValue};

#[derive(Debug)]
pub struct CompileError(pub String);

pub struct Compiler<'a> {
    ast: &'a Ast,
    chunk: Chunk,
}

