use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static OPERATORS: ConceptEntry = ConceptEntry {
    name: "operators",
    descriptions: &[
        DescriptionEntry {
            description: "arithmetic: `+`, `-`, `*`, `/`",
            examples: &[
                "dec int x = 10 + 5   // 15",
                "dec int y = 10 - 3   // 7",
                "dec int z = 4  * 3   // 12",
                "dec int w = 10 / 2   // 5",
            ],
        },
        DescriptionEntry {
            description: "comparison: `==`, `!=`, `<`, `<=`, `>`, `>=` and always return bool",
            examples: &[
                "5 == 5    // true",
                "5 != 3    // true",
                "3 < 10    // true",
                "10 >= 10  // true",
            ],
        },
        DescriptionEntry {
            description: "logical: `!`",
            examples: &["!true           // false"],
        },
        DescriptionEntry {
            description: "unary negation with `-`",
            examples: &["dec int x = 5\ndec int y = -x  // -5"],
        },
        DescriptionEntry {
            description: "method-style call with `.` — calls a function with the value as first argument",
            examples: &[
                "get std::str::to_upper\n\ndec string s = \"hello\"\ns.to_upper()  // \"HELLO\"",
                "get std::array::arr_push\n\ndec arr[int] nums = [1, 2]\nnums = nums.arr_push(3)  // [1, 2, 3]",
            ],
        },
        DescriptionEntry {
            description: "grouping with `()` controls evaluation order",
            examples: &["dec int x = (2 + 3) * 4  // 20\ndec int y = 2 + 3 * 4    // 14"],
        },
    ],
};
