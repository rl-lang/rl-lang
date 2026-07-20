use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
                "// csv.rl structure\n// imports\nget split, trim, is_empty, join, concat from std::str\nget arr_push, arr_map, arr_filter, arr_find,\n    arr_reduce, len                      from std::array\nget read_file, write_file                from std::io\nget path_exists                          from std::path\nget to_int, to_string                    from std::types\n\n// constants\nCONST string DELIMITER   = \";\"\nCONST string HEADER      = \"id;status;created_at;text\"\nCONST int    COL_ID         = 0\nCONST int    COL_STATUS     = 1\nCONST int    COL_CREATED_AT = 2\nCONST int    COL_TEXT       = 3\n\n// functions - every stdlib call that can fail (arr_push, arr_map, arr_filter,\n// arr_find, arr_reduce, join, read_file, to_int, to_string) is unwrapped with `?`\n// csv_parse_row, csv_parse, csv_serialize_row, csv_serialize\n// csv_load, csv_save\n// csv_filter_by, csv_find_by_id\n// csv_next_id, csv_add_row, csv_remove_by_id, csv_update_field",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("one more piece of syntax: match"),
            description: "main.rl's dispatch has grown into a long if/else-if chain, one branch per command. rl has a cleaner tool for exactly this: match. it compares one value against a list of literal patterns and runs the block for whichever one hits, with `_` as a catch-all for everything else",
            examples: &[
                "match cmd {\n    \"quit\" => {\n        break\n    }\n    \"add\" => {\n        tasks = cmd_add(tasks, args)\n    }\n    _ => {\n        println(format(\"unknown command: '{}'\", cmd))\n    }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: None,
            description: "match arms only compare against literal values, like the string literals here - there is no OR-pattern and no way to bind a variable out of the match yet. that is fine for command dispatch since every command name is a fixed string, but keep using if/elif for anything fancier",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "the main.rl structure, with dispatch rewritten as a match",
            examples: &[
                "// main.rl structure\nget csv\nget time_now, time_diff, format_date_str from std::time\nget read                                  from std::io\nget format, trim, is_empty,\n    index_of, slice                       from std::str\nget is_null                               from std::types\nget arr_filter, len                       from std::array\nget to_int, to_string                     from std::types\nget println                               from std::io\n\nCONST string TASKS_FILE = \"tasks.csv\"\nCONST arr[string] HELP_LINES = [ ... ]\n\n// functions\n// parse_command, format_age, print_task\n// print_startup_summary, cmd_help\n// cmd_add, cmd_list, cmd_done, cmd_remove, cmd_clear, cmd_stats\n\nfn main() {\n    dec arr[arr[string]] tasks = csv_load(TASKS_FILE)\n    print_startup_summary(tasks)\n\n    while (true) {\n        dec arr[string] parts = parse_command(trim(read(\"> \")?))\n        dec string cmd  = parts[0]\n        dec string args = parts[1]\n\n        match cmd {\n            \"quit\" => {\n                break\n            }\n            \"add\" => {\n                if (is_empty(args)) {\n                    println(\"usage: add <task text>\")\n                } else {\n                    tasks = cmd_add(tasks, args)\n                }\n            }\n            \"done\" => {\n                if (is_empty(args)) {\n                    println(\"usage: done <id>\")\n                } else {\n                    tasks = cmd_done(tasks, args)\n                }\n            }\n            \"remove\" => {\n                if (is_empty(args)) {\n                    println(\"usage: remove <id>\")\n                } else {\n                    tasks = cmd_remove(tasks, args)\n                }\n            }\n            \"list\" => {\n                cmd_list(tasks, args)\n            }\n            \"clear\" => {\n                tasks = cmd_clear(tasks)\n            }\n            \"stats\" => {\n                cmd_stats(tasks)\n            }\n            \"help\" => {\n                cmd_help()\n            }\n            _ => {\n                println(format(\"unknown command: '{}'. type 'help' for commands\", cmd))\n            }\n        }\n    }\n\n    println(\"goodbye\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "notice format_age's time_diff call needs its own import (get time_diff from std::time) alongside time_now and format_date_str - easy to forget since format_age lives further down the file than the imports block. and `read(\"> \")?` picks up the `?` you added back in the program-loop chapter, since read is fallible too",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: extend the program with one or more of these ideas:\n  a) search command - 'search <term>' filters tasks whose text contains the term\n  b) edit command - 'edit <id> <new text>' updates a task's text\n  c) due dates - add a due_at column, 'due <id> <days>' sets a due date, list shows overdue tasks differently\n  d) priorities - add a priority column (low/normal/high), 'list high' shows only high priority tasks\n  e) export command - 'export' writes a human-readable text report to tasks_report.txt",
            examples: &[
                "// search example\nget contains from std::str\n\nfn cmd_search(arr[arr[string]] tasks, string term) {\n    if (is_empty(term)) {\n        println(\"usage: search <term>\")\n        return\n    }\n    dec arr[arr[string]] results = arr_filter(tasks, fn(arr[string] row) -> bool {\n        return contains(row[COL_TEXT], term)\n    })?\n    if (len(results) == 0) {\n        println(format(\"no tasks matching '{}'\", term))\n        return\n    }\n    for task in results {\n        print_task(task)\n    }\n}\n\n// and add a \"search\" arm to the match in main():\n// \"search\" => {\n//     cmd_search(tasks, args)\n// }",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
