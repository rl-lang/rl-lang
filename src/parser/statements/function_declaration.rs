use crate::{
    ast::statements::{Param, Statement, StatementKind, TypeAnnotation},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    // fn name(param1, param2) {
    //     body
    // }
    //
    // fn name(param1, param2) -> int {   ← return type ignored for now
    //     body
    // }

    pub fn parse_function(&mut self, start: Span) -> Result<Statement, Error> {
        // function name
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected function name", self.peek_span())),
        };

        // opening paren
        self.match_type(&[TokenType::LeftParen]);

        fn nested_array(env: &mut Parser) -> Result<Box<TypeAnnotation>, Error> {
            if !env.match_type(&[TokenType::LeftBracket]) {
                return Err(env.err("expected '[' after arr", env.peek_span()));
            }

            let param_type = if matches!(env.peek(), TokenType::Array) {
                env.advance();
                Box::new(TypeAnnotation::Array(nested_array(env)?))
            } else {
                match env.peek() {
                    TokenType::Int => {
                        env.advance();
                        Box::new(TypeAnnotation::Int)
                    }
                    TokenType::Float => {
                        env.advance();
                        Box::new(TypeAnnotation::Float)
                    }
                    TokenType::Bool => {
                        env.advance();
                        Box::new(TypeAnnotation::Bool)
                    }
                    TokenType::String => {
                        env.advance();
                        Box::new(TypeAnnotation::String)
                    }
                    TokenType::Char => {
                        env.advance();
                        Box::new(TypeAnnotation::Char)
                    }
                    _ => {
                        return Err(env.err("expected parameter type", env.peek_span()));
                    }
                }
            };

            if !env.match_type(&[TokenType::RightBracket]) {
                return Err(env.err("expected ']' after type", env.peek_span()));
            }

            Ok(param_type)
        }

        // parameters
        let mut params: Vec<Param> = Vec::new();
        while !self.match_type(&[TokenType::RightParen]) {
            let param_type = if matches!(self.peek(), TokenType::Array) {
                self.advance();
                TypeAnnotation::Array(nested_array(self)?)
            } else {
                match self.peek() {
                    TokenType::Int => {
                        self.advance();
                        TypeAnnotation::Int
                    }
                    TokenType::Float => {
                        self.advance();
                        TypeAnnotation::Float
                    }
                    TokenType::Bool => {
                        self.advance();
                        TypeAnnotation::Bool
                    }
                    TokenType::String => {
                        self.advance();
                        TypeAnnotation::String
                    }
                    TokenType::Char => {
                        self.advance();
                        TypeAnnotation::Char
                    }
                    _ => {
                        return Err(self.err("expected parameter type", self.peek_span()));
                    }
                }
            };

            match self.peek() {
                TokenType::Identifier(p) => {
                    self.advance();
                    params.push(Param {
                        param_name: p,
                        param_type: param_type,
                    });
                }
                _ => return Err(self.err("expected parameter name", self.peek_span())),
            }
            if !self.match_type(&[TokenType::Comma]) {
                break;
            }
        }
        self.match_type(&[TokenType::RightParen]);

        // optional return type annotation — skip it for now
        let return_type = if self.match_type(&[TokenType::Arrow]) {
            let x = if matches!(self.peek(), TokenType::Array) {
                self.advance();
                TypeAnnotation::Array(nested_array(self)?)
            } else {
                match self.peek() {
                    TokenType::Int => {
                        self.advance();
                        TypeAnnotation::Int
                    }
                    TokenType::Float => {
                        self.advance();
                        TypeAnnotation::Float
                    }
                    TokenType::Bool => {
                        self.advance();
                        TypeAnnotation::Bool
                    }
                    TokenType::String => {
                        self.advance();
                        TypeAnnotation::String
                    }
                    TokenType::Char => {
                        self.advance();
                        TypeAnnotation::Char
                    }
                    _ => {
                        return Err(self.err("expected parameter type", self.peek_span()));
                    }
                }
            };
            x
        } else {
            TypeAnnotation::Null
        };

        // body

        let body = self.parse_block()?;

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::FunctionDeclaration {
                name,
                params,
                return_type: return_type,
                body,
            },
            span,
        ))
    }
}
