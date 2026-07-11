use crate::vm::chunk::{Chunk, OpCode, VmValue};

#[derive(Debug)]
pub struct VmError(pub String);

pub struct Vm {
    stack: Vec<VmValue>,
    locals: Vec<Vec<VmValue>>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            locals: vec![Vec::new()],
        }
    }

    /// Vm entry function
    pub fn run(&mut self, chunk: &Chunk) -> Result<Option<VmValue>, VmError> {
        let mut ip = 0usize; // intruction pointer
        while ip < chunk.code.len() {
            let op = OpCode::from_u8(chunk.code[ip]);
            ip += 1; // advance the pointer
            match op {
                // reads two bytes
                // advances the pointer twice
                // and pushes to stack
                OpCode::Const => {
                    let idx = chunk.read_u16(ip) as usize;
                    ip += 2;
                    self.stack.push(chunk.constants[idx].clone());
                }
                // pops two values
                // applying the operator on operands
                // pushes the result into stack
                OpCode::Add => self.binary_numeric(|a, b| a + b, |a, b| a + b)?,
                OpCode::Sub => self.binary_numeric(|a, b| a - b, |a, b| a - b)?,
                OpCode::Mul => self.binary_numeric(|a, b| a * b, |a, b| a * b)?,
                OpCode::Div => self.binary_div()?,
                // unary operations
                // if math negation OpCode -> pop once then negate
                // handles int and float otherwise error
                // if logical negation OpCode -> pop once then push bool onto stack
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
                // --- binary operations ---
                // pops two value and compare them either equal or not equal
                // then push bool of reuslt into stack
                // or error
                OpCode::Eq => {
                    let (a, b) = self.pop_two()?;
                    self.stack.push(VmValue::Bool(a == b));
                }
                OpCode::NotEq => {
                    let (a, b) = self.pop_two()?;
                    self.stack.push(VmValue::Bool(a != b));
                }
                // pops two values
                // then applies the comparsion ordering
                // then push bool of result into stack
                // or error
                OpCode::Less => self.binary_cmp(|o| o.is_lt())?,
                OpCode::LessEq => self.binary_cmp(|o| o.is_le())?,
                OpCode::Greater => self.binary_cmp(|o| o.is_gt())?,
                OpCode::GreaterEq => self.binary_cmp(|o| o.is_ge())?,
                // reads two operands depth then slot
                // errors on:
                // - more than one frame/depth
                // - out of range lookup
                // if no errors push the value to stack
                OpCode::GetLocal => {
                    let depth = chunk.read_u16(ip);
                    ip += 2;
                    let slot = chunk.read_u16(ip) as usize;
                    ip += 2;
                    if depth != 0 {
                        return Err(VmError(
                            "vm v0 supports only depth 0 (no closures/calls yet)".into(),
                        ));
                    }
                    let val = self
                        .locals
                        .get(slot)
                        .ok_or_else(|| VmError(format!("read of undefined local slot {slot}")))?
                        .clone();
                    self.stack.push(val);
                }
                // reads two operands depth then slot
                // then look/peek at value without popping
                // errors on:
                // - more than one frame/depth
                // - out of range lookup
                OpCode::SetLocal => {
                    let depth = chunk.read_u16(ip);
                    ip += 2;
                    let slot = chunk.read_u16(ip) as usize;
                    ip += 2;
                    if depth != 0 {
                        return Err(VmError(
                            "vm v0 supports only depth 0 (no closures/calls yet)".into(),
                        ));
                    }
                    let val = self
                        .stack
                        .last()
                        .cloned()
                        .ok_or_else(|| VmError("stack underflow on assignment".into()))?;
                    if slot >= self.locals.len() {
                        return Err(VmError(format!(
                            "assignment to undefined local slot {slot}"
                        )));
                    }
                    self.locals[slot] = val;
                }
                // reads slot only (no other depth than 0)
                // pops the value then grow the locals vector with Null value
                // replace Null value with the actual popped value
                OpCode::DefineLocal => {
                    let slot = chunk.read_u16(ip) as usize;
                    ip += 2;
                    let val = self.pop()?;
                    if slot >= self.locals.len() {
                        self.locals.resize(slot + 1, VmValue::Null);
                    }
                    self.locals[slot] = val;
                }
                // pops and discard one value
                OpCode::Pop => {
                    self.pop()?;
                }
                // halts the program
                OpCode::Return => return Ok(self.stack.pop()),
            }
        }
        Ok(self.stack.pop())
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
