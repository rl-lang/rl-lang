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
        ]) {
            let operator = self.previous();
            let right = self.parse_term();
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
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
        // println!("current index: {:?}", self.current);
        // println!("current token: {:?}", self.peek());

        // is it identifier
        if self.match_type(&[TokenType::Identifier(String::new())]) {
            if let TokenType::Identifier(name) = self.previous() {
                // is it function call?
                if self.match_type(&[TokenType::LeftParen]) {
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
                    let value = self.parse_expression();
                    return Expression::Assign {
                        name,
                        value: Box::new(value),
                    };
                }
                return Expression::Identifier(name);
            }
        }

        // is it integer?
        if self.match_type(&[TokenType::NumberLiteral(0)]) {
            // println!("found number");
            if let TokenType::NumberLiteral(n) = self.previous() {
                return Expression::Integer(n);
            }
        }

        // is it String?
        if self.match_type(&[TokenType::StringLiteral(String::new())]) {
            // println!("found string");
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
            // println!("found characher");
            if let TokenType::CharacterLiteral(c) = self.previous() {
                return Expression::Character(c);
            }
        }

        // is it bool?
        if self.match_type(&[TokenType::BoolLiteral(false)]) {
            // println!("found bool");
            if let TokenType::BoolLiteral(b) = self.previous() {
                return Expression::Bool(b);
            }
        }

        // is it float??
        if self.match_type(&[TokenType::FloatLiteral(0.0)]) {
            // println!("oh no found float");
            if let TokenType::FloatLiteral(f) = self.previous() {
                return Expression::Float(f);
            }
        }

        // is it (Expression)?
        if self.match_type(&[TokenType::LeftParen]) {
            // println!("found group start");
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
