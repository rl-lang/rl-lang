use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static VARIABLES: ConceptEntry = ConceptEntry {
    name: "variables",
    descriptions: &[
        DescriptionEntry {
            description: "declare a mutable variable with `dec <type> <name> = <value>`",
            examples: &[
                "dec int x = 10\ndec float y = 3.14\ndec bool flag = true\ndec string name = \"Mohamed\"\ndec char c = 'a'",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "reassign a mutable variable with `=`",
            examples: &["dec int x = 1\nx = 2"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "compound assignment: `+=`, `-=`, `*=`, `/=`",
            examples: &[
                "dec int x = 10\nx += 5   // 15\nx -= 3   // 12\nx *= 2   // 24\nx /= 4   // 6",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
    ],
    summary: "",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
