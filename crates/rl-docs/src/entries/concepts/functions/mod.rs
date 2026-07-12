use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static FUNCTIONS: ConceptEntry = ConceptEntry {
    name: "functions",
    descriptions: &[
        DescriptionEntry {
            description: "declare a function with `fn <name>(<type> <param>, ...) { <body> }`",
            examples: &["fn greet(string name) {\n    println(name)\n}\n\ngreet(\"Mohamed\")"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "specify a return type with `-> <type>` and use `return` to return a value",
            examples: &[
                "fn add(int a, int b) -> int {\n    return a + b\n}\n\ndec int res = add(3, 4)  // 7",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "functions are first-class values and can be stored in variables",
            examples: &[
                "fn double(int x) -> int {\n    return x * 2\n}\n\ndec fn f = double\nprintln(f(5))  // 10",
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
