use crate::{ast::statements::Statement, lexer::tokentypes::Token};

/// parses list of tokens and produce [`Vec<Statement>`]
pub struct Parser {
    /// the full tokens list from the lexer
    pub tokens: Vec<Token>,
    /// index of current token that being parsed
    pub current: usize,
}

impl Parser {
    /// consumes tokens to return a list of [`Statement`]s
    pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
        let mut parser = Parser { tokens, current: 0 };
        log::info!("parser initialized");
        let mut statements = Vec::new();

        while !parser.is_at_end() {
            statements.push(parser.parse_statement_to_ast());
        }

        log::info!("parsing complete");
        statements
    }
}
