use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};
use crate::utils::errors::Error;

impl Tokenizer {
    /// scans the current character and returns the correct token
    ///
    /// for multi character tokens (e.g. '==') it peeks ahead
    /// and decide which token to return
    pub fn scan_tokens(&mut self) -> Result<(), Error> {
        let current_character = self.advance();
        match current_character {
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '#' => self.add_token(TokenType::Hash),
            ',' => self.add_token(TokenType::Comma),
            ';' => self.add_token(TokenType::Semicolon),

            ':' => {
                if self.peek() == ':' {
                    self.advance();
                    self.add_token(TokenType::ColonColon);
                } else {
                    self.add_token(TokenType::Colon);
                }
            }

            '.' => {
                if self.peek() == '.' {
                    self.advance();
                    self.add_token(TokenType::DotDot);
                } else {
                    self.add_token(TokenType::Dot);
                }
            }

            '/' => {
                if self.peek() == '/' {
                    while self.peek() != '\n' {
                        self.advance();
                    }
                } else if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::SlashEqual);
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::Compare);
                } else {
                    self.add_token(TokenType::Assign);
                }
            }

            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::BangEqual);
                } else if self.peek() == '#' {
                    self.advance();
                    self.add_token(TokenType::BangHash);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }

            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }

            '+' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::PlusEqual);
                } else {
                    self.add_token(TokenType::Plus);
                }
            }

            '-' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::MinusEqual);
                } else if self.peek() == '>' {
                    self.advance();
                    self.add_token(TokenType::Arrow);
                } else {
                    self.add_token(TokenType::Minus);
                }
            }

            '*' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::StarEqual);
                } else {
                    self.add_token(TokenType::Star);
                }
            }

            ' ' | '\t' | '\r' => {}

            '\n' => {
                self.line += 1;
                self.add_token(TokenType::Newline)
            }

            '\'' => self.character_literal()?,
            '"' => self.string_literal()?,

            '0'..='9' => self.number_literal(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            other => {
                return Err(self.err(
                    format!("unexpected character `{}`", other),
                    self.current_span(),
                ));
            }
        }
        Ok(())
    }
}
