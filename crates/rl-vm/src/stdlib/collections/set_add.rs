use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::{VmMapKey, VmValue},
};

pub fn std_set_add(_: &mut Vm, set: VmValue, value: VmValue) -> VmValue {
    match set {
        VmValue::Set(items) => match VmMapKey::from_value(&value) {
            Some(key) => {
                items.borrow_mut().insert(key);
                vok!(VmValue::Set(items))
            }
            None => verr!(vs!(format!(
                "set_add: cannot add {} to a set",
                value.type_name()
            ))),
        },
        other => verr!(vs!(format!(
            "set_add: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
