use crate::entry::{FnEntry, StdEntry};

mod args;
mod arch;
mod cwd;
mod env;
mod env_keys;
mod exec;
mod exec_background;
mod exec_code;
mod exec_fg;
mod exec_lines;
mod exec_with_cwd;
mod exec_with_env;
mod exec_with_stdin;
mod exec_with_timeout;
mod exit;
mod kill_pid;
mod num_cpus;
mod os_name;
mod parent_pid;
mod pipe;
mod pipe_all;
mod pid;
mod process_exists;
mod remove_env;
mod set_cwd;
mod set_env;
mod sleep;
mod term_pid;
mod wait_pid;
mod with_exec;
mod with_exec_background;
mod with_exec_code;
mod with_exec_fg;
mod with_exec_lines;
mod with_exec_with_cwd;
mod with_exec_with_env;
mod with_exec_with_stdin;

pub static PROCESS: StdEntry = StdEntry {
    name: "process",
    description: "functions for interacting with the current process and running shell commands",
    functions: FUNCTIONS,
    since: Some("v2.1.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &arch::ARCH,
    &args::ARGS,
    &cwd::CWD,
    &env::ENV,
    &env_keys::ENV_KEYS,
    &exec::EXEC,
    &exec_background::EXEC_BACKGROUND,
    &exec_code::EXEC_CODE,
    &exec_fg::EXEC_FG,
    &exec_lines::EXEC_LINES,
    &exec_with_cwd::EXEC_WITH_CWD,
    &exec_with_env::EXEC_WITH_ENV,
    &exec_with_stdin::EXEC_WITH_STDIN,
    &exec_with_timeout::EXEC_WITH_TIMEOUT,
    &exit::EXIT,
    &kill_pid::KILL_PID,
    &num_cpus::NUM_CPUS,
    &os_name::OS_NAME,
    &parent_pid::PARENT_PID,
    &pid::PID,
    &pipe::PIPE,
    &pipe_all::PIPE_ALL,
    &process_exists::PROCESS_EXISTS,
    &remove_env::REMOVE_ENV,
    &set_cwd::SET_CWD,
    &set_env::SET_ENV,
    &sleep::SLEEP,
    &term_pid::TERM_PID,
    &wait_pid::WAIT_PID,
    &with_exec::WITH_EXEC,
    &with_exec_background::WITH_EXEC_BACKGROUND,
    &with_exec_code::WITH_EXEC_CODE,
    &with_exec_fg::WITH_EXEC_FG,
    &with_exec_lines::WITH_EXEC_LINES,
    &with_exec_with_cwd::WITH_EXEC_WITH_CWD,
    &with_exec_with_env::WITH_EXEC_WITH_ENV,
    &with_exec_with_stdin::WITH_EXEC_WITH_STDIN,
];
