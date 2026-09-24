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
            description: "specify a return type with `-> <type>` and use `return` to return a value. Omit `->` and the return type is inferred from the body instead, so any type - `handle`, records, tags, results - flows through calls",
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
            kind: DescriptionKind::Explanation,
            title: Some("trailing expression is returned"),
            description: "a body ending with a bare expression returns its value, exactly like an explicit `return` - the checker infers the undeclared return type from it on both the VM and compiled backends",
            examples: &[
                "fn add(int a, int b) {\n    a + b\n}\n\nprintln(add(2, 3))  // 5",
            ],
            expected_output: &["5"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("`?` in the body means a result return"),
            description: "a body using `?` can return `err`, so the inferred return type is wrapped in `result` - `fn grab(string p) { read_file(p)? }` infers `result[string]`, mirroring a `-> result[string]` annotation",
            examples: &[
                "fn grab(string p) { read_file(p)? }\n\ndec s = grab(\"data.txt\")?  // unwrapped string",
            ],
            expected_output: &[],
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
        "a variable holding a function uses the plain `fn` type, which doesn't capture the function's parameter or return types",
        "an undeclared return type is only inferred when every `return` (or the trailing expression) agrees on one concrete type - mixed returns stay `null`-typed",
    ],
    related: &["lambdas", "types", "result", "null"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
