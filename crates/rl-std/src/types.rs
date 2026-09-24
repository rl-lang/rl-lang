//! `std::types` - type inspection and conversion functions.
//!
//! `is_*` functions check the runtime type of a value without conversion.
//! `to_int`/`to_byte` accept hex strings prefixed with `0x`/`0X`.
//!
//! Ported once from the former per-runtime `stdlib/types/*.rs` copies. The
//! conversion functions are value-polymorphic: they take a raw `R::Value`,
//! inspect it with `R::as_*`, and return a language `result[T]` value built
//! from `R::ok`/`R::err`. Their explicit `sig(...)` overloads mirror
//! `rl-commons/src/stdlib_signatures/types.rs`.
//!
//! The scalar extractors are type-precise: `R::as_i64` matches only `int`,
//! `R::as_u8` only `byte`, `R::as_f64` only `float`, etc. (never a widened /
//! sibling numeric type). This mirrors the old `match VmValue::Int(v)` arms
//! exactly, so the conversion bodies below need no extra `type_name` guards.

use rl_ast::statements::HandleKind;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- type predicates (`_ -> bool`) ----------------------------------------

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_bool<R: Runtime>(value: R::Value) -> bool {
    R::as_bool(&value).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_null<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "null"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_char<R: Runtime>(value: R::Value) -> bool {
    R::as_char(&value).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_int<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "int"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_float<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "float"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_string<R: Runtime>(value: R::Value) -> bool {
    R::as_str(&value).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_byte<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "byte"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_error<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "error"
}

// ---- error helpers --------------------------------------------------------

// Untyped, like the old `error_unwrap`: takes any value, returns a raw value
// (`ok(inner)` for an error, `err(msg)` otherwise).
//
// NOTE: this is the ONE function that cannot be expressed with the current
// `Runtime` trait. The old body is `Value::Error(inner) => vok!(*inner)`, i.e.
// it must extract the value *wrapped inside* an `error(...)`. The trait exposes
// `error_value(v)` (the constructor) but no inverse accessor, so this relies on
// a new `R::as_error_inner(&value) -> Option<R::Value>` method being added to
// `Runtime` (and both runtime impls). See the migration report. Every other
// function in this module ports cleanly against the trait as it stands.
#[native_fn(module = "types", untyped)]
pub fn error_unwrap<R: Runtime>(value: R::Value) -> R::Value {
    match R::as_error_inner(&value) {
        Some(inner) => R::ok(inner),
        None => R::err(R::from_string(format!(
            "error_unwrap: expected error, got {}",
            R::type_name(&value)
        ))),
    }
}

// ---- radix / base conversions ---------------------------------------------

// `to_bin(v) -> result[string]` - byte/int/bool/char/string only.
#[native_fn(module = "types",
    sig(byte -> result[string]),
    sig(int -> result[string]),
    sig(bool -> result[string]),
    sig(char -> result[string]),
    sig(string -> result[string]))]
pub fn to_bin<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(v) = R::as_u8(&value) {
        format!("{:b}", v)
    } else if let Some(v) = R::as_i64(&value) {
        format!("{:b}", v)
    } else if let Some(v) = R::as_bool(&value) {
        if v { "1".to_string() } else { "0".to_string() }
    } else if let Some(v) = R::as_char(&value) {
        format!("{:b}", v as u32)
    } else if let Some(s) = R::as_str(&value) {
        s.bytes().map(|b| format!("{:b}", b)).collect::<String>()
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as binary",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_string(result))
}

// `to_hex(v) -> result[string]` - int/byte/char/string.
#[native_fn(module = "types",
    sig(int -> result[string]),
    sig(byte -> result[string]),
    sig(char -> result[string]),
    sig(string -> result[string]))]
pub fn to_hex<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(v) = R::as_i64(&value) {
        format!("{:x}", v)
    } else if let Some(v) = R::as_u8(&value) {
        format!("{:x}", v)
    } else if let Some(v) = R::as_char(&value) {
        format!("{:x}", v as u32)
    } else if let Some(s) = R::as_str(&value) {
        s.bytes().map(|b| format!("{:x}", b)).collect::<String>()
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as hexadecimal",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_string(result))
}

// `to_oct(v) -> result[string]` - int/byte/char/string.
#[native_fn(module = "types",
    sig(int -> result[string]),
    sig(byte -> result[string]),
    sig(char -> result[string]),
    sig(string -> result[string]))]
pub fn to_oct<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(v) = R::as_i64(&value) {
        format!("{:o}", v)
    } else if let Some(v) = R::as_u8(&value) {
        format!("{:o}", v)
    } else if let Some(v) = R::as_char(&value) {
        format!("{:o}", v as u32)
    } else if let Some(s) = R::as_str(&value) {
        s.bytes().map(|b| format!("{:o}", b)).collect::<String>()
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as octal",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_string(result))
}

// ---- scalar conversions ---------------------------------------------------

// `to_bool(v) -> result[bool]` - bool/int/byte/float/null/string.
#[native_fn(module = "types",
    sig(bool -> result[bool]),
    sig(int -> result[bool]),
    sig(byte -> result[bool]),
    sig(float -> result[bool]),
    sig(null -> result[bool]),
    sig(string -> result[bool]))]
pub fn to_bool<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(b) = R::as_bool(&value) {
        b
    } else if let Some(i) = R::as_i64(&value) {
        i != 0
    } else if let Some(i) = R::as_u8(&value) {
        i != 0
    } else if let Some(f) = R::as_f64(&value) {
        f != 0.0
    } else if R::type_name(&value) == "null" {
        false
    } else if let Some(s) = R::as_str(&value) {
        !matches!(s.trim(), "false" | "0" | "")
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as bool",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_bool(result))
}

// `to_byte(v) -> result[byte]` - int/byte/float/bool/char/string.
#[native_fn(module = "types",
    sig(int -> result[byte]),
    sig(byte -> result[byte]),
    sig(float -> result[byte]),
    sig(bool -> result[byte]),
    sig(char -> result[byte]),
    sig(string -> result[byte]))]
pub fn to_byte<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(v) = R::as_i64(&value) {
        v as u8
    } else if let Some(v) = R::as_u8(&value) {
        v
    } else if let Some(v) = R::as_f64(&value) {
        v as u8
    } else if let Some(v) = R::as_bool(&value) {
        if v { 1u8 } else { 0u8 }
    } else if let Some(v) = R::as_char(&value) {
        v as u8
    } else if let Some(s) = R::as_str(&value) {
        let s = s.trim();
        if s.starts_with("0x") || s.starts_with("0X") {
            match u8::from_str_radix(&s[2..], 16) {
                Ok(i) => i,
                Err(_) => {
                    return R::err(R::from_string(format!("cannot parse \"{}\" as byte", s)));
                }
            }
        } else {
            match s.parse::<u8>() {
                Ok(i) => i,
                Err(_) => {
                    return R::err(R::from_string(format!("cannot parse \"{}\" as byte", s)));
                }
            }
        }
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as byte",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_u8(result))
}

// `to_char(v) -> result[char]` - char/int/byte/string (one-char string only).
#[native_fn(module = "types",
    sig(char -> result[char]),
    sig(int -> result[char]),
    sig(byte -> result[char]),
    sig(string -> result[char]))]
pub fn to_char<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(c) = R::as_char(&value) {
        c
    } else if let Some(i) = R::as_i64(&value) {
        match char::from_u32(i as u32) {
            Some(c) => c,
            None => {
                return R::err(R::from_string(format!(
                    "{} is not a valid unicode codepoint",
                    i
                )));
            }
        }
    } else if let Some(i) = R::as_u8(&value) {
        match char::from_u32(i as u32) {
            Some(c) => c,
            None => {
                return R::err(R::from_string(format!(
                    "{} is not a valid unicode codepoint",
                    i
                )));
            }
        }
    } else if let Some(s) = R::as_str(&value) {
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => c,
            _ => {
                return R::err(R::from_string(
                    "string must be exactly one character".to_string(),
                ));
            }
        }
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as character",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_char(result))
}

// `to_float(v) -> result[float]` - float/int/byte/bool/string.
#[native_fn(module = "types",
    sig(float -> result[float]),
    sig(int -> result[float]),
    sig(byte -> result[float]),
    sig(bool -> result[float]),
    sig(string -> result[float]))]
pub fn to_float<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(f) = R::as_f64(&value) {
        f
    } else if let Some(i) = R::as_i64(&value) {
        i as f64
    } else if let Some(i) = R::as_u8(&value) {
        i as f64
    } else if let Some(b) = R::as_bool(&value) {
        if b { 1.0 } else { 0.0 }
    } else if let Some(s) = R::as_str(&value) {
        match s.trim().parse::<f64>() {
            Ok(f) => f,
            Err(_) => {
                return R::err(R::from_string(format!("cannot parse \"{}\" as float", s)));
            }
        }
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as float",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_f64(result))
}

// `to_int(v) -> result[int]` - int/byte/float/bool/char/string
// (hex strings prefixed `0x`/`0X` accepted).
#[native_fn(module = "types",
    sig(int -> result[int]),
    sig(byte -> result[int]),
    sig(float -> result[int]),
    sig(bool -> result[int]),
    sig(char -> result[int]),
    sig(string -> result[int]))]
pub fn to_int<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(v) = R::as_i64(&value) {
        v
    } else if let Some(v) = R::as_u8(&value) {
        v as i64
    } else if let Some(v) = R::as_f64(&value) {
        v as i64
    } else if let Some(v) = R::as_bool(&value) {
        if v { 1 } else { 0 }
    } else if let Some(v) = R::as_char(&value) {
        v as i64
    } else if let Some(s) = R::as_str(&value) {
        let s = s.trim();
        if s.starts_with("0x") || s.starts_with("0X") {
            match i64::from_str_radix(&s[2..], 16) {
                Ok(i) => i,
                Err(_) => {
                    return R::err(R::from_string(format!("cannot parse \"{}\" as int", s)));
                }
            }
        } else {
            match s.parse::<i64>() {
                Ok(i) => i,
                Err(_) => {
                    return R::err(R::from_string(format!("cannot parse \"{}\" as int", s)));
                }
            }
        }
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as int",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_i64(result))
}

// `to_string(v) -> result[string]` - int/byte/float/bool/char/string.
#[native_fn(module = "types",
    sig(int -> result[string]),
    sig(byte -> result[string]),
    sig(float -> result[string]),
    sig(bool -> result[string]),
    sig(char -> result[string]),
    sig(string -> result[string]))]
pub fn to_string<R: Runtime>(value: R::Value) -> R::Value {
    let result = if let Some(v) = R::as_i64(&value) {
        format!("{}", v)
    } else if let Some(v) = R::as_u8(&value) {
        format!("{}", v)
    } else if let Some(v) = R::as_f64(&value) {
        format!("{}", v)
    } else if let Some(v) = R::as_bool(&value) {
        format!("{}", v)
    } else if let Some(v) = R::as_char(&value) {
        v.to_string()
    } else if let Some(s) = R::as_str(&value) {
        s.to_string()
    } else {
        return R::err(R::from_string(format!(
            "cannot parse \"{}\" as string",
            R::type_name(&value)
        )));
    };
    R::ok(R::from_string(result))
}

// ---- compound type predicates ----------------------------------------------

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_array<R: Runtime>(value: R::Value) -> bool {
    R::as_array(&value).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_map<R: Runtime>(value: R::Value) -> bool {
    R::as_map(&value).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_set<R: Runtime>(value: R::Value) -> bool {
    R::as_set(&value).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_tuple<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "tuple"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_function<R: Runtime>(value: R::Value) -> bool {
    R::is_callable(&value)
}

// ---- numeric type predicates ----------------------------------------------

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_uint<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "uint"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_sbyte<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "sbyte"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_bsbyte<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "big sbyte"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_bbyte<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "big byte"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_sint<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "small int"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_suint<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "small uint"
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_sfloat<R: Runtime>(value: R::Value) -> bool {
    R::type_name(&value) == "small float"
}

// ---- handle type predicates -----------------------------------------------

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_c_handle<R: Runtime>(value: R::Value) -> bool {
    R::as_handle(&value, HandleKind::C).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_net_handle<R: Runtime>(value: R::Value) -> bool {
    R::as_handle(&value, HandleKind::Net).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_http_handle<R: Runtime>(value: R::Value) -> bool {
    R::as_handle(&value, HandleKind::Http).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_audio_handle<R: Runtime>(value: R::Value) -> bool {
    R::as_handle(&value, HandleKind::Audio).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_gui_handle<R: Runtime>(value: R::Value) -> bool {
    R::as_handle(&value, HandleKind::Gui).is_some()
}

#[native_fn(module = "types", sig(_ -> bool))]
pub fn is_file_handle<R: Runtime>(value: R::Value) -> bool {
    R::as_handle(&value, HandleKind::File).is_some()
}

rl_std_core::native_module!("types";
    funcs: [
        is_bool, is_null, is_char, is_int, is_float, is_string, is_byte, is_error,
        is_uint, is_sbyte, is_bsbyte, is_bbyte, is_sint, is_suint, is_sfloat,
        is_array, is_map, is_set, is_tuple, is_function,
        is_c_handle, is_net_handle, is_http_handle, is_audio_handle, is_gui_handle, is_file_handle,
        error_unwrap,
        to_bin, to_hex, to_oct,
        to_bool, to_byte, to_char, to_float, to_int, to_string,
    ],
);
