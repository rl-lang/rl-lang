use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::chunk::Chunk;
use crate::values::VmValue;

const MAGIC: &[u8; 4] = b"RLC1";

#[derive(Debug)]
pub struct BytecodeError(pub String);

impl Display for BytecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub fn serialize_chunk(chunk: &Chunk) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    write_chunk(chunk, &mut out);
    out
}

fn write_chunk(chunk: &Chunk, out: &mut Vec<u8>) {
    out.extend_from_slice(&(chunk.code.len() as u32).to_le_bytes());
    out.extend_from_slice(&chunk.code);
    for line in &chunk.lines {
        out.extend_from_slice(&line.to_le_bytes());
    }
    out.extend_from_slice(&(chunk.constants.len() as u32).to_le_bytes());
    for c in &chunk.constants {
        write_value(c, out);
    }
}

fn write_str(s: &str, out: &mut Vec<u8>) {
    out.extend_from_slice(&(s.len() as u32).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

fn write_value(value: &VmValue, out: &mut Vec<u8>) {
    match value {
        VmValue::Null => out.push(0),
        VmValue::Int(i) => {
            out.push(1);
            out.extend_from_slice(&i.to_le_bytes());
        }
        VmValue::Float(f) => {
            out.push(2);
            out.extend_from_slice(&f.to_le_bytes());
        }
        VmValue::Bool(b) => {
            out.push(3);
            out.push(*b as u8);
        }
        VmValue::Byte(b) => {
            out.push(4);
            out.push(*b);
        }
        VmValue::Char(c) => {
            out.push(5);
            out.extend_from_slice(&(*c as u32).to_le_bytes());
        }
        VmValue::Str(s) => {
            out.push(6);
            write_str(s, out);
        }
        VmValue::Function(func) => {
            out.push(7);
            write_str(&func.name, out);
            out.extend_from_slice(&(func.arity as u32).to_le_bytes());
            write_chunk(&func.chunk, out);
        }
        VmValue::Native(native) => {
            out.push(8);
            write_str(&native.name, out);
        }
        VmValue::Ok(inner) => {
            out.push(9);
            write_value(inner, out);
        }
        VmValue::Err(inner) => {
            out.push(10);
            write_value(inner, out);
        }
    }
}

