use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ADV_STRING_PARSING: ConceptEntry = ConceptEntry {
    name: "3. parsing strings",
    summary: "parsing strings",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "parsing means taking raw text and turning it into structured data your program can work with. it is one of the most common real-world tasks. you already know split from std::str - it is the foundation of CSV parsing",
            examples: &[
                "get split from std::str\n\ndec string row = \"1;pending;1750000000;buy groceries\"\ndec arr[string] fields = split(row, \";\")\n\nprintln(fields[0]) // 1\nprintln(fields[1]) // pending\nprintln(fields[2]) // 1750000000\nprintln(fields[3]) // buy groceries",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "to parse a full CSV string you split it into lines first, then split each line into fields. you get an array of rows where each row is an array of strings",
            examples: &[
                "get split from std::str\n\ndec string csv = \"1;pending;buy milk\\n2;done;write code\"\ndec arr[string]        lines = split(csv, \"\\n\")\ndec arr[arr[string]]   rows  = []\n\n// we will fill this in with arr_push soon",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "trim is important when parsing. files often have trailing newlines or spaces that will silently break comparisons if you do not strip them first",
            examples: &[
                "get split, trim from std::str\n\ndec string line = \"  1;pending;buy milk  \\n\"\ndec string clean = trim(line)\ndec arr[string] fields = split(clean, \";\")\nprintln(fields[1]) // pending  (not \"pending  \\n\")",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "is_empty lets you skip blank lines - files often end with a trailing newline that produces an empty string when split",
            examples: &[
                "get split, trim, is_empty from std::str\n\ndec string csv   = \"row1\\nrow2\\n\"\ndec arr[string] lines = split(csv, \"\\n\")\n\nfor line in lines {\n    if (is_empty(trim(line))) { continue }\n    println(line) // row1, row2 - trailing empty line skipped\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: write a function csv_parse_row in csv.rl that takes a single CSV line as a string and returns an array of trimmed fields. test it in main.rl\n\nexpected output:\n  [1, pending, 1750000000, buy groceries]",
            examples: &[
                "// csv.rl\nget split, trim from std::str\n\nCONST string DELIMITER = \";\"\n\nfn csv_parse_row(string line) -> arr[string] {\n    get arr_map from std::array\n    dec arr[string] fields = split(line, DELIMITER)\n    return arr_map(fields, fn(string f) -> string { return trim(f) })\n}\n\n// main.rl\nget csv\nget concat from std::str\n\nfn main() {\n    dec arr[string] row = csv_parse_row(\"1;pending;1750000000;buy groceries\")\n    println(row)\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
