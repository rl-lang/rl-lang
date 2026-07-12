use std::rc::Rc;

use crate::vm::chunk::{Chunk, OpCode, VmFunction, VmValue};

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

    /// Vm entry function
    pub fn run(&mut self, chunk: &Chunk) -> Result<Option<VmValue>, VmError> {
        let mut frames: Vec<CallFrame> = vec![CallFrame {
            source: FrameSource::Top(chunk),
            ip: 0,
            scope_base: self.scope_starts.len(),
        }];

        loop {
            let top = frames.len() - 1;

            if frames[top].ip >= frames[top].source.chunk().code.len() {
                self.stack.push(VmValue::Null);
                if !self.finish_call(&mut frames)? {
                    return Ok(self.stack.pop());
                }
                continue;
            }

            let op = OpCode::from_u8(frames[top].source.chunk().code[frames[top].ip]);
            frames[top].ip += 1;

            match op {
                OpCode::Const => {
                    let idx = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;
                    let val = frames[top].source.chunk().constants[idx].clone();
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
                    let depth = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;
                    let slot = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;

                    let scope_base = frames[top].scope_base;
                    let val = match self.resolve(scope_base, depth)? {
                        None => self.globals.get(slot),
                        Some(scope_idx) => self.local_slot(scope_idx, slot),
                    }
                    .ok_or_else(|| {
                        VmError(format!(
                            "read of undefined local slot {slot} at depth {depth}"
                        ))
                    })?
                    .clone();
                    self.stack.push(val);
                }
                OpCode::SetLocal => {
                    let depth = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;
                    let slot = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;

                    let val = self
                        .stack
                        .last()
                        .cloned()
                        .ok_or_else(|| VmError("stack underflow on assignment".into()))?;

                    let scope_base = frames[top].scope_base;
                    match self.resolve(scope_base, depth)? {
                        None => {
                            if slot >= self.globals.len() {
                                return Err(VmError(format!(
                                    "assignment to undefined global slot {slot}"
                                )));
                            }
                            self.globals[slot] = val;
                        }
                        Some(scope_idx) => {
                            let (base, end) = self.scope_range(scope_idx);
                            if base + slot >= end {
                                return Err(VmError(format!(
                                    "assignment to undefined local slot {slot} at depth {depth}"
                                )));
                            }
                            self.locals[base + slot] = val;
                        }
                    }
                }
                OpCode::DefineLocal => {
                    let slot = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;
                    let val = self.pop()?;

                    let scope_base = frames[top].scope_base;
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
                    if !self.finish_call(&mut frames)? {
                        return Ok(self.stack.pop());
                    }
                }

                OpCode::PushScope => self.scope_starts.push(self.locals.len()),
                OpCode::PopScope => {
                    let scope_base = frames[top].scope_base;
                    let num_active = self.scope_starts.len() - scope_base;
                    if num_active <= 1 {
                        return Err(VmError("cannot pop the base call frame".into()));
                    }
                    let start = self.scope_starts.pop().unwrap();
                    self.locals.truncate(start);
                }

                OpCode::Jump => {
                    let offset = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;
                    frames[top].ip += offset;
                }
                OpCode::JumpIfFalse => {
                    let offset = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;
                    match self.pop()? {
                        VmValue::Bool(false) => frames[top].ip += offset,
                        VmValue::Bool(true) => {}
                        other => {
                            return Err(VmError(format!(
                                "if/while condition must be bool, got {other:?}"
                            )));
                        }
                    }
                }
                OpCode::Loop => {
                    let offset = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;
                    frames[top].ip -= offset;
                }

                OpCode::Call => {
                    let arg_count = frames[top].source.chunk().read_u16(frames[top].ip) as usize;
                    frames[top].ip += 2;

                    let base = self.locals.len();
                    self.locals.resize(base + arg_count, VmValue::Null);
                    for i in (0..arg_count).rev() {
                        self.locals[base + i] = self.pop()?;
                    }

                    let func = match self.pop()? {
                        VmValue::Function(f) => f,
                        other => return Err(VmError(format!("cannot call {other:?}"))),
                    };
                    if arg_count != func.arity {
                        return Err(VmError(format!(
                            "{} expects {} args, got {}",
                            func.name, func.arity, arg_count
                        )));
                    }

                    self.scope_starts.push(base);
                    let scope_base = self.scope_starts.len() - 1;
                    frames.push(CallFrame {
                        source: FrameSource::Func(func),
                        ip: 0,
                        scope_base,
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

    fn resolve(&self, frame_scope_base: usize, depth: usize) -> Result<Option<usize>, VmError> {
        let num_active = self.scope_starts.len() - frame_scope_base;
        if depth == num_active {
            return Ok(None);
        }
        self.scope_starts
            .len()
            .checked_sub(1 + depth)
            .filter(|&idx| idx >= frame_scope_base)
            .map(Some)
            .ok_or_else(|| VmError(format!("depth {depth} exceeds active scopes")))
    }

    fn scope_range(&self, scope_idx: usize) -> (usize, usize) {
        let start = self.scope_starts[scope_idx];
        let end = self
            .scope_starts
            .get(scope_idx + 1)
            .copied()
            .unwrap_or(self.locals.len());
        (start, end)
    }

    fn local_slot(&self, scope_idx: usize, slot: usize) -> Option<&VmValue> {
        let (base, end) = self.scope_range(scope_idx);
        if base + slot < end {
            self.locals.get(base + slot)
        } else {
            None
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

    /// Helper function for arth operations
    /// handles +, -, *
    /// currently promotes int to float
    fn binary_numeric(
        &mut self,
        int_op: fn(i64, i64) -> i64,
        float_op: fn(f64, f64) -> f64,
    ) -> Result<(), VmError> {
        let (a, b) = self.pop_two()?;
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
        let (a, b) = self.pop_two()?;
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
        let (a, b) = self.pop_two()?;
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
