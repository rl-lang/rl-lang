use crate::docs::entry::{ConceptEntry, StdEntry};

mod concepts;
mod stdlib;

pub fn stdlib_entries() -> Vec<&'static StdEntry> {
    vec![
        &stdlib::math::MATH,
        &stdlib::constants::MATH_CONSTS,
        &stdlib::display::DISPLAY,
        &stdlib::io::IO,
        &stdlib::arrays::ARRAY,
        &stdlib::str::STR,
        &stdlib::types::TYPES,
        &stdlib::path::PATH,
        &stdlib::fs::FS,
    ]
}

pub fn concept_entries() -> Vec<&'static ConceptEntry> {
    vec![
        &concepts::arrays::ARRAYS,
        &concepts::constants::CONSTANTS,
        &concepts::flow::CONTROL_FLOW,
        &concepts::flow::FOR_LOOPS,
        &concepts::functions::FUNCTIONS,
        &concepts::general::COMMENTS,
        &concepts::imports::IMPORTS,
        &concepts::lambdas::LAMBDAS,
        &concepts::null::NULL,
        &concepts::operators::OPERATORS,
        &concepts::types::TYPES,
        &concepts::variables::VARIABLES,
    ]
}
