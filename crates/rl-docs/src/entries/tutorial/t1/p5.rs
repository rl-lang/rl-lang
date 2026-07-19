use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_RESULTS: ConceptEntry = ConceptEntry {
    name: "5. handling failure with result and ?",
    summary: "handling failure with result and ?",
    category: ConceptCategory::ErrorHandling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "last chapter you slapped a `?` onto read_int without knowing what it does. here is the full story: a function that can fail returns a result[T] instead of a plain value. result[T] is either ok(value) on success or err(value) on failure",
            examples: &["dec result[int]    r = ok(42)\ndec result[string] e = err(\"not found\")"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "`?` is shorthand for unwrapping a result: on ok(v) it evaluates to v, on err(e) it stops your program right there with that error - no crash-with-a-stack-trace, just a clean error message",
            examples: &[
                "get read_int from std::io\n\n// if the user types something that isn't a number, read_int returns\n// an err, and \"?\" stops the program right here with that error\ndec int guess = read_int(\"enter your guess: \")?\nprintln(guess)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "you do not have to use `?` right away. you can hold onto a result[T] and check it yourself with is_ok/is_err from std::res, then pull out the value with result_unwrap",
            examples: &[
                "get read_int                     from std::io\nget is_ok, result_unwrap          from std::res\n\ndec result[int] r = read_int(\"enter a number: \")\nif (is_ok(r)) {\n    println(result_unwrap(r))\n} else {\n    println(\"that wasn't a number\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "`?` is just a shortcut for that same is_ok/result_unwrap dance. most of the time `?` is all you need - you will use it constantly from here on, every time you call a function that can fail",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: None,
            description: "`?` only makes sense where it is allowed to stop execution and hand back the error - for a script like your game (no fn main), that means the whole script stops there. do not reach for `?` on a value that is not a result[T] in the first place - it is a no-op on anything else, so it will not save you from forgetting to check a plain value",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: read two guesses in a row with read_int, using `?` on each one, and print both. if you are testing this by hand, try typing something that is not a number and see the program stop with an error instead of crashing confusingly\n\nexpected output:\n  first guess: 10\n  second guess: 20\n  you guessed 10 then 20",
            examples: &[
                "get read_int from std::io\nget format   from std::str\n\ndec int first  = read_int(\"first guess: \")?\ndec int second = read_int(\"second guess: \")?\nprintln(format(\"you guessed {} then {}\", first, second))",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
