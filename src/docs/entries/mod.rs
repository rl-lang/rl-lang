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
        &stdlib::result::RES,
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
        &concepts::casting::CASTING,
    ]
}

/// Returns all tutorial step entries in display order.
pub fn tutorial_entries() -> Vec<&'static ConceptEntry> {
    vec![
        &tutorial::t1::p1::STEP_FIRST_PROGRAM,
        &tutorial::t1::p2::STEP_VARIABLES,
        &tutorial::t1::p3::STEP_TYPES,
        &tutorial::t1::p4::STEP_IO,
        &tutorial::t1::p5::STEP_OPERATORS_AND_DECISIONS,
        &tutorial::t1::p6::STEP_LOOPS,
        &tutorial::t1::p7::STEP_FOR_LOOPS,
        &tutorial::t1::p8::STEP_FUNCTIONS,
        &tutorial::t1::p9::STEP_ARRAYS,
        &tutorial::t1::p10::STEP_STDLIB,
        &tutorial::t1::p11::STEP_LAMBDAS,
        &tutorial::t1::p12::STEP_NULL,
        &tutorial::t1::p13::STEP_COMPLETE_GAME,
        &tutorial::t2::ADV_INTRO,
        &tutorial::t2::ADV_MODULES,
        &tutorial::t2::ADV_STRING_PARSING,
        &tutorial::t2::ADV_CSV_PARSER,
        &tutorial::t2::ADV_CSV_IO,
        &tutorial::t2::ADV_CSV_QUERY,
        &tutorial::t2::ADV_CSV_MUTATION,
        &tutorial::t2::ADV_PROGRAM_LOOP,
        &tutorial::t2::ADV_TIME,
        &tutorial::t2::ADV_COMMANDS_ADD_LIST,
        &tutorial::t2::ADV_COMMANDS_DONE_REMOVE,
        &tutorial::t2::ADV_FILTERED_VIEWS,
        &tutorial::t2::ADV_HELP_POLISH,
        &tutorial::t2::ADV_COMPLETE,
    ]
}
