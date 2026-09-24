use crate::entry::FnEntry;

pub static RAND_SEED: FnEntry = FnEntry {
    signature: "rand_seed(seed)",
    description: "re-seeds the random number generator; subsequent random calls will produce deterministic output for the same seed",
    example: r#"get std::random::rand_seed
get std::random::rand_int_range

rand_seed(42)
rand_int_range(0, 100)"#,
    expected_output: Some("37"),
    returns: "null",
    errors: None,
    see_also: &["rand_int", "rand_float", "rand_choice"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
