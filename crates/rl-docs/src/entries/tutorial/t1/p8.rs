use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_FUNCTIONS: ConceptEntry = ConceptEntry {
    name: "8. grouping logic into functions",
    summary: "grouping logic into functions",
    category: ConceptCategory::Functions,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "a function is a named block of code you can call by name. instead of copying the same lines everywhere you write them once as a function and call it wherever you need it",
            examples: &[
                "fn greet(string name) {\n    println(\"hello \")\n    println(name)\n}\n\ngreet(\"Ali\")\ngreet(\"Sara\")",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "functions can give back a value with return. you declare what type they return with -> after the parameter list",
            examples: &[
                "fn add(int a, int b) -> int {\n    return a + b\n}\n\ndec int res = add(3, 4)\nprintln(result) // 7",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "functions can call themselves, this is called recursion. each call works on a smaller version of the problem until it hits a base case that stops the recursion",
            examples: &[
                "fn countdown(int n) {\n    if (n == 0) {\n        println(\"go!\")\n        return\n    }\n    println(n)\n    countdown(n - 1)\n}\n\ncountdown(3)\n// 3\n// 2\n// 1\n// go!",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: pull the hint logic and the divider printer out of your game loop into their own functions. your main game loop should just call them\n\nwrite:\n  fn print_divider() - prints a row of dashes\n  fn check_guess(int guess, int secret) -> string - returns \"low\", \"high\", or \"correct\"",
            examples: &[
                "get print   from std::io\nget concat  from std::str\n\nfn print_divider() {\n    for i in 0..20 {\n        print(\"-\")\n    }\n    println(\"\")\n}\n\nfn check_guess(int guess, int secret) -> string {\n    if (guess < secret) { return \"low\" }\n    if (guess > secret) { return \"high\" }\n    return \"correct\"\n}\n\n// in your game loop:\n// dec string result = check_guess(guess, secret)\n// if (result == \"correct\") { ... }",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
