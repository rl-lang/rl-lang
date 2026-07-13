use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vi, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;
use std::process::Command;

#[cfg(target_os = "windows")]
fn shell_command(cmd: &str) -> Command {
    let mut c = Command::new("cmd");
    c.args(["/C", cmd]);
    c
}

#[cfg(not(target_os = "windows"))]
fn shell_command(cmd: &str) -> Command {
    let mut c = Command::new("sh");
    c.args(["-c", cmd]);
    c
}

pub fn std_exec(_: &mut Evaluator, cmd: String) -> Value {
    let output = match shell_command(&cmd).output() {
        Ok(o) => o,
        Err(e) => return verr!(vs!(format!("exec: failed to run \"{}\": {}", cmd, e))),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    vok!(vs!(stdout.trim_end_matches('\n').to_string()))
}

pub fn std_exec_code(_: &mut Evaluator, cmd: String) -> Value {
    let status = match shell_command(&cmd).status() {
        Ok(s) => s,
        Err(e) => {
            return verr!(vs!(format!("exec_code: failed to run \"{}\": {}", cmd, e)));
        }
    };
    vok!(vi!(status.code().unwrap_or(-1) as i64))
}

pub fn std_exec_lines(_: &mut Evaluator, cmd: String) -> Value {
    let output = match shell_command(&cmd).output() {
        Ok(o) => o,
        Err(e) => {
            return verr!(vs!(format!("exec_lines: failed to run \"{}\": {}", cmd, e)));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<Value> = stdout
        .lines()
        .map(|l| Value::String(l.to_string()))
        .collect();

    vok!(Value::Values {
        items_type: TypeAnnotation::String,
        items: lines,
    })
}
