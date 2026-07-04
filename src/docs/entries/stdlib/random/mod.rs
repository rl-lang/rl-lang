use crate::docs::entry::{FnEntry, StdEntry};

pub static RANDOM: StdEntry = StdEntry {
    name: "random",
    description: "functions for random number and value generation",
    functions: FUNCTIONS,
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
    example: "get std::random::rand_int\n\nrand_int() // 4650267523947147985",
};

static RAND_INT_RANGE: FnEntry = FnEntry {
    signature: "rand_int_range(min, max)",
    description: "returns a random int between min and max (inclusive)",
    example: "get std::random::rand_int_range\n\nrand_int_range(1, 6) // 4",
};

static RAND_FLOAT: FnEntry = FnEntry {
    signature: "rand_float()",
    description: "returns a random float between 0.0 and 1.0",
    example: "get std::random::rand_float\n\nrand_float() // 0.3528",
};

static RAND_FLOAT_RANGE: FnEntry = FnEntry {
    signature: "rand_float_range(min, max)",
    description: "returns a random float between min and max",
    example: "get std::random::rand_float_range\n\nrand_float_range(1.0, 2.0) // 1.5124",
};

static RAND_BOOL: FnEntry = FnEntry {
    signature: "rand_bool()",
    description: "returns a random bool, using an internally randomized probability",
    example: "get std::random::rand_bool\n\nrand_bool() // true",
};

static RAND_BOOL_WEIGHTED: FnEntry = FnEntry {
    signature: "rand_bool_weighted(probability)",
    description: "returns a random bool that is true with the given probability (0.0 to 1.0) values smaller than 0.0 will be 0.0 and values bigger than 1.0 will be 1.0",
    example: "get std::random::rand_bool_weighted\n\nrand_bool_weighted(0.8) // true",
};

static RAND_DICE: FnEntry = FnEntry {
    signature: "rand_dice(sides)",
    description: "rolls a single die with the given number of sides and returns the result",
    example: "get std::random::rand_dice\n\nrand_dice(6) // 5",
};

static RAND_DICES: FnEntry = FnEntry {
    signature: "rand_dices(count, sides)",
    description: "rolls count dice with the given number of sides and returns the individual results as an array",
    example: "get std::random::rand_dices\n\nrand_dices(3, 6) // [4, 1, 6]",
};

static RAND_RANGE: FnEntry = FnEntry {
    signature: "rand_range(stop)",
    description: "returns a random int from 0 to stop (exclusive), stop must be greater than zero",
    example: "get std::random::rand_range\n\nrand_range(10) // 7",
};

static RAND_RANGE_STEP: FnEntry = FnEntry {
    signature: "rand_range_step(start, end, step)",
    description: "returns a random int from start to end, aligned to step; reaches end only if step divides (end - start) evenly, otherwise caps at the highest reachable multiple below end",
    example: "get std::random::rand_range_step\n\nrand_range_step(0, 9, 2) // 8 (max possible, since 9 isn't reachable)",
};

static RAND_CHOICE: FnEntry = FnEntry {
    signature: "rand_choice(arr)",
    description: "returns a random element from the array",
    example: "get std::random::rand_choice\n\nrand_choice([1, 2, 3]) // 1",
};

static RAND_CHOICES: FnEntry = FnEntry {
    signature: "rand_choices(arr, count)",
    description: "returns an array of count random elements from arr, with replacement",
    example: "get std::random::rand_choices\n\nrand_choices([1, 2, 3], 5) // [1, 1, 3, 1, 1]",
};

static RAND_SAMPLE: FnEntry = FnEntry {
    signature: "rand_sample(arr, count)",
    description: "returns an array of count random elements from arr, without replacement (count must not exceed arr's length)",
    example: "get std::random::rand_sample\n\nrand_sample([1, 2, 3, 4], 2) // [4, 2]",
};

static RAND_SHUFFLE: FnEntry = FnEntry {
    signature: "rand_shuffle(arr)",
    description: "returns the array with its elements in random order",
    example: "get std::random::rand_shuffle\n\nrand_shuffle([1, 2, 3, 4, 5]) // [3, 5, 1, 4, 2]",
};

static RAND_BYTE: FnEntry = FnEntry {
    signature: "rand_byte()",
    description: "returns a random byte (0 to 255)",
    example: "get std::random::rand_byte\n\nrand_byte() // 110",
};

static RAND_BYTES: FnEntry = FnEntry {
    signature: "rand_bytes(count)",
    description: "returns an array of count random bytes",
    example: "get std::random::rand_bytes\n\nrand_bytes(4) // [226, 232, 81, 178]",
};

static RAND_CHAR: FnEntry = FnEntry {
    signature: "rand_char()",
    description: "returns a random printable ascii character (32 to 126)",
    example: "get std::random::rand_char\n\nrand_char() // 'c'",
};

static RAND_STRING: FnEntry = FnEntry {
    signature: "rand_string(count)",
    description: "returns a random printable ascii string of the given length",
    example: "get std::random::rand_string\n\nrand_string(8) // \"oNU7'=^:\"",
};
