use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::{VmMapKey, VmValue},
};

pub fn std_set_contains(_: &mut Vm, set: VmValue, value: VmValue) -> VmValue {
    match set {
        VmValue::Set(items) => match VmMapKey::from_value(&value) {
            Some(key) => vok!(VmValue::Bool(items.borrow().contains(&key))),
            None => vok!(VmValue::Bool(false)),
        },
        other => verr!(vs!(format!(
            "set_contains: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
