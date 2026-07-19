use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_LAMBDAS: ConceptEntry = ConceptEntry {
    name: "12. functions as values",
    summary: "functions as values",
    category: ConceptCategory::Functions,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "in rl functions are values just like numbers and strings. you can store a function in a variable with dec fn and call it through that variable",
            examples: &[
                "fn double(int x) -> int {\n    return x * 2\n}\n\ndec fn f = double\nprintln(f(5)) // 10",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "a lambda is an anonymous function defined inline without a name. useful when you need a short function just once and do not want to name it",
            examples: &[
                "dec fn square = fn(int x) -> int {\n    return x * x\n}\n\nprintln(square(4)) // 16",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "lambdas capture variables from the surrounding scope automatically",
            examples: &[
                "dec int base = 10\n\ndec fn add_base = fn(int x) -> int {\n    return x + base\n}\n\nprintln(add_base(5))  // 15\nprintln(add_base(20)) // 30",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "the real power of lambdas is passing them to functions like arr_map and arr_filter which apply your function to every element of an array. both can fail (again, if you pass something that isn't actually an array), so both return a result you unwrap with `?`",
            examples: &[
                "get arr_map, arr_filter from std::array\n\ndec arr[int] nums    = [1, 2, 3, 4, 5, 6]\ndec arr[int] evens   = arr_filter(nums, fn(int x) -> bool { return x > 3 })?\ndec arr[int] doubled = arr_map(evens, fn(int x) -> int { return x * 2 })?\nprintln(doubled) // [8, 10, 12]",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: use arr_filter and a lambda to count how many of the player's guesses were below the secret number, and how many were above\n\nexpected output:\n  guesses below: 2\n  guesses above: 1",
            examples: &[
                "get arr_filter, len from std::array\nget format          from std::str\n\n// assume secret and guesses exist from the game\ndec arr[int] below = arr_filter(guesses, fn(int g) -> bool { return g < secret })?\ndec arr[int] above = arr_filter(guesses, fn(int g) -> bool { return g > secret })?\n\nprintln(format(\"guesses below: {}\", len(below)))\nprintln(format(\"guesses above: {}\", len(above)))",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
