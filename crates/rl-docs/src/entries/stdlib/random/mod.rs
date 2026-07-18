use crate::entry::{FnEntry, StdEntry};

pub static RANDOM: StdEntry = StdEntry {
    name: "random",
    description: "functions for random number and value generation",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &RAND_INT,
    &RAND_INT_RANGE,
    &RAND_FLOAT,
    &RAND_FLOAT_RANGE,
    &RAND_BOOL,
    &RAND_BOOL_WEIGHTED,
    &RAND_DICE,
    &RAND_DICES,
    &RAND_RANGE,
    &RAND_RANGE_STEP,
    &RAND_CHOICE,
    &RAND_CHOICES,
    &RAND_SAMPLE,
    &RAND_SHUFFLE,
    &RAND_BYTE,
    &RAND_BYTES,
    &RAND_CHAR,
    &RAND_STRING,
];

static RAND_INT: FnEntry = FnEntry {
    signature: "rand_int()",
    description: "returns a random int across the full int range",
    example: "get std::random::rand_int\n\nrand_int() // e.g. 4650267523947147985",
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["rand_int_range"],
    since: Some("v0.1.5"),
};

static RAND_INT_RANGE: FnEntry = FnEntry {
    signature: "rand_int_range(min, max)",
    description: "returns a random int between min and max (inclusive)",
    example: "get std::random::rand_int_range\n\nrand_int_range(1, 6)?",
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if `min` is greater than or equal to `max`"),
    see_also: &["rand_int", "rand_range"],
    since: Some("v0.1.5"),
};

static RAND_FLOAT: FnEntry = FnEntry {
    signature: "rand_float()",
    description: "returns a random float in [0.0, 1.0)",
    example: "get std::random::rand_float\n\nrand_float() // e.g. 0.3528",
    expected_output: None,
    returns: "float",
    errors: None,
    see_also: &["rand_float_range"],
    since: Some("v0.1.5"),
};

static RAND_FLOAT_RANGE: FnEntry = FnEntry {
    signature: "rand_float_range(min, max)",
    description: "returns a random float in [min, max)",
    example: "get std::random::rand_float_range\n\nrand_float_range(1.0, 2.0)?",
    expected_output: None,
    returns: "result[float]",
    errors: Some("Will return error if `min` is greater than or equal to `max`"),
    see_also: &["rand_float"],
    since: Some("v0.1.5"),
};

static RAND_BOOL: FnEntry = FnEntry {
    signature: "rand_bool()",
    description: "returns a random bool, using an internally randomized probability",
    example: "get std::random::rand_bool\n\nrand_bool() // e.g. true",
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["rand_bool_weighted"],
    since: Some("v0.1.5"),
};

static RAND_BOOL_WEIGHTED: FnEntry = FnEntry {
    signature: "rand_bool_weighted(probability)",
    description: "returns a random bool that is true with the given probability; values are clamped to [0.0, 1.0] rather than erroring",
    example: "get std::random::rand_bool_weighted\n\nrand_bool_weighted(0.8) // e.g. true",
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["rand_bool"],
    since: Some("v0.1.5"),
};

static RAND_DICE: FnEntry = FnEntry {
    signature: "rand_dice(sides)",
    description: "rolls a single die with the given number of sides and returns the result",
    example: "get std::random::rand_dice\n\nrand_dice(6)?",
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if `sides` is 0 or negative"),
    see_also: &["rand_dices"],
    since: Some("v0.1.5"),
};

static RAND_DICES: FnEntry = FnEntry {
    signature: "rand_dices(count, sides)",
    description: "rolls count dice with the given number of sides and returns the individual results as an array",
    example: "get std::random::rand_dices\n\nrand_dices(3, 6)?",
    expected_output: None,
    returns: "result[arr[int]]",
    errors: Some(
        "Will return error on the following:\n\n- `count` is 0 or negative\n- `sides` is 0 or negative",
    ),
    see_also: &["rand_dice"],
    since: Some("v0.1.5"),
};

static RAND_RANGE: FnEntry = FnEntry {
    signature: "rand_range(stop)",
    description: "returns a random int from 0 to stop (exclusive), stop must be greater than zero",
    example: "get std::random::rand_range\n\nrand_range(10)?",
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if `stop` is 0 or negative"),
    see_also: &["rand_int_range", "rand_range_step"],
    since: Some("v0.1.5"),
};

static RAND_RANGE_STEP: FnEntry = FnEntry {
    signature: "rand_range_step(start, end, step)",
    description: "returns a random int from start to end, aligned to step; reaches end only if step divides (end - start) evenly, otherwise caps at the highest reachable multiple below end",
    example: "get std::random::rand_range_step\n\nrand_range_step(0, 9, 2)? // max possible is 8, since 9 isn't reachable",
    expected_output: None,
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- `step` is 0\n- `start` is greater than or equal to `end`\n\nNote: a negative `step` is not validated and will produce an\nincorrectly-sized reachable range rather than an error - pass a\npositive `step` only.",
    ),
    see_also: &["rand_range"],
    since: Some("v0.1.5"),
};

static RAND_CHOICE: FnEntry = FnEntry {
    signature: "rand_choice(arr)",
    description: "returns a random element from the array",
    example: "get std::random::rand_choice\n\nrand_choice([1, 2, 3])?",
    expected_output: None,
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["rand_choices", "rand_sample"],
    since: Some("v0.1.5"),
};

static RAND_CHOICES: FnEntry = FnEntry {
    signature: "rand_choices(arr, count)",
    description: "returns an array of count random elements from arr, with replacement",
    example: "get std::random::rand_choices\n\nrand_choices([1, 2, 3], 5)?",
    expected_output: None,
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `count` is 0 or negative\n\nNote: unlike `rand_choice`, an empty `arr` is not validated here and\nwill panic at runtime rather than returning an error.",
    ),
    see_also: &["rand_choice", "rand_sample"],
    since: Some("v0.1.5"),
};

static RAND_SAMPLE: FnEntry = FnEntry {
    signature: "rand_sample(arr, count)",
    description: "returns an array of count random elements from arr, without replacement (count must not exceed arr's length)",
    example: "get std::random::rand_sample\n\nrand_sample([1, 2, 3, 4], 2)?",
    expected_output: None,
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `count` is 0 or negative\n- `count` is greater than the length of `arr`",
    ),
    see_also: &["rand_choices", "rand_shuffle"],
    since: Some("v0.1.5"),
};

static RAND_SHUFFLE: FnEntry = FnEntry {
    signature: "rand_shuffle(arr)",
    description: "returns the array with its elements in random order",
    example: "get std::random::rand_shuffle\n\nrand_shuffle([1, 2, 3, 4, 5])?",
    expected_output: None,
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["rand_sample"],
    since: Some("v0.1.5"),
};

static RAND_BYTE: FnEntry = FnEntry {
    signature: "rand_byte()",
    description: "returns a random value in 0 to 255",
    example: "get std::random::rand_byte\n\nrand_byte() // e.g. 110",
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["rand_bytes"],
    since: Some("v0.1.5"),
};

static RAND_BYTES: FnEntry = FnEntry {
    signature: "rand_bytes(count)",
    description: "returns an array of count random values, each in 0 to 255",
    example: "get std::random::rand_bytes\n\nrand_bytes(4)?",
    expected_output: None,
    returns: "result[arr[int]]",
    errors: Some(
        "Will return error if `count` is exactly 0.\n\nNote: despite the name, this returns `arr[int]`, not `arr[byte]` -\neach element is an `int` in 0-255, not a `byte` value. Also, a negative\n`count` is not caught by the current validation and silently returns an\nempty array instead of an error.",
    ),
    see_also: &["rand_byte", "rand_string"],
    since: Some("v0.1.5"),
};

static RAND_CHAR: FnEntry = FnEntry {
    signature: "rand_char()",
    description: "returns a random printable ascii character (32 to 126)",
    example: "get std::random::rand_char\n\nrand_char() // e.g. 'c'",
    expected_output: None,
    returns: "char",
    errors: None,
    see_also: &["rand_string"],
    since: Some("v0.1.5"),
};

static RAND_STRING: FnEntry = FnEntry {
    signature: "rand_string(count)",
    description: "returns a random printable ascii string of the given length",
    example: "get std::random::rand_string\n\nrand_string(8)?",
    expected_output: None,
    returns: "result[string]",
    errors: Some(
        "Will return error if `count` is exactly 0.\n\nNote: same as `rand_bytes`, a negative `count` is not caught by the\ncurrent validation and silently returns an empty string instead of an\nerror.",
    ),
    see_also: &["rand_char", "rand_bytes"],
    since: Some("v0.1.5"),
};
