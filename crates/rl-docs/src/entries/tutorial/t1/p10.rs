use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_STDLIB: ConceptEntry = ConceptEntry {
    name: "10. the standard library",
    summary: "the standard library",
    category: ConceptCategory::Modules,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "rl comes with a standard library of useful functions organized into modules. you import what you need with get. you have already used std::io and std::str. here are the ones most useful for your game",
            examples: &[
                "get rand_int_range from std::random\nget concat, format from std::str\nget arr_push, len  from std::array",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "std::random gives you unpredictable numbers. rand_int_range returns a random int between two values inclusive. this is how your game will pick the secret number",
            examples: &[
                "get rand_int_range from std::random\n\ndec int secret = rand_int_range(1, 100)\nprintln(secret) // different every run",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "std::str has format which works like concat but uses {} as placeholders. cleaner for building messages with multiple values",
            examples: &[
                "get format from std::str\n\ndec string name = \"Mohamed\"\ndec int    score = 95\nprintln(format(\"hello {}, your score is {}\", name, score))\n// hello Mohamed, your score is 95",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "std::array has arr_sum, arr_min, arr_max for number arrays. useful for showing stats about the player's guesses at the end",
            examples: &[
                "get arr_sum, arr_min, arr_max from std::array\n\ndec arr[int] guesses = [30, 60, 42]\nprintln(arr_min(guesses)) // 30\nprintln(arr_max(guesses)) // 60\nprintln(arr_sum(guesses)) // 132",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: replace the hardcoded secret number with rand_int_range. then at the end of the game show the player their lowest guess, highest guess, and total number of guesses using arr_min, arr_max, and len\n\nexpected output:\n  game over!\n  total guesses: 3\n  lowest guess:  30\n  highest guess: 60",
            examples: &[
                "get rand_int_range        from std::random\nget arr_min, arr_max, len from std::array\nget format                from std::str\n\ndec int secret = rand_int_range(1, 100)\n\n// ... game loop ...\n\nprintln(\"game over!\")\nprintln(format(\"total guesses: {}\", len(guesses)))\nprintln(format(\"lowest guess:  {}\", arr_min(guesses)))\nprintln(format(\"highest guess: {}\", arr_max(guesses)))",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
