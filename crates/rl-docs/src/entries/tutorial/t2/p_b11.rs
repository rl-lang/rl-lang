use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
