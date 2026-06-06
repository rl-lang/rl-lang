use rl_lang::{
    ast::statements::Statement, lexer::tokenizer::Tokenizer, parser::parser_logic::Parser,
    utils::source::SourceFile,
};

pub fn lex(source: &str) -> Vec<rl_lang::lexer::tokentypes::Token> {
    Tokenizer::lex(SourceFile::new("test", source.to_string()))?
}

pub fn parse(source: &str) -> Vec<Statement> {
    let sf = SourceFile::new("test", source.to_string());
    let tokens = Tokenizer::lex(sf.clone())?;
    Parser::parse(tokens, sf)?
}
