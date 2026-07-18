get csv
get time_now, format_date_str from std::time
get read from std::io
get format, trim, is_empty, index_of, slice from std::str
get is_null from std::types
get arr_filter, len from std::array
get to_int, to_string from std::types
get println from std::io

CONST string TASKS_FILE = "tasks.csv"

CONST arr[string] HELP_LINES = [
"  add <text>       add a new task",
"  done <id>        mark a task as done",
"  remove <id>      delete a task",
"  list             show all tasks",
"  list done        show completed tasks",
"  list pending     show pending tasks",
"  clear            remove all completed tasks",
"  stats            show task counts",
"  help             show this message",
"  quit             exit the program",
]

fn parse_command(string input) -> arr[string] {
    dec int space = index_of(input, " ")
    if (space == -1) {
        return [trim(input), ""]
    }
    dec string cmd = slice(input, 0, space)?
    dec string args = trim(slice(input, space + 1, len(input))?)
    return [cmd, args]
}

fn format_age(int created_at) -> string {
    dec int diff = time_diff(time_now(), created_at)
    dec int minutes = diff / 60
    dec int hours = minutes / 60
    dec int days = hours / 24

    if (diff < 60) {
        return "just now"
    }
    if (minutes < 60) {
        return format("{} minutes ago", minutes)
    }
    if (hours < 24) {
        return format("{} hours ago", hours)
    }
    return format("{} days ago", days)
}

fn print_task(arr[string] row) {
    dec string date = format_date_str(to_int(row[COL_CREATED_AT])?)?
    println(format("[{}] [{}] {}  {}", row[COL_ID], row[COL_STATUS], date, row[COL_TEXT]))
}

fn print_startup_summary(arr[arr[string]] tasks) {
    dec int total = len(tasks)
    dec int pending = len(csv_filter_by(tasks, COL_STATUS, "pending"))
    println(format("task manager ready - {} task(s), {} pending. type 'help' for commands", total, pending))
}

fn cmd_help() {
    println("commands:")
    for line in HELP_LINES {
        println(line)
    }
}

fn cmd_add(arr[arr[string]] tasks, string text) -> arr[arr[string]] {
    dec string id = csv_next_id(tasks)
    dec string ts = to_string(time_now())?
    tasks = csv_add_row(tasks, [id, "pending", ts, text])
    csv_save(TASKS_FILE, tasks)
    println(format("added task {}: {}", id, text))
    return tasks
}

fn cmd_list(arr[arr[string]] tasks, string filter) {
    dec arr[arr[string]] view = tasks
    if (filter == "done") {
        view = csv_filter_by(tasks, COL_STATUS, "done")
    }
    if (filter == "pending") {
        view = csv_filter_by(tasks, COL_STATUS, "pending")
    }
    if (len(view) == 0) {
        println("no tasks")
        return
    }
    for task in view {
        print_task(task)
    }
}

fn cmd_done(arr[arr[string]] tasks, string id) -> arr[arr[string]] {
    dec arr[string] task = csv_find_by_id(tasks, id)

    if (is_null(task)) {
        println(format("no task with id {}", id))

        return tasks
    }

    tasks = csv_update_field(tasks, id, COL_STATUS, "done")
    csv_save(TASKS_FILE, tasks)
    println(format("marked task {} as done", id))

    return tasks
}

fn cmd_remove(arr[arr[string]] tasks, string id) -> arr[arr[string]] {
    dec arr[string] task = csv_find_by_id(tasks, id)

    if (is_null(task)) {
        println(format("no task with id {}", id))

        return tasks
    }

    tasks = csv_remove_by_id(tasks, id)
    csv_save(TASKS_FILE, tasks)
    println(format("removed task {}", id))

    return tasks
}

fn cmd_clear(arr[arr[string]] tasks) -> arr[arr[string]] {
    dec arr[arr[string]] remaining = csv_filter_by(tasks, COL_STATUS, "pending")
    dec int removed = len(tasks) - len(remaining)
    csv_save(TASKS_FILE, remaining)
    println(format("cleared {} completed task(s)", removed))
    return remaining
}

fn cmd_stats(arr[arr[string]] tasks) {
    dec int total = len(tasks)
    dec int done = len(csv_filter_by(tasks, COL_STATUS, "done"))
    dec int pending = total - done
    println(format("total:   {}", total))
    println(format("done:    {}", done))
    println(format("pending: {}", pending))
}

fn main() {
    println("starting..")
    dec arr[arr[string]] tasks = csv_load(TASKS_FILE)
    println("loaded")
    print_startup_summary(tasks)

    while (true) {
        dec arr[string] parts = parse_command(trim(read("> ")?))
        dec string cmd = parts[0]
        dec string args = parts[1]

        match cmd {
            "quit" => {
                break
            }

            "add" => {
                if (is_empty(args)) {
                    println("usage: add <task text>")
                }
                else {
                    tasks = cmd_add(tasks, args)
                }
            }

            "done" => {
                if (is_empty(args)) {
                    println("usage: done <id>")
                }
                else {
                    tasks = cmd_done(tasks, args)
                }
            }

            "remove" => {
                if (is_empty(args)) {
                    println("usage: remove <id>")
                }
                else {
                    tasks = cmd_remove(tasks, args)
                }
            }

            "list" => {
                cmd_list(tasks, args)
            }

            "clear" => {
                tasks = cmd_clear(tasks)
            }

            "stats" => {
                cmd_stats(tasks)
            }

            "help" => {
                cmd_help()
            }

            _ => {
                println(format("unknown command: '{}'. type 'help' for commands", cmd))
            }
        }
    }

    println("goodbye")
}
