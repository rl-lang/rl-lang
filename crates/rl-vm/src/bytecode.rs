//! Binary (de)serialization of a compiled [Chunk] to/from the .rlc
//! bytecode file format.
//!
//! # Layout
//!
//!
//! [ magic: b"RLZ2" ]
//! [ zstd-compressed payload ]
//!
//!
//! The payload, once decompressed, is the RLC2 format:
//!
//!
//! [ string pool ]
//! [ chunk ]
//!
//!
//! The string pool deduplicates every string used anywhere in the file
//! (string constants, function names, native names) so repeated
//! identifiers - which are common across nested function chunks - are
//! stored once. It is encoded as:
//!
//!
//! uvarint string_count | string*
//!
//!
//! where each string is `uvarint byte_len | utf8 bytes`.
//!
//! A chunk is encoded as:
//!
//!
//! uvarint code_len | code bytes | line_table | uvarint const_count | constant*
//!
//!
//! The line table is run-length encoded rather than storing one line
//! number per bytecode byte, since consecutive instructions usually
//! share a line:
//!
//!
//! uvarint run_count | (ivarint line_delta, uvarint run_length)*
//!
//!
//! line_delta is relative to the previous run's line (first run
//! relative to 0), which keeps it small since consecutive lines are
//! usually +0 or +1.
//!
//! Each constant starts with a 1-byte tag (see [write_value] /
//! [read_value]) followed by its payload. Function constants embed a
//! nested chunk, so the format is recursive. Native constants store
//! only their name (as a pool index); they're re-resolved against the
//! running process's stdlib [Module] tree on load, since function
//! pointers can't be serialized across processes/builds.
//!
//! All integers (lengths, counts, arities, ints, line deltas) are
//! encoded as ULEB128/zigzag-LEB128 varints rather than fixed-width
//! fields, since most values in practice are small.
//!
//! The outer zstd wrapper is a load-time-only cost: decompression
//! happens once, before the chunk is handed to the VM, so it has no
//! effect on bytecode execution speed. It trades a small amount of
//! compress/decompress time for smaller files on disk.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::chunk::Chunk;
use crate::native::Module;
use crate::values::{RecordFields, VmFunction, VmMapKey, VmValue};

const MAGIC: &[u8; 4] = b"RLZ2";

/// zstd compression level. 1 = fastest/worst ratio, 22 = slowest/best
/// ratio. 19 is "high" without being the extreme, slow tail of the
/// range - reasonable for a one-time compile-time cost.
const ZSTD_LEVEL: i32 = 19;

#[derive(Debug)]
pub struct BytecodeError(pub String);

impl Display for BytecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Serializes chunk into the .rlc binary format: an RLC2 payload
/// (string pool + chunk, both varint/RLE-encoded) wrapped in zstd
/// compression.
pub fn serialize_chunk(chunk: &Chunk) -> Vec<u8> {
    let payload = serialize_payload(chunk);

    let compressed = zstd::encode_all(&payload[..], ZSTD_LEVEL)
        .expect("compressing an in-memory buffer cannot fail");

    let mut out = Vec::with_capacity(4 + compressed.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&compressed);
    out
}

/// Parses a .rlc file previously produced by [serialize_chunk].
///
/// stdlib is used to re-resolve any Native function constants by
/// name; pass rl_vm::stdlib::root().
pub fn deserialize_chunk(bytes: &[u8], stdlib: &Module) -> Result<Chunk, BytecodeError> {
    if bytes.len() < 4 || &bytes[0..4] != MAGIC {
        return Err(BytecodeError(
            "not a valid .rlc file (bad magic header)".to_string(),
        ));
    }

    let payload = zstd::decode_all(&bytes[4..])
        .map_err(|e| BytecodeError(format!("failed to decompress .rlc file: {e}")))?;

    deserialize_payload(&payload, stdlib)
}

// ---- RLC2 payload (pre-compression) ----

/// Encodes chunk as an uncompressed RLC2 payload: string pool
/// followed by the chunk itself. This is what gets zstd-compressed
/// by [serialize_chunk].
fn serialize_payload(chunk: &Chunk) -> Vec<u8> {
    // Pass 1: walk the chunk (and every nested function chunk) to
    // collect every string that will need to be written, deduplicating
    // as we go.
    let mut pool = StringPoolBuilder::default();
    collect_strings_chunk(chunk, &mut pool);

    // Pass 2: write the pool, then the chunk itself, with strings
    // replaced by indices into the pool.
    let mut out = Vec::new();
    write_string_pool(&pool, &mut out);
    write_chunk(chunk, &pool, &mut out);
    out
}

/// Parses an uncompressed RLC2 payload previously produced by
/// [serialize_payload]. Called by [deserialize_chunk] after
/// decompression.
fn deserialize_payload(bytes: &[u8], stdlib: &Module) -> Result<Chunk, BytecodeError> {
    let mut cursor = Cursor {
        data: bytes,
        pos: 0,
    };
    let pool = read_string_pool(&mut cursor)?;
    read_chunk(&mut cursor, stdlib, &pool)
}

// ---- string pool ----

/// Built while writing: deduplicates strings and remembers the order
/// they were first seen in, which becomes their index in the file.
#[derive(Default)]
struct StringPoolBuilder {
    index: HashMap<String, u32>,
    strings: Vec<String>,
}

impl StringPoolBuilder {
    /// Interns `s`, returning its (possibly pre-existing) index.
    fn intern(&mut self, s: &str) -> u32 {
        if let Some(&i) = self.index.get(s) {
            return i;
        }
        let i = self.strings.len() as u32;
        self.index.insert(s.to_string(), i);
        self.strings.push(s.to_string());
        i
    }

    /// Looks up the index of a string that was already interned during
    /// the collection pass. Panics if it wasn't - that indicates
    /// collect_strings_* missed a string that write_value tried to use.
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

fn pool_str(pool: &[String], idx: u32) -> Result<&str, BytecodeError> {
    pool.get(idx as usize)
        .map(String::as_str)
        .ok_or_else(|| BytecodeError("corrupt .rlc file: string pool index out of range".into()))
}

/// Recursively collects every string referenced by chunk (string
/// constants, function names, native names) into pool, including
/// strings nested inside function constants' own chunks.
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
        VmValue::Ok(inner) | VmValue::Err(inner) | VmValue::Error(inner) => {
            collect_strings_value(inner, pool)
        }
        VmValue::Null
        | VmValue::Int(_)
        | VmValue::Float(_)
        | VmValue::Bool(_)
        | VmValue::Byte(_)
        | VmValue::Char(_) => {}

        VmValue::Arr(items) | VmValue::Tuple(items) => {
            for item in items.iter() {
                collect_strings_value(item, pool);
            }
        }

        VmValue::Set(items) => {
            for item in items.iter() {
                collect_strings_value(item, pool);
            }
        }
        VmValue::Map(entries) => {
            for (k, v) in entries.borrow().iter() {
                collect_strings_value(&k.clone().into_value(), pool);
                collect_strings_value(v, pool);
            }
        }

        VmValue::Record { fields, .. } => {
            for (fname, fval) in fields.iter() {
                pool.intern(&fname);
                collect_strings_value(&fval, pool);
            }
        }
        VmValue::Tag { name, variant } => {
            pool.intern(name);
            pool.intern(variant);
        }
    }
}

// ---- varint helpers ----

/// Writes an unsigned LEB128 varint.
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

/// Zigzag-encodes a signed value then writes it as an unsigned varint,
/// so small negative numbers stay small: (0, -1, 1, -2, 2, ...) ->
/// (0, 1, 2, 3, 4, ...).
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

/// Encodes a per-instruction line table as runs of (line, run_length)
/// rather than one line number per bytecode byte, since consecutive
/// instructions overwhelmingly share the same line.
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
        VmValue::Error(inner) => {
            out.push(11);
            write_value(inner, pool, out);
        }
        VmValue::Arr(items) => {
            out.push(12);
            write_uvarint(items.len() as u64, out);
            for item in items.iter() {
                write_value(item, pool, out);
            }
        }
        VmValue::Tuple(items) => {
            out.push(13);
            write_uvarint(items.len() as u64, out);
            for item in items.iter() {
                write_value(item, pool, out);
            }
        }
        VmValue::Set(items) => {
            out.push(14);
            write_uvarint(items.len() as u64, out);
            for item in items.iter() {
                write_value(item, pool, out);
            }
        }
        VmValue::Map(entries) => {
            out.push(15);
            let entries = entries.borrow();
            write_uvarint(entries.len() as u64, out);
            for (k, v) in entries.iter() {
                write_value(&k.clone().into_value(), pool, out);
                write_value(v, pool, out);
            }
        }
        VmValue::Record { name, fields } => {
            out.push(16);
            write_uvarint(pool.get(name) as u64, out);
            let fields = fields;
            write_uvarint(fields.len() as u64, out);
            for (fname, fval) in fields.iter() {
                write_uvarint(pool.get(&fname) as u64, out);
                write_value(&fval, pool, out);
            }
        }
        VmValue::Tag { name, variant } => {
            out.push(17);
            write_uvarint(pool.get(name) as u64, out);
            write_uvarint(pool.get(variant) as u64, out);
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

    /// Reads an unsigned LEB128 varint.
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

    /// Reads a zigzag-encoded signed varint.
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
        11 => VmValue::Error(Box::new(read_value(cursor, stdlib, pool)?)),
        12 => {
            let len = cursor.uvarint()? as usize;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(read_value(cursor, stdlib, pool)?);
            }
            VmValue::Arr(Rc::new(items))
        }
        13 => {
            let len = cursor.uvarint()? as usize;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(read_value(cursor, stdlib, pool)?);
            }
            VmValue::Tuple(Rc::new(items))
        }
        14 => {
            let len = cursor.uvarint()? as usize;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(read_value(cursor, stdlib, pool)?);
            }
            VmValue::Set(Rc::new(items))
        }
        15 => {
            let len = cursor.uvarint()? as usize;
            let mut map = HashMap::with_capacity(len);
            for _ in 0..len {
                let k = read_value(cursor, stdlib, pool)?;
                let v = read_value(cursor, stdlib, pool)?;
                let key = VmMapKey::from_value(&k)
                    .ok_or_else(|| BytecodeError("corrupt .rlc: invalid map key".into()))?;
                map.insert(key, v);
            }
            VmValue::Map(Rc::new(RefCell::new(map)))
        }
        16 => {
            let name_idx = cursor.uvarint()? as u32;
            let name = pool_str(pool, name_idx)?;
            let len = cursor.uvarint()? as usize;
            let mut fields = Vec::with_capacity(len);
            for _ in 0..len {
                let fname_idx = cursor.uvarint()? as u32;
                let fname = pool_str(pool, fname_idx)?;
                let fval = read_value(cursor, stdlib, pool)?;
                fields.push((Rc::from(fname), fval));
            }
            VmValue::Record {
                name: Rc::from(name),
                fields: RecordFields::new(fields),
            }
        }
        17 => {
            let name_idx = cursor.uvarint()? as u32;
            let name = pool_str(pool, name_idx)?;
            let variant_idx = cursor.uvarint()? as u32;
            let variant = pool_str(pool, variant_idx)?;
            VmValue::Tag {
                name: Rc::from(name),
                variant: Rc::from(variant),
            }
        }
        other => {
            return Err(BytecodeError(format!(
                "corrupt .rlc file: unknown constant tag {other}"
            )));
        }
    })
}

/// Recursively searches module and its submodules for a native function
/// with leaf name name.
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
