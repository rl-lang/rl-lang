use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_ARRAYS: ConceptEntry = ConceptEntry {
    name: "9. collecting data",
    summary: "collecting data",
    category: ConceptCategory::Types,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "an array holds multiple values of the same type in a sequence. you declare one with dec arr[type] and access elements by index starting from zero",
            examples: &[
                "dec arr[int] scores = [10, 20, 30]\n\nprintln(scores[0]) // 10\nprintln(scores[2]) // 30\n\nscores[1] = 99\nprintln(scores) // [10, 99, 30]",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "arr_push adds an element to the end and gives you back the updated array. arr_pop removes the last element. these are in std::array",
            examples: &[
                "get arr_push, arr_pop from std::array\n\ndec arr[int] nums = [1, 2, 3]\nnums = arr_push(nums, 4)\nprintln(nums) // [1, 2, 3, 4]\n\nnums = arr_pop(nums)\nprintln(nums) // [1, 2, 3]",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "len returns the number of elements in an array. arr_contains tells you if a value is in the array",
            examples: &[
                "get len, arr_contains from std::array\n\ndec arr[string] names = [\"ali\", \"sara\", \"omar\"]\nprintln(len(names))               // 3\nprintln(arr_contains(names, \"sara\")) // true",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "you can loop over an array with for ... in to visit every element",
            examples: &[
                "dec arr[int] guesses = [30, 60, 42]\n\nfor g in guesses {\n    println(g)\n}\n// 30\n// 60\n// 42",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: track every guess the player makes in an array. at the end of the game print the full guess history\n\nexpected output:\n  your guesses: [30, 60, 42]",
            examples: &[
                "get arr_push      from std::array\nget read_int      from std::io\nget concat        from std::str\n\ndec arr[int] guesses = []\n\n// inside your game loop, after reading a guess:\n// guesses = arr_push(guesses, guess)\n\n// after the game:\nprintln(concat(\"your guesses: \", guesses))",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
