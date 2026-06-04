use std::process::exit;

use crate::{ast::nodes::Expression, lexer::tokentypes::TokenType, parser::parser::Parser};

impl Parser {
    pub fn parse_expression(&mut self) -> Expression {
        // offloads to term for now
        self.parse_equality()
    }

    pub fn parse_equality(&mut self) -> Expression {
        let mut left = self.parse_comparsion();
        while self.match_type(&[TokenType::BangEqual, TokenType::Compare]) {
            let operator = self.previous();
            let right = self.parse_comparsion();
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }

    pub fn parse_comparsion(&mut self) -> Expression {
        let mut left = self.parse_term();
        while self.match_type(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::PlusEqual,
            TokenType::MinusEqual,
            TokenType::StarEqual,
            TokenType::SlashEqual,
        ]) {
            let operator = self.previous();
            let right = self.parse_term();

            match operator {
                TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual => {
                    if let Expression::Identifier(name) = left {
                        let operator = match operator {
                            TokenType::PlusEqual => TokenType::Plus,
                            TokenType::MinusEqual => TokenType::Minus,
                            TokenType::StarEqual => TokenType::Star,
                            TokenType::SlashEqual => TokenType::Slash,
                            _ => unreachable!(),
                        };
                        let binary = Expression::Binary {
                            left: Box::new(Expression::Identifier(name.clone())),
                            operator,
                            right: Box::new(right),
                        };
                        left = Expression::Assign {
                            name: name.clone(),
                            value: Box::new(binary),
                        };
                    } else {
                        left = Expression::Binary {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        };
                    }
                }
                _ => {
                    left = Expression::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };
                }
            }
        }
        left
    }

    pub fn parse_term(&mut self) -> Expression {
        // left operand into factor to return it if no case match for operator
        let mut left = self.parse_factor();
        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            // get the operator the match_type applied advance on
            let operator = self.previous();
            let right = self.parse_factor();
            // boxes the output
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }

    pub fn parse_factor(&mut self) -> Expression {
        let mut left = self.parse_unary();
        while self.match_type(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.parse_unary();
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }

    pub fn parse_unary(&mut self) -> Expression {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let operand = self.parse_unary();
            return Expression::Unary {
                operator,
                operand: Box::new(operand),
            };
        }
        self.parse_primary()
    }

    pub fn parse_primary(&mut self) -> Expression {
        log::debug!("current index: {:?}", self.current);
        log::debug!("current token: {:?}", self.peek());

        // is it identifier
        if self.match_type(&[TokenType::Identifier(String::new())]) {
            log::debug!("found identifier");
            if let TokenType::Identifier(name) = self.previous() {
                // is it function call?
                if self.match_type(&[TokenType::LeftParen]) {
                    log::debug!("found function call");
                    let mut args = Vec::new();
                    // need to extract this as helper function that returns bool tho
                    if !(std::mem::discriminant(&self.peek())
                        == std::mem::discriminant(&TokenType::RightParen))
                    {
                        loop {
                            args.push(self.parse_expression());
                            if !self.match_type(&[TokenType::Comma]) {
                                break;
                            }
                        }
                    }
                    self.match_type(&[TokenType::RightParen]);
                    return Expression::Call { name, args };
                }

                // is it assignment?
                if self.match_type(&[TokenType::Assign]) {
                    log::debug!("found variable assignment");
                    let value = self.parse_expression();
                    return Expression::Assign {
                        name,
                        value: Box::new(value),
                    };
                }
                if self.match_type(&[TokenType::LeftBracket]) {
                    let index = self.parse_expression();
                    self.match_type(&[TokenType::RightBracket]);

                    let mut expr = Expression::Index {
                        target: Box::new(Expression::Identifier(name.clone())),
                        index: Box::new(index),
                    };

                    // consume chained indices for nested arrays: arr[0][1][2]
                    while self.peek() == TokenType::LeftBracket {
                        self.advance();
                        let next_index = self.parse_expression();
                        self.match_type(&[TokenType::RightBracket]);
                        expr = Expression::Index {
                            target: Box::new(expr),
                            index: Box::new(next_index),
                        };
                    }

                    // is it index-assign?
                    if self.match_type(&[TokenType::Assign]) {
                        log::debug!("found array item assignment");
                        let value = self.parse_expression();
                        if let Expression::Index { target, index } = expr {
                            return Expression::IndexAssign {
                                target,
                                index,
                                value: Box::new(value),
                            };
                        }
                    }

                    return expr;
                }
                return Expression::Identifier(name);
            }
        }

        // is it array literal?
        if self.match_type(&[TokenType::LeftBracket]) {
            let mut items = Vec::new();
            while self.peek() != TokenType::RightBracket {
                items.push(self.parse_expression());
                if self.peek() == TokenType::RightBracket {
                    break;
                }
                if !self.match_type(&[TokenType::Comma]) {
                    crate::utils::errors::Error::init(
                        "expected ',' between array items".to_string(),
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
            return Expression::ArrayLiteral(items);
        }
        // panic
        // is it integer?
        if self.match_type(&[TokenType::NumberLiteral(0)]) {
            log::debug!("found number");
            if let TokenType::NumberLiteral(n) = self.previous() {
                return Expression::Integer(n);
            }
        }

        // is it String?
        if self.match_type(&[TokenType::StringLiteral(String::new())]) {
            log::debug!("found string");
            if let TokenType::StringLiteral(s) = self.previous() {
                return Expression::String(s);
            }
        }

        // is it character?
        if matches!(
            self.tokens[self.current].token,
            TokenType::CharacterLiteral(_)
        ) {
            self.advance();
            log::debug!("found characher");
            if let TokenType::CharacterLiteral(c) = self.previous() {
                return Expression::Character(c);
            }
        }

        // is it bool?
        if self.match_type(&[TokenType::BoolLiteral(false)]) {
            // log::debug!("found bool");
            if let TokenType::BoolLiteral(b) = self.previous() {
                return Expression::Bool(b);
            }
        }

        // is it float??
        if self.match_type(&[TokenType::FloatLiteral(0.0)]) {
            log::debug!("oh no found float");
            if let TokenType::FloatLiteral(f) = self.previous() {
                return Expression::Float(f);
            }
        }

        // is it (Expression)?
        if self.match_type(&[TokenType::LeftParen]) {
            log::debug!("found group start");
            let inner = self.parse_expression();
            self.match_type(&[TokenType::RightParen]);
            return Expression::Grouping(Box::new(inner));
        }

        // panic
        crate::utils::errors::Error::init(
            "Expected expression".to_string(),
            None,
            Some(crate::utils::errors::ErrorReason::init(
                crate::utils::errors::Reason::Parse,
                None,
            )),
        )
        .print_error();
        exit(0)
    }
}
