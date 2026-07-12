use crate::entry::{FnEntry, StdEntry};

pub static PROCESS: StdEntry = StdEntry {
    name: "process",
    description: "functions for interacting with the current process and running shell commands",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &ARGS,
    &CWD,
    &ENV,
    &EXEC,
    &EXEC_CODE,
    &EXEC_LINES,
    &EXIT,
    &PID,
    &SET_CWD,
    &SLEEP,
];

static ARGS: FnEntry = FnEntry {
    signature: "args()",
    description: "returns the command-line arguments passed to the script as an array of strings",
    example: "get std::process::args\n\ndec string[] a = args()\nprintln(a) // [\"--verbose\", \"input.txt\"]",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static CWD: FnEntry = FnEntry {
    signature: "cwd()",
    description: "returns the current working directory as a string",
    example: "get std::process::cwd\n\nprintln(cwd()) // \"/home/crimson/project\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static SET_CWD: FnEntry = FnEntry {
    signature: "set_cwd(path)",
    description: "changes the current working directory",
    example: "get std::process::set_cwd\n\nset_cwd(\"/tmp\")",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static ENV: FnEntry = FnEntry {
    signature: "env(key)",
    description: "returns the value of an environment variable, or null if not set",
    example: "get std::process::env\n\nprintln(env(\"HOME\")) // \"/home/crimson\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static EXIT: FnEntry = FnEntry {
    signature: "exit(code)",
    description: "terminates the process with the given exit code",
    example: "get std::process::exit\n\nexit(0)  // success\nexit(1)  // error",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PID: FnEntry = FnEntry {
    signature: "pid()",
    description: "returns the process ID of the current process",
    example: "get std::process::pid\n\nprintln(pid()) // 12345",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static SLEEP: FnEntry = FnEntry {
    signature: "sleep(ms)",
    description: "pauses the process for the given number of milliseconds",
    example: "get std::process::sleep\n\nsleep(1000) // wait 1 second",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static EXEC: FnEntry = FnEntry {
    signature: "exec(cmd)",
    description: "runs a shell command and returns its stdout as a trimmed string",
    example: "get std::process::exec\n\ndec string out = exec(\"echo hello\")\nprintln(out) // \"hello\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static EXEC_CODE: FnEntry = FnEntry {
    signature: "exec_code(cmd)",
    description: "runs a shell command and returns its exit code as an int",
    example: "get std::process::exec_code\n\ndec int code = exec_code(\"ls /nonexistent\")\nprintln(code) // 2",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static EXEC_LINES: FnEntry = FnEntry {
    signature: "exec_lines(cmd)",
    description: "runs a shell command and returns its stdout split into an array of lines",
    example: "get std::process::exec_lines\n\ndec string[] files = exec_lines(\"ls src\")\nprintln(files) // [\"main.rl\", \"lib.rl\"]",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
