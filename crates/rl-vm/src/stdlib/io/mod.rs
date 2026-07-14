//! `std::io` - VM stdlib. Only `print`/`println` for now, since they're
//! variadic and every `VmValue` variant already has a `Display` impl.
//!
//! Unlike the interpreter's version, there's no `output_buffer` so
//! it will write to stdout directly

use crate::native::Module;
use crate::values::VmValue;
use crate::vm_logic::{Vm, VmError};

pub fn module() -> Module {
    Module::new("io")
        .with_raw_function("print", std_print)
        .with_raw_function("println", std_println)
}

fn std_print(_: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    let text = args.iter().map(|v| v.to_string()).collect::<String>();
    print!("{}", text);
    Ok(VmValue::Null)
}

fn std_println(_: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    let text = args.iter().map(|v| v.to_string()).collect::<String>();
    println!("{}", text);
    Ok(VmValue::Null)
}
