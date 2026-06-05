use rl_lang::{
    ast::statements::Statement, lexer::tokenizer::Tokenizer, parser::parser_logic::Parser,
};

pub fn lex(source: &str) -> Vec<rl_lang::lexer::tokentypes::Token> {
    Tokenizer::lex(source)
}

pub fn parse(source: &str) -> Vec<Statement> {
    Parser::parse(lex(source))
}
