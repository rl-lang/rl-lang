use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_FOR_LOOPS: ConceptEntry = ConceptEntry {
    name: "8. for loops",
    summary: "for loops",
    category: ConceptCategory::ControlFlow,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "while loops are great when you do not know how many times you will loop. when you do know, a for loop is cleaner. the range form goes from a start number up to but not including the end",
            examples: &["for i in 0..5 {\n    println(i)\n}\n// 0\n// 1\n// 2\n// 3\n// 4"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "the C-style for loop gives you full control: an initializer, a condition, and an increment, all in one line",
            examples: &[
                "for [int i = 1, i <= 5, i += 1] {\n    println(i)\n}\n// 1\n// 2\n// 3\n// 4\n// 5",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "you can also loop over the elements of an array directly without needing an index at all",
            examples: &[
                "dec arr[string] days = [\"sat\", \"sun\", \"mon\"]\n\nfor day in days {\n    println(day)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: before the game starts, print a row of dashes as a divider using a for loop. then after the game ends print a summary showing each attempt number\n\nexpected output:\n  ----------\n  welcome to the guessing game!\n  ----------",
            examples: &[
                "get print from std::io\n\n// print 10 dashes without a newline each\nfor i in 0..10 {\n    print(\"-\")\n}\nprintln(\"\") // move to next line\n\nprintln(\"welcome to the guessing game!\")\n\nfor i in 0..10 {\n    print(\"-\")\n}\nprintln(\"\")",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
