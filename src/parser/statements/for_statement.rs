use crate::{
    ast::{nodes::Expression, statements::Statement},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    pub fn parse_for(&mut self) -> Statement {
        if matches!(self.peek(), TokenType::LeftBracket) {
            self.advance();
            let initializer = Box::new(self.parse_variable_declartion());
            self.match_type(&[TokenType::Comma]);
            let condition = self.parse_expression();
            self.match_type(&[TokenType::Comma]);
            let increment = self.parse_expression();
            self.match_type(&[TokenType::RightBracket]);
            let body = self.parse_block();
            Statement::For {
                initializer,
                condition,
                increment,
                body,
            }
        } else if matches!(self.peek(), TokenType::Identifier(_)) {
            let variable_name = match self.parse_expression() {
                Expression::Identifier(name) => name,
                _ => {
                    Error::init("for-range expects identifier".to_string(), None, None)
                        .print_error();
                    unreachable!();
                }
            };
            self.advance();
            self.match_type(&[TokenType::In]);

            let range = if matches!(self.peek(), TokenType::NumberLiteral(_)) {
                let range_start = match self.parse_expression() {
                    Expression::Integer(i) => i,
                    _ => {
                        Error::init("range should be integers only".to_string(), None, None)
                            .print_error();
                        unreachable!()
                    }
                };

                self.match_type(&[TokenType::DotDot]);
                let range_end = match self.parse_expression() {
                    Expression::Integer(i) => i,
                    _ => {
                        Error::init("range should be integers only".to_string(), None, None)
                            .print_error();
                        unreachable!()
                    }
                };
                let range_vec: Vec<i64> = (range_start..range_end).collect();
                Box::new(Statement::Range(range_vec))
            } else if self.match_type(&[TokenType::LeftBracket]) {
                let mut items = Vec::new();

                while self.peek() != TokenType::RightBracket {
                    let value = self.parse_expression();
                    items.push(value);
                    if self.peek() == TokenType::RightBracket {
                        break;
                    }
                    if !self.match_type(&[TokenType::Comma]) {
                        crate::utils::errors::Error::init(
                            "expected ',' between list items".to_string(),
                            None,
                            Some(crate::utils::errors::ErrorReason::init(
                                crate::utils::errors::Reason::Parse,
                                None,
                            )),
                        )
                        .print_error();
                    }
                }
                self.match_type(&[TokenType::RightBracket]);

                let iterable = {
                    let mut iterable_list = Vec::new();
                    for item in items {
                        match item {
                            Expression::Integer(i) => iterable_list.push(i),
                            _ => {
                                Error::init(format!("{item:?} is not integer"), None, None)
                                    .print_error();
                                unreachable!()
                            }
                        }
                    }
                    iterable_list
                };

                Box::new(Statement::Range(iterable))
            } else {
                Error::init("expected range or array".to_string(), None, None).print_error();
                unreachable!()
            };

            let body = self.parse_block();

            Statement::ForRange {
                variable: variable_name,
                range,
                body,
            }
        } else {
            Error::init("wrong usage of for".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
