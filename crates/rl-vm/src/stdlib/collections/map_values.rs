use std::rc::Rc;

use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_map_values(_: &mut Vm, map: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => {
            let items: Vec<VmValue> = entries.borrow().values().cloned().collect();
            vok!(VmValue::Arr(Rc::new(items)))
        }
        other => verr!(vs!(format!(
            "map_values: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
