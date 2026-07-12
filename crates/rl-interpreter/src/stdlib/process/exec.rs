use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};
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

pub fn std_exec(eval: &mut Evaluator, cmd: String, span: Span) -> Result<Value, Error> {
    let output = shell_command(&cmd)
        .output()
        .map_err(|e| eval.err(format!("exec(): failed to run \"{}\": {}", cmd, e), span))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(Value::String(stdout.trim_end_matches('\n').to_string()))
}

pub fn std_exec_code(eval: &mut Evaluator, cmd: String, span: Span) -> Result<Value, Error> {
    let status = shell_command(&cmd).status().map_err(|e| {
        eval.err(
            format!("exec_code(): failed to run \"{}\": {}", cmd, e),
            span,
        )
    })?;
    Ok(Value::Integer(status.code().unwrap_or(-1) as i64))
}

pub fn std_exec_lines(eval: &mut Evaluator, cmd: String, span: Span) -> Result<Value, Error> {
    let output = shell_command(&cmd).output().map_err(|e| {
        eval.err(
            format!("exec_lines(): failed to run \"{}\": {}", cmd, e),
            span,
        )
    })?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<Value> = stdout
        .lines()
        .map(|l| Value::String(l.to_string()))
        .collect();
    Ok(Value::Values {
        items_type: TypeAnnotation::String,
        items: lines,
    })
}
