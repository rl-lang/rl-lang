use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
                "get time_now, format_date_str, format_time_str, format_time from std::time\n\ndec int ts = time_now()\nprintln(format_date_str(ts)?)          // 2026-06-20\nprintln(format_time_str(ts)?)          // 14:32:07\nprintln(format_time(ts, \"%d/%m/%Y\")?)  // 20/06/2026",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "time_diff gives you the number of seconds between two timestamps: time_diff(a, b) is a - b. put the more recent timestamp first to get a positive age",
            examples: &[
                "get time_now, time_diff from std::time\nget format              from std::str\n\ndec int created = 1750000000\ndec int now     = time_now()\ndec int age     = time_diff(now, created)\n\nprintln(format(\"{} seconds old\", age))",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "time_parts breaks a timestamp into its components as an array: [year, month, day, hour, minute, second]",
            examples: &[
                "get time_now, time_parts from std::time\n\ndec arr[int] parts = time_parts(time_now())?\nprintln(parts[0]) // year  e.g. 2026\nprintln(parts[1]) // month e.g. 6\nprintln(parts[2]) // day   e.g. 20",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: write a function format_age(int created_at) -> string that returns a human readable age string. use time_diff and some arithmetic\n\nexpected output examples:\n  just now\n  5 minutes ago\n  3 hours ago\n  2 days ago",
            examples: &[
                "get time_now, time_diff from std::time\nget format              from std::str\n\nfn format_age(int created_at) -> string {\n    dec int diff    = time_diff(time_now(), created_at)\n    dec int minutes = diff / 60\n    dec int hours   = minutes / 60\n    dec int days    = hours / 24\n\n    if (diff < 60)     { return \"just now\" }\n    if (minutes < 60)  { return format(\"{} minutes ago\", minutes) }\n    if (hours < 24)    { return format(\"{} hours ago\", hours) }\n    return format(\"{} days ago\", days)\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
