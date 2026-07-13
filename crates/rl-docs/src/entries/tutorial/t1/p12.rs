use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_NULL: ConceptEntry = ConceptEntry {
    name: "12. null",
    summary: "null",
    category: ConceptCategory::Types,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "null means the absence of a value. a variable can hold null regardless of its declared type. functions that do not explicitly return anything return null implicitly",
            examples: &[
                "dec int x = null\nprintln(x) // null\n\nfn do_nothing() {\n    // returns null implicitly\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "use is_null from std::types to check if a value is null before using it. this avoids surprises when a function might return nothing",
            examples: &[
                "get is_null from std::types\n\nfn find_even(arr[int] nums) -> int {\n    for n in nums {\n        if (n > 10) { return n }\n    }\n    return null // nothing matched\n}\n\ndec int res = find_even([1, 2, 3])\n\nif (is_null(res)) {\n    println(\"nothing found\")\n} else {\n    println(res)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: write a function find_first_correct that takes the guesses array and the secret number and returns the index of the first correct guess, or null if the player never guessed correctly\n\nhint: use a for loop with a counter variable alongside it",
            examples: &[
                "get is_null from std::types\n\nfn find_first_correct(arr[int] guesses, int secret) -> int {\n    dec int i = 0\n    for g in guesses {\n        if (g == secret) { return i }\n        i += 1\n    }\n    return null\n}\n\ndec int idx = find_first_correct(guesses, secret)\n\nif (is_null(idx)) {\n    println(\"you never guessed correctly\")\n} else {\n    println(format(\"you got it on guess number {}\", idx + 1))\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
