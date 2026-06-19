use crate::{
    ast::statements::{Statement, StatementKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// caled after hitting [`TokenType::If`] returing [`Statement::Conditional`]
    pub fn parse_if(&mut self, start: Span) -> Result<Statement, Error> {
        let if_condition = self.parse_expression()?;
        let if_body = self.parse_block()?;
        let if_branch_span = start.join(self.previous_span());
        let if_branch = Statement::new(
            StatementKind::ConditionalBranch {
                condition: Some(if_condition),
                body: if_body,
            },
            if_branch_span,
        );

        while matches!(self.peek(), TokenType::Newline) {
            self.advance();
        }

        let else_branch = if self.peek() == TokenType::Else {
            let branch_start = self.peek_span();
            self.advance();
            if self.peek() == TokenType::If {
                let elif_start = self.peek_span();
                self.advance();
                // Recurse and produces a nested Conditional
                Some(Box::new(self.parse_if(elif_start)?))
            } else {
                let else_body = self.parse_block()?;
                let span = branch_start.join(self.previous_span());
                Some(Box::new(Statement::new(
                    StatementKind::ConditionalBranch {
                        condition: None,
                        body: else_body,
                    },
                    span,
                )))
            }
        } else {
            None
        };

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::Conditional {
                if_branch: Box::new(if_branch),
                else_branch,
            },
            span,
        ))
    }
}
