use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ADV_MODULES: ConceptEntry = ConceptEntry {
    name: "2. splitting code across files",
    summary: "splitting code across files",
    category: ConceptCategory::Modules,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "you already know get from the beginner tutorial - you used it to import stdlib functions. the same keyword imports your own files. when you write get csv, rl looks for csv.rl in the same directory and runs it, making everything declared in it available",
            examples: &[
                "// main.rl\nget csv\n\n// now everything declared in csv.rl is available\n// csv_parse(...)\n// csv_serialize(...)\n// etc",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "you can also import specific names from a file using the from syntax. this is cleaner when you only need a few things",
            examples: &[
                "// import specific functions from csv.rl\nget csv_parse, csv_serialize from csv\n\n// or from a subdirectory\nget csv_parse from lib::csv",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "a file that is meant to be imported is just a regular .rl file with functions and constants declared at the top level. it should not have side effects - no println calls, no read calls, just declarations",
            examples: &[
                "// csv.rl - a library file\n// only declarations, no side effects\n\nCONST string DELIMITER = \";\"\n\nfn csv_parse(string raw) -> arr[arr[string]] {\n    // ...\n}\n\nfn csv_serialize(arr[arr[string]] rows) -> string {\n    // ...\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: create two files. csv.rl with a single constant DELIMITER = \";\", and main.rl that imports csv.rl and prints the delimiter\n\nexpected output:\n  delimiter is: ;",
            examples: &[
                "// csv.rl\nCONST string DELIMITER = \";\"\n\n// main.rl\nget csv\nget concat from std::str\n\nfn main() {\n    println(concat(\"delimiter is: \", DELIMITER))\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
