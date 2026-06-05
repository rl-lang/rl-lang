use crate::{
    ast::{nodes::Expression, statements::Statement},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    pub fn parse_for(&mut self) -> Statement {
        if self.match_type(&[TokenType::LeftBracket]) {
            let initializer = Box::new(self.parse_variable_declartion());
            self.match_type(&[TokenType::Comma]);
            let condition = self.parse_expression();
            self.match_type(&[TokenType::Comma]);
            let increment = self.parse_expression();
            self.match_type(&[TokenType::RightBracket]);
            self.match_type(&[TokenType::LeftBrace]);
            let body = self.parse_block();
            self.match_type(&[TokenType::RightBrace]);
            return Statement::For {
                initializer,
                condition,
                increment,
                body,
            };
        } else if self.peek() == TokenType::Identifier(String::new()) {
            let variable = self.parse_expression();
            self.match_type(&[TokenType::In]);

            let range = if self.peek() == TokenType::NumberLiteral(0) {
                let range_start = match self.parse_expression() {
                    Expression::Integer(i) => i,
                    _ => {
                        Error::init("range should be integers only".to_string(), None, None);
                        unreachable!()
                    }
                };
                self.match_type(&[TokenType::DotDot]);
                let range_end = match self.parse_expression() {
                    Expression::Integer(i) => i,
                    _ => {
                        Error::init("range should be integers only".to_string(), None, None);
                        unreachable!()
                    }
                };
                Box::new(Statement::Range((range_start..range_end).collect()))
            };

            self.match_type(&[TokenType::LeftBrace]);
            let body = self.parse_block();
            self.match_type(&[TokenType::RightBrace]);

            return Statement::ForRange {
                variable,
                range,
                body,
            };
        } else if self.peek() == TokenType::Array {
            let iterable = match self.parse_expression() {
                Expression::ArrayLiteral(array) => array,
                _ => {
                    Error::init("not iterable".to_string(), None, None);
                    unreachable!()
                }
            };

            let range = Box::new(Statement::IterableRange(iterable));
            self.match_type(&[TokenType::LeftBrace]);
            let body = self.parse_block();
            self.match_type(&[TokenType::RightBrace]);

            return Statement::ForRange {
                variable,
                range,
                body,
            };
        }

        unreachable!()
    }
}
