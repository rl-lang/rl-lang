use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_LOOPS: ConceptEntry = ConceptEntry {
    name: "7. repeating things",
    summary: "repeating things",
    category: ConceptCategory::ControlFlow,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "a while loop runs its block over and over as long as its condition stays true. as soon as the condition becomes false the loop stops",
            examples: &[
                "dec int count = 3\n\nwhile (count > 0) {\n    println(count)\n    count -= 1\n}\n// 3\n// 2\n// 1",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "break exits the loop immediately no matter what the condition says. use it when something happens inside the loop that means you are done",
            examples: &[
                "dec int i = 0\n\nwhile (true) {\n    println(i)\n    i += 1\n    if (i == 3) { break } // stops after printing 0, 1, 2\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "continue skips the rest of the current iteration and jumps back to the condition check",
            examples: &[
                "dec int i = 0\n\nwhile (i < 5) {\n    i += 1\n    if (i == 3) { continue } // skip 3\n    println(i)\n}\n// 1\n// 2\n// 4\n// 5",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: wrap your guess check in a loop so the player keeps guessing until they get it right or run out of attempts. decrease attempts_left each round and break when they win or lose\n\nexpected output:\n  attempts left: 10\n  enter your guess: 30\n  too low!\n  attempts left: 9\n  enter your guess: 42\n  correct! you got it!",
            examples: &[
                "get read_int from std::io\nget concat   from std::str\n\nCONST int MAX_ATTEMPTS = 10\ndec int secret         = 42\ndec int attempts_left  = MAX_ATTEMPTS\n\nwhile (attempts_left > 0) {\n    println(concat(\"attempts left: \", attempts_left))\n    dec int guess = read_int(\"enter your guess: \")?\n    attempts_left -= 1\n\n    if (guess < secret) {\n        println(\"too low!\")\n    } else if (guess > secret) {\n        println(\"too high!\")\n    } else {\n        println(\"correct! you got it!\")\n        break\n    }\n}\n\nif (attempts_left == 0) {\n    println(concat(\"out of attempts! the number was \", secret))\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
