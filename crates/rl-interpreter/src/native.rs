//! The native function binding system - [`Module`], [`NativeFn`], and the
//! [`IntoNativeFn`] / [`FromValue`] / [`IntoValue`] trait machinery.
//!
//! # How it works
//!
//! Stdlib functions are plain Rust functions. The [`IntoNativeFn`] trait
//! (implemented by a macro for up to 10 arguments) wraps them into [`NativeFn`]
//! closures that handle arity checking and argument extraction automatically.
//!
//! Two variants exist:
//! - **Infallible** (`IntoNativeFn<(A1, A2, ...)>`) - for functions that return `R: IntoValue`
//! - **Fallible** (`IntoNativeFn<(Fallible<(A1, A2, ...)>,)>`) - for functions returning `Result<R, Error>`
//! - **Spanned** (`IntoNativeFn<(Spanned<(A1, A2, ...)>,)>`) - fallible functions that also receive the call [`Span`]
//!
//! # Example
//!
//! ```rust
//! use rl_interpreter::{evaluator::Evaluator, native::Module};
//!
//! fn my_fn(_: &mut Evaluator, x: i64, y: i64) -> i64 { x + y }
//! Module::new("math").with_function("add", my_fn);
//! ```

use crate::evaluator::Evaluator;
use crate::values::Value;
use rl_ast::statements::TypeAnnotation;
use rl_utils::errors::{Error, Reason};
use rl_utils::span::Span;
use std::collections::HashMap;
use std::sync::Arc;

/// A thread-safe, heap-allocated native function callable from rl.
pub type NativeFn =
    Arc<dyn Fn(&mut Evaluator, Vec<Value>, Span) -> Result<Value, Error> + Send + Sync>;

/// A named collection of [`NativeFn`]s, optionally containing sub-[`Module`]s.
pub struct Module {
    /// The module name as used in import paths (e.g. `"io"`, `"math"`).
    pub name: String,
    /// Named functions registered in this module.
    pub functions: HashMap<String, NativeFn>,
    /// Named submodules (e.g. `math::consts`).
    pub submodules: HashMap<String, Module>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: HashMap::new(),
            submodules: HashMap::new(),
        }
    }

    /// Registers a typed Rust function using the [`IntoNativeFn`] trait.
    pub fn with_function<F, A>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: IntoNativeFn<A>,
    {
        self.functions.insert(name.into(), f.into_native());
        self
    }

    /// Registers a raw `fn(&mut Evaluator, Vec<Value>, Span) -> Result<Value, Error>` directly,
    /// bypassing the [`IntoNativeFn`] machinery (used for variadic functions like `print`).
    pub fn with_raw_function<F>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: Fn(&mut Evaluator, Vec<Value>, Span) -> Result<Value, Error> + Send + Sync + 'static,
    {
        self.functions.insert(name.into(), Arc::new(f));
        self
    }

    /// Registers a submodule.
    pub fn with_module(mut self, m: Module) -> Self {
        self.submodules.insert(m.name.clone(), m);
        self
    }

    /// Walks the module tree along `path`, returning the [`NativeFn`] at the leaf,
    /// or `None` if any segment is missing.
    pub fn resolve(&self, path: &[String]) -> Option<&NativeFn> {
        if path.is_empty() {
            return None;
        }
        let mut module = self;
        for seg in &path[..path.len() - 1] {
            module = module.submodules.get(seg)?;
        }
        module.functions.get(&path[path.len() - 1])
    }
}

/// Extracts the static [`TypeAnnotation`] for a Rust type.
/// Used by [`IntoValue`] for `Vec<T>` to set the array's `items_type`.
pub trait ValueType {
    fn type_annotation() -> TypeAnnotation;
}

impl ValueType for i64 {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Int
    }
}

impl ValueType for f64 {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Float
    }
}

impl ValueType for String {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::String
    }
}

impl ValueType for bool {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Bool
    }
}

impl ValueType for char {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Char
    }
}

impl<T: ValueType> ValueType for Vec<T> {
    fn type_annotation() -> TypeAnnotation {
        TypeAnnotation::Array(Box::new(T::type_annotation()))
    }
}

/// Converts a [`Value`] into a typed Rust value, or returns a runtime error.
/// Implemented for `i64`, `f64`, `String`, `bool`, `char`, `Vec<T>`, and `Value` itself.
pub trait FromValue: Sized {
    fn from_value(v: Value, span: Span) -> Result<Self, Error>;
}

impl FromValue for Value {
    fn from_value(v: Value, _span: Span) -> Result<Self, Error> {
        Ok(v)
    }
}

impl FromValue for i64 {
    fn from_value(v: Value, span: Span) -> Result<Self, Error> {
        match v {
            Value::Integer(i) => Ok(i),
            other => Err(Error::at(
                Reason::Runtime,
                format!("expected integer, got {}", other.type_name()),
                span,
            )),
        }
    }
}

impl FromValue for f64 {
    fn from_value(v: Value, span: Span) -> Result<Self, Error> {
        match v {
            Value::Float(f) => Ok(f),
            other => Err(Error::at(
                Reason::Runtime,
                format!("expected float, got {}", other.type_name()),
                span,
            )),
        }
    }
}

impl FromValue for String {
    fn from_value(v: Value, span: Span) -> Result<Self, Error> {
        match v {
            Value::String(s) => Ok(s),
            other => Err(Error::at(
                Reason::Runtime,
                format!("expected string, got {}", other.type_name()),
                span,
            )),
        }
    }
}

impl FromValue for bool {
    fn from_value(v: Value, span: Span) -> Result<Self, Error> {
        match v {
            Value::Bool(b) => Ok(b),
            other => Err(Error::at(
                Reason::Runtime,
                format!("expected bool, got {}", other.type_name()),
                span,
            )),
        }
    }
}

impl FromValue for char {
    fn from_value(v: Value, span: Span) -> Result<Self, Error> {
        match v {
            Value::Char(c) => Ok(c),
            other => Err(Error::at(
                Reason::Runtime,
                format!("expected char, got {}", other.type_name()),
                span,
            )),
        }
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(v: Value, span: Span) -> Result<Self, Error> {
        match v {
            Value::Values { items, .. } => items
                .into_iter()
                .map(|item| T::from_value(item, span))
                .collect(),
            other => Err(Error::at(
                Reason::Runtime,
                format!("expected array, got {}", other.type_name()),
                span,
            )),
        }
    }
}

/// Converts a typed Rust value into a [`Value`].
/// Implemented for `()` (→ `Null`), `i64`, `f64`, `String`, `bool`, `char`, `Vec<T>`, and `Value`.
pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for () {
    fn into_value(self) -> Value {
        Value::Null
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        Value::Integer(self)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Float(self)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl IntoValue for char {
    fn into_value(self) -> Value {
        Value::Char(self)
    }
}

impl<T: IntoValue + ValueType> IntoValue for Vec<T> {
    fn into_value(self) -> Value {
        Value::Values {
            items_type: T::type_annotation(),
            items: self.into_iter().map(T::into_value).collect(),
        }
    }
}

/// Wraps a typed Rust function into a [`NativeFn`] with automatic arity checking
/// and argument extraction. Implemented by macro for 0–10 arguments.
pub trait IntoNativeFn<Args> {
    fn into_native(self) -> NativeFn;
}

impl<F, R> IntoNativeFn<()> for F
where
    F: Fn(&mut Evaluator) -> R + Send + Sync + 'static,
    R: IntoValue,
{
    fn into_native(self) -> NativeFn {
        Arc::new(
            move |rt: &mut Evaluator, args: Vec<Value>, span: Span| -> Result<Value, Error> {
                if !args.is_empty() {
                    return Err(Error::at(
                        Reason::Runtime,
                        format!("expected 0 argument(s), got {}", args.len()),
                        span,
                    ));
                }
                Ok(self(rt).into_value())
            },
        )
    }
}

/// Marker type for fallible native functions (returning `Result<R, Error>`).
pub struct Fallible<T>(std::marker::PhantomData<T>);

macro_rules! impl_into_native_fn {
    ($count:literal, $(($ty:ident, $var:ident)),+) => {
        impl<F, R, $($ty),+> IntoNativeFn<($($ty,)+)> for F
        where
            F: Fn(&mut Evaluator, $($ty),+) -> R + Send + Sync + 'static,
            R: IntoValue,
            $($ty: FromValue),+
        {
            fn into_native(self) -> NativeFn {
                Arc::new(move |rt: &mut Evaluator, args: Vec<Value>, span: Span| -> Result<Value, Error> {
                    if args.len() != $count {
                        return Err(Error::at(
                            Reason::Runtime,
                            format!("expected {} argument(s), got {}", $count, args.len()),
                            span, ));
                    }
                    let mut iter = args.into_iter();
                    $(let $var = <$ty>::from_value(iter.next().unwrap(), span)?;)+
                    Ok(self(rt, $($var),+).into_value())
                })
            }
        }

        impl<F, R, $($ty),+> IntoNativeFn<(Fallible<($($ty,)+)>,)> for F
        where
            F: Fn(&mut Evaluator, $($ty),+) -> Result<R, Error> + Send + Sync + 'static,
            R: IntoValue,
            $($ty: FromValue),+
        {
            fn into_native(self) -> NativeFn {
                Arc::new(move |rt: &mut Evaluator, args: Vec<Value>, span: Span| -> Result<Value, Error> {
                    if args.len() != $count {
                        return Err(Error::at(
                            Reason::Runtime,
                            format!("expected {} argument(s), got {}", $count, args.len()),
                            span, ));
                    }
                    let mut iter = args.into_iter();
                    $(let $var = <$ty>::from_value(iter.next().unwrap(), span)?;)+
                    Ok(self(rt, $($var),+)?.into_value())
                })
            }
        }
    };
}
impl_into_native_fn!(1, (A1, a1));
impl_into_native_fn!(2, (A1, a1), (A2, a2));
impl_into_native_fn!(3, (A1, a1), (A2, a2), (A3, a3));
impl_into_native_fn!(4, (A1, a1), (A2, a2), (A3, a3), (A4, a4));
impl_into_native_fn!(5, (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5));
impl_into_native_fn!(
    6,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6)
);
impl_into_native_fn!(
    7,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7)
);
impl_into_native_fn!(
    8,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8)
);
impl_into_native_fn!(
    9,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9)
);
impl_into_native_fn!(
    10,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10)
);

/// Marker type for spanned fallible native functions (also receiving the call [`Span`]).
pub struct Spanned<T>(std::marker::PhantomData<T>);

macro_rules! impl_into_native_fn_spanned {
    ($count:literal, $(($ty:ident, $var:ident)),+) => {
        impl<F, R, $($ty),+> IntoNativeFn<(Spanned<($($ty,)+)>,)> for F
        where
            F: Fn(&mut Evaluator, $($ty),+, Span) -> Result<R, Error> + Send + Sync + 'static,
            R: IntoValue,
            $($ty: FromValue),+
        {
            fn into_native(self) -> NativeFn {
                Arc::new(move |rt: &mut Evaluator, args: Vec<Value>, span: Span| -> Result<Value, Error> {
                    if args.len() != $count {
                        return Err(rt.err(
                            format!("expected {} argument(s), got {}", $count, args.len()),
                            span,
                        ));
                    }
                    let mut iter = args.into_iter();
                    $(let $var = <$ty>::from_value(iter.next().unwrap(), span)?;)+
                    Ok(self(rt, $($var),+, span)?.into_value())
                })
            }
        }
    };
}

impl_into_native_fn_spanned!(1, (A1, a1));
impl_into_native_fn_spanned!(2, (A1, a1), (A2, a2));
impl_into_native_fn_spanned!(3, (A1, a1), (A2, a2), (A3, a3));
impl_into_native_fn_spanned!(4, (A1, a1), (A2, a2), (A3, a3), (A4, a4));
impl_into_native_fn_spanned!(5, (A1, a1), (A2, a2), (A3, a3), (A4, a4), (A5, a5));
impl_into_native_fn_spanned!(
    6,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6)
);
impl_into_native_fn_spanned!(
    7,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7)
);
impl_into_native_fn_spanned!(
    8,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8)
);
impl_into_native_fn_spanned!(
    9,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9)
);
impl_into_native_fn_spanned!(
    10,
    (A1, a1),
    (A2, a2),
    (A3, a3),
    (A4, a4),
    (A5, a5),
    (A6, a6),
    (A7, a7),
    (A8, a8),
    (A9, a9),
    (A10, a10)
);
