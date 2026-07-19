use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ADV_CSV_IO: ConceptEntry = ConceptEntry {
    name: "5. reading and writing CSV files",
    summary: "reading and writing CSV files",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "you know the parser works on strings. now connect it to the filesystem. read_file from std::io gives you the file contents as a string - pass it straight to csv_parse",
            examples: &[
                "// csv.rl\nget read_file, write_file from std::io\nget path_exists          from std::path\n\nfn csv_load(string path) -> arr[arr[string]] {\n    if (!path_exists(path)) { return [] }\n    dec string raw = read_file(path)?\n    return csv_parse(raw)\n}\n\nfn csv_save(string path, arr[arr[string]] rows) {\n    write_file(path, csv_serialize(rows))\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "path_exists is important here - if the file does not exist yet (first run of the program) you want to return an empty array, not crash",
            examples: &[
                "get read_file    from std::io\nget path_exists  from std::path\n\nfn csv_load(string path) -> arr[arr[string]] {\n    if (!path_exists(path)) {\n        return [] // first run, no file yet\n    }\n    return csv_parse(read_file(path)?)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: add csv_load and csv_save to csv.rl. in main.rl load tasks.csv, print how many rows it has, add a test row, save it back, then load again and verify the count increased\n\nexpected output (first run):\n  loaded 0 rows\n  saved 1 row\n  reloaded 1 row",
            examples: &[
                "// main.rl\nget csv\nget arr_push, len from std::array\nget format         from std::str\n\nCONST string TASKS_FILE = \"tasks.csv\"\n\nfn main() {\n    dec arr[arr[string]] rows = csv_load(TASKS_FILE)\n    println(format(\"loaded {} rows\", len(rows)))\n\n    rows = arr_push(rows, [\"1\", \"pending\", \"1750000000\", \"test task\"])?\n    csv_save(TASKS_FILE, rows)\n    println(format(\"saved {} row\", len(rows)))\n\n    dec arr[arr[string]] reloaded = csv_load(TASKS_FILE)\n    println(format(\"reloaded {} row\", len(reloaded)))\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "write_file is fallible too (it returns a result), but csv_save just calls it as a bare statement without `?`. that's allowed - you only need `?` when you actually want the unwrapped value or want a failure to stop the program right there. here we're fine letting a write failure pass silently for now",
    ],
    related: &[],
    related_stdlib: &[],
    since: None,
};
