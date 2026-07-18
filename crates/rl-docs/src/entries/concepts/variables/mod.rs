use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static VARIABLES: ConceptEntry = ConceptEntry {
    name: "variables",
    summary: "mutable bindings declared with `dec <type> <name> = <value>`, reassigned with `=` or a compound operator - every compound assignment is sugar for the plain operator it names",
    category: ConceptCategory::Syntax,
    prerequisites: &["types"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("declaring a variable"),
            description: "declare a mutable variable with `dec <type> <name> = <value>`",
            examples: &[
                "dec int x = 10\ndec float y = 3.14\ndec bool flag = true\ndec string name = \"Mohamed\"\ndec char c = 'a'",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("reassignment"),
            description: "reassign a mutable variable with `=`",
            examples: &["dec int x = 1\nx = 2\nprintln(x)  // 2"],
            expected_output: &["2"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("compound assignment"),
            description: "compound assignment: `+=`, `-=`, `*=`, `/=` - these are sugar for `x = x <op> value`",
            examples: &[
                "dec int x = 10\nx += 5\nprintln(x)  // 15",
                "dec int x = 10\nx -= 3\nprintln(x)  // 7",
                "dec int x = 10\nx *= 2\nprintln(x)  // 20",
                "dec int x = 10\nx /= 4\nprintln(x)  // 2",
            ],
            expected_output: &["15", "7", "20", "2"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("compound assignment only works on a plain variable"),
            description: "`x += 5` only desugars into an assignment when the left side is a plain variable name; on anything else - an array index, a record field, and so on - the operator is parsed as an ordinary (non-assigning) binary expression instead, so it silently computes a value and throws it away without updating anything, and nothing flags it as an error",
            examples: &[
                "dec arr[int] nums = [1, 2, 3]\nnums[0] += 1  // computes nums[0] + 1 and discards it\nprintln(nums)  // [1, 2, 3], unchanged",
            ],
            expected_output: &["[1, 2, 3]"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("declaring shadows, it doesn't error"),
            description: "declaring another `dec` variable with a name that's already in scope doesn't error - it just rebinds the name in that scope; only declaring a new `CONST` with an already-used name is a compile-time error (see the `constants` concept)",
            examples: &["dec int x = 1\ndec int x = 2\nprintln(x)  // 2"],
            expected_output: &["2"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: Some("multi-variable declarations"),
            description: "a single `dec` can also destructure a tuple into several differently-typed variables at once, e.g. `dec int x, string y = (10, \"hi\")` - see the `tuples` concept",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "`x += 5` (and `-=`/`*=`/`/=`) only works when the left side is a plain variable - on an array index or record field it's silently parsed as a plain expression instead of an assignment, so nothing gets updated and there's no error",
        "declaring a `dec` variable with a name already in scope silently shadows it instead of erroring - only re-declaring as `CONST` with a used name is a compile error",
    ],
    related: &["constants", "types", "tuples", "operators"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};
