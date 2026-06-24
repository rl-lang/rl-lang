//! `std::process` - process management: args, env, cwd, exec, exit, pid, sleep.
//!
//! `exec` captures stdout and returns it as a string (trailing newline stripped).
//! `exec_code` returns only the exit code as `int`.
//! `exec_lines` returns stdout split into lines as `arr[string]`.
//! Both `args` and `cwd` use `with_raw_function` since they take no rl arguments.
//! `env` returns `null` (not an error) when the variable is not set.

mod args;
mod cwd;
mod env;
mod exec;
mod exit;
mod pid;
mod sleep;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "args",
    "exit",
    "env",
    "cwd",
    "set_cwd",
    "pid",
    "sleep",
    "exec",
    "exec_code",
    "exec_lines",
];

pub fn module() -> Module {
    Module::new("process")
        .with_raw_function("args", args::std_args)
        .with_function("exit", exit::std_exit)
        .with_function("env", env::std_env)
        .with_raw_function("cwd", cwd::std_cwd)
        .with_function("set_cwd", cwd::std_set_cwd)
        .with_raw_function("pid", pid::std_pid)
        .with_function("sleep", sleep::std_sleep)
        .with_function("exec", exec::std_exec)
        .with_function("exec_code", exec::std_exec_code)
        .with_function("exec_lines", exec::std_exec_lines)
}
