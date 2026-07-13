use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ADV_CSV_PARSER: ConceptEntry = ConceptEntry {
    name: "4. building the CSV parser",
    summary: "building the CSV parser",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "now build csv_parse - takes a full CSV string, splits into lines, skips blanks and the header, parses each row, returns an array of rows. this is the core of your library",
            examples: &[
                "// csv.rl\nget split, trim, is_empty from std::str\nget arr_push             from std::array\n\nfn csv_parse(string raw) -> arr[arr[string]] {\n    dec arr[string]      lines  = split(raw, \"\\n\")\n    dec arr[arr[string]] rows   = []\n    dec bool             header = true\n\n    for line in lines {\n        if (is_empty(trim(line))) { continue }\n        if (header) { header = false    continue } // skip header row\n        rows = arr_push(rows, csv_parse_row(line))\n    }\n\n    return rows\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "now build csv_serialize - the reverse. takes an array of rows, joins each row's fields with the delimiter, joins rows with newlines, prepends the header line",
            examples: &[
                "// csv.rl\nget join, concat from std::str\n\nCONST string HEADER = \"id;status;created_at;text\"\n\nfn csv_serialize_row(arr[string] row) -> string {\n    return join(row, DELIMITER)\n}\n\nfn csv_serialize(arr[arr[string]] rows) -> string {\n    get arr_map from std::array\n    dec arr[string] lines = arr_map(rows, fn(arr[string] r) -> string {\n        return csv_serialize_row(r)\n    })\n    return concat(HEADER, \"\\n\", join(lines, \"\\n\"))\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: test the round-trip. parse a CSV string then serialize it back and check that the output matches the input (minus any trailing whitespace)\n\nexpected output:\n  round-trip ok: true",
            examples: &[
                "// main.rl\nget csv\nget trim from std::str\n\nfn main() {\n    dec string input = \"id;status;created_at;text\\n1;pending;1750000000;buy milk\\n2;done;1750000100;write code\"\n\n    dec arr[arr[string]] rows   = csv_parse(input)\n    dec string           output = csv_serialize(rows)\n\n    println(concat(\"round-trip ok: \", trim(input) == trim(output)))\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
