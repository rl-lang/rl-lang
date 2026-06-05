use rl_lang::lexer::tokenizer::Tokenizer;

pub fn lex(source: &str) -> Vec<rl_lang::lexer::tokentypes::Token> {
    Tokenizer::lex(source)
}
