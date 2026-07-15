use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::chunk::Chunk;
use crate::native::Module;
use crate::values::{VmFunction, VmValue};

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

pub fn deserialize_chunk(bytes: &[u8], stdlib: &Module) -> Result<Chunk, BytecodeError> {
    if bytes.len() < 4 || &bytes[0..4] != MAGIC {
        return Err(BytecodeError(
            "not a valid .rlc file (bad magic header)".to_string(),
        ));
    }
    let mut cursor = Cursor {
        data: bytes,
        pos: 4,
    };
    read_chunk(&mut cursor, stdlib)
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

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], BytecodeError> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|&e| e <= self.data.len())
            .ok_or_else(|| BytecodeError("truncated .rlc file".to_string()))?;
        let slice = &self.data[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn u32(&mut self) -> Result<u32, BytecodeError> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    fn u8(&mut self) -> Result<u8, BytecodeError> {
        Ok(self.take(1)?[0])
    }

    fn i64(&mut self) -> Result<i64, BytecodeError> {
        Ok(i64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    fn f64(&mut self) -> Result<f64, BytecodeError> {
        Ok(f64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    fn string(&mut self) -> Result<String, BytecodeError> {
        let len = self.u32()? as usize;
        let bytes = self.take(len)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|_| BytecodeError("invalid utf-8 in .rlc string".to_string()))
    }
}

fn read_chunk(cursor: &mut Cursor, stdlib: &Module) -> Result<Chunk, BytecodeError> {
    let code_len = cursor.u32()? as usize;
    let code = cursor.take(code_len)?.to_vec();

    let mut lines = Vec::with_capacity(code_len);
    for _ in 0..code_len {
        lines.push(cursor.u32()?);
    }

    let const_count = cursor.u32()? as usize;
    let mut constants = Vec::with_capacity(const_count);
    for _ in 0..const_count {
        constants.push(read_value(cursor, stdlib)?);
    }

    Ok(Chunk {
        code,
        constants,
        lines,
    })
}

fn read_value(cursor: &mut Cursor, stdlib: &Module) -> Result<VmValue, BytecodeError> {
    let tag = cursor.u8()?;
    Ok(match tag {
        0 => VmValue::Null,
        1 => VmValue::Int(cursor.i64()?),
        2 => VmValue::Float(cursor.f64()?),
        3 => VmValue::Bool(cursor.u8()? != 0),
        4 => VmValue::Byte(cursor.u8()?),
        5 => {
            let code_point = cursor.u32()?;
            let c = char::from_u32(code_point)
                .ok_or_else(|| BytecodeError("invalid char in .rlc file".to_string()))?;
            VmValue::Char(c)
        }
        6 => VmValue::Str(cursor.string()?.into()),
        7 => {
            let name = cursor.string()?;
            let arity = cursor.u32()? as usize;
            let chunk = read_chunk(cursor, stdlib)?;
            VmValue::Function(Rc::new(VmFunction { name, arity, chunk }))
        }
        8 => {
            let name = cursor.string()?;
            let native = find_native_by_name(stdlib, &name).ok_or_else(|| {
                BytecodeError(format!(
                    "unresolved native function '{}' - this .rlc file may have been \
                     compiled by an incompatible rl version",
                    name
                ))
            })?;
            VmValue::Native(native)
        }
        9 => VmValue::Ok(Box::new(read_value(cursor, stdlib)?)),
        10 => VmValue::Err(Box::new(read_value(cursor, stdlib)?)),
        other => {
            return Err(BytecodeError(format!(
                "corrupt .rlc file: unknown constant tag {other}"
            )));
        }
    })
}

fn find_native_by_name(module: &Module, name: &str) -> Option<Rc<crate::values::VmNativeFn>> {
    if let Some(f) = module.functions.get(name) {
        return Some(f.clone());
    }
    for sub in module.submodules.values() {
        if let Some(f) = find_native_by_name(sub, name) {
            return Some(f);
        }
    }
    None
}
