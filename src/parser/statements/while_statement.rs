use crate::{
    ast::statements::{Statement, StatementKind},
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// called when parser finds [`crate::lexer::tokentypes::TokenType::While`]
    ///
    /// parses the condition of loop into [`crate::ast::nodes::Expression`] then parses
    /// the body of it into [`Vec<Statement>`] then returns [`Statement::While`]
    pub fn parse_while(&mut self, start: Span) -> Result<Statement, Error> {
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        let span = start.join(self.previous_span());
        Ok(Statement::new(StatementKind::While { condition, body }, span))
    }
}
