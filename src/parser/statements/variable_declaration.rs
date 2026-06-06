use crate::{
    ast::statements::{Statement, StatementKind, TypeAnnotation},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    pub fn parse_variable_declartion(&mut self, start: Span) -> Result<Statement, Error> {
        log::debug!("{:?}", self.peek());
        log::debug!("parsing type");
        if self.match_type(&[TokenType::Array]) && self.peek() == TokenType::LeftBracket {
            self.advance();
            let annoation_type = match self.peek() {
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
                TokenType::Array => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(true)?;
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::Array(Box::new(inner))
                }
                _ => return Err(self.err("expected type after `dec`", self.peek_span())),
            };
            if !self.match_type(&[TokenType::RightBracket]) {
                return Err(self.err("expected `]` after type", self.peek_span()));
            }

            let name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => return Err(self.err("expected name after array type", self.peek_span())),
            };

            if !self.match_type(&[TokenType::Assign]) {
                return Err(self.err("expected `=` after name", self.peek_span()));
            }

            if !self.match_type(&[TokenType::LeftBracket]) {
                return Err(self.err("expected `[` after `=`", self.peek_span()));
            }
            let mut items = Vec::new();

            while self.peek() != TokenType::RightBracket {
                let value = self.parse_expression()?;
                items.push(value);
                if self.peek() == TokenType::RightBracket {
                    break;
                }
                if !self.match_type(&[TokenType::Comma]) {
                    return Err(self.err("expected `,` between list items", self.peek_span()));
                }
            }
            self.match_type(&[TokenType::RightBracket]);
            let span = start.join(self.previous_span());
            return Ok(Statement::new(
                StatementKind::Array {
                    name,
                    type_annotation: annoation_type,
                    value: items,
                },
                span,
            ));
        }

        let var_type = self.parse_type(true)?;

        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected name after type", self.peek_span())),
        };

        if !self.match_type(&[TokenType::Assign]) {
            return Err(self.err("expected `=` after name", self.peek_span()));
        }

        let value = self.parse_expression()?;
        let span = start.join(value.span);

        Ok(Statement::new(
            StatementKind::VariableDeclaration {
                name,
                type_annotation: var_type,
                value,
            },
            span,
        ))
    }
}

// should separate later
impl Parser {
    pub fn parse_type(&mut self, is_mut: bool) -> Result<TypeAnnotation, Error> {
        let span = self.peek_span();
        Ok(match is_mut {
            true => match self.peek() {
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
                TokenType::Array => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(true)?;
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::Array(Box::new(inner))
                }
                _ => return Err(self.err("expected a type", span)),
            },
            false => match self.peek() {
                TokenType::Int => {
                    self.advance();
                    TypeAnnotation::CInt
                }
                TokenType::Float => {
                    self.advance();
                    TypeAnnotation::CFloat
                }
                TokenType::Bool => {
                    self.advance();
                    TypeAnnotation::CBool
                }
                TokenType::String => {
                    self.advance();
                    TypeAnnotation::CString
                }
                TokenType::Char => {
                    self.advance();
                    TypeAnnotation::CChar
                }
                TokenType::Array => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(false)?;
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::CArray(Box::new(inner))
                }
                _ => return Err(self.err("expected a type", span)),
            },
        })
    }
}
