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

