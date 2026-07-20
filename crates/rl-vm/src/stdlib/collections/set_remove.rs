use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::{VmMapKey, VmValue},
};

pub fn std_set_remove(_: &mut Vm, set: VmValue, value: VmValue) -> VmValue {
    match set {
        VmValue::Set(items) => match VmMapKey::from_value(&value) {
            Some(key) => {
                items.borrow_mut().remove(&key);
                vok!(VmValue::Set(items))
            }
            None => verr!(vs!(format!(
                "set_remove: cannot remove {} from a set",
                value.type_name()
            ))),
        },
        other => verr!(vs!(format!(
            "set_remove: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
