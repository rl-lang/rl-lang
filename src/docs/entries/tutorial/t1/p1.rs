use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_FIRST_PROGRAM: ConceptEntry = ConceptEntry {
    name: "1. your first program",
    summary: "your first program",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "programming is just giving a computer instructions, one line at a time. your first instruction: print something to the screen. println prints a value followed by a newline",
            examples: &["println(\"hello, world\")"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "you can print numbers and expressions too, not just strings",
            examples: &["println(42)\nprintln(1 + 1)"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "comments let you leave notes in your code. the computer ignores everything after //",
            examples: &["// this prints a greeting\nprintln(\"hello\") // inline comment"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "print does the same as println but does not add a newline at the end, so the next output appears on the same line",
            examples: &[
                "get print from std::io\n\nprint(\"hello \")\nprint(\"world\")\n// output: hello world",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: print a welcome message for your guessing game. it should greet the player and tell them what the game is about\n\nexpected output:\n  welcome to the guessing game!\n  i am thinking of a number between 1 and 100\n  can you guess it?",
            examples: &[
                "// your game starts here\nprintln(\"welcome to the guessing game!\")\nprintln(\"i am thinking of a number between 1 and 100\")\nprintln(\"can you guess it?\")",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
