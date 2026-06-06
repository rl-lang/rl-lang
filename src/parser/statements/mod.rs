mod const_declaration;
mod for_statement;
mod if_statement;
mod import_statement;
mod variable_declaration;
mod while_statement;

use crate::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::{Statement, StatementKind},
    },
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// parsing [`TokenType`]s into [`Statement`]s
    pub fn parse_statement_to_ast(&mut self) -> Result<Statement, Error> {
        let start = self.peek_span();
        match self.peek() {
            TokenType::Newline => {
                self.advance();
                log::info!("found newline while parsing... skipping");
                let span = self.previous_span();
                Ok(Statement::new(
                    StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), span)),
                    span,
                ))
            }

            // the new import
            TokenType::Get => {
                // consume it
                self.advance();
                // log it
                log::info!("found `get` for import while parsing");
                // parse it
                self.parse_import()
            }
            TokenType::Dec => {
                self.advance();
                log::info!("found `declaration` for variable while parsing");
                self.parse_variable_declartion(start)
            }
            TokenType::Const => {
                self.advance();
                log::info!("found `declaration` for constant while parsing");
                self.parse_const_declartion(start)
            }
            TokenType::While => {
                self.advance();
                log::info!("found `while` while parsing");
                self.parse_while(start)
            }
            TokenType::For => {
                let span = self.peek_span();
                Ok(Statement::new(
                    StatementKind::Expression(Expression::new(ExpressionKind::Integer(0), span)),
                    span,
                )) // for now
            }
            TokenType::If => {
                self.advance();
                log::info!("found `if` while parsing");
                self.parse_if(start)
            }
            _ => {
                log::info!("parsing the current tokens as expression");
                let expr = self.parse_expression()?;
                let span = expr.span;
                Ok(Statement::new(StatementKind::Expression(expr), span))
            }
        }
    }

    /// parses the body between '{' '}' into list of [`Statement`]s
    pub fn parse_block(&mut self) -> Result<Vec<Statement>, Error> {
        if !self.match_type(&[TokenType::LeftBrace]) {
            return Err(self.err("expected `{`", self.peek_span()));
        }
        let mut statements = Vec::new();

        log::info!("parsing body into statements");
        while !self.match_type(&[TokenType::RightBrace, TokenType::Eof]) {
            if matches!(self.peek(), TokenType::Newline) {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement_to_ast()?);
        }
        self.match_type(&[TokenType::RightBrace]);
        Ok(statements)
    }
}
