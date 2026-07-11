#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Const = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4,
    Negate = 5,
    Not = 6,
    Eq = 7,
    NotEq = 8,
    Less = 9,
    LessEq = 10,
    Greater = 11,
    GreaterEq = 12,
    GetLocal = 13,
    SetLocal = 14,
    DefineLocal = 15,
    Pop = 16,
    Return = 17,
}

