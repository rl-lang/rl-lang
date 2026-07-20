use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static FUNCTIONS: ConceptEntry = ConceptEntry {
    name: "functions",
    summary: "named blocks declared with `fn <name>(<type> <param>, ...) { <body> }`, optionally typed with `-> <type>` and returned from with `return` - first-class values that can be stored in a `fn`-typed variable",
    category: ConceptCategory::Functions,
    prerequisites: &["types", "variables"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("declaring a function"),
            description: "declare a function with `fn <name>(<type> <param>, ...) { <body> }`",
            examples: &["fn greet(string name) {\n    println(name)\n}\n\ngreet(\"Mohamed\")"],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("return type and return"),
            description: "specify a return type with `-> <type>` and use `return` to return a value",
            examples: &[
                "fn add(int a, int b) -> int {\n    return a + b\n}\n\ndec int res = add(3, 4)  // 7",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("functions as values"),
            description: "functions are first-class values and can be stored in variables",
            examples: &[
                "fn double(int x) -> int {\n    return x * 2\n}\n\ndec fn f = double\nprintln(f(5))  // 10",
            ],
            expected_output: &["10"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("no implicit last-expression return"),
            description: "the final line of a function body isn't returned unless it's written explicitly with `return` - unlike languages where the last expression is the implicit return value, leaving off `return` here just discards it and the function returns `null`",
            examples: &[
                "fn add(int a, int b) {\n    a + b  // never returned\n}\n\nprintln(add(2, 3))  // null",
            ],
            expected_output: &["null"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("the fn type doesn't encode a signature"),
            description: "storing a function in a variable uses the plain `fn` type (`dec fn f = double`) - it doesn't encode the parameter or return types the way a tuple's or array's declared type does, unlike a fully-typed function pointer in some other languages",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "the last line of a function body isn't implicitly returned - it needs an explicit `return`, or the function returns `null`",
        "a variable holding a function uses the plain `fn` type, which doesn't capture the function's parameter or return types",
    ],
    related: &["lambdas", "types", "result", "null"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
