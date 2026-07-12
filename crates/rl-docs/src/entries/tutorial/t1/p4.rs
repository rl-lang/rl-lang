use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_IO: ConceptEntry = ConceptEntry {
    name: "4. talking to the user",
    summary: "talking to the user",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "so far your program only talks at the user. to make it interactive you need to read input. read waits for the user to type something and press enter, then gives you back what they typed as a string",
            examples: &[
                "get read from std::io\n\ndec string name = read(\"what is your name? \")\nprintln(\"hello \")\nprintln(name)",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "read_int and read_float do the same but convert the input to a number for you. use these when you expect the user to type a number",
            examples: &[
                "get read_int from std::io\n\ndec int age = read_int(\"how old are you? \")\nprintln(\"you are \")\nprintln(age)\nprintln(\" years old\")",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "to build nicer output you can use concat from std::str to join multiple values into one string before printing",
            examples: &[
                "get concat from std::str\n\ndec string name = \"Mohamed\"\ndec int    age  = 25\nprintln(concat(\"hello \", name, \", you are \", age, \" years old\"))",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: ask the player for a guess and echo it back. do not worry about checking if it is correct yet\n\nexpected output:\n  enter your guess: 50\n  you guessed: 50",
            examples: &[
                "get read_int from std::io\nget concat   from std::str\n\ndec int guess = read_int(\"enter your guess: \")\nprintln(concat(\"you guessed: \", guess))",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
