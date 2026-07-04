use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_TYPES: ConceptEntry = ConceptEntry {
    name: "3. types",
    summary: "types",
    category: ConceptCategory::Types,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "every value in rl has a type. the type tells rl what kind of data it is and what you can do with it. you already saw int, float, bool, string. here is what each one means",
            examples: &[
                "dec int    x = 42       // whole numbers, positive or negative\ndec float  y = 3.14     // numbers with a decimal point\ndec bool   b = true     // either true or false, nothing else\ndec string s = \"hello\"  // text, always in double quotes\ndec char   c = 'a'      // a single character, always in single quotes",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "rl will not let you mix types. assigning the wrong type is caught before your program even runs",
            examples: &[
                "dec int x = 10\n// x = \"hello\"  // error: expected int, got string\n// x = 3.14    // error: expected int, got float",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "byte is a special type for small unsigned integers from 0 to 255. plain number literals like 1, 42, 100 are bytes by default. they widen to int automatically when you assign them to an int variable",
            examples: &[
                "dec byte small = 200    // stays as byte\ndec int  big   = 200    // byte literal 200 widens to int\n\n// byte + byte = byte\n// byte + int  = int",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: look at your game variables from the previous step. what type should each one be? write a comment next to each variable explaining your choice",
            examples: &[
                "CONST int MAX_ATTEMPTS = 10   // int: whole number, no decimals needed\nCONST int MIN_NUMBER   = 1    // int: a position in a range\nCONST int MAX_NUMBER   = 100  // int: a position in a range\n\ndec int  secret_number = 42   // int: we compare it with guesses\ndec int  attempts_left = 10   // int: a counter we will decrease",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
