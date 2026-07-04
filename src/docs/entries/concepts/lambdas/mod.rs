use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static LAMBDAS: ConceptEntry = ConceptEntry {
    name: "lambdas",
    descriptions: &[
        DescriptionEntry {
            description: "lambdas are anonymous functions defined inline with `fn(<type> <param>, ...) { <body> }`",
            examples: &[
                "dec fn square = fn(int x) -> int {\n    return x * x\n}\n\nprintln(square(5))  // 25",
            ],
        },
        DescriptionEntry {
            description: "lambdas capture variables from their surrounding scope (closures)",
            examples: &[
                "dec int factor = 3\n\ndec fn triple = fn(int x) -> int {\n    return x * factor\n}\n\nprintln(triple(4))  // 12",
            ],
        },
    ],
};
