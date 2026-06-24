//! Token-based syntax highlighting for the REPL input bar and output.
//!
//! Lexes the input with the real rl lexer and maps each [`TokenType`] to a
//! ratatui [`Style`]. Gaps between token spans are emitted as unstyled raw spans
//! to preserve whitespace exactly. On lex failure the entire input is returned
//! as a single red span.
//!
//! # Color scheme
//!
//! | Token group         | Color / modifier         |
//! |---------------------|--------------------------|
//! | Control flow        | Cyan bold                |
//! | Declarations        | Cyan italic              |
//! | Import keywords     | Cyan dim                 |
//! | Type keywords       | Light blue italic        |
//! | Logical operators   | Yellow bold              |
//! | Number literals     | Magenta                  |
//! | String literals     | Yellow                   |
//! | Char literals       | Light yellow             |
//! | Bool literals       | Magenta italic           |
//! | `null`              | Dark gray italic         |
//! | Comparison ops      | Light cyan               |
//! | Punctuation/braces  | Dark gray                |
//! | Identifiers         | White                    |

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::{
    lexer::{tokenizer::Tokenizer, tokentypes::TokenType},
    utils::source::SourceFile,
};

/// Returns the ratatui [`Style`] for a given [`TokenType`].
fn token_color(tt: &TokenType) -> Style {
    match tt {
        // control flow - cyan bold
        TokenType::If
        | TokenType::Else
        | TokenType::While
        | TokenType::For
        | TokenType::Break
        | TokenType::Continue
        | TokenType::Return => Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),

        // declarations cyan italic
        TokenType::Dec | TokenType::Const | TokenType::Fn | TokenType::Array => Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::ITALIC),

        // import keywords cyan dim
        TokenType::Get | TokenType::From => {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM)
        }

        // types light blue italic
        TokenType::Int
        | TokenType::Float
        | TokenType::Bool
        | TokenType::String
        | TokenType::Char => Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::ITALIC),

        // logic operators yellow
        TokenType::Or | TokenType::And => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),

        // number literals magenta
        TokenType::NumberLiteral(_) | TokenType::FloatLiteral(_) => {
            Style::default().fg(Color::Magenta)
        }

        // string literals yellow
        TokenType::StringLiteral(_) => Style::default().fg(Color::Yellow),

        // char literals light yellow
        TokenType::CharacterLiteral(_) => Style::default().fg(Color::LightYellow),

        // bool literals magenta italic
        TokenType::BoolLiteral(_) => Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::ITALIC),

        // null dark gray italic
        TokenType::Null => Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),

        // arrow white dim (->)
        TokenType::Arrow => Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::DIM),

        // operators white
        TokenType::Plus
        | TokenType::Minus
        | TokenType::Star
        | TokenType::Slash
        | TokenType::Bang
        | TokenType::PlusEqual
        | TokenType::MinusEqual
        | TokenType::StarEqual
        | TokenType::SlashEqual => Style::default().fg(Color::White),

        // comparison operators light cyan
        TokenType::Compare
        | TokenType::BangEqual
        | TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual => Style::default().fg(Color::LightCyan),

        // assignment white
        TokenType::Assign => Style::default().fg(Color::White),

        // punctuation dark gray
        TokenType::Comma
        | TokenType::Semicolon
        | TokenType::Colon
        | TokenType::ColonColon
        | TokenType::Dot
        | TokenType::DotDot
        | TokenType::Hash
        | TokenType::BangHash => Style::default().fg(Color::DarkGray),

        // braces/parens/brackets dark gray
        TokenType::LeftBrace
        | TokenType::RightBrace
        | TokenType::LeftParen
        | TokenType::RightParen
        | TokenType::LeftBracket
        | TokenType::RightBracket => Style::default().fg(Color::DarkGray),

        // identifiers white
        TokenType::Identifier(_) => Style::default().fg(Color::White),

        _ => Style::default().fg(Color::White),
    }
}

/// Lexes `input` and returns a vec of syntax-highlighted [`Span`]s.
///
/// On lex error returns a single red span containing the raw input.
pub fn highlight(input: &str) -> Vec<Span<'static>> {
    let source = SourceFile::new("<hl>", input.to_string());
    let tokens = match Tokenizer::lex(source) {
        Ok(t) => t,
        Err(_) => {
            return vec![Span::styled(
                input.to_string(),
                Style::default().fg(Color::Red),
            )];
        }
    };

    let mut spans = Vec::new();
    let mut last = 0usize;
    let chars: Vec<char> = input.chars().collect();

    for tok in &tokens {
        let start = tok.span.start;
        let end = tok.span.end;

        if start > last {
            let gap: String = chars[last..start].iter().collect();
            spans.push(Span::raw(gap));
        }

        let text: String = chars[start..end].iter().collect();
        let style = token_color(&tok.token); // ← Style now, not Color
        spans.push(Span::styled(text, style));
        last = end;
    }

    if last < chars.len() {
        let tail: String = chars[last..].iter().collect();
        spans.push(Span::raw(tail));
    }

    spans
}
