use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_set_is_empty(_: &mut Vm, set: VmValue) -> VmValue {
    match set {
        VmValue::Set(items) => vok!(VmValue::Bool(items.borrow().is_empty())),
        other => verr!(vs!(format!(
            "set_is_empty: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
