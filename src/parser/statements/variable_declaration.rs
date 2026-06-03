use crate::{ast::statements::Statement, lexer::tokentypes::TokenType, parser::parser::Parser};

impl Parser {
    pub fn parse_variable_declartion(&mut self) -> Statement {
        println!("{:?}", self.peek());
        println!("parsing type");
        if self.match_type(&[TokenType::Array]) {
            if self.peek() == TokenType::LeftBracket {
                self.advance();
                let annoation_type = match self.peek() {
                    TokenType::Int
                    | TokenType::Float
                    | TokenType::Bool
                    | TokenType::String
                    | TokenType::Char => {
                        let t = self.peek();
                        self.advance();
                        t
                    }
                    _ => {
                        crate::utils::errors::Error::init(
                            "expected type after dec".to_string(),
                            None,
                            Some(crate::utils::errors::ErrorReason::init(
                                crate::utils::errors::Reason::Parse,
                                None,
                            )),
                        )
                        .print_error();
                        unreachable!()
                    }
                };
                if !self.match_type(&[TokenType::RightBracket]) {
                    crate::utils::errors::Error::init(
                        "expected ']' after type".to_string(),
                        None,
                        Some(crate::utils::errors::ErrorReason::init(
                            crate::utils::errors::Reason::Parse,
                            None,
                        )),
                    )
                    .print_error();
                }

                let name = match self.peek() {
                    TokenType::Identifier(n) => {
                        let n = n.clone();
                        self.advance();
                        n
                    }
                    _ => {
                        crate::utils::errors::Error::init(
                            "expected name after array type".to_string(),
                            None,
                            Some(crate::utils::errors::ErrorReason::init(
                                crate::utils::errors::Reason::Parse,
                                None,
                            )),
                        )
                        .print_error();
                        unreachable!()
                    }
                };

                if !self.match_type(&[TokenType::Assign]) {
                    crate::utils::errors::Error::init(
                        "expected '=' after name".to_string(),
                        None,
                        Some(crate::utils::errors::ErrorReason::init(
                            crate::utils::errors::Reason::Parse,
                            None,
                        )),
                    )
                    .print_error();
                }

                if !self.match_type(&[TokenType::LeftBracket]) {
                    crate::utils::errors::Error::init(
                        "expected '[' after type".to_string(),
                        None,
                        Some(crate::utils::errors::ErrorReason::init(
                            crate::utils::errors::Reason::Parse,
                            None,
                        )),
                    )
                    .print_error();
                }
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
                return Statement::Array {
                    name,
                    type_annotation: annoation_type,
                    value: items,
                };
            }
        }
        let var_type = match self.peek() {
            TokenType::Int
            | TokenType::Float
            | TokenType::Bool
            | TokenType::String
            | TokenType::Char => {
                let t = self.peek();
                self.advance();
                t
            }
            _ => {
                crate::utils::errors::Error::init(
                    "expected type after dec".to_string(),
                    None,
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Parse,
                        None,
                    )),
                )
                .print_error();
                unreachable!()
            }
        };

        let name = match self.peek() {
            TokenType::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => {
                crate::utils::errors::Error::init(
                    "expected name after type".to_string(),
                    None,
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Parse,
                        None,
                    )),
                )
                .print_error();
                unreachable!()
            }
        };

        if !self.match_type(&[TokenType::Assign]) {
            crate::utils::errors::Error::init(
                "expected '=' after name".to_string(),
                None,
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Parse,
                    None,
                )),
            )
            .print_error();
        }

        let value = self.parse_expression();

        crate::ast::statements::Statement::VariableDeclaration {
            name,
            type_annotation: var_type,
            value,
        }
    }
}
