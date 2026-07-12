use crate::{
    stdlib::common::{extract_int, extract_number},
    values::Value,
};

pub fn extract_byte(v: Value, name: &str) -> Result<u8, String> {
    let byte = extract_number(v, name)?;
    Ok(byte as u8)
}

pub fn extract_u16(v: Value, name: &str) -> Result<u16, String> {
    let v = extract_int(v, name)?;
    match v {
        v if v >= 0 => Ok(v as u16),
        v => Err(format!("{} must be >= 0, got {}", name, v)),
    }
}
