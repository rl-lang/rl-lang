use crate::docs::entry::{ConceptEntry, StdEntry};

mod concepts;
mod stdlib;
mod tutorial;

pub fn stdlib_entries() -> Vec<&'static StdEntry> {
    vec![
        &stdlib::math::MATH,
        &stdlib::constants::MATH_CONSTS,
        &stdlib::io::IO,
        &stdlib::arrays::ARRAY,
        &stdlib::str::STR,
        &stdlib::types::TYPES,
        &stdlib::path::PATH,
        &stdlib::fs::FS,
        &stdlib::random::RANDOM,
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
        &concepts::entry::ENTRY_POINTS,
        &concepts::tooling::TOOLING,
        &concepts::byte::BYTES,
    ]
}

pub fn tutorial_entries() -> Vec<&'static ConceptEntry> {
    vec![
        &tutorial::STEP_FIRST_PROGRAM,
        &tutorial::STEP_VARIABLES,
        &tutorial::STEP_TYPES,
        &tutorial::STEP_IO,
        &tutorial::STEP_OPERATORS_AND_DECISIONS,
        &tutorial::STEP_LOOPS,
        &tutorial::STEP_FOR_LOOPS,
        &tutorial::STEP_FUNCTIONS,
        &tutorial::STEP_ARRAYS,
        &tutorial::STEP_STDLIB,
        &tutorial::STEP_LAMBDAS,
        &tutorial::STEP_NULL,
        &tutorial::STEP_COMPLETE_GAME,
    ]
}
