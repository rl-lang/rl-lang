use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_OPERATORS_AND_DECISIONS: ConceptEntry = ConceptEntry {
    name: "5. making decisions",
    summary: "making decisions",
    category: ConceptCategory::ControlFlow,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "comparison operators compare two values and produce a bool. you use these to ask questions about your data",
            examples: &[
                "println(5 == 5)   // true  (equal)\nprintln(5 != 3)   // true  (not equal)\nprintln(3 < 10)   // true  (less than)\nprintln(10 > 3)   // true  (greater than)\nprintln(3 <= 3)   // true  (less than or equal)\nprintln(10 >= 10) // true  (greater than or equal)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "if runs a block of code only when a condition is true. else runs when it is false. you can chain as many else if branches as you need",
            examples: &[
                "dec int score = 75\n\nif (score >= 90) {\n    println(\"excellent\")\n} else if (score >= 60) {\n    println(\"good\")\n} else {\n    println(\"keep trying\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "! flips a bool. true becomes false, false becomes true",
            examples: &[
                "dec bool ready = false\n\nif (!ready) {\n    println(\"not ready yet\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "arithmetic works as you expect. parentheses control the order of evaluation",
            examples: &[
                "dec int a = (2 + 3) * 4  // 20\ndec int b = 2 + 3 * 4    // 14 (multiplication first)\ndec int c = 10 / 2       // 5",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: add a guess check to your game. compare the guess to the secret number (still hardcoded as 42) and tell the player if they guessed too low, too high, or correctly\n\nexpected output (if guess is 30):\n  enter your guess: 30\n  too low!\n\nexpected output (if guess is 42):\n  enter your guess: 42\n  correct!",
            examples: &[
                "get read_int from std::io\nget concat   from std::str\n\ndec int secret = 42\ndec int guess  = read_int(\"enter your guess: \")\n\nif (guess < secret) {\n    println(\"too low!\")\n} else if (guess > secret) {\n    println(\"too high!\")\n} else {\n    println(\"correct!\")\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
