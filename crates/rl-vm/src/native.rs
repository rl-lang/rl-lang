use crate::values::{VmNativeFn, VmValue};
use crate::vm_logic::{Vm, VmError};
use std::collections::HashMap;
use std::rc::Rc;

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
            other => Err(VmError(format!("expected int, got {other:?}"))),
        }
    }
}

impl FromValue for f64 {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Float(f) => Ok(f),
            other => Err(VmError(format!("expected float, got {other:?}"))),
        }
    }
}

impl FromValue for String {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Str(s) => Ok(s.to_string()),
            other => Err(VmError(format!("expected string, got {other:?}"))),
        }
    }
}

impl FromValue for bool {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Bool(b) => Ok(b),
            other => Err(VmError(format!("expected bool, got {other:?}"))),
        }
    }
}

impl FromValue for char {
    fn from_value(v: VmValue) -> Result<Self, VmError> {
        match v {
            VmValue::Char(c) => Ok(c),
            other => Err(VmError(format!("expected char, got {other:?}"))),
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

