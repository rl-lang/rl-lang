//! `std::process` - process management: args, env, cwd, exec, exit, pid, sleep.
//!
//! `exec` captures stdout and returns it as a string (trailing newline
//! stripped). `exec_code` returns only the exit code as `int`. `exec_lines`
//! returns stdout split into lines as `arr[string]`. The `with_*` variants take
//! an explicit executable plus a shell-word-split argument string (via the
//! `shell-words` crate) instead of running through the platform shell. `env`
//! returns `null` (not an error) when the variable is not set, so it builds a
//! raw `R::Value`; it is registered `untyped` (matching the canonical
//! `rl-commons` signature, which lists it via `with_functions(&["env"])`).
//!
//! Background process functions (`exec_background`, `wait_pid`, `term_pid`,
//! `kill_pid`) allow spawning commands without blocking and managing them
//! afterward. `pipe` / `pipe_all` chain commands together like shell pipes.
//!
//! Ported once from the former per-runtime `stdlib/process/*.rs` copies.

#[cfg(feature = "impls")]
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use std::collections::HashMap;
#[cfg(feature = "impls")]
use std::process::{Child, Command, Stdio};
#[cfg(feature = "impls")]
use std::sync::{LazyLock, Mutex};
#[cfg(feature = "impls")]
use std::time::Duration;

// ---- args (no rl arguments, `array[string]`) ------------------------------

// NOTE: the original per-runtime `std_args` skipped `cx.user_args_offset`, a
// field on `Vm`/`Evaluator` that the CLI can override via
// `with_user_args_offset`. That field is NOT exposed through the `Runtime`
// trait, so this generic port cannot read it and falls back to the default
// offset of `1` (which matches both runtimes' default constructor). Preserving
// the configurable offset requires adding an accessor to `Runtime` (e.g.
// `fn user_args_offset(cx: &Self::Cx) -> usize`) and swapping the literal `1`
// below for `R::user_args_offset(cx)`.
#[native_fn(module = "process")]
pub fn args<R: Runtime>(cx: &mut R::Cx) -> Vec<String> {
    std::env::args().skip(R::user_args_offset(cx)).collect()
}

// ---- exit / pid / sleep ---------------------------------------------------

#[native_fn(module = "process")]
pub fn exit(code: i64) {
    std::process::exit(code as i32);
}

#[native_fn(module = "process")]
pub fn pid() -> i64 {
    std::process::id() as i64
}

#[native_fn(module = "process")]
pub fn sleep(ms: i64) {
    std::thread::sleep(Duration::from_millis(ms.max(0) as u64));
}

// ---- env (string-or-null, untyped, raw value) -----------------------------

#[native_fn(module = "process", untyped)]
pub fn env<R: Runtime>(key: String) -> R::Value {
    match std::env::var(&key) {
        Ok(val) => R::from_string(val),
        Err(_) => R::null(),
    }
}

// ---- set_env / remove_env / env_keys --------------------------------------

#[native_fn(module = "process")]
pub fn set_env(key: String, value: String) -> Result<(), String> {
    // SAFETY: set_var is unsafe in edition 2024 because it can cause data races
    // in multi-threaded programs. rl-lang's process functions are called from
    // a single VM thread, and the user is explicitly requesting env mutation.
    unsafe { std::env::set_var(&key, &value) };
    Ok(())
}

#[native_fn(module = "process")]
pub fn remove_env(key: String) -> Result<(), String> {
    // SAFETY: same reasoning as set_env above.
    unsafe { std::env::remove_var(&key) };
    Ok(())
}

#[native_fn(module = "process")]
pub fn env_keys() -> Vec<String> {
    std::env::vars().map(|(k, _)| k).collect()
}

// ---- cwd / set_cwd (language `result`) ------------------------------------

#[native_fn(module = "process")]
pub fn cwd() -> Result<String, String> {
    match std::env::current_dir() {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => Err(format!("cwd: {}", e)),
    }
}

#[native_fn(module = "process")]
pub fn set_cwd(path: String) -> Result<(), String> {
    match std::env::set_current_dir(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("set_cwd: failed to change to \"{}\": {}", path, e)),
    }
}

// ---- os_name / arch / num_cpus --------------------------------------------

#[native_fn(module = "process")]
pub fn os_name() -> String {
    std::env::consts::OS.to_string()
}

#[native_fn(module = "process")]
pub fn arch() -> String {
    std::env::consts::ARCH.to_string()
}

#[native_fn(module = "process")]
pub fn num_cpus() -> i64 {
    std::thread::available_parallelism()
        .map(|n| n.get() as i64)
        .unwrap_or(1)
}

// ---- parent_pid / process_exists ------------------------------------------

#[native_fn(module = "process")]
pub fn parent_pid() -> i64 {
    #[cfg(unix)]
    {
        unsafe { libc::getppid() as i64 }
    }
    #[cfg(not(unix))]
    {
        0
    }
}

#[native_fn(module = "process")]
pub fn process_exists(pid: i64) -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        false
    }
}

#[native_fn(module = "process")]
pub fn process_running(pid: i64) -> bool {
    let p = pid as u32;
    let mut map = SPAWNED.lock().unwrap();
    match map.get_mut(&p) {
        Some(child) => match child.try_wait() {
            Ok(None) => true,
            Ok(Some(_)) => false,
            Err(_) => false,
        },
        None => false,
    }
}

// ---- shell helpers --------------------------------------------------------

#[cfg(feature = "impls")]
#[cfg(target_os = "windows")]
fn shell_command(cmd: &str) -> Command {
    let mut c = Command::new("cmd");
    c.args(["/C", cmd]);
    c
}

#[cfg(feature = "impls")]
#[cfg(not(target_os = "windows"))]
fn shell_command(cmd: &str) -> Command {
    let mut c = Command::new("sh");
    c.args(["-c", cmd]);
    c
}

#[cfg(feature = "impls")]
fn with_command(e: &str, cmd: &str) -> Result<Command, shell_words::ParseError> {
    let args = shell_words::split(cmd)?;
    let mut c = Command::new(e);
    c.args(args);
    Ok(c)
}

// ---- exec (result[string]) ------------------------------------------------

#[native_fn(module = "process")]
pub fn exec(cmd: String) -> Result<String, String> {
    let output = match shell_command(&cmd).output() {
        Ok(o) => o,
        Err(e) => return Err(format!("exec: failed to run \"{}\": {}", cmd, e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

#[native_fn(module = "process")]
pub fn with_exec(e: String, cmd: String) -> Result<String, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => return Err(format!("with_exec: invalid args \"{}\": {}", cmd, err)),
    };
    let output = match command.output() {
        Ok(o) => o,
        Err(e) => return Err(format!("with_exec: failed to run \"{}\": {}", cmd, e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

// ---- exec_fg (result[int]) ------------------------------------------------
// Runs a command with inherited stdin/stdout/stderr (foreground).
// Useful for launching interactive programs (editors, pagers, TUIs) that need
// direct terminal access. The caller should leave raw mode / alternate screen
// before calling this and re-enter afterward.

#[native_fn(module = "process")]
pub fn exec_fg(cmd: String) -> Result<i64, String> {
    let status = match shell_command(&cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(s) => s,
        Err(e) => return Err(format!("exec_fg: failed to run \"{}\": {}", cmd, e)),
    };
    Ok(status.code().unwrap_or(-1) as i64)
}

#[native_fn(module = "process")]
pub fn with_exec_fg(e: String, cmd: String) -> Result<i64, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => return Err(format!("with_exec_fg: invalid args \"{}\": {}", cmd, err)),
    };
    let status = match command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(s) => s,
        Err(e) => return Err(format!("with_exec_fg: failed: {}", e)),
    };
    Ok(status.code().unwrap_or(-1) as i64)
}

// ---- exec_code (result[int]) ----------------------------------------------

#[native_fn(module = "process")]
pub fn exec_code(cmd: String) -> Result<i64, String> {
    let status = match shell_command(&cmd).status() {
        Ok(s) => s,
        Err(e) => return Err(format!("exec_code: failed to run \"{}\": {}", cmd, e)),
    };
    Ok(status.code().unwrap_or(-1) as i64)
}

#[native_fn(module = "process")]
pub fn with_exec_code(e: String, cmd: String) -> Result<i64, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => return Err(format!("with_exec_code: invalid args \"{}\": {}", cmd, err)),
    };
    let status = match command.status() {
        Ok(s) => s,
        Err(e) => return Err(format!("with_exec_code: failed to run \"{}\": {}", cmd, e)),
    };
    Ok(status.code().unwrap_or(-1) as i64)
}

// ---- exec_lines (result[array[string]]) -----------------------------------

#[native_fn(module = "process")]
pub fn exec_lines(cmd: String) -> Result<Vec<String>, String> {
    let output = match shell_command(&cmd).output() {
        Ok(o) => o,
        Err(e) => return Err(format!("exec_lines: failed to run \"{}\": {}", cmd, e)),
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<String> = stdout.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}

#[native_fn(module = "process")]
pub fn with_exec_lines(e: String, cmd: String) -> Result<Vec<String>, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => {
            return Err(format!(
                "with_exec_lines: invalid args \"{}\": {}",
                cmd, err
            ));
        }
    };
    let output = match command.output() {
        Ok(o) => o,
        Err(e) => return Err(format!("with_exec_lines: failed to run \"{}\": {}", cmd, e)),
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<String> = stdout.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}

// ---- exec_with_stdin (result[string]) -------------------------------------

#[native_fn(module = "process")]
pub fn exec_with_stdin(cmd: String, input: String) -> Result<String, String> {
    let mut child = match shell_command(&cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return Err(format!("exec_with_stdin: failed to run \"{}\": {}", cmd, e)),
    };
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(input.as_bytes());
    }
    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => return Err(format!("exec_with_stdin: failed: {}", e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

#[native_fn(module = "process")]
pub fn with_exec_with_stdin(e: String, cmd: String, input: String) -> Result<String, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => {
            return Err(format!(
                "with_exec_with_stdin: invalid args \"{}\": {}",
                cmd, err
            ));
        }
    };
    let mut child = match command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return Err(format!("with_exec_with_stdin: failed: {}", e)),
    };
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(input.as_bytes());
    }
    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => return Err(format!("with_exec_with_stdin: failed: {}", e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

// ---- exec_with_env (result[string]) ---------------------------------------

#[native_fn(module = "process")]
pub fn exec_with_env(cmd: String, envs: Vec<Vec<String>>) -> Result<String, String> {
    let mut command = shell_command(&cmd);
    for pair in &envs {
        if pair.len() == 2 {
            command.env(&pair[0], &pair[1]);
        }
    }
    let output = match command.output() {
        Ok(o) => o,
        Err(e) => return Err(format!("exec_with_env: failed to run \"{}\": {}", cmd, e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

#[native_fn(module = "process")]
pub fn with_exec_with_env(
    e: String,
    cmd: String,
    envs: Vec<Vec<String>>,
) -> Result<String, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => {
            return Err(format!(
                "with_exec_with_env: invalid args \"{}\": {}",
                cmd, err
            ));
        }
    };
    for pair in &envs {
        if pair.len() == 2 {
            command.env(&pair[0], &pair[1]);
        }
    }
    let output = match command.output() {
        Ok(o) => o,
        Err(e) => return Err(format!("with_exec_with_env: failed: {}", e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

// ---- exec_with_cwd (result[string]) ---------------------------------------

#[native_fn(module = "process")]
pub fn exec_with_cwd(cmd: String, dir: String) -> Result<String, String> {
    let mut command = shell_command(&cmd);
    command.current_dir(&dir);
    let output = match command.output() {
        Ok(o) => o,
        Err(e) => {
            return Err(format!(
                "exec_with_cwd: failed to run \"{}\" in \"{}\": {}",
                cmd, dir, e
            ));
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

#[native_fn(module = "process")]
pub fn with_exec_with_cwd(e: String, cmd: String, dir: String) -> Result<String, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => {
            return Err(format!(
                "with_exec_with_cwd: invalid args \"{}\": {}",
                cmd, err
            ));
        }
    };
    command.current_dir(&dir);
    let output = match command.output() {
        Ok(o) => o,
        Err(e) => return Err(format!("with_exec_with_cwd: failed: {}", e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

// ---- exec_with_timeout (result[string]) -----------------------------------

#[native_fn(module = "process")]
pub fn exec_with_timeout(cmd: String, timeout_ms: i64) -> Result<String, String> {
    let mut child = match shell_command(&cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(format!(
                "exec_with_timeout: failed to run \"{}\": {}",
                cmd, e
            ));
        }
    };

    let timeout = Duration::from_millis(timeout_ms.max(0) as u64);
    let start = std::time::Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = match child.wait_with_output() {
                    Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                    Err(_) => String::new(),
                };
                if status.success() {
                    return Ok(stdout.trim_end_matches('\n').to_string());
                } else {
                    return Err(format!(
                        "exec_with_timeout: \"{}\" exited with code {}",
                        cmd,
                        status.code().unwrap_or(-1)
                    ));
                }
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!(
                        "exec_with_timeout: \"{}\" timed out after {}ms",
                        cmd, timeout_ms
                    ));
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => return Err(format!("exec_with_timeout: {}", e)),
        }
    }
}

// ---- background process management ----------------------------------------

#[cfg(feature = "impls")]
static SPAWNED: LazyLock<Mutex<HashMap<u32, Child>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

// ---- exec_background (result[int]) ----------------------------------------

#[native_fn(module = "process")]
pub fn exec_background(cmd: String) -> Result<i64, String> {
    let child = match shell_command(&cmd).spawn() {
        Ok(c) => c,
        Err(e) => {
            return Err(format!(
                "exec_background: failed to spawn \"{}\": {}",
                cmd, e
            ));
        }
    };
    let pid = child.id();
    SPAWNED.lock().unwrap().insert(pid, child);
    Ok(pid as i64)
}

#[native_fn(module = "process")]
pub fn with_exec_background(e: String, cmd: String) -> Result<i64, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => {
            return Err(format!(
                "with_exec_background: invalid args \"{}\": {}",
                cmd, err
            ));
        }
    };
    let child = match command.spawn() {
        Ok(c) => c,
        Err(e) => return Err(format!("with_exec_background: failed: {}", e)),
    };
    let pid = child.id();
    SPAWNED.lock().unwrap().insert(pid, child);
    Ok(pid as i64)
}

// ---- wait_pid (result[int]) -----------------------------------------------

#[native_fn(module = "process")]
pub fn wait_pid(pid: i64) -> Result<i64, String> {
    let p = pid as u32;
    let mut child = {
        let mut map = SPAWNED.lock().unwrap();
        match map.remove(&p) {
            Some(c) => c,
            None => return Err(format!("wait_pid: no tracked process with pid {}", pid)),
        }
    };
    match child.wait() {
        Ok(status) => Ok(status.code().unwrap_or(-1) as i64),
        Err(e) => Err(format!("wait_pid: failed waiting for pid {}: {}", pid, e)),
    }
}

// ---- term_pid / kill_pid (result[null]) -----------------------------------

#[native_fn(module = "process")]
pub fn term_pid(pid: i64) -> Result<(), String> {
    #[cfg(unix)]
    {
        let ret = unsafe { libc::kill(pid as i32, libc::SIGTERM) };
        if ret == 0 {
            Ok(())
        } else {
            Err(format!("term_pid: failed to send SIGTERM to pid {}", pid))
        }
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        Err("term_pid: not supported on this platform".to_string())
    }
}

#[native_fn(module = "process")]
pub fn kill_pid(pid: i64) -> Result<(), String> {
    #[cfg(unix)]
    {
        let ret = unsafe { libc::kill(pid as i32, libc::SIGKILL) };
        if ret == 0 {
            Ok(())
        } else {
            Err(format!("kill_pid: failed to send SIGKILL to pid {}", pid))
        }
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        Err("kill_pid: not supported on this platform".to_string())
    }
}

// ---- pipe / pipe_all (result[string]) -------------------------------------

#[native_fn(module = "process")]
pub fn pipe(cmd1: String, cmd2: String) -> Result<String, String> {
    let mut c1 = shell_command(&cmd1);
    c1.stdout(Stdio::piped());
    c1.stderr(Stdio::inherit());

    let mut child1 = match c1.spawn() {
        Ok(c) => c,
        Err(e) => return Err(format!("pipe: failed to spawn \"{}\": {}", cmd1, e)),
    };

    let stdout1 = match child1.stdout.take() {
        Some(s) => s,
        None => return Err(format!("pipe: failed to capture stdout of \"{}\"", cmd1)),
    };

    let mut c2 = shell_command(&cmd2);
    c2.stdin(stdout1);
    c2.stdout(Stdio::piped());
    c2.stderr(Stdio::inherit());

    let child2 = match c2.spawn() {
        Ok(c) => c,
        Err(e) => {
            let _ = child1.wait();
            return Err(format!("pipe: failed to spawn \"{}\": {}", cmd2, e));
        }
    };

    let output = match child2.wait_with_output() {
        Ok(o) => o,
        Err(e) => return Err(format!("pipe: failed: {}", e)),
    };

    let _ = child1.wait();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

#[native_fn(module = "process")]
pub fn pipe_all(cmds: Vec<String>) -> Result<String, String> {
    if cmds.is_empty() {
        return Err("pipe_all: command list is empty".to_string());
    }
    if cmds.len() == 1 {
        return exec(cmds.into_iter().next().unwrap());
    }

    let mut prev_stdout: Option<std::process::ChildStdout> = None;
    let mut children: Vec<Child> = Vec::new();

    for (i, cmd) in cmds.iter().enumerate() {
        let mut c = shell_command(cmd);
        c.stderr(Stdio::inherit());

        if let Some(stdout) = prev_stdout.take() {
            c.stdin(stdout);
        }

        c.stdout(Stdio::piped());

        let mut child = match c.spawn() {
            Ok(c) => c,
            Err(e) => {
                for mut ch in children {
                    let _ = ch.kill();
                    let _ = ch.wait();
                }
                return Err(format!("pipe_all: failed to spawn \"{}\": {}", cmd, e));
            }
        };

        if i < cmds.len() - 1 {
            prev_stdout = child.stdout.take();
        }

        children.push(child);
    }

    let last = children.pop().unwrap();
    let output = match last.wait_with_output() {
        Ok(o) => o,
        Err(e) => return Err(format!("pipe_all: failed: {}", e)),
    };

    for mut child in children {
        let _ = child.wait();
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

// ---- module registration --------------------------------------------------

rl_std_core::native_module!("process";
    funcs: [
        args,
        exit, pid, sleep,
        env, set_env, remove_env, env_keys,
        cwd, set_cwd,
        os_name, arch, num_cpus,
        parent_pid, process_exists, process_running,
        exec, with_exec,
        exec_fg, with_exec_fg,
        exec_code, with_exec_code,
        exec_lines, with_exec_lines,
        exec_with_stdin, with_exec_with_stdin,
        exec_with_env, with_exec_with_env,
        exec_with_cwd, with_exec_with_cwd,
        exec_with_timeout,
        exec_background, with_exec_background,
        wait_pid, term_pid, kill_pid,
        pipe, pipe_all,
    ],
);
