use std::{
    fmt::{Display, Formatter, Result},
};

const MAGIC: &[u8; 4] = b"RLC1";

#[derive(Debug)]
pub struct BytecodeError(pub String);

impl Display for BytecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}
