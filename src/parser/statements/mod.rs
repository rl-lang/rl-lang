mod const_declaration;
// mod for_statement;
mod if_statement;
mod variable_declaration;
mod while_statement;

use crate::{
    ast::{nodes::Expression, statements::Statement},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// parsing [`TokenType`]s into [`Statement`]s
    pub fn parse_statement_to_ast(&mut self) -> Statement {
        match self.peek() {
            TokenType::Newline => {
                self.advance();
                log::info!("found newline while parsing... skipping");
                Statement::Expression(Expression::Integer(0))
            }
            TokenType::Dec => {
                self.advance();
                log::info!("found `declaration` for variable while parsing");
                self.parse_variable_declartion()
            }
            TokenType::Const => {
                self.advance();
                log::info!("found `declaration` for constant while parsing");
                self.parse_const_declartion()
            }
            TokenType::While => {
                self.advance();
                log::info!("found `while` while parsing");
                self.parse_while()
            }
            TokenType::For => {
                return Statement::Expression(Expression::Integer(0)); // for now
                // self.advance();
                // log::info!("found `for` while parsing");
                // self.parse_for()
            }
            TokenType::If => {
                self.advance();
                log::info!("found `if` while parsing");
                self.parse_if()
            }
            _ => {
                log::info!("parsing the current tokens as expression");
                let expr = self.parse_expression();
                Statement::Expression(expr)
            }
        }
    }

    /// parses the body between '{' '}' into list of [`Statement`]s
    pub fn parse_block(&mut self) -> Vec<Statement> {
        if !self.match_type(&[TokenType::LeftBrace]) {
            Error::init("expected '{'".to_string(), None, None).print_error();
        }
        let mut statements = Vec::new();

        log::info!("parsing body into statements");
        while !self.match_type(&[TokenType::RightBrace, TokenType::Eof]) {
            if matches!(self.peek(), TokenType::Newline) {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement_to_ast());
        }
        self.match_type(&[TokenType::RightBrace]);
        statements
    }
}
