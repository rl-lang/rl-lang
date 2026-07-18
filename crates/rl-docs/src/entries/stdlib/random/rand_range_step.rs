use crate::entry::FnEntry;

pub static RAND_RANGE_STEP: FnEntry = FnEntry {
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
