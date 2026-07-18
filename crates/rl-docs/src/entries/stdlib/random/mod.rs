use crate::entry::{FnEntry, StdEntry};

mod rand_bool;
mod rand_bool_weighted;
mod rand_byte;
mod rand_bytes;
mod rand_char;
mod rand_choice;
mod rand_choices;
mod rand_dice;
mod rand_dices;
mod rand_float;
mod rand_float_range;
mod rand_int;
mod rand_int_range;
mod rand_range;
mod rand_range_step;
mod rand_sample;
mod rand_shuffle;
mod rand_string;

pub static RANDOM: StdEntry = StdEntry {
    name: "random",
    description: "functions for random number and value generation",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &rand_int::RAND_INT,
    &rand_int_range::RAND_INT_RANGE,
    &rand_float::RAND_FLOAT,
    &rand_float_range::RAND_FLOAT_RANGE,
    &rand_bool::RAND_BOOL,
    &rand_bool_weighted::RAND_BOOL_WEIGHTED,
    &rand_dice::RAND_DICE,
    &rand_dices::RAND_DICES,
    &rand_range::RAND_RANGE,
    &rand_range_step::RAND_RANGE_STEP,
    &rand_choice::RAND_CHOICE,
    &rand_choices::RAND_CHOICES,
    &rand_sample::RAND_SAMPLE,
    &rand_shuffle::RAND_SHUFFLE,
    &rand_byte::RAND_BYTE,
    &rand_bytes::RAND_BYTES,
    &rand_char::RAND_CHAR,
    &rand_string::RAND_STRING,
];
