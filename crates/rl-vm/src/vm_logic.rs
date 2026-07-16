use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::chunk::{Chunk, OpCode};
use crate::values::{RecordFields, VmFunction, VmMapKey, VmValue};

#[derive(Debug)]
pub struct VmError(pub String);

enum FrameSource<'a> {
    Top(&'a Chunk),
    Func(Rc<VmFunction>),
}

impl<'a> FrameSource<'a> {
    #[inline]
    fn chunk(&self) -> &Chunk {
        match self {
            FrameSource::Top(c) => c,
            FrameSource::Func(f) => &f.chunk,
        }
    }
}

struct CallFrame<'a> {
    source: FrameSource<'a>,
    ip: usize,
    scope_base: usize,
}

pub struct Vm {
    stack: Vec<VmValue>,
    globals: Vec<VmValue>,
    locals: Vec<VmValue>,
    scope_starts: Vec<usize>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            globals: Vec::new(),
            locals: Vec::new(),
            scope_starts: Vec::new(),
        }
    }

    pub fn run_and_return(&mut self, chunk: &Chunk) -> Result<VmValue, VmError> {
        self.run(chunk)?;
        Ok(self.stack.pop().unwrap_or(VmValue::Null))
    }

    /// Vm entry function
    pub fn run(&mut self, chunk: &Chunk) -> Result<(), VmError> {
        let mut frames: Vec<CallFrame> = vec![CallFrame {
            source: FrameSource::Top(chunk),
            ip: 0,
            scope_base: self.scope_starts.len(),
        }];

        // caching method
        let mut cur_chunk: *const Chunk = frames[0].source.chunk();
        let mut ip: usize = 0;
        let mut scope_base: usize = frames[0].scope_base;

        macro_rules! chunk {
            () => {
                unsafe { &*cur_chunk }
            };
        }

        loop {
            if ip >= chunk!().code.len() {
                self.stack.push(VmValue::Null);
                frames.last_mut().unwrap().ip = ip;
                if !self.finish_call(&mut frames)? {
                    return Ok(());
                }
                let top = frames.last().unwrap();
                cur_chunk = top.source.chunk();
                ip = top.ip;
                scope_base = top.scope_base;
                continue;
            }

            let op = OpCode::from_u8_unchecked(chunk!().code[ip]);
            ip += 1;

            match op {
                OpCode::Const => {
                    let idx = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let val = chunk!().constants[idx].clone();
                    self.stack.push(val);
                }

                OpCode::Add => self.binary_numeric(|a, b| a + b, |a, b| a + b)?,
                OpCode::Sub => self.binary_numeric(|a, b| a - b, |a, b| a - b)?,
                OpCode::Mul => self.binary_numeric(|a, b| a * b, |a, b| a * b)?,
                OpCode::Div => self.binary_div()?,

                OpCode::Negate => {
                    let v = self.pop()?;
                    let out = match v {
                        VmValue::Int(n) => VmValue::Int(-n),
                        VmValue::Float(n) => VmValue::Float(-n),
                        other => return Err(VmError(format!("cannot negate {other:?}"))),
                    };
                    self.stack.push(out);
                }
                OpCode::Not => {
                    let v = self.pop()?;
                    let out = match v {
                        VmValue::Bool(b) => VmValue::Bool(!b),
                        other => return Err(VmError(format!("cannot apply ! to {other:?}"))),
                    };
                    self.stack.push(out);
                }
                OpCode::Eq => {
                    let (a, b) = self.pop_two()?;
                    self.stack.push(VmValue::Bool(a == b));
                }
                OpCode::NotEq => {
                    let (a, b) = self.pop_two()?;
                    self.stack.push(VmValue::Bool(a != b));
                }
                OpCode::Less => self.binary_cmp(|o| o.is_lt())?,
                OpCode::LessEq => self.binary_cmp(|o| o.is_le())?,
                OpCode::Greater => self.binary_cmp(|o| o.is_gt())?,
                OpCode::GreaterEq => self.binary_cmp(|o| o.is_ge())?,

                OpCode::GetLocal => {
                    let flat = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let frame_base = self.scope_starts[scope_base];
                    let val = self.locals[frame_base + flat].clone();
                    self.stack.push(val);
                }
                OpCode::SetLocal => {
                    let flat = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let val = self
                        .stack
                        .last()
                        .cloned()
                        .ok_or_else(|| VmError("stack underflow on assignment".into()))?;
                    let frame_base = self.scope_starts[scope_base];
                    self.locals[frame_base + flat] = val;
                }
                OpCode::GetGlobal => {
                    let slot = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let val =
                        self.globals.get(slot).cloned().ok_or_else(|| {
                            VmError(format!("read of undefined global slot {slot}"))
                        })?;
                    self.stack.push(val);
                }
                OpCode::SetGlobal => {
                    let slot = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let val = self
                        .stack
                        .last()
                        .cloned()
                        .ok_or_else(|| VmError("stack underflow on assignment".into()))?;
                    if slot >= self.globals.len() {
                        return Err(VmError(format!(
                            "assignment to undefined global slot {slot}"
                        )));
                    }
                    self.globals[slot] = val;
                }
                OpCode::DefineLocal => {
                    let slot = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let val = self.pop()?;

                    if self.scope_starts.len() == scope_base {
                        if slot >= self.globals.len() {
                            self.globals.resize(slot + 1, VmValue::Null);
                        }
                        self.globals[slot] = val;
                    } else {
                        let base = *self.scope_starts.last().unwrap();
                        if base + slot >= self.locals.len() {
                            self.locals.resize(base + slot + 1, VmValue::Null);
                        }
                        self.locals[base + slot] = val;
                    }
                }
                OpCode::Pop => {
                    self.pop()?;
                }

                OpCode::Return => {
                    let ret = self.pop().ok();
                    self.stack.push(ret.unwrap_or(VmValue::Null));
                    frames.last_mut().unwrap().ip = ip;
                    if !self.finish_call(&mut frames)? {
                        return Ok(());
                    }
                    let top = frames.last().unwrap();
                    cur_chunk = top.source.chunk();
                    ip = top.ip;
                    scope_base = top.scope_base;
                }

                OpCode::PushScope => self.scope_starts.push(self.locals.len()),
                OpCode::PopScope => {
                    let num_active = self.scope_starts.len() - scope_base;
                    if num_active <= 1 {
                        return Err(VmError("cannot pop the base call frame".into()));
                    }
                    let start = self.scope_starts.pop().unwrap();
                    self.locals.truncate(start);
                }

                OpCode::Jump => {
                    let offset = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    ip += offset;
                }
                OpCode::JumpIfFalse => {
                    let offset = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    match self.pop()? {
                        VmValue::Bool(false) => ip += offset,
                        VmValue::Bool(true) => {}
                        other => {
                            return Err(VmError(format!(
                                "if/while condition must be bool, got {other:?}"
                            )));
                        }
                    }
                }
                OpCode::Loop => {
                    let offset = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    ip -= offset;
                }

                OpCode::Call => {
                    let arg_count = chunk!().read_u16(ip) as usize;
                    ip += 2;

                    let callee_idx = self.stack.len() - 1 - arg_count;
                    match self.stack[callee_idx].clone() {
                        VmValue::Function(func) => {
                            let base = self.locals.len();
                            self.locals.resize(base + arg_count, VmValue::Null);
                            for i in (0..arg_count).rev() {
                                self.locals[base + i] = self.pop()?;
                            }
                            self.pop()?; // discard the callee itself

                            if arg_count != func.arity {
                                return Err(VmError(format!(
                                    "{} expects {} args, got {}",
                                    func.name, func.arity, arg_count
                                )));
                            }

                            self.scope_starts.push(base);
                            let new_scope_base = self.scope_starts.len() - 1;

                            frames.last_mut().unwrap().ip = ip;
                            cur_chunk = &func.chunk as *const Chunk;
                            frames.push(CallFrame {
                                source: FrameSource::Func(func),
                                ip: 0,
                                scope_base: new_scope_base,
                            });
                            ip = 0;
                            scope_base = new_scope_base;
                        }

                        VmValue::Native(native) => {
                            let mut call_args = Vec::with_capacity(arg_count);
                            for _ in 0..arg_count {
                                call_args.push(self.pop()?);
                            }
                            call_args.reverse();
                            self.pop()?; // discard the callee itself

                            let result = (native.func)(self, call_args)?;
                            self.stack.push(result);
                        }

                        VmValue::Closure {
                            func,
                            captured,
                            capture_start,
                        } => {
                            if arg_count != func.arity {
                                return Err(VmError(format!(
                                    "closure expects {} args, got {}",
                                    func.arity, arg_count
                                )));
                            }
                            let base = self.locals.len();
                            self.locals
                                .resize(base + capture_start as usize, VmValue::Null);
                            self.locals.extend_from_slice(&captured);
                            let params_start = base + capture_start as usize + captured.len();
                            self.locals.resize(params_start + arg_count, VmValue::Null);
                            for i in (0..arg_count).rev() {
                                self.locals[params_start + i] = self.pop()?;
                            }
                            self.pop()?; // discard the closure itself

                            self.scope_starts.push(base);
                            let new_scope_base = self.scope_starts.len() - 1;

                            frames.last_mut().unwrap().ip = ip;
                            cur_chunk = &func.chunk as *const Chunk;
                            frames.push(CallFrame {
                                source: FrameSource::Func(func),
                                ip: 0,
                                scope_base: new_scope_base,
                            });
                            ip = 0;
                            scope_base = new_scope_base;
                        }

                        other => return Err(VmError(format!("cannot call {other:?}"))),
                    }
                }

                OpCode::Ok => {
                    let v = self.pop()?;
                    self.stack.push(VmValue::Ok(Box::new(v)));
                }

                OpCode::Err => {
                    let v = self.pop()?;
                    self.stack.push(VmValue::Err(Box::new(v)));
                }

                OpCode::Propagate => {
                    let v = self.pop()?;
                    match v {
                        VmValue::Ok(inner) => self.stack.push(*inner),
                        VmValue::Err(_) => {
                            self.stack.push(v);
                            frames.last_mut().unwrap().ip = ip;
                            if !self.finish_call(&mut frames)? {
                                return Ok(());
                            }
                            let top = frames.last().unwrap();
                            cur_chunk = top.source.chunk();
                            ip = top.ip;
                            scope_base = top.scope_base;
                        }
                        other => self.stack.push(other),
                    }
                }

                OpCode::Error => {
                    let v = self.pop()?;
                    self.stack.push(VmValue::Error(Box::new(v)));
                }

                OpCode::BuildArr => {
                    let count = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    if self.stack.len() < count {
                        return Err(VmError("stack underflow building array".into()));
                    }
                    let items = self.stack.split_off(self.stack.len() - count);
                    self.stack.push(VmValue::Arr(Rc::new(items)));
                }

                OpCode::BuildTuple => {
                    let count = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    if self.stack.len() < count {
                        return Err(VmError("stack underflow building tuple".into()));
                    }
                    let items = self.stack.split_off(self.stack.len() - count);
                    self.stack.push(VmValue::Tuple(Rc::new(items)));
                }

                OpCode::Index => {
                    let index = self.pop()?;
                    let arr = self.pop()?;
                    let elem = Self::index_get(&arr, &index)?;
                    self.stack.push(elem);
                }

                OpCode::ArrSet => {
                    let value = self.pop()?;
                    let index = self.pop()?;
                    let arr = self.pop()?;
                    self.stack.push(Self::index_set(arr, &index, value)?);
                }

                OpCode::BuildSet => {
                    let count = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    if self.stack.len() < count {
                        return Err(VmError("stack underflow building set".into()));
                    }
                    let items = self.stack.split_off(self.stack.len() - count);
                    let mut set = HashSet::with_capacity(count);
                    for v in items {
                        let key = VmMapKey::from_value(&v).ok_or_else(|| {
                            VmError(format!("type {} cannot be a set element", v.type_name()))
                        })?;
                        set.insert(key);
                    }
                    self.stack.push(VmValue::Set(Rc::new(RefCell::new(set))));
                }

                OpCode::BuildMap => {
                    let count = chunk!().read_u16(ip) as usize; // number of entries
                    ip += 2;
                    if self.stack.len() < count * 2 {
                        return Err(VmError("stack underflow building map".into()));
                    }
                    let flat = self.stack.split_off(self.stack.len() - count * 2);
                    let mut map = HashMap::with_capacity(count);
                    for pair in flat.chunks_exact(2) {
                        let key = VmMapKey::from_value(&pair[0]).ok_or_else(|| {
                            VmError(format!(
                                "type {} cannot be used as a map key",
                                pair[0].type_name()
                            ))
                        })?;
                        map.insert(key, pair[1].clone());
                    }
                    self.stack.push(VmValue::Map(Rc::new(RefCell::new(map))));
                }

                OpCode::BuildRecord => {
                    let name_idx = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let fields_idx = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let count = chunk!().read_u16(ip) as usize;
                    ip += 2;

                    let VmValue::Str(name) = chunk!().constants[name_idx].clone() else {
                        return Err(VmError(
                            "corrupt bytecode: struct name is not a string".into(),
                        ));
                    };
                    let VmValue::Arr(field_names) = chunk!().constants[fields_idx].clone() else {
                        return Err(VmError(
                            "corrupt bytecode: struct field list is not an array".into(),
                        ));
                    };
                    if self.stack.len() < count {
                        return Err(VmError("stack underflow building struct".into()));
                    }
                    let values = self.stack.split_off(self.stack.len() - count);
                    let fields = field_names
                        .iter()
                        .zip(values)
                        .map(|(fname, val)| {
                            let VmValue::Str(fname) = fname else {
                                unreachable!("field name constant must be a string");
                            };
                            (fname.clone(), val)
                        })
                        .collect();
                    self.stack.push(VmValue::Record {
                        name,
                        fields: RecordFields::new(fields),
                    });
                }

                OpCode::FieldGet => {
                    let field_idx = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let VmValue::Str(field) = chunk!().constants[field_idx].clone() else {
                        return Err(VmError(
                            "corrupt bytecode: field name is not a string".into(),
                        ));
                    };
                    let target = self.pop()?;
                    let VmValue::Record { name, fields } = &target else {
                        return Err(VmError(format!(
                            "cannot access field `{}` on {}",
                            field,
                            target.type_name()
                        )));
                    };
                    let value = fields.get(&field).ok_or_else(|| {
                        VmError(format!("record `{}` has no field `{}`", name, field))
                    })?;
                    self.stack.push(value);
                }

                OpCode::FieldSet => {
                    let field_idx = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let VmValue::Str(field) = chunk!().constants[field_idx].clone() else {
                        return Err(VmError(
                            "corrupt bytecode: field name is not a string".into(),
                        ));
                    };
                    let value = self.pop()?;
                    let target = self.pop()?;
                    let VmValue::Record { name, fields } = &target else {
                        return Err(VmError(format!(
                            "cannot assign field `{}` on {}",
                            field,
                            target.type_name()
                        )));
                    };
                    if !fields.has(&field) {
                        return Err(VmError(format!(
                            "record `{}` has no field `{}`",
                            name, field
                        )));
                    }
                    fields.set(&field, value.clone());
                    self.stack.push(value);
                }
                OpCode::BuildClosure => {
                    let const_idx = chunk!().read_u16(ip) as usize;
                    ip += 2;
                    let capture_start = chunk!().read_u16(ip);
                    ip += 2;
                    let VmValue::Function(func) = chunk!().constants[const_idx].clone() else {
                        return Err(VmError(
                            "corrupt bytecode: closure template is not a function".into(),
                        ));
                    };

                    let captured = if self.scope_starts.len() == scope_base {
                        Vec::new()
                    } else {
                        let frame_base = self.scope_starts[scope_base];
                        let start = frame_base + capture_start as usize;
                        if start > self.locals.len() {
                            return Err(VmError(format!(
                                "corrupt bytecode: closure capture_start {capture_start} exceeds live locals ({} available)",
                                self.locals.len() - frame_base
                            )));
                        }
                        self.locals[start..].to_vec()
                    };

                    self.stack.push(VmValue::Closure {
                        func,
                        captured: Rc::new(captured),
                        capture_start,
                    });
                }
            }
        }
    }

    fn finish_call(&mut self, frames: &mut Vec<CallFrame>) -> Result<bool, VmError> {
        let finished = frames.pop().expect("frame stack must not be empty");
        if finished.scope_base < self.scope_starts.len() {
            let cut = self.scope_starts[finished.scope_base];
            self.scope_starts.truncate(finished.scope_base);
            self.locals.truncate(cut);
        }
        Ok(!frames.is_empty())
    }

    fn index_as_usize(index: &VmValue, len: usize) -> Result<usize, VmError> {
        let i = match index {
            VmValue::Int(n) => *n,
            VmValue::Byte(b) => *b as i64,
            other => {
                return Err(VmError(format!(
                    "array index must be int or byte, got {}",
                    other.type_name()
                )));
            }
        };
        if i < 0 || i as usize >= len {
            return Err(VmError(format!(
                "array index out of bounds: {i} (len {len})"
            )));
        }
        Ok(i as usize)
    }

    fn index_get(arr: &VmValue, index: &VmValue) -> Result<VmValue, VmError> {
        match arr {
            VmValue::Arr(items) | VmValue::Tuple(items) => {
                let i = Self::index_as_usize(index, items.len())?;
                Ok(items[i].clone())
            }
            VmValue::Map(entries) => {
                let key = VmMapKey::from_value(index).ok_or_else(|| {
                    VmError(format!(
                        "type {} cannot be used as a map key",
                        index.type_name()
                    ))
                })?;
                entries
                    .borrow()
                    .get(&key)
                    .cloned()
                    .ok_or_else(|| VmError(format!("key {} not found in map", index)))
            }
            other => Err(VmError(format!("cannot index into {}", other.type_name()))),
        }
    }

    fn index_set(arr: VmValue, index: &VmValue, value: VmValue) -> Result<VmValue, VmError> {
        match arr {
            VmValue::Arr(items) => {
                let i = Self::index_as_usize(index, items.len())?;
                let mut items = Rc::try_unwrap(items).unwrap_or_else(|rc| (*rc).clone());
                items[i] = value;
                Ok(VmValue::Arr(Rc::new(items)))
            }
            VmValue::Map(entries) => {
                let key = VmMapKey::from_value(index).ok_or_else(|| {
                    VmError(format!(
                        "type {} cannot be used as a map key",
                        index.type_name()
                    ))
                })?;
                entries.borrow_mut().insert(key, value);
                Ok(VmValue::Map(entries))
            }
            other => Err(VmError(format!("cannot index into {}", other.type_name()))),
        }
    }

    #[inline(always)]
    fn pop_unchecked(&mut self) -> VmValue {
        debug_assert!(!self.stack.is_empty(), "stack underflow");
        let new_len = self.stack.len() - 1;
        unsafe {
            self.stack.set_len(new_len);
            std::ptr::read(self.stack.as_ptr().add(new_len))
        }
    }
    #[inline(always)]
    fn pop_two_unchecked(&mut self) -> (VmValue, VmValue) {
        let b = self.pop_unchecked();
        let a = self.pop_unchecked();
        (a, b)
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

    /// Helper function for arth operations
    /// handles +, -, *
    /// currently promotes int to float
    fn binary_numeric(
        &mut self,
        int_op: fn(i64, i64) -> i64,
        float_op: fn(f64, f64) -> f64,
    ) -> Result<(), VmError> {
        let (a, b) = self.pop_two_unchecked();
        let out = match (a, b) {
            (VmValue::Int(a), VmValue::Int(b)) => VmValue::Int(int_op(a, b)),
            (VmValue::Float(a), VmValue::Float(b)) => VmValue::Float(float_op(a, b)),
            // for now int promoted to float
            // should disable later after wiring `as` Cast keyword
            (VmValue::Int(a), VmValue::Float(b)) => VmValue::Float(float_op(a as f64, b)),
            (VmValue::Float(a), VmValue::Int(b)) => VmValue::Float(float_op(a, b as f64)),
            (a, b) => {
                return Err(VmError(format!(
                    "cannot apply arithmetic op to {a:?} and {b:?}"
                )));
            }
        };
        self.stack.push(out);
        Ok(())
    }

    /// Helper function for arth operations
    /// handles /
    /// currently promotes int to float
    fn binary_div(&mut self) -> Result<(), VmError> {
        let (a, b) = self.pop_two_unchecked();
        let out = match (a, b) {
            (VmValue::Int(_), VmValue::Int(0)) => return Err(VmError("division by zero".into())),
            (VmValue::Int(a), VmValue::Int(b)) => VmValue::Int(a / b),
            (VmValue::Float(a), VmValue::Float(b)) => VmValue::Float(a / b),
            (VmValue::Int(a), VmValue::Float(b)) => VmValue::Float(a as f64 / b),
            (VmValue::Float(a), VmValue::Int(b)) => VmValue::Float(a / b as f64),
            (a, b) => return Err(VmError(format!("cannot divide {a:?} by {b:?}"))),
        };
        self.stack.push(out);
        Ok(())
    }

    /// Helper function for comparsion operations
    /// accepts float/float, int/int and promotes int to float
    /// handles >, <, >=, <=
    fn binary_cmp(&mut self, pred: fn(std::cmp::Ordering) -> bool) -> Result<(), VmError> {
        let (a, b) = self.pop_two_unchecked();
        let ord = match (&a, &b) {
            (VmValue::Int(a), VmValue::Int(b)) => a.partial_cmp(b),
            (VmValue::Float(a), VmValue::Float(b)) => a.partial_cmp(b),
            (VmValue::Int(a), VmValue::Float(b)) => (*a as f64).partial_cmp(b),
            (VmValue::Float(a), VmValue::Int(b)) => a.partial_cmp(&(*b as f64)),
            _ => return Err(VmError(format!("cannot compare {a:?} and {b:?}"))),
        }
        .ok_or_else(|| VmError("comparison produced no ordering (NaN?)".into()))?;
        self.stack.push(VmValue::Bool(pred(ord)));
        Ok(())
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
