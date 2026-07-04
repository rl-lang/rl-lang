use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
