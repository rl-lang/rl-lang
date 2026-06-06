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
        log::debug!("parsed if branch");
        let if_branch_span = start.join(self.previous_span());
        let if_branch = Statement::new(
            StatementKind::ConditionalBranch {
                condition: Some(if_condition),
                body: if_body,
            },
            if_branch_span,
        );
        let mut elseif_branch = Vec::new();
        let mut else_body = Vec::new();
        let mut else_start: Option<Span> = None;
        let mut else_end_span: Span = if_branch_span;
        while matches!(self.peek(), TokenType::Newline) {
            self.advance();
        }
        while self.peek() == TokenType::Else {
            let branch_start = self.peek_span();
            self.advance();
            if self.peek() == TokenType::If {
                self.advance();
                let branch_condition = self.parse_expression()?;
                let branch_body = self.parse_block()?;
                let span = branch_start.join(self.previous_span());
                let branch = Statement::new(
                    StatementKind::ConditionalBranch {
                        condition: Some(branch_condition),
                        body: branch_body,
                    },
                    span,
                );
                log::debug!("parsed else if branch");
                elseif_branch.push(branch);
                while matches!(self.peek(), TokenType::Newline) {
                    self.advance();
                }
            } else {
                else_start = Some(branch_start);
                else_body = self.parse_block()?;
                else_end_span = self.previous_span();
            }
        }

        let else_branch = if else_body.is_empty() {
            None
        } else {
            log::debug!("parsed else branch");
            let span = else_start.unwrap_or(if_branch_span).join(else_end_span);
            Some(Box::new(Statement::new(
                StatementKind::ConditionalBranch {
                    condition: None,
                    body: else_body,
                },
                span,
            )))
        };

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::Conditional {
                if_branch: Box::new(if_branch),
                elseif_branch: Some(elseif_branch),
                else_branch,
            },
            span,
        ))
    }
}
