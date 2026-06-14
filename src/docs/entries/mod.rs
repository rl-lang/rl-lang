use crate::docs::entry::StdEntry;

mod stdlib;

pub fn stdlib_entries() -> Vec<&'static StdEntry> {
    vec![
        &stdlib::math::MATH,
        &stdlib::constants::MATH_CONSTS,
        &stdlib::display::DISPLAY,
        &stdlib::io::IO,
        &stdlib::arrays::ARRAY,
    ]
}
