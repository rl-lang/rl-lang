//! Type-directed conversion between rl runtime values and concrete Rust types.
//!
//! - [`ValueType`] maps a Rust type to its [`TypeAnnotation`] (runtime-agnostic;
//!   used by `#[native_fn]` to derive a function's checker signature).
//! - [`FromValueR`] extracts a Rust value out of a `R::Value` (used by the
//!   generated wrapper for each typed argument).
//! - [`IntoValueR`] converts a Rust value back into a `R::Value` (used for the
//!   return value).
//!
//! These replace the per-runtime `FromValue`/`IntoValue`/`ValueType` traits in
//! the old `native.rs` files.

use crate::runtime::Runtime;
use rl_ast::statements::TypeAnnotation;

/// Maps a Rust type to the rl [`TypeAnnotation`] it corresponds to. Independent
/// of any runtime - the same annotation is used to build the checker signature.
pub trait ValueType {
    fn type_annotation() -> TypeAnnotation;
}

/// Extracts a concrete Rust value from a runtime value. On a type mismatch the
/// original value is returned (via `Err`) so the caller can report its
/// `type_name` in the error message.
pub trait FromValueR<R: Runtime>: Sized {
    fn from_value(v: R::Value) -> Result<Self, R::Value>;
}

/// Converts a concrete Rust value into a runtime value.
pub trait IntoValueR<R: Runtime> {
    fn into_value(self) -> R::Value;
}

macro_rules! scalar_conv {
    ($ty:ty, $ann:expr, $as:ident, $from:ident) => {
        impl ValueType for $ty {
            fn type_annotation() -> TypeAnnotation {
                $ann
            }
        }
        impl<R: Runtime> FromValueR<R> for $ty {
            fn from_value(v: R::Value) -> Result<Self, R::Value> {
                match R::$as(&v) {
                    Some(x) => Ok(x),
                    None => Err(v),
                }
            }
        }
        impl<R: Runtime> IntoValueR<R> for $ty {
            fn into_value(self) -> R::Value {
                R::$from(self)
            }
        }
    };
}

scalar_conv!(i64, TypeAnnotation::Int, as_i64, from_i64);
scalar_conv!(u64, TypeAnnotation::UInt, as_u64, from_u64);
scalar_conv!(i32, TypeAnnotation::SInt, as_i32, from_i32);
scalar_conv!(u32, TypeAnnotation::SUInt, as_u32, from_u32);
scalar_conv!(i16, TypeAnnotation::BSByte, as_i16, from_i16);
scalar_conv!(u16, TypeAnnotation::BByte, as_u16, from_u16);
scalar_conv!(i8, TypeAnnotation::SByte, as_i8, from_i8);
scalar_conv!(u8, TypeAnnotation::Byte, as_u8, from_u8);
scalar_conv!(f64, TypeAnnotation::Float, as_f64, from_f64);
scalar_conv!(f32, TypeAnnotation::SFloat, as_f32, from_f32);
scalar_conv!(bool, TypeAnnotation::Bool, as_bool, from_bool);
scalar_conv!(char, TypeAnnotation::Char, as_char, from_char);

impl ValueType for String {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::String
    }
}
impl<R: Runtime> FromValueR<R> for String {
    fn from_value(v: R::Value) -> Result<Self, R::Value> {
        match R::as_str(&v) {
            Some(s) => Ok(s.to_owned()),
            None => Err(v),
        }
    }
}
impl<R: Runtime> IntoValueR<R> for String {
    fn into_value(self) -> R::Value {
        R::from_string(self)
    }
}

/// The unit type maps to the rl `null` value (a function returning `()`).
impl ValueType for () {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Null
    }
}
impl<R: Runtime> IntoValueR<R> for () {
    fn into_value(self) -> R::Value {
        R::null()
    }
}

impl<T: ValueType> ValueType for Vec<T> {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Array(Box::new(T::type_annotation()))
    }
}
impl<R: Runtime, T: FromValueR<R>> FromValueR<R> for Vec<T> {
    fn from_value(v: R::Value) -> Result<Self, R::Value> {
        // Borrow and clone per element instead of `to_vec()` up front:
        // a full pre-clone keeps two 85M-element copies alive at once.
        // The flag dance ends the borrow before `Err(v)` moves `v`.
        let out = {
            let (items, _) = match R::as_array(&v) {
                Some(x) => x,
                None => return Err(v),
            };
            let mut out = Vec::with_capacity(items.len());
            let mut ok = true;
            for item in items {
                match T::from_value(item.clone()) {
                    Ok(x) => out.push(x),
                    Err(_) => {
                        ok = false;
                        break;
                    }
                }
            }
            ok.then_some(out)
        };
        match out {
            Some(out) => Ok(out),
            None => Err(v),
        }
    }
}
impl<R: Runtime, T: IntoValueR<R> + ValueType> IntoValueR<R> for Vec<T> {
    fn into_value(self) -> R::Value {
        let elem = T::type_annotation();
        R::array(self.into_iter().map(IntoValueR::into_value).collect(), elem)
    }
}

/// Byte-array extraction with integer-literal tolerance. There is no byte
/// literal syntax, so `[104, 105]` must work for `array[byte]` params:
/// `Byte` elements pass through, `Int` elements in 0-255 coerce, anything
/// else (or out of range) is a loud runtime type error via the wrapper's
/// `R::error`. Mirrors the numeric leniency already in `as_f64`, scoped
/// to whole-array extraction so scalar `as_u8` dispatch (bitwise, `c`)
/// stays strict.
pub struct Bytes(pub Vec<u8>);

impl ValueType for Bytes {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Array(Box::new(TypeAnnotation::Byte))
    }
}
impl<R: Runtime> FromValueR<R> for Bytes {
    fn from_value(v: R::Value) -> Result<Self, R::Value> {
        // Borrow the slice directly: cloning 85M `VmValue`s here (≈2.7GB)
        // OOM-kills large inputs before hashing even starts. The flag
        // dance below ends the borrow before `Err(v)` moves `v`.
        let out = {
            let (items, _) = match R::as_array(&v) {
                Some(x) => x,
                None => return Err(v),
            };
            let mut out = Vec::with_capacity(items.len());
            let mut ok = true;
            for item in items {
                if let Some(b) = R::as_u8(item) {
                    out.push(b);
                } else if let Some(i) = R::as_i64(item) {
                    match u8::try_from(i) {
                        Ok(b) => out.push(b),
                        Err(_) => {
                            ok = false;
                            break;
                        }
                    }
                } else {
                    ok = false;
                    break;
                }
            }
            ok.then_some(out)
        };
        match out {
            Some(out) => Ok(Bytes(out)),
            None => Err(v),
        }
    }
}
impl<R: Runtime> IntoValueR<R> for Bytes {
    fn into_value(self) -> R::Value {
        R::array(
            self.0.into_iter().map(R::from_u8).collect(),
            TypeAnnotation::Byte,
        )
    }
}
