use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_VARIABLES: ConceptEntry = ConceptEntry {
    name: "2. storing values",
    summary: "storing values",
    category: ConceptCategory::Types,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "a variable is a named box that holds a value. you declare one with dec, then its type, then its name, then its value. rl needs to know the type upfront and it never changes",
            examples: &[
                "dec int    score  = 0\ndec string name   = \"Mohamed\"\ndec bool   active = true\ndec float  ratio  = 1.5",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "once declared you can read a variable by name, and reassign it with =",
            examples: &["dec int lives = 3\nprintln(lives) // 3\n\nlives = 2\nprintln(lives) // 2"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "for numbers you can use += -= *= /= to update a variable in place instead of writing the full reassignment",
            examples: &[
                "dec int score = 0\nscore += 10  // score is now 10\nscore += 5   // score is now 15\nscore -= 3   // score is now 12\nprintln(score) // 12",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "constants are like variables but they can never be reassigned. declare with CONST. by convention use UPPER_CASE names",
            examples: &[
                "CONST int    MAX_SCORE = 100\nCONST string LANG      = \"rl\"\n\nprintln(MAX_SCORE) // 100\n// MAX_SCORE = 200  // error: cannot assign to constant",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: store the game configuration in variables and constants. the secret number will come from random later, so use a placeholder for now\n\nexpected output:\n  secret number is: 42\n  you have 10 attempts",
            examples: &[
                "CONST int MAX_ATTEMPTS = 10\nCONST int MIN_NUMBER   = 1\nCONST int MAX_NUMBER   = 100\n\ndec int secret_number = 42 // placeholder until we learn random\ndec int attempts_left = MAX_ATTEMPTS\n\nprintln(\"secret number is: \") // just for testing\nprintln(secret_number)\nprintln(\"you have \")\nprintln(attempts_left)\nprintln(\"attempts\")",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
