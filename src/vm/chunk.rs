#[derive(Debug, Clone, PartialEq)]
pub enum VmValue {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    Byte(u8),
    Char(char),
    Str(String),
}

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
}

impl OpCode {
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
            other => panic!("corrupt bytecode: unknown opcode byte {other}"),
        }
    }
}

