use std::{fs, path::PathBuf};

use ratatui::style::{Color, Modifier, Style};

use crate::{
    interpreter::evaluator::Evaluator,
    lexer::tokenizer::Tokenizer,
    parser::parser_logic::Parser,
    repl::{lines_types::OutputLine, stdlib_helper::stdlib_entries, utils::push_error},
    utils::source::SourceFile,
};

pub fn handle_command(
    cmd: &str,
    output: &mut Vec<OutputLine>,
    evaluator: &mut Evaluator,
    attached: &mut Vec<PathBuf>,
) {
    let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
    match parts[0] {
        ":help" => {
            let cmd = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
            let arg = Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::ITALIC);
            let sep = Style::default().fg(Color::DarkGray);
            let desc = Style::default().fg(Color::White);

            output.push(OutputLine::Styled(vec![(
                "Commands".to_string(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )]));
            let entries: &[(&str, Option<&str>, &str)] = &[
                (":help", None, "show this"),
                (":stdlib", None, "list stdlib modules"),
                (":stdlib", Some(" <mod>"), "list module functions"),
                (":save", Some(" <file>"), "save session to file"),
                (":load", Some(" <file>"), "load and print file"),
                (":attach", Some(" <file>"), "import file into env"),
                (":detach", Some(" <file>"), "remove attached file"),
                (":exit", None, "quit  (ctrl+c also works)"),
            ];
            for (command, argument, description) in entries {
                let mut parts = vec![("  ".to_string(), sep), (command.to_string(), cmd)];
                if let Some(a) = argument {
                    parts.push((a.to_string(), arg));
                }
                // pad to column 24
                let used = 2 + command.len() + argument.map(|a| a.len()).unwrap_or(0);
                let pad = " ".repeat(24_usize.saturating_sub(used));
                parts.push((pad, sep));
                parts.push(("— ".to_string(), sep));
                parts.push((description.to_string(), desc));
                output.push(OutputLine::Styled(parts));
            }
        }
        ":stdlib" => {
            let header = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
            let modname = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
            let sig = Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::ITALIC);
            let sep = Style::default().fg(Color::DarkGray);
            let desc = Style::default().fg(Color::White);

            if parts.len() == 1 {
                output.push(OutputLine::Styled(vec![(
                    "stdlib modules".to_string(),
                    header,
                )]));
                for entry in stdlib_entries() {
                    output.push(OutputLine::Styled(vec![
                        ("  std".to_string(), sep),
                        ("::".to_string(), sep),
                        (entry.name.to_string(), modname),
                    ]));
                }
            } else {
                let mod_name = parts[1].trim();
                let entries = stdlib_entries();
                if let Some(entry) = entries.iter().find(|e| e.name == mod_name) {
                    output.push(OutputLine::Styled(vec![
                        ("std".to_string(), sep),
                        ("::".to_string(), sep),
                        (entry.name.to_string(), modname),
                    ]));
                    for (signature, description) in entry.functions {
                        let pad = " ".repeat(32_usize.saturating_sub(signature.len()));
                        output.push(OutputLine::Styled(vec![
                            ("  ".to_string(), sep),
                            (signature.to_string(), sig),
                            (pad, sep),
                            (description.to_string(), desc),
                        ]));
                    }
                } else {
                    output.push(OutputLine::Error(format!("unknown module: {}", mod_name)));
                }
            }
        }
        ":save" => {
            if parts.len() < 2 || parts[1].trim().is_empty() {
                output.push(OutputLine::Error(":save requires a filename".into()));
                return;
            }
            let path = parts[1].trim();
            let content: String = output
                .iter()
                .filter_map(|l| match l {
                    OutputLine::Input(s) if !s.trim_start().starts_with(':') => Some(s.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            match fs::write(path, content) {
                Ok(_) => output.push(OutputLine::Info(format!("saved to {}", path))),
                Err(e) => output.push(OutputLine::Error(format!("save failed: {}", e))),
            }
        }

        ":load" => {
            if parts.len() < 2 || parts[1].trim().is_empty() {
                output.push(OutputLine::Error(":load requires a filename".into()));
                return;
            }
            let path = parts[1].trim();
            match fs::read_to_string(path) {
                Ok(content) => {
                    output.push(OutputLine::Info(format!("--- {} ---", path)));
                    for line in content.lines() {
                        output.push(OutputLine::Info(line.to_string()));
                    }
                }
                Err(e) => output.push(OutputLine::Error(format!("load failed: {}", e))),
            }
        }

        ":attach" => {
            if parts.len() < 2 || parts[1].trim().is_empty() {
                output.push(OutputLine::Error(":attach requires a filename".into()));
                return;
            }
            let path = PathBuf::from(parts[1].trim());
            match fs::read_to_string(&path) {
                Ok(content) => {
                    let source =
                        SourceFile::new(path.to_str().unwrap_or("<file>"), content.clone());
                    let tokens = match Tokenizer::lex(source.clone()) {
                        Ok(t) => t,
                        Err(e) => {
                            push_error(output, &e);
                            return;
                        }
                    };
                    let stmts = match Parser::parse(tokens, source.clone()) {
                        Ok(s) => s,
                        Err(e) => {
                            push_error(output, &e);
                            return;
                        }
                    };
                    evaluator.set_source_file(source);
                    let mut ok = true;
                    for stmt in &stmts {
                        if let Err(e) = evaluator.evaluate_statement(stmt) {
                            push_error(output, &e);
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        attached.push(path.clone());
                        output.push(OutputLine::Info(format!("attached {}", path.display())));
                    }
                }
                Err(e) => output.push(OutputLine::Error(format!("attach failed: {}", e))),
            }
        }
        ":detach" => {
            if parts.len() < 2 || parts[1].trim().is_empty() {
                output.push(OutputLine::Error(":detach requires a filename".into()));
                return;
            }
            let path = PathBuf::from(parts[1].trim());
            if let Some(pos) = attached.iter().position(|p| p == &path) {
                attached.remove(pos);
                output.push(OutputLine::Info(format!("detached {}", path.display())));
                output.push(OutputLine::Info(
                    "note: variables from that file remain in env until restart".into(),
                ));
            } else {
                output.push(OutputLine::Error(format!(
                    "{} is not attached",
                    path.display()
                )));
            }
        }

        _ => {
            output.push(OutputLine::Error(format!("unknown command: {}", parts[0])));
            output.push(OutputLine::Info("type :help for commands".into()));
        }
    }
}
