//! Aggregates all documentation entries and exposes them as flat `Vec`s
//! for the renderer functions in [`crate::docs`].
use crate::entry::{ConceptEntry, StdEntry};

mod concepts;
mod stdlib;
mod tutorial;

/// Returns all stdlib module entries in display order.
pub fn stdlib_entries() -> Vec<&'static StdEntry> {
    vec![
        &stdlib::math::MATH,
        &stdlib::math_consts::MATH_CONSTS,
        &stdlib::io::IO,
        &stdlib::array::ARRAY,
        &stdlib::str::STR,
        &stdlib::types::TYPES,
        &stdlib::path::PATH,
        &stdlib::fs::FS,
        &stdlib::random::RANDOM,
        &stdlib::time::TIME,
        &stdlib::process::PROCESS,
        &stdlib::result::RES,
        &stdlib::bitwise::BITWISE,
        &stdlib::term::TERM,
        &stdlib::rl::RL,
        &stdlib::debug::DEBUG,
        &stdlib::net::NET,
        &stdlib::http::HTTP,
        &stdlib::collections::COLLECTIONS,
    ]
}

/// Returns all language concept entries in display order.
pub fn concept_entries() -> Vec<&'static ConceptEntry> {
    vec![
        &concepts::arrays::ARRAYS,
        &concepts::constants::CONSTANTS,
        &concepts::flow_control::CONTROL_FLOW,
        &concepts::flow_control::FOR_LOOPS,
        &concepts::functions::FUNCTIONS,
        &concepts::generals::COMMENTS,
        &concepts::imports::IMPORTS,
        &concepts::lambdas::LAMBDAS,
        &concepts::nulls::NULL,
        &concepts::operators::OPERATORS,
        &concepts::types::TYPES,
        &concepts::variables::VARIABLES,
        &concepts::entries::ENTRY_POINTS,
        &concepts::tooling::TOOLING,
        &concepts::bytes::BYTES,
        &concepts::tuples::TUPLES,
        &concepts::errors::ERROR_TYPE,
        &concepts::casts::CASTING,
        &concepts::r#match::MATCH,
        &concepts::propagate::PROPAGATE,
        &concepts::logical_operators::LOGICAL_OPERATORS,
        &concepts::records::RECORDS,
        &concepts::records::IMPL_BLOCKS,
        &concepts::tags::TAGS,
        &concepts::maps::MAPS,
        &concepts::sets::SETS,
    ]
}

/// Returns all tutorial step entries in display order.
pub fn tutorial_entries() -> Vec<&'static ConceptEntry> {
    vec![
        &tutorial::t1::p1::STEP_FIRST_PROGRAM,
        &tutorial::t1::p2::STEP_VARIABLES,
        &tutorial::t1::p3::STEP_TYPES,
        &tutorial::t1::p4::STEP_IO,
        &tutorial::t1::p5::STEP_RESULTS,
        &tutorial::t1::p6::STEP_OPERATORS_AND_DECISIONS,
        &tutorial::t1::p7::STEP_LOOPS,
        &tutorial::t1::p8::STEP_FOR_LOOPS,
        &tutorial::t1::p9::STEP_FUNCTIONS,
        &tutorial::t1::p10::STEP_ARRAYS,
        &tutorial::t1::p11::STEP_STDLIB,
        &tutorial::t1::p12::STEP_LAMBDAS,
        &tutorial::t1::p13::STEP_NULL,
        &tutorial::t1::p14::STEP_COMPLETE_GAME,
        &tutorial::t2::p_a1::ADV_INTRO,
        &tutorial::t2::p_a2::ADV_MODULES,
        &tutorial::t2::p_a3::ADV_STRING_PARSING,
        &tutorial::t2::p_a4::ADV_CSV_PARSER,
        &tutorial::t2::p_a5::ADV_CSV_IO,
        &tutorial::t2::p_a6::ADV_CSV_QUERY,
        &tutorial::t2::p_a7::ADV_CSV_MUTATION,
        &tutorial::t2::p_b8::ADV_PROGRAM_LOOP,
        &tutorial::t2::p_b9::ADV_TIME,
        &tutorial::t2::p_b10::ADV_COMMANDS_ADD_LIST,
        &tutorial::t2::p_b11::ADV_COMMANDS_DONE_REMOVE,
        &tutorial::t2::p_b12::ADV_FILTERED_VIEWS,
        &tutorial::t2::p_b13::ADV_HELP_POLISH,
        &tutorial::t2::p_b14::ADV_COMPLETE,
    ]
}
