use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::chunk::Chunk;
use crate::native::Module;
use crate::values::{VmFunction, VmValue};

const MAGIC: &[u8; 4] = b"RLC2";

#[derive(Debug)]
pub struct BytecodeError(pub String);

impl Display for BytecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub fn serialize_chunk(chunk: &Chunk) -> Vec<u8> {
    let mut pool = StringPoolBuilder::default();
    collect_strings_chunk(chunk, &mut pool);

    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    write_string_pool(&pool, &mut out);
    write_chunk(chunk, &pool, &mut out);
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
    let pool = read_string_pool(&mut cursor)?;
    read_chunk(&mut cursor, stdlib, &pool)
}

// ---- string pool ----

#[derive(Default)]
struct StringPoolBuilder {
    index: HashMap<String, u32>,
    strings: Vec<String>,
}

impl StringPoolBuilder {
    fn intern(&mut self, s: &str) -> u32 {
        if let Some(&i) = self.index.get(s) {
            return i;
        }
        let i = self.strings.len() as u32;
        self.index.insert(s.to_string(), i);
        self.strings.push(s.to_string());
        i
    }

    fn get(&self, s: &str) -> u32 {
        *self
            .index
            .get(s)
            .expect("string not interned during collection pass")
    }
}

fn write_string_pool(pool: &StringPoolBuilder, out: &mut Vec<u8>) {
    write_uvarint(pool.strings.len() as u64, out);
    for s in &pool.strings {
        write_str(s, out);
    }
}

fn read_string_pool(cursor: &mut Cursor) -> Result<Vec<String>, BytecodeError> {
    let count = cursor.uvarint()? as usize;
    let mut strings = Vec::with_capacity(count);
    for _ in 0..count {
        strings.push(cursor.string()?);
    }
    Ok(strings)
}

fn pool_str<'a>(pool: &'a [String], idx: u32) -> Result<&'a str, BytecodeError> {
    pool.get(idx as usize)
        .map(String::as_str)
        .ok_or_else(|| BytecodeError("corrupt .rlc file: string pool index out of range".into()))
}

fn collect_strings_chunk(chunk: &Chunk, pool: &mut StringPoolBuilder) {
    for c in &chunk.constants {
        collect_strings_value(c, pool);
    }
}

fn collect_strings_value(value: &VmValue, pool: &mut StringPoolBuilder) {
    match value {
        VmValue::Str(s) => {
            pool.intern(s);
        }
        VmValue::Function(func) => {
            pool.intern(&func.name);
            collect_strings_chunk(&func.chunk, pool);
        }
        VmValue::Native(native) => {
            pool.intern(&native.name);
        }
        VmValue::Ok(inner) | VmValue::Err(inner) => collect_strings_value(inner, pool),
        VmValue::Null
        | VmValue::Int(_)
        | VmValue::Float(_)
        | VmValue::Bool(_)
        | VmValue::Byte(_)
        | VmValue::Char(_) => {}
    }
}

// ---- varint helpers ----

fn write_uvarint(mut value: u64, out: &mut Vec<u8>) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte);
            break;
        } else {
            out.push(byte | 0x80);
        }
    }
}

fn write_ivarint(value: i64, out: &mut Vec<u8>) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    write_uvarint(zigzag, out);
}

fn write_str(s: &str, out: &mut Vec<u8>) {
    write_uvarint(s.len() as u64, out);
    out.extend_from_slice(s.as_bytes());
}

// ---- writing ----

fn write_chunk(chunk: &Chunk, pool: &StringPoolBuilder, out: &mut Vec<u8>) {
    write_uvarint(chunk.code.len() as u64, out);
    out.extend_from_slice(&chunk.code);
    write_lines_rle(&chunk.lines, out);
    write_uvarint(chunk.constants.len() as u64, out);
    for c in &chunk.constants {
        write_value(c, pool, out);
    }
}

fn write_lines_rle(lines: &[u32], out: &mut Vec<u8>) {
    let mut runs: Vec<(u32, u32)> = Vec::new();
    for &line in lines {
        match runs.last_mut() {
            Some((last_line, count)) if *last_line == line => *count += 1,
            _ => runs.push((line, 1)),
        }
    }

    write_uvarint(runs.len() as u64, out);
    let mut prev_line: i64 = 0;
    for (line, count) in runs {
        write_ivarint(line as i64 - prev_line, out);
        write_uvarint(count as u64, out);
        prev_line = line as i64;
    }
}

fn write_value(value: &VmValue, pool: &StringPoolBuilder, out: &mut Vec<u8>) {
    match value {
        VmValue::Null => out.push(0),
        VmValue::Int(i) => {
            out.push(1);
            write_ivarint(*i, out);
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
            write_uvarint(*c as u64, out);
        }
        VmValue::Str(s) => {
            out.push(6);
            write_uvarint(pool.get(s) as u64, out);
        }
        VmValue::Function(func) => {
            out.push(7);
            write_uvarint(pool.get(&func.name) as u64, out);
            write_uvarint(func.arity as u64, out);
            write_chunk(&func.chunk, pool, out);
        }
        VmValue::Native(native) => {
            out.push(8);
            write_uvarint(pool.get(&native.name) as u64, out);
        }
        VmValue::Ok(inner) => {
            out.push(9);
            write_value(inner, pool, out);
        }
        VmValue::Err(inner) => {
            out.push(10);
            write_value(inner, pool, out);
        }
    }
}

// ---- reading ----

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

    fn u8(&mut self) -> Result<u8, BytecodeError> {
        Ok(self.take(1)?[0])
    }

    fn f64(&mut self) -> Result<f64, BytecodeError> {
        Ok(f64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    fn uvarint(&mut self) -> Result<u64, BytecodeError> {
        let mut result: u64 = 0;
        let mut shift = 0;
        loop {
            let byte = self.u8()?;
            result |= ((byte & 0x7f) as u64) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 64 {
                return Err(BytecodeError("varint too long in .rlc file".to_string()));
            }
        }
        Ok(result)
    }

    fn ivarint(&mut self) -> Result<i64, BytecodeError> {
        let zigzag = self.uvarint()?;
        Ok(((zigzag >> 1) as i64) ^ -((zigzag & 1) as i64))
    }

    fn string(&mut self) -> Result<String, BytecodeError> {
        let len = self.uvarint()? as usize;
        let bytes = self.take(len)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|_| BytecodeError("invalid utf-8 in .rlc string".to_string()))
    }
}

fn read_chunk(
    cursor: &mut Cursor,
    stdlib: &Module,
    pool: &[String],
) -> Result<Chunk, BytecodeError> {
    let code_len = cursor.uvarint()? as usize;
    let code = cursor.take(code_len)?.to_vec();

    let lines = read_lines_rle(cursor, code_len)?;

    let const_count = cursor.uvarint()? as usize;
    let mut constants = Vec::with_capacity(const_count);
    for _ in 0..const_count {
        constants.push(read_value(cursor, stdlib, pool)?);
    }

    Ok(Chunk {
        code,
        constants,
        lines,
    })
}

fn read_lines_rle(cursor: &mut Cursor, code_len: usize) -> Result<Vec<u32>, BytecodeError> {
    let run_count = cursor.uvarint()? as usize;
    let mut lines = Vec::with_capacity(code_len);
    let mut prev_line: i64 = 0;
    for _ in 0..run_count {
        let delta = cursor.ivarint()?;
        let line = prev_line + delta;
        prev_line = line;
        let count = cursor.uvarint()?;
        for _ in 0..count {
            lines.push(line as u32);
        }
    }
    if lines.len() != code_len {
        return Err(BytecodeError(
            "corrupt .rlc file: line table length mismatch".to_string(),
        ));
    }
    Ok(lines)
}

fn read_value(
    cursor: &mut Cursor,
    stdlib: &Module,
    pool: &[String],
) -> Result<VmValue, BytecodeError> {
    let tag = cursor.u8()?;
    Ok(match tag {
        0 => VmValue::Null,
        1 => VmValue::Int(cursor.ivarint()?),
        2 => VmValue::Float(cursor.f64()?),
        3 => VmValue::Bool(cursor.u8()? != 0),
        4 => VmValue::Byte(cursor.u8()?),
        5 => {
            let code_point = cursor.uvarint()? as u32;
            let c = char::from_u32(code_point)
                .ok_or_else(|| BytecodeError("invalid char in .rlc file".to_string()))?;
            VmValue::Char(c)
        }
        6 => {
            let idx = cursor.uvarint()? as u32;
            VmValue::Str(pool_str(pool, idx)?.into())
        }
        7 => {
            let name_idx = cursor.uvarint()? as u32;
            let name = pool_str(pool, name_idx)?.to_string();
            let arity = cursor.uvarint()? as usize;
            let chunk = read_chunk(cursor, stdlib, pool)?;
            VmValue::Function(Rc::new(VmFunction { name, arity, chunk }))
        }
        8 => {
            let name_idx = cursor.uvarint()? as u32;
            let name = pool_str(pool, name_idx)?;
            let native = find_native_by_name(stdlib, name).ok_or_else(|| {
                BytecodeError(format!(
                    "unresolved native function '{}' - this .rlc file may have been \
                     compiled by an incompatible rl version",
                    name
                ))
            })?;
            VmValue::Native(native)
        }
        9 => VmValue::Ok(Box::new(read_value(cursor, stdlib, pool)?)),
        10 => VmValue::Err(Box::new(read_value(cursor, stdlib, pool)?)),
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
