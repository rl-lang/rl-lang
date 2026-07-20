use std::rc::Rc;

use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_set_to_array(_: &mut Vm, set: VmValue) -> VmValue {
    match set {
        VmValue::Set(items) => {
            let items: Vec<VmValue> = items
                .borrow()
                .iter()
                .map(|k| k.clone().into_value())
                .collect();
            vok!(VmValue::Arr(Rc::new(items)))
        }
        other => verr!(vs!(format!(
            "set_to_array: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
