use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
                "get time_now   from std::time\nget to_string  from std::types\nget format     from std::str\n\nfn cmd_add(arr[arr[string]] tasks, string text) -> arr[arr[string]] {\n    dec string id         = csv_next_id(tasks)\n    dec string created_at = to_string(time_now())?\n    dec arr[string] row   = [id, \"pending\", created_at, text]\n    tasks = csv_add_row(tasks, row)\n    csv_save(TASKS_FILE, tasks)\n    println(format(\"added task {}: {}\", id, text))\n    return tasks\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "list prints all tasks in a readable table. use format to align columns. format_date_str converts the stored timestamp string to a readable date",
            examples: &[
                "get format_date_str from std::time\nget to_int          from std::types\nget format          from std::str\nget len             from std::array\n\nfn print_task(arr[string] row) {\n    dec string id      = row[COL_ID]\n    dec string status  = row[COL_STATUS]\n    dec string date    = format_date_str(to_int(row[COL_CREATED_AT])?)?\n    dec string text    = row[COL_TEXT]\n    println(format(\"[{}] [{}] {}  {}\", id, status, date, text))\n}\n\nfn cmd_list(arr[arr[string]] tasks) {\n    if (len(tasks) == 0) {\n        println(\"no tasks\")\n        return\n    }\n    for task in tasks {\n        print_task(task)\n    }\n}",
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
