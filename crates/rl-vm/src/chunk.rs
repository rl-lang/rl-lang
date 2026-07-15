use crate::VmValue;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    /// literal
    Const = 0,
    /// +
    Add = 1,
    /// -
    Sub = 2,
    /// *
    Mul = 3,
    /// /
    Div = 4,
    /// unary -
    Negate = 5,
    /// !
    Not = 6,
    /// ==
    Eq = 7,
    /// !=
    NotEq = 8,
    /// <
    Less = 9,
    /// <=
    LessEq = 10,
    /// >
    Greater = 11,
    /// >=
    GreaterEq = 12,
    /// read value
    GetLocal = 13,
    /// reassign value
    SetLocal = 14,
    /// dec / CONST
    DefineLocal = 15,
    /// discard
    Pop = 16,
    /// end of program
    Return = 17,
    /// enter a new local-variable frame
    PushScope = 18,
    /// discard the innermost local-variable frame
    PopScope = 19,
    /// unconditional forward jump by u16 offset
    Jump = 20,
    /// pop a bool; jump forward by u16 offset if false
    JumpIfFalse = 21,
    /// unconditional backward jump by u16 offset (loop back-edge)
    Loop = 22,
    /// call function
    Call = 23,
    GetGlobal = 24,
    SetGlobal = 25,
    /// Ok(...)
    Ok = 26,
    /// Err(...)
    Err = 27,
    /// ?
    Propagate = 28,
    /// Error(...)
    Error = 29,
}

impl OpCode {
    /// # Safety
    /// `byte` must be valid discriminant (0..28)
    /// and that's only true for bytecode emitted by this compiler
    #[inline(always)]
    pub fn from_u8_unchecked(byte: u8) -> Self {
        debug_assert!(
            byte <= OpCode::Error as u8,
            "corrupt bytecode: opcode {byte}"
        );
        unsafe { std::mem::transmute::<u8, OpCode>(byte) }
    }

    // checked variant of the unsafe function above
    // might have a use for it later
    pub fn from_u8(byte: u8) -> Self {
        match byte {
            0 => OpCode::Const,
            1 => OpCode::Add,
            2 => OpCode::Sub,
            3 => OpCode::Mul,
            4 => OpCode::Div,
            5 => OpCode::Negate,
            6 => OpCode::Not,
            7 => OpCode::Eq,
            8 => OpCode::NotEq,
            9 => OpCode::Less,
            10 => OpCode::LessEq,
            11 => OpCode::Greater,
            12 => OpCode::GreaterEq,
            13 => OpCode::GetLocal,
            14 => OpCode::SetLocal,
            15 => OpCode::DefineLocal,
            16 => OpCode::Pop,
            17 => OpCode::Return,
            18 => OpCode::PushScope,
            19 => OpCode::PopScope,
            20 => OpCode::Jump,
            21 => OpCode::JumpIfFalse,
            22 => OpCode::Loop,
            23 => OpCode::Call,
            24 => OpCode::GetGlobal,
            25 => OpCode::SetGlobal,
            26 => OpCode::Ok,
            27 => OpCode::Err,
            28 => OpCode::Propagate,
            29 => OpCode::Error,
            other => panic!("corrupt bytecode: unknown opcode byte {other}"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Chunk {
    /// instructions
    pub code: Vec<u8>,
    /// value/literals storage
    pub constants: Vec<VmValue>,
    /// instruction location in source
    pub lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    /// Helper function for appending one OpCode to Chunk.code
    /// also appends the line of OpCode of source to lines
    pub fn write_op(&mut self, op: OpCode, line: u32) {
        self.code.push(op as u8);
        self.lines.push(line);
    }

    /// Helpder function same as write_op() but for 2 byte operand
    pub fn write_u16(&mut self, val: u16, line: u32) {
        let bytes = val.to_le_bytes();
        self.code.push(bytes[0]);
        self.code.push(bytes[1]);
        self.lines.push(line);
        self.lines.push(line);
    }

    /// Helper function appends the given constant/literal to Chunk.constants
    /// search for same value in constants vector and return its index
    /// if no similar values found it will append new value and return its index
    pub fn add_constant(&mut self, value: VmValue) -> u16 {
        if let Some(pos) = self.constants.iter().position(|c| *c == value) {
            return pos as u16;
        }
        self.constants.push(value);
        (self.constants.len() - 1) as u16
    }

    /// Inverse function of write_u16
    /// returns 2 byte operand
    pub fn read_u16(&self, offset: usize) -> u16 {
        u16::from_le_bytes([self.code[offset], self.code[offset + 1]])
    }
}
