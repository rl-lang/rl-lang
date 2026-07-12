use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
