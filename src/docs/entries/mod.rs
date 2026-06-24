//! Aggregates all documentation entries and exposes them as flat `Vec`s
//! for the renderer functions in [`crate::docs`].
use crate::docs::entry::{ConceptEntry, StdEntry};

mod concepts;
mod stdlib;
mod tutorial;

/// Returns all stdlib module entries in display order.
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
        &stdlib::time::TIME,
        &stdlib::process::PROCESS,
    ]
}

/// Returns all language concept entries in display order.
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
        &concepts::tuples::TUPLES,
        &concepts::errors::ERROR_TYPE,
    ]
}

/// Returns all tutorial step entries in display order.
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
        &tutorial::ADV_INTRO,
        &tutorial::ADV_MODULES,
        &tutorial::ADV_STRING_PARSING,
        &tutorial::ADV_CSV_PARSER,
        &tutorial::ADV_CSV_IO,
        &tutorial::ADV_CSV_QUERY,
        &tutorial::ADV_CSV_MUTATION,
        &tutorial::ADV_PROGRAM_LOOP,
        &tutorial::ADV_TIME,
        &tutorial::ADV_COMMANDS_ADD_LIST,
        &tutorial::ADV_COMMANDS_DONE_REMOVE,
        &tutorial::ADV_FILTERED_VIEWS,
        &tutorial::ADV_HELP_POLISH,
        &tutorial::ADV_COMPLETE,
    ]
}
