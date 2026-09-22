use crate::entry::{FnEntry, StdEntry};

mod parse_args;
mod parse_args_or_exit;
mod prompt;
mod prompt_choice;
mod prompt_confirm;
mod prompt_password;
mod progress_bar;
mod read_line_editable;
mod read_line_with_history;
mod shell_join;
mod shell_split;
mod spinner_tick;
mod usage_string;

pub static CLI: StdEntry = StdEntry {
    name: "cli",
    description: "functions for building command-line interfaces: arg parsing, prompts, editable input, progress bars",
    functions: FUNCTIONS,
    since: Some("v2.2.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &parse_args::PARSE_ARGS,
    &parse_args_or_exit::PARSE_ARGS_OR_EXIT,
    &usage_string::USAGE_STRING,
    &prompt::PROMPT,
    &prompt_password::PROMPT_PASSWORD,
    &prompt_confirm::PROMPT_CONFIRM,
    &prompt_choice::PROMPT_CHOICE,
    &shell_split::SHELL_SPLIT,
    &shell_join::SHELL_JOIN,
    &read_line_editable::READ_LINE_EDITABLE,
    &read_line_with_history::READ_LINE_WITH_HISTORY,
    &progress_bar::PROGRESS_BAR,
    &spinner_tick::SPINNER_TICK,
];
