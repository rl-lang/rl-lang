//! The VM's native function binding system - [`Module`], [`NativeFn`], and the
//! [`IntoNativeFn`] / [`FromValue`] / [`IntoValue`] trait machinery.
//!
//! This mirrors `rl-interpreter`'s `native.rs`, scoped down to what `VmValue`
//! currently supports: no array/tuple/map/error/ok variants yet (so no
//! `Vec<T>` impls). Errors built in here (arity mismatches, `FromValue`
//! conversions) have no access to a `Span` - this generic machinery runs
//! before the call site is known - so they're built with a dummy span via
//! [`rt_err`], then re-anchored at the actual call site by `Vm::annotate`
//! the moment they rejoin the main dispatch loop (see the `OpCode::Call`
//! handler in `vm_logic.rs`).

use crate::values::{VmNativeFn, VmValue};
use crate::vm_logic::{Vm, VmError};
use rl_utils::errors::{Error, Reason};
use rl_utils::span::Span;
use std::collections::HashMap;
use std::rc::Rc;

/// Builds a runtime error with no span/source context yet; callers further
/// up the stack re-anchor it once the call site is known (see module docs).
fn rt_err(message: impl Into<String>) -> VmError {
    Error::at(Reason::Runtime, message, Span::dummy())
}

/// A heap-allocated native function callable from rl bytecode.
pub type NativeFn = Rc<dyn Fn(&mut Vm, Vec<VmValue>) -> Result<VmValue, VmError>>;

/// A named collection of [`VmNativeFn`]s, optionally containing sub-[`Module`]s.
pub struct Module {
    /// The module name as used in import paths (e.g. `"io"`, `"math"`).
    pub name: String,
    /// Named functions registered in this module.
    pub functions: HashMap<String, Rc<VmNativeFn>>,
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
        let name = name.into();
        self.functions.insert(
            name.clone(),
            Rc::new(VmNativeFn {
                name,
                func: f.into_native(),
            }),
        );
        self
    }

    /// Registers a raw `fn(&mut Vm, Vec<VmValue>) -> Result<VmValue, VmError>` directly,
    /// bypassing the [`IntoNativeFn`] machinery (used for variadic functions like `print`).
    pub fn with_raw_function<F>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: Fn(&mut Vm, Vec<VmValue>) -> Result<VmValue, VmError> + 'static,
    {
        let name = name.into();
        self.functions.insert(
            name.clone(),
            Rc::new(VmNativeFn {
                name,
                func: Rc::new(f),
            }),
        );
        self
    }

    /// Registers a submodule.
    pub fn with_module(mut self, m: Module) -> Self {
        self.submodules.insert(m.name.clone(), m);
        self
    }

    /// Walks the module tree along `path`, returning the [`VmNativeFn`] at the leaf,
    /// or `None` if any segment is missing.
    pub fn resolve(&self, path: &[String]) -> Option<Rc<VmNativeFn>> {
        if path.is_empty() {
            return None;
        }
        let mut module = self;
        for seg in &path[..path.len() - 1] {
            module = module.submodules.get(seg)?;
        }
        module.functions.get(&path[path.len() - 1]).cloned()
    }
}

/// Converts a [`VmValue`] into a typed Rust value, or returns a `VmError`.
/// Implemented for `i64`, `f64`, `String`, `bool`, `char`, and `VmValue` itself.
pub trait FromValue: Sized {
    fn from_value(v: VmValue) -> Result<Self, VmError>;
}

impl FromValue for VmValue {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        Ok(v)
    }
}

impl FromValue for i64 {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Int(i) => Ok(i),
            VmValue::Byte(b) => Ok(b as i64),
            other => Err(rt_err(format!("expected int, got {other:?}"))),
        }
    }
}

impl FromValue for f64 {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Float(f) => Ok(f),
            other => Err(rt_err(format!("expected float, got {other:?}"))),
        }
    }
}

impl FromValue for String {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Str(s) => Ok(s.to_string()),
            other => Err(rt_err(format!("expected string, got {other:?}"))),
        }
    }
}

impl FromValue for bool {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Bool(b) => Ok(b),
            other => Err(rt_err(format!("expected bool, got {other:?}"))),
        }
    }
}

impl FromValue for char {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Char(c) => Ok(c),
            other => Err(rt_err(format!("expected char, got {other:?}"))),
        }
    }
}

/// Converts a typed Rust value into a [`VmValue`].
pub trait IntoValue {
    fn into_value(self) -> VmValue;
}

impl IntoValue for VmValue {
    fn into_value(self) -> VmValue {
        self
    }
}

impl IntoValue for () {
    fn into_value(self) -> VmValue {
        VmValue::Null
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> VmValue {
        VmValue::Int(self)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> VmValue {
        VmValue::Float(self)
    }
}

impl IntoValue for String {
    fn into_value(self) -> VmValue {
        VmValue::Str(Rc::from(self.as_str()))
    }
}

impl IntoValue for bool {
    fn into_value(self) -> VmValue {
        VmValue::Bool(self)
    }
}

impl IntoValue for char {
    fn into_value(self) -> VmValue {
        VmValue::Char(self)
    }
}

/// Wraps a typed Rust function into a [`NativeFn`] with automatic arity checking
/// and argument extraction. Implemented by macro for 0-10 arguments.
pub trait IntoNativeFn<Args> {
    fn into_native(self) -> NativeFn;
}

impl<F, R> IntoNativeFn<()> for F
where
    F: Fn(&mut Vm) -> R + 'static,
    R: IntoValue,
{
    fn into_native(self) -> NativeFn {
        Rc::new(
            move |vm: &mut Vm, args: Vec<VmValue>| -> Result<VmValue, VmError> {
                if !args.is_empty() {
                    return Err(rt_err(format!(
                        "expected 0 argument(s), got {}",
                        args.len()
                    )));
                }
                Ok(self(vm).into_value())
            },
        )
    }
}

/// Marker type for fallible native functions (returning `Result<R, VmError>`).
pub struct Fallible<T>(std::marker::PhantomData<T>);

macro_rules! impl_into_native_fn {
    ($count:literal, $(($ty:ident, $var:ident)),+) => {
        impl<F, R, $($ty),+> IntoNativeFn<($($ty,)+)> for F
        where
            F: Fn(&mut Vm, $($ty),+) -> R + 'static,
            R: IntoValue,
            $($ty: FromValue),+
        {
            fn into_native(self) -> NativeFn {
                Rc::new(move |vm: &mut Vm, args: Vec<VmValue>| -> Result<VmValue, VmError> {
                    if args.len() != $count {
                        return Err(rt_err(format!(
                            "expected {} argument(s), got {}",
                            $count,
                            args.len()
                        )));
                    }
                    let mut iter = args.into_iter();
                    $(let $var = <$ty>::from_value(iter.next().unwrap())?;)+
                    Ok(self(vm, $($var),+).into_value())
                })
            }
        }

        impl<F, R, $($ty),+> IntoNativeFn<(Fallible<($($ty,)+)>,)> for F
        where
            F: Fn(&mut Vm, $($ty),+) -> Result<R, VmError> + 'static,
            R: IntoValue,
            $($ty: FromValue),+
        {
            fn into_native(self) -> NativeFn {
                Rc::new(move |vm: &mut Vm, args: Vec<VmValue>| -> Result<VmValue, VmError> {
                    if args.len() != $count {
                        return Err(rt_err(format!(
                            "expected {} argument(s), got {}",
                            $count,
                            args.len()
                        )));
                    }
                    let mut iter = args.into_iter();
                    $(let $var = <$ty>::from_value(iter.next().unwrap())?;)+
                    Ok(self(vm, $($var),+)?.into_value())
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
