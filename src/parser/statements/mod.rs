mod if_statement;
mod variable_declaration;
mod while_statement;

use crate::{
    ast::{nodes::Expression, statements::Statement},
    lexer::tokentypes::TokenType,
    parser::parser::Parser,
    utils::errors::Error,
};

impl Parser {
    /// parsing [`TokenType`]s into [`Statement`]s
    pub fn parse_statement_to_ast(&mut self) -> Statement {
        match self.peek() {
            TokenType::Newline => {
                self.advance();
                log::info!("found newline while parser... skipping");
                Statement::Expression(Expression::Integer(0))
            }
            TokenType::Dec => {
                self.advance();
                self.parse_variable_declartion()
                // println!("{:?}", stmt);
            }
            TokenType::While => {
                self.advance();
                self.parse_while()
            }
            TokenType::If => {
                self.advance();
                self.parse_if()
            }
            _ => {
                let expr = self.parse_expression();
                Statement::Expression(expr)
                // println!("{:?}", expr);
                // println!("{:?}", evaluator.evaluate(&expr));
            }
        }
    }

    /// parses the body between '{' '}' into list of [`Statement`]s
    pub fn parse_block(&mut self) -> Vec<Statement> {
        if !self.match_type(&[TokenType::LeftBrace]) {
            Error::init("expected '{'".to_string(), None, None).print_error();
        }
        let mut statements = Vec::new();

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
