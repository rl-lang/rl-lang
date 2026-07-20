//! Binary (de)serialization of a compiled [Chunk] to/from the .rlc
//! bytecode file format.
//!
//! # Layout
//!
//!
//! [ magic: b"RLZ3" ]
//! [ zstd-compressed payload ]
//!
//!
//! The payload, once decompressed, is the RLC3 format:
//!
//!
//! [ string pool ]
//! [ chunk ]
//! [ line index ]
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
//! uvarint code_len | code bytes | span_table | uvarint const_count | constant*
//!
//!
//! The span table is run-length encoded rather than storing one
//! (start, end) byte-offset pair per bytecode byte, since consecutive
//! instructions usually share a span:
//!
//!
//! uvarint run_count | (ivarint start_delta, ivarint end_delta, uvarint run_length)*
//!
//!
//! start_delta/end_delta are relative to the previous run's start/end
//! (first run relative to 0), which keeps them small since nearby
//! instructions usually point at nearby source ranges.
//!
//!
//! The line index is a source-free `rl_utils::line_index::LineIndex`:
//! just the byte offset where each source line starts, so runtime errors
//! raised from this file can report a `file:line:col` location without
//! the file embedding (or leaking) the original source text. It's
//! optional - `rl compile`/`rl package --vm` always include one when a
//! source file was available, but the format tolerates its absence:
//!
//!
//! u8 present_flag | (uvarint source_name_pool_idx | uvarint line_count | ivarint line_start_delta*) if present
//!
//!
//! line_start_delta is relative to the previous entry (first relative
//! to 0), same rationale as the span table above.
//!
//! Each constant starts with a 1-byte tag (see [write_value] /
//! [read_value]) followed by its payload. Function constants embed a
//! nested chunk, so the format is recursive. Native constants store
//! only their name (as a pool index); they're re-resolved against the
//! running process's stdlib [Module] tree on load, since function
//! pointers can't be serialized across processes/builds.
//!
//! All integers (lengths, counts, arities, ints, span deltas) are
//! encoded as ULEB128/zigzag-LEB128 varints rather than fixed-width
//! fields, since most values in practice are small.
//!
//! The outer zstd wrapper is a load-time-only cost: decompression
//! happens once, before the chunk is handed to the VM, so it has no
//! effect on bytecode execution speed. It trades a small amount of
//! compress/decompress time for smaller files on disk.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::chunk::Chunk;
use crate::native::Module;
use crate::values::{RecordFields, VmFunction, VmMapKey, VmValue};
use rl_utils::line_index::LineIndex;
use rl_utils::span::Span;

const MAGIC: &[u8; 4] = b"RLZ3";

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
/// (string pool + chunk + line index, all varint/RLE-encoded) wrapped
/// in zstd compression.
///
/// `line_index` should be `Some` whenever the chunk was compiled from a
/// real source file, so runtime errors from this bytecode can still
/// report a `file:line:col` location even though the source text itself
/// isn't embedded. Pass `None` only when no source was ever available
/// (e.g. re-serializing bytecode that was itself loaded without one).
pub fn serialize_chunk(chunk: &Chunk, line_index: Option<&LineIndex>) -> Vec<u8> {
    let payload = serialize_payload(chunk, line_index);

    let compressed = zstd::encode_all(&payload[..], ZSTD_LEVEL)
        .expect("compressing an in-memory buffer cannot fail");

    let mut out = Vec::with_capacity(4 + compressed.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&compressed);
    out
}

/// Parses a .rlc file previously produced by [serialize_chunk], returning
/// the chunk and its embedded [`LineIndex`] (if the file was compiled
/// with one).
///
/// stdlib is used to re-resolve any Native function constants by
/// name; pass rl_vm::stdlib::root().
pub fn deserialize_chunk(
    bytes: &[u8],
    stdlib: &Module,
) -> Result<(Chunk, Option<LineIndex>), BytecodeError> {
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
fn serialize_payload(chunk: &Chunk, line_index: Option<&LineIndex>) -> Vec<u8> {
    // Pass 1: walk the chunk (and every nested function chunk) to
    // collect every string that will need to be written, deduplicating
    // as we go. The line index's source name is just another string.
    let mut pool = StringPoolBuilder::default();
    collect_strings_chunk(chunk, &mut pool);
    if let Some(index) = line_index {
        pool.intern(index.source_name());
    }

    // Pass 2: write the pool, then the chunk, then the line index, with
    // strings replaced by indices into the pool.
    let mut out = Vec::new();
    write_string_pool(&pool, &mut out);
    write_chunk(chunk, &pool, &mut out);
    write_line_index(line_index, &pool, &mut out);
    out
}

/// Parses an uncompressed RLC2 payload previously produced by
/// [serialize_payload]. Called by [deserialize_chunk] after
/// decompression.
fn deserialize_payload(
    bytes: &[u8],
    stdlib: &Module,
) -> Result<(Chunk, Option<LineIndex>), BytecodeError> {
    let mut cursor = Cursor {
        data: bytes,
        pos: 0,
    };
    let pool = read_string_pool(&mut cursor)?;
    let chunk = read_chunk(&mut cursor, stdlib, &pool)?;
    let line_index = read_line_index(&mut cursor, &pool)?;
    Ok((chunk, line_index))
}

/// Writes the optional line index: a single flag byte, then (when
/// present) the pooled source name followed by the RLE-delta-encoded
/// line-start table. See the module-level format docs.
fn write_line_index(index: Option<&LineIndex>, pool: &StringPoolBuilder, out: &mut Vec<u8>) {
    match index {
        None => out.push(0),
        Some(index) => {
            out.push(1);
            write_uvarint(pool.get(index.source_name()) as u64, out);
            let starts = index.line_starts();
            write_uvarint(starts.len() as u64, out);
            let mut prev: i64 = 0;
            for &start in starts {
                write_ivarint(start as i64 - prev, out);
                prev = start as i64;
            }
        }
    }
}

fn read_line_index(
    cursor: &mut Cursor,
    pool: &[String],
) -> Result<Option<LineIndex>, BytecodeError> {
    match cursor.u8()? {
        0 => Ok(None),
        1 => {
            let name_idx = cursor.uvarint()? as u32;
            let name = pool_str(pool, name_idx)?.to_string();
            let count = cursor.uvarint()? as usize;
            let mut starts = Vec::with_capacity(count);
            let mut prev: i64 = 0;
            for _ in 0..count {
                prev += cursor.ivarint()?;
                starts.push(prev as u32);
            }
            Ok(Some(LineIndex::from_raw(name, starts)))
        }
        other => Err(BytecodeError(format!(
            "corrupt .rlc file: unknown line-index tag {other}"
        ))),
    }
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
            for item in items.borrow().iter() {
                collect_strings_value(&item.clone().into_value(), pool);
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
        VmValue::Closure { func, captured, .. } => {
            collect_strings_value(&VmValue::Function(func.clone()), pool);
            for v in captured.iter() {
                collect_strings_value(v, pool);
            }
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
    write_spans_rle(&chunk.spans, out);
    write_uvarint(chunk.constants.len() as u64, out);
    for c in &chunk.constants {
        write_value(c, pool, out);
    }
}

/// Encodes a per-instruction span table as runs of (span, run_length)
/// rather than one (start, end) pair per bytecode byte, since
/// consecutive instructions overwhelmingly share the same span.
fn write_spans_rle(spans: &[Span], out: &mut Vec<u8>) {
    let mut runs: Vec<(Span, u32)> = Vec::new();
    for &span in spans {
        match runs.last_mut() {
            Some((last_span, count)) if *last_span == span => *count += 1,
            _ => runs.push((span, 1)),
        }
    }

    write_uvarint(runs.len() as u64, out);
    let mut prev_start: i64 = 0;
    let mut prev_end: i64 = 0;
    for (span, count) in runs {
        write_ivarint(span.start as i64 - prev_start, out);
        write_ivarint(span.end as i64 - prev_end, out);
        write_uvarint(count as u64, out);
        prev_start = span.start as i64;
        prev_end = span.end as i64;
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
            let items = items.borrow();
            write_uvarint(items.len() as u64, out);
            for item in items.iter() {
                write_value(&item.clone().into_value(), pool, out);
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
        VmValue::Closure {
            func,
            captured,
            capture_start,
        } => {
            out.push(18);
            write_value(&VmValue::Function(func.clone()), pool, out);
            write_uvarint(*capture_start as u64, out);
            write_uvarint(captured.len() as u64, out);
            for v in captured.iter() {
                write_value(v, pool, out);
            }
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

    let spans = read_spans_rle(cursor, code_len)?;

    let const_count = cursor.uvarint()? as usize;
    let mut constants = Vec::with_capacity(const_count);
    for _ in 0..const_count {
        constants.push(read_value(cursor, stdlib, pool)?);
    }

    Ok(Chunk {
        code,
        constants,
        spans,
    })
}

fn read_spans_rle(cursor: &mut Cursor, code_len: usize) -> Result<Vec<Span>, BytecodeError> {
    let run_count = cursor.uvarint()? as usize;
    let mut spans = Vec::with_capacity(code_len);
    let mut prev_start: i64 = 0;
    let mut prev_end: i64 = 0;
    for _ in 0..run_count {
        let start = prev_start + cursor.ivarint()?;
        let end = prev_end + cursor.ivarint()?;
        prev_start = start;
        prev_end = end;
        let span = Span::new(start as usize, end as usize);
        let count = cursor.uvarint()?;
        for _ in 0..count {
            spans.push(span);
        }
    }
    if spans.len() != code_len {
        return Err(BytecodeError(
            "corrupt .rlc file: span table length mismatch".to_string(),
        ));
    }
    Ok(spans)
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
            let mut set = HashSet::with_capacity(len);
            for _ in 0..len {
                let v = read_value(cursor, stdlib, pool)?;
                let key = VmMapKey::from_value(&v)
                    .ok_or_else(|| BytecodeError("corrupt .rlc: invalid set element".into()))?;
                set.insert(key);
            }
            VmValue::Set(Rc::new(RefCell::new(set)))
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
        18 => {
            let VmValue::Function(func) = read_value(cursor, stdlib, pool)? else {
                return Err(BytecodeError(
                    "corrupt .rlc: closure template is not a function".into(),
                ));
            };
            let capture_start = cursor.uvarint()? as u16;
            let len = cursor.uvarint()? as usize;
            let mut captured = Vec::with_capacity(len);
            for _ in 0..len {
                captured.push(read_value(cursor, stdlib, pool)?);
            }
            VmValue::Closure {
                func,
                captured: Rc::new(captured),
                capture_start,
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
