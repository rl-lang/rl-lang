use crate::{
    ast::statements::Statement, lexer::tokentypes::TokenType, parser::parser_logic::Parser,
};

impl Parser {
    /// caled after hitting [`TokenType::If`] returing [`Statement::Conditional`]
    ///
    /// will parse the condition and returns [`crate::ast::nodes::Expression`]
    /// then it will parse the body after checking for [`TokenType::LeftBrace`] and
    /// returns [`Vec<Statement>`] if there is any else or else if branches it will
    /// detect them and return them as [`Statement::ConditionalBranch`]
    pub fn parse_if(&mut self) -> Statement {
        let if_condition = self.parse_expression();
        let if_body = self.parse_block();
        log::debug!("parsed if branch");
        let if_branch = Statement::ConditionalBranch {
            condition: Some(if_condition),
            body: if_body,
        };
        let mut elseif_branch = Vec::new();
        let mut else_body = Vec::new();
        while matches!(self.peek(), TokenType::Newline) {
            self.advance();
        }
        while self.peek() == TokenType::Else {
            self.advance();
            if self.peek() == TokenType::If {
                self.advance();
                let branch_condition = self.parse_expression();
                let branch_body = self.parse_block();
                let branch = Statement::ConditionalBranch {
                    condition: Some(branch_condition),
                    body: branch_body,
                };
                log::debug!("parsed else if branch");
                elseif_branch.push(branch);
                while matches!(self.peek(), TokenType::Newline) {
                    self.advance();
                }
            } else {
                else_body = self.parse_block();
            }
        }

        let else_branch = if else_body.is_empty() {
            None
        } else {
            log::debug!("parsed else branch");
            Some(Box::new(Statement::ConditionalBranch {
                condition: None,
                body: else_body,
            }))
        };

        Statement::Conditional {
            if_branch: Box::new(if_branch),
            elseif_branch: Some(elseif_branch),
            else_branch,
        }
    }
}
