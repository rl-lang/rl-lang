use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_set_len(_: &mut Vm, set: VmValue) -> VmValue {
    match set {
        VmValue::Set(items) => vok!(VmValue::Int(items.borrow().len() as i64)),
        other => verr!(vs!(format!(
            "set_len: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
