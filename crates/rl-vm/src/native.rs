use crate::values::{VmNativeFn, VmValue};
use crate::vm_logic::{Vm, VmError};

/// A heap-allocated native function callable from rl bytecode.
pub type NativeFn = Rc<dyn Fn(&mut Vm, Vec<VmValue>) -> Result<VmValue, VmError>>;
