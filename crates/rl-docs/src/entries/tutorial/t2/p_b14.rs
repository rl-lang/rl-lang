use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
