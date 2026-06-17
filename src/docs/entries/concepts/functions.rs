use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static FUNCTIONS: ConceptEntry = ConceptEntry {
    name: "functions",
    descriptions: &[
        DescriptionEntry {
            description: "declare a function with `fn <name>(<type> <param>, ...) { <body> }`",
            examples: &["fn greet(string name) {\n    println(name)\n}\n\ngreet(\"Mohamed\")"],
        },
        DescriptionEntry {
            description: "specify a return type with `-> <type>` and use `return` to return a value",
            examples: &[
                "fn add(int a, int b) -> int {\n    return a + b\n}\n\ndec int result = add(3, 4)  // 7",
            ],
        },
        DescriptionEntry {
            description: "functions are first-class values and can be stored in variables",
            examples: &[
                "fn double(int x) -> int {\n    return x * 2\n}\n\ndec fn f = double\nprintln(f(5))  // 10",
            ],
        },
    ],
};
