use crate::lexer::tokentypes::{Token, TokenType, Trivia};

pub fn format_tokens(tokens: &[Token]) -> String {
    let mut out = String::new();
    let mut indent: usize = 0;
    let mut at_line_start = true;
    let mut prev: Option<&TokenType> = None;

    for tok in tokens {
        for t in &tok.leading_trivia {
            match t {
                Trivia::LineComment(c) => {
                    out.push_str(&"    ".repeat(indent));
                    out.push_str("// ");
                    out.push_str(c);
                    out.push('\n');
                }
                Trivia::DocComment(c) => {
                    out.push_str(&"    ".repeat(indent));
                    out.push_str("/// ");
                    out.push_str(c);
                    out.push('\n');
                }
                Trivia::BlockComment(c) => {
                    out.push_str(&"    ".repeat(indent));
                    out.push_str("/* ");
                    out.push_str(c);
                    out.push_str(" */\n");
                }
                Trivia::BlankLine => out.push('\n'),
            }
            at_line_start = true;
        }

        match &tok.token {
            TokenType::Eof => {}
            TokenType::Newline => {
                if !at_line_start {
                    out.push('\n');
                    at_line_start = true;
                } else if !out.ends_with("\n\n") {
                    out.push('\n');
                }
                prev = Some(&tok.token);
            }
            TokenType::RightBrace => {
                indent = indent.saturating_sub(1);
                if at_line_start {
                    out.push_str(&"    ".repeat(indent));
                } else {
                    out.push('\n');
                    out.push_str(&"    ".repeat(indent));
                }
                out.push('}');
                at_line_start = false;
                prev = Some(&tok.token);
            }
            TokenType::LeftBrace => {
                out.push_str(" {");
                indent += 1;
                at_line_start = false;
                prev = Some(&tok.token);
            }
            _ => {
                if at_line_start {
                    out.push_str(&"    ".repeat(indent));
                } else if let Some(p) = prev
                    && needs_space(p, &tok.token) {
                        out.push(' ');
                    }
                out.push_str(&tok.lexeme);
                at_line_start = false;
                prev = Some(&tok.token);
            }
        }

        for t in &tok.trailing_trivia {
            if let Trivia::LineComment(c) = t {
                out.push_str(" // ");
                out.push_str(c);
            }
        }
    }
    out
}

/// Decides whether a space belongs between two adjacent real tokens.
fn needs_space(prev: &TokenType, curr: &TokenType) -> bool {
    use TokenType::*;

    // never a space right after these "opening"/tight tokens
    let no_space_after = matches!(
        prev,
        LeftParen | LeftBracket | Dot | DotDot | ColonColon | Hash | Bang | BangHash
    );
    // never a space right before these "closing"/tight tokens
    let no_space_before = matches!(
        curr,
        RightParen
            | RightBracket
            | Dot
            | DotDot
            | ColonColon
            | Comma
            | Colon
            | Semicolon
            | Question
    );

    if no_space_after || no_space_before {
        return false;
    }

    // function/index calls: identifier immediately followed by ( or [
    if matches!(curr, LeftParen | LeftBracket)
        && matches!(prev, Identifier(_) | RightParen | RightBracket) {
            return false;
        }

    true
}
