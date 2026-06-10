use rl_lang::{ast::statements::Statement, utils::source::SourceFile};

pub fn lex(source: &str) -> Vec<rl_lang::lexer::tokentypes::Token> {
    let text = SourceFile::new("test", source.to_string());
    rl_lang::logic_loops::lexing_loop(text)
}

pub fn parse(source: &str) -> Vec<Statement> {
    rl_lang::logic_loops::parsing_loop(SourceFile::new("test", source.to_string()), lex(source))
}
