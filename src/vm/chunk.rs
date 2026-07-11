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

