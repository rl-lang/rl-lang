use std::rc::Rc;

use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_map_keys(_: &mut Vm, map: VmValue) -> VmValue {
    match map {
        VmValue::Map(entries) => {
            let items: Vec<VmValue> = entries
                .borrow()
                .keys()
                .map(|k| k.clone().into_value())
                .collect();
            vok!(VmValue::Arr(Rc::new(items)))
        }
        other => verr!(vs!(format!(
            "map_keys: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
