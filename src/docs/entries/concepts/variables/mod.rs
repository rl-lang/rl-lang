use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static VARIABLES: ConceptEntry = ConceptEntry {
    name: "variables",
    descriptions: &[
        DescriptionEntry {
            description: "declare a mutable variable with `dec <type> <name> = <value>`",
            examples: &[
                "dec int x = 10\ndec float y = 3.14\ndec bool flag = true\ndec string name = \"Mohamed\"\ndec char c = 'a'",
            ],
        },
        DescriptionEntry {
            description: "reassign a mutable variable with `=`",
            examples: &["dec int x = 1\nx = 2"],
        },
        DescriptionEntry {
            description: "compound assignment: `+=`, `-=`, `*=`, `/=`",
            examples: &[
                "dec int x = 10\nx += 5   // 15\nx -= 3   // 12\nx *= 2   // 24\nx /= 4   // 6",
            ],
        },
    ],
};
