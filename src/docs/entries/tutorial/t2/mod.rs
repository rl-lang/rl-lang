//! Tutorial documentation entries (beginner, advanced, etc.).
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
                "// csv.rl\nget read_file, write_file from std::io\nget path_exists          from std::path\n\nfn csv_load(string path) -> arr[arr[string]] {\n    if (!path_exists(path)) { return [] }\n    dec string raw = read_file(path)\n    return csv_parse(raw)\n}\n\nfn csv_save(string path, arr[arr[string]] rows) {\n    write_file(path, csv_serialize(rows))\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "path_exists is important here - if the file does not exist yet (first run of the program) you want to return an empty array, not crash",
            examples: &[
                "get path_exists from std::path\n\nfn csv_load(string path) -> arr[arr[string]] {\n    if (!path_exists(path)) {\n        return [] // first run, no file yet\n    }\n    return csv_parse(read_file(path))\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: add csv_load and csv_save to csv.rl. in main.rl load tasks.csv, print how many rows it has, add a test row, save it back, then load again and verify the count increased\n\nexpected output (first run):\n  loaded 0 rows\n  saved 1 row\n  reloaded 1 row",
            examples: &[
                "// main.rl\nget csv\nget arr_push, len from std::array\nget format         from std::str\n\nCONST string TASKS_FILE = \"tasks.csv\"\n\nfn main() {\n    dec arr[arr[string]] rows = csv_load(TASKS_FILE)\n    println(format(\"loaded {} rows\", len(rows)))\n\n    rows = arr_push(rows, [\"1\", \"pending\", \"1750000000\", \"test task\"])\n    csv_save(TASKS_FILE, rows)\n    println(format(\"saved {} row\", len(rows)))\n\n    dec arr[arr[string]] reloaded = csv_load(TASKS_FILE)\n    println(format(\"reloaded {} row\", len(reloaded)))\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_CSV_QUERY: ConceptEntry = ConceptEntry {
    name: "6. querying CSV data",
    summary: "querying CSV data",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "raw rows are just arrays of strings. to query them usefully you need helper functions that know which column index means what. define column constants so you never use magic numbers",
            examples: &[
                "// csv.rl\nCONST int COL_ID         = 0\nCONST int COL_STATUS     = 1\nCONST int COL_CREATED_AT = 2\nCONST int COL_TEXT       = 3\n\n// now instead of row[1] you write row[COL_STATUS]\n// readable and safe if columns ever change",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "arr_filter with a lambda is how you query rows. the lambda receives a row and returns true if it matches. this is the pattern you will use for every filtered view",
            examples: &[
                "get arr_filter from std::array\n\n// get all pending tasks\ndec arr[arr[string]] pending = arr_filter(rows, fn(arr[string] row) -> bool {\n    return row[COL_STATUS] == \"pending\"\n})\n\n// get all done tasks\ndec arr[arr[string]] done = arr_filter(rows, fn(arr[string] row) -> bool {\n    return row[COL_STATUS] == \"done\"\n})",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "arr_find lets you locate a single row by id. it returns the first matching row or null if nothing matches - always check with is_null before using the result",
            examples: &[
                "get arr_find  from std::array\nget is_null   from std::types\n\nfn csv_find_by_id(arr[arr[string]] rows, string id) -> arr[string] {\n    return arr_find(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] == id\n    })\n}\n\ndec arr[string] task = csv_find_by_id(rows, \"2\")\nif (is_null(task)) {\n    println(\"not found\")\n} else {\n    println(task[COL_TEXT])\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: add these query functions to csv.rl:\n  csv_filter_by(rows, col, value) -> arr[arr[string]]\n  csv_find_by_id(rows, id)        -> arr[string]\n\nthen test them in main.rl against a hardcoded set of rows",
            examples: &[
                "// csv.rl\nget arr_filter, arr_find from std::array\n\nfn csv_filter_by(arr[arr[string]] rows, int col, string value) -> arr[arr[string]] {\n    return arr_filter(rows, fn(arr[string] row) -> bool {\n        return row[col] == value\n    })\n}\n\nfn csv_find_by_id(arr[arr[string]] rows, string id) -> arr[string] {\n    return arr_find(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] == id\n    })\n}\n\n// main.rl test\ndec arr[arr[string]] rows = [\n    [\"1\", \"pending\", \"1750000000\", \"buy milk\"],\n    [\"2\", \"done\",    \"1750000100\", \"write code\"],\n    [\"3\", \"pending\", \"1750000200\", \"fix bug\"],\n]\n\ndec arr[arr[string]] pending = csv_filter_by(rows, COL_STATUS, \"pending\")\nprintln(len(pending)) // 2\n\ndec arr[string] task = csv_find_by_id(rows, \"2\")\nprintln(task[COL_TEXT]) // write code",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_CSV_MUTATION: ConceptEntry = ConceptEntry {
    name: "7. mutating CSV data",
    summary: "mutating CSV data",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "arrays in rl are values - when you filter or map you get a new array, the original is unchanged. mutation means building a new array with the change applied. this is the same pattern arr_push uses",
            examples: &[
                "get arr_map from std::array\n\n// update a field in one row, return a new rows array\nfn csv_update_field(arr[arr[string]] rows, string id, int col, string value) -> arr[arr[string]] {\n    return arr_map(rows, fn(arr[string] row) -> arr[string] {\n        if (row[COL_ID] != id) { return row }\n        // build updated row\n        dec arr[string] updated = row\n        updated[col] = value\n        return updated\n    })\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "deleting a row means filtering it out. arr_filter with the opposite condition gives you every row except the one you want gone",
            examples: &[
                "get arr_filter from std::array\n\nfn csv_remove_by_id(arr[arr[string]] rows, string id) -> arr[arr[string]] {\n    return arr_filter(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] != id\n    })\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "adding a row means generating a new ID first. the simplest approach: find the current max ID and add 1. arr_reduce works well here",
            examples: &[
                "get arr_reduce  from std::array\nget to_int, to_string from std::types\n\nfn csv_next_id(arr[arr[string]] rows) -> string {\n    if (len(rows) == 0) { return \"1\" }\n    dec int max_id = arr_reduce(\n        rows,\n        fn(int acc, arr[string] row) -> int {\n            dec int id = to_int(row[COL_ID])\n            if (id > acc) { return id }\n            return acc\n        },\n        0\n    )\n    return to_string(max_id + 1)\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: add these mutation functions to csv.rl:\n  csv_add_row(rows, fields)           -> arr[arr[string]]\n  csv_remove_by_id(rows, id)          -> arr[arr[string]]\n  csv_update_field(rows, id, col, val)-> arr[arr[string]]\n  csv_next_id(rows)                   -> string",
            examples: &[
                "// test in main.rl\ndec arr[arr[string]] rows = []\n\n// add\nrows = csv_add_row(rows, [csv_next_id(rows), \"pending\", \"1750000000\", \"buy milk\"])\nrows = csv_add_row(rows, [csv_next_id(rows), \"pending\", \"1750000100\", \"write code\"])\nprintln(len(rows)) // 2\n\n// update\nrows = csv_update_field(rows, \"1\", COL_STATUS, \"done\")\nprintln(csv_find_by_id(rows, \"1\")[COL_STATUS]) // done\n\n// remove\nrows = csv_remove_by_id(rows, \"1\")\nprintln(len(rows)) // 1",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_PROGRAM_LOOP: ConceptEntry = ConceptEntry {
    name: "8. the program loop",
    summary: "the program loop",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "part 1 gave you a complete CSV library. now build the task manager on top of it. the program is a REPL - read, evaluate, print, loop. it reads a command, acts on it, prints the result, and loops until the user quits",
            examples: &[
                "// main.rl skeleton\nget csv\nget read, println from std::io\nget trim           from std::str\n\nCONST string TASKS_FILE = \"tasks.csv\"\n\nfn main() {\n    dec arr[arr[string]] tasks = csv_load(TASKS_FILE)\n    println(\"task manager ready. type 'help' for commands\")\n\n    while (true) {\n        dec string input   = read(\"> \")\n        dec string command = trim(input)\n\n        if (command == \"quit\") { break }\n\n        // dispatch commands here\n        println(concat(\"unknown command: \", command))\n    }\n\n    println(\"goodbye\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "commands like 'add buy milk' have two parts: the command name and the arguments. split on the first space to separate them. use index_of to find where the first space is, then slice to extract each part",
            examples: &[
                "get index_of, slice, trim from std::str\n\nfn parse_command(string input) -> arr[string] {\n    dec int space = index_of(input, \" \")\n    if (space == -1) {\n        return [trim(input), \"\"] // no args\n    }\n    dec string cmd  = slice(input, 0, space)\n    dec string args = trim(slice(input, space + 1, len(input)))\n    return [cmd, args]\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: get the REPL loop running. it should read input, parse the command, and for now just echo back what command and args it parsed. quit should exit cleanly\n\nexpected output:\n  > add buy milk\n  command: add  args: buy milk\n  > list\n  command: list  args:\n  > quit\n  goodbye",
            examples: &[
                "fn main() {\n    println(\"task manager ready. type 'help' for commands\")\n\n    while (true) {\n        dec string       input  = read(\"> \")\n        dec arr[string]  parts  = parse_command(trim(input))\n        dec string       cmd    = parts[0]\n        dec string       args   = parts[1]\n\n        if (cmd == \"quit\") { break }\n\n        println(format(\"command: {}  args: {}\", cmd, args))\n    }\n\n    println(\"goodbye\")\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_TIME: ConceptEntry = ConceptEntry {
    name: "9. working with time",
    summary: "working with time",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "std::time gives you the current time as a unix timestamp - an integer counting seconds since January 1 1970. time_now() returns it. store this when a task is created so you know when it was added",
            examples: &[
                "get time_now from std::time\n\ndec int created_at = time_now()\nprintln(created_at) // e.g. 1750000000",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "raw timestamps are not readable. format_date_str turns a timestamp into a date string. format_time_str gives you the time. format_time lets you build any pattern you want",
            examples: &[
                "get time_now, format_date_str, format_time_str, format_time from std::time\n\ndec int ts = time_now()\nprintln(format_date_str(ts))          // 2026-06-20\nprintln(format_time_str(ts))          // 14:32:07\nprintln(format_time(ts, \"%d/%m/%Y\"))  // 20/06/2026",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "time_diff gives you the number of seconds between two timestamps. useful for showing how old a task is",
            examples: &[
                "get time_now, time_diff from std::time\n\ndec int created = 1750000000\ndec int now     = time_now()\ndec int age     = time_diff(created, now)\n\nprintln(format(\"{} seconds old\", age))",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "time_parts breaks a timestamp into its components as an array: [year, month, day, hour, minute, second]",
            examples: &[
                "get time_now, time_parts from std::time\n\ndec arr[int] parts = time_parts(time_now())\nprintln(parts[0]) // year  e.g. 2026\nprintln(parts[1]) // month e.g. 6\nprintln(parts[2]) // day   e.g. 20",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: write a function format_age(int created_at) -> string that returns a human readable age string. use time_diff and some arithmetic\n\nexpected output examples:\n  just now\n  5 minutes ago\n  3 hours ago\n  2 days ago",
            examples: &[
                "get time_now, time_diff from std::time\nget format              from std::str\n\nfn format_age(int created_at) -> int {\n    dec int diff    = time_diff(created_at, time_now())\n    dec int minutes = diff / 60\n    dec int hours   = minutes / 60\n    dec int days    = hours / 24\n\n    if (diff < 60)     { return \"just now\" }\n    if (minutes < 60)  { return format(\"{} minutes ago\", minutes) }\n    if (hours < 24)    { return format(\"{} hours ago\", hours) }\n    return format(\"{} days ago\", days)\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_COMMANDS_ADD_LIST: ConceptEntry = ConceptEntry {
    name: "10. add and list commands",
    summary: "add and list commands",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "now wire up real commands. add creates a new task row using csv_next_id, time_now for the timestamp, and the args string as the text. it then saves immediately so nothing is lost if the program crashes",
            examples: &[
                "get time_now   from std::time\nget to_string  from std::types\nget concat     from std::str\n\nfn cmd_add(arr[arr[string]] tasks, string text) -> arr[arr[string]] {\n    dec string id         = csv_next_id(tasks)\n    dec string created_at = to_string(time_now())\n    dec arr[string] row   = [id, \"pending\", created_at, text]\n    tasks = csv_add_row(tasks, row)\n    csv_save(TASKS_FILE, tasks)\n    println(format(\"added task {}: {}\", id, text))\n    return tasks\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "list prints all tasks in a readable table. use format to align columns. format_date_str converts the stored timestamp string to a readable date",
            examples: &[
                "get format_date_str from std::time\nget to_int          from std::types\nget format          from std::str\n\nfn print_task(arr[string] row) {\n    dec string id      = row[COL_ID]\n    dec string status  = row[COL_STATUS]\n    dec string date    = format_date_str(to_int(row[COL_CREATED_AT]))\n    dec string text    = row[COL_TEXT]\n    println(format(\"[{}] [{}] {}  {}\", id, status, date, text))\n}\n\nfn cmd_list(arr[arr[string]] tasks) {\n    if (len(tasks) == 0) {\n        println(\"no tasks\")\n        return\n    }\n    for task in tasks {\n        print_task(task)\n    }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: implement cmd_add and cmd_list. wire them into your main loop. the program should persist tasks between runs\n\nexpected output:\n  > add buy groceries\n  added task 1: buy groceries\n  > add write tutorial\n  added task 2: write tutorial\n  > list\n  [1] [pending] 2026-06-20  buy groceries\n  [2] [pending] 2026-06-20  write tutorial",
            examples: &[
                "// in main() dispatch block:\nif (cmd == \"add\") {\n    if (is_empty(args)) {\n        println(\"usage: add <task text>\")\n    } else {\n        tasks = cmd_add(tasks, args)\n    }\n} else if (cmd == \"list\") {\n    cmd_list(tasks)\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_COMMANDS_DONE_REMOVE: ConceptEntry = ConceptEntry {
    name: "11. done, remove, and clear",
    summary: "done, remove, and clear",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "done and remove both take a task ID as their argument. the ID comes in as a string from the command parser. validate it exists before acting - use csv_find_by_id and is_null",
            examples: &[
                "get is_null   from std::types\nget format    from std::str\n\nfn cmd_done(arr[arr[string]] tasks, string id) -> arr[arr[string]] {\n    dec arr[string] task = csv_find_by_id(tasks, id)\n    if (is_null(task)) {\n        println(format(\"no task with id {}\", id))\n        return tasks\n    }\n    tasks = csv_update_field(tasks, id, COL_STATUS, \"done\")\n    csv_save(TASKS_FILE, tasks)\n    println(format(\"marked task {} as done\", id))\n    return tasks\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "remove works the same way - find first, then delete. give the user a confirmation message either way so they know what happened",
            examples: &[
                "fn cmd_remove(arr[arr[string]] tasks, string id) -> arr[arr[string]] {\n    dec arr[string] task = csv_find_by_id(tasks, id)\n    if (is_null(task)) {\n        println(format(\"no task with id {}\", id))\n        return tasks\n    }\n    tasks = csv_remove_by_id(tasks, id)\n    csv_save(TASKS_FILE, tasks)\n    println(format(\"removed task {}\", id))\n    return tasks\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "clear removes all tasks with status done in one pass. arr_filter does the work - keep everything that is not done",
            examples: &[
                "fn cmd_clear(arr[arr[string]] tasks) -> arr[arr[string]] {\n    dec arr[arr[string]] remaining = csv_filter_by(tasks, COL_STATUS, \"pending\")\n    dec int removed = len(tasks) - len(remaining)\n    csv_save(TASKS_FILE, remaining)\n    println(format(\"cleared {} completed task(s)\", removed))\n    return remaining\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: implement done, remove, and clear. wire all five commands into the main loop. test the full flow: add tasks, mark some done, clear them, verify the list\n\nexpected output:\n  > done 1\n  marked task 1 as done\n  > clear\n  cleared 1 completed task(s)\n  > list\n  [2] [pending] 2026-06-20  write tutorial",
            examples: &[
                "} else if (cmd == \"done\") {\n    if (is_empty(args)) {\n        println(\"usage: done <id>\")\n    } else {\n        tasks = cmd_done(tasks, args)\n    }\n} else if (cmd == \"remove\") {\n    if (is_empty(args)) {\n        println(\"usage: remove <id>\")\n    } else {\n        tasks = cmd_remove(tasks, args)\n    }\n} else if (cmd == \"clear\") {\n    tasks = cmd_clear(tasks)\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_FILTERED_VIEWS: ConceptEntry = ConceptEntry {
    name: "12. filtered views and stats",
    summary: "filtered views and stats",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "list alone shows everything. extend it to accept an optional argument: 'list done' or 'list pending' shows only those tasks. the args string you already parse makes this straightforward",
            examples: &[
                "fn cmd_list(arr[arr[string]] tasks, string filter) {\n    dec arr[arr[string]] view = tasks\n\n    if (filter == \"done\")    { view = csv_filter_by(tasks, COL_STATUS, \"done\") }\n    if (filter == \"pending\") { view = csv_filter_by(tasks, COL_STATUS, \"pending\") }\n\n    if (len(view) == 0) {\n        println(\"no tasks\")\n        return\n    }\n    for task in view {\n        print_task(task)\n    }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "a stats command gives a useful summary. use arr_filter to count done vs pending, and format a clean report",
            examples: &[
                "fn cmd_stats(arr[arr[string]] tasks) {\n    dec int total   = len(tasks)\n    dec int done    = len(csv_filter_by(tasks, COL_STATUS, \"done\"))\n    dec int pending = total - done\n\n    println(format(\"total:   {}\", total))\n    println(format(\"done:    {}\", done))\n    println(format(\"pending: {}\", pending))\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: update cmd_list to accept the filter argument and add cmd_stats. add 'stats' to the dispatch loop\n\nexpected output:\n  > list pending\n  [2] [pending] 2026-06-20  write tutorial\n  > stats\n  total:   2\n  done:    1\n  pending: 1",
            examples: &[
                "// updated dispatch for list:\n} else if (cmd == \"list\") {\n    cmd_list(tasks, args) // args is \"done\", \"pending\", or \"\"\n} else if (cmd == \"stats\") {\n    cmd_stats(tasks)\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_HELP_POLISH: ConceptEntry = ConceptEntry {
    name: "13. help and polish",
    summary: "help and polish",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "help should print every available command with a short description. store the help text as a constant array of strings - one entry per command - and loop over it to print",
            examples: &[
                "CONST arr[string] HELP_LINES = [\n    \"  add <text>       add a new task\",\n    \"  done <id>        mark a task as done\",\n    \"  remove <id>      delete a task\",\n    \"  list             show all tasks\",\n    \"  list done        show completed tasks\",\n    \"  list pending     show pending tasks\",\n    \"  clear            remove all completed tasks\",\n    \"  stats            show task counts\",\n    \"  help             show this message\",\n    \"  quit             exit the program\",\n]\n\nfn cmd_help() {\n    println(\"commands:\")\n    for line in HELP_LINES {\n        println(line)\n    }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "unknown commands should not silently do nothing. print a helpful message pointing the user to help",
            examples: &[
                "// at the end of the dispatch chain:\n} else {\n    println(format(\"unknown command: '{}'. type 'help' for commands\", cmd))\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "on startup show a summary so the user immediately knows what state they are in. this uses the same stats logic but formatted as a one-liner",
            examples: &[
                "fn print_startup_summary(arr[arr[string]] tasks) {\n    dec int total   = len(tasks)\n    dec int pending = len(csv_filter_by(tasks, COL_STATUS, \"pending\"))\n    println(format(\"task manager ready - {} task(s), {} pending. type 'help' for commands\", total, pending))\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: add help, unknown command handling, and the startup summary. run the full program and make sure every command works end to end",
            examples: &[
                "// startup\nprint_startup_summary(tasks)\n\n// in dispatch:\n} else if (cmd == \"help\") {\n    cmd_help()\n} else {\n    println(format(\"unknown command: '{}'. type 'help' for commands\", cmd))\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};

pub static ADV_COMPLETE: ConceptEntry = ConceptEntry {
    name: "14. the complete program",
    summary: "the complete program",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "your project should now have two files:\n  csv.rl  - the reusable CSV library\n  main.rl - the task manager that imports it\n\nhere is the full structure of each file",
            examples: &[
                "// csv.rl structure\n// imports\nget split, trim, is_empty, join, concat, format from std::str\nget arr_push, arr_map, arr_filter, arr_find,\n    arr_reduce, len                              from std::array\nget read_file, write_file                        from std::io\nget path_exists                                  from std::path\nget to_int, to_string                            from std::types\n\n// constants\nCONST string DELIMITER   = \";\"\nCONST string HEADER      = \"id;status;created_at;text\"\nCONST int    COL_ID         = 0\nCONST int    COL_STATUS     = 1\nCONST int    COL_CREATED_AT = 2\nCONST int    COL_TEXT       = 3\n\n// functions\n// csv_parse_row, csv_parse, csv_serialize_row, csv_serialize\n// csv_load, csv_save\n// csv_filter_by, csv_find_by_id\n// csv_next_id, csv_add_row, csv_remove_by_id, csv_update_field",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "the main.rl structure",
            examples: &[
                "// main.rl structure\nget csv\nget time_now, format_date_str from std::time\nget read, println              from std::io\nget format, trim, is_empty,\n    index_of, slice, len       from std::str\nget is_null                    from std::types\nget arr_filter, len            from std::array\nget to_int                     from std::types\n\nCONST string TASKS_FILE = \"tasks.csv\"\nCONST arr[string] HELP_LINES = [ ... ]\n\n// functions\n// parse_command, format_age, print_task\n// print_startup_summary, cmd_help\n// cmd_add, cmd_list, cmd_done, cmd_remove, cmd_clear, cmd_stats\n\nfn main() {\n    dec arr[arr[string]] tasks = csv_load(TASKS_FILE)\n    print_startup_summary(tasks)\n\n    while (true) {\n        dec arr[string] parts = parse_command(trim(read(\"> \")))\n        dec string cmd  = parts[0]\n        dec string args = parts[1]\n\n        if      (cmd == \"quit\")   { break }\n        else if (cmd == \"add\")    { tasks = cmd_add(tasks, args) }\n        else if (cmd == \"done\")   { tasks = cmd_done(tasks, args) }\n        else if (cmd == \"remove\") { tasks = cmd_remove(tasks, args) }\n        else if (cmd == \"list\")   { cmd_list(tasks, args) }\n        else if (cmd == \"clear\")  { tasks = cmd_clear(tasks) }\n        else if (cmd == \"stats\")  { cmd_stats(tasks) }\n        else if (cmd == \"help\")   { cmd_help() }\n        else { println(format(\"unknown command: '{}'. type 'help'\", cmd)) }\n    }\n\n    println(\"goodbye\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: extend the program with one or more of these ideas:\n  a) search command - 'search <term>' filters tasks whose text contains the term\n  b) edit command - 'edit <id> <new text>' updates a task's text\n  c) due dates - add a due_at column, 'due <id> <days>' sets a due date, list shows overdue tasks differently\n  d) priorities - add a priority column (low/normal/high), 'list high' shows only high priority tasks\n  e) export command - 'export' writes a human-readable text report to tasks_report.txt",
            examples: &[
                "// search example\nget contains from std::str\n\nfn cmd_search(arr[arr[string]] tasks, string term) {\n    if (is_empty(term)) {\n        println(\"usage: search <term>\")\n        return\n    }\n    dec arr[arr[string]] results = arr_filter(tasks, fn(arr[string] row) -> bool {\n        return contains(row[COL_TEXT], term)\n    })\n    if (len(results) == 0) {\n        println(format(\"no tasks matching '{}'\", term))\n        return\n    }\n    for task in results {\n        print_task(task)\n    }\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
