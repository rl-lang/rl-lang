use crate::{ast::statements::Statement, parser::parser_logic::Parser};

impl Parser {
    /// called when parser finds [`crate::lexer::tokentypes::TokenType::While`]
    ///
    /// parses the condition of loop into [`crate::ast::nodes::Expression`] then parses
    /// the body of it into [`Vec<Statement>`] then returns [`Statement::While`]
    pub fn parse_while(&mut self) -> Statement {
        let condition = self.parse_expression();
        let body = self.parse_block();
        Statement::While { condition, body }
    }
}
