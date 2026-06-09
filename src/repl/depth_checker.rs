use crate::{
    lexer::{tokenizer::Tokenizer, tokentypes::TokenType},
    utils::source::SourceFile,
};

pub fn is_complete(input: &str) -> bool {
    let source = SourceFile::new("<check>", input.to_string());
    let tokens = match Tokenizer::lex(source) {
        Ok(t) => t,
        Err(_) => return true,
    };
    let mut brace_depth: i32 = 0;
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    for tok in &tokens {
        match tok.token {
            TokenType::LeftBrace => brace_depth += 1,
            TokenType::RightBrace => brace_depth -= 1,
            TokenType::LeftParen => paren_depth += 1,
            TokenType::RightParen => paren_depth -= 1,
            TokenType::LeftBracket => bracket_depth += 1,
            TokenType::RightBracket => bracket_depth -= 1,
            _ => {}
        }
    }

    if paren_depth > 0 || bracket_depth > 0 {
        return false;
    }

    if brace_depth > 0 {
        return input.trim_end().ends_with('{');
    }
    true
}
