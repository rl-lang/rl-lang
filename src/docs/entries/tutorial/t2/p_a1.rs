use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ADV_INTRO: ConceptEntry = ConceptEntry {
    name: "1. what we are building",
    summary: "what we are building",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "in the beginner tutorial you built a self-contained game. everything lived in one file and nothing persisted between runs. real programs are different - they are split across multiple files, they save and load data, and they are built from reusable pieces.\n\nthis tutorial builds two things:\n  part 1 - a CSV library in csv.rl that parses, queries, and writes CSV files\n  part 2 - a task manager CLI in main.rl that imports and uses that library\n\nby the end you will have a program you can actually use day to day",
            examples: &[
                "// what the finished program looks like\n// $ rl run main.rl\n// task manager ready. type 'help' for commands\n// > add buy groceries\n// added task 1: buy groceries\n// > add write tutorial\n// added task 2: write tutorial\n// > done 1\n// marked task 1 as done\n// > list\n// [1] [done]    2026-06-20  buy groceries\n// [2] [pending] 2026-06-20  write tutorial\n// > remove 1\n// removed task 1\n// > quit\n// goodbye",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "the CSV format we will use is simple: each row is one line, fields are separated by semicolons (not commas, to avoid conflicts with task text). the task file looks like this",
            examples: &[
                "// tasks.csv\n// id;status;created_at;text\n// 1;pending;1750000000;buy groceries\n// 2;done;1750000100;write tutorial\n// 3;pending;1750000200;fix bug in parser",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "we use semicolons instead of commas so task text can contain commas freely. no quoted field handling needed - keep it simple, keep it readable",
            examples: &[
                "// valid task text with our format:\n// buy milk, eggs, and bread   <- comma in text, fine because delimiter is ;\n\n// would break a comma-delimited CSV:\n// buy milk, eggs, and bread   <- parser would split this into 4 fields",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
