//! Top-level `std::len` - kept as a per-runtime function because its return
//! convention diverges between the backends: the VM returns a `result[int]`
//! (`len(x)?`), while the interpreter returns a bare `int`. The shared `rl-std`
//! root therefore only declares the name; each runtime registers its own.

use crate::{
    Vm,
    stdlib::macros::{verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_len(_: &mut Vm, value: VmValue) -> VmValue {
    match value {
        VmValue::Arr(items) => vok!(vi!(items.len() as i64)),
        VmValue::Tuple(items) => vok!(vi!(items.len() as i64)),
        VmValue::Str(s) => vok!(vi!(s.len() as i64)),
        other => verr!(vs!(format!(
            "len: expects an array, tuple, or string, found {}",
            other.type_name()
        ))),
    }
}
