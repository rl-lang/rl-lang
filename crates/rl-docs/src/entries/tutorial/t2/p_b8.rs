use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
                "// main.rl skeleton\nget csv\nget read, println from std::io\nget trim, concat   from std::str\n\nCONST string TASKS_FILE = \"tasks.csv\"\n\nfn main() {\n    dec arr[arr[string]] tasks = csv_load(TASKS_FILE)\n    println(\"task manager ready. type 'help' for commands\")\n\n    while (true) {\n        dec string input   = read(\"> \")?\n        dec string command = trim(input)\n\n        if (command == \"quit\") { break }\n\n        // dispatch commands here\n        println(concat(\"unknown command: \", command))\n    }\n\n    println(\"goodbye\")\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "commands like 'add buy milk' have two parts: the command name and the arguments. split on the first space to separate them. use index_of to find where the first space is, then slice to extract each part",
            examples: &[
                "get index_of, slice, trim from std::str\nget len                   from std::array\n\nfn parse_command(string input) -> arr[string] {\n    dec int space = index_of(input, \" \")\n    if (space == -1) {\n        return [trim(input), \"\"] // no args\n    }\n    dec string cmd  = slice(input, 0, space)?\n    dec string args = trim(slice(input, space + 1, len(input))?)\n    return [cmd, args]\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: get the REPL loop running. it should read input, parse the command, and for now just echo back what command and args it parsed. quit should exit cleanly\n\nexpected output:\n  > add buy milk\n  command: add  args: buy milk\n  > list\n  command: list  args:\n  > quit\n  goodbye",
            examples: &[
                "get read, println from std::io\nget format         from std::str\n\nfn main() {\n    println(\"task manager ready. type 'help' for commands\")\n\n    while (true) {\n        dec string       input  = read(\"> \")?\n        dec arr[string]  parts  = parse_command(trim(input))\n        dec string       cmd    = parts[0]\n        dec string       args   = parts[1]\n\n        if (cmd == \"quit\") { break }\n\n        println(format(\"command: {}  args: {}\", cmd, args))\n    }\n\n    println(\"goodbye\")\n}",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
