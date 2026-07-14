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

