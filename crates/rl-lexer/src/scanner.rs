//! Top-level scan driver.
//!
//! Exposes the single entry point that the pipeline calls to turn a source
//! string into a token stream. Delegates all real work to [`Tokenizer`].
use crate::tokentypes::Trivia;
use crate::{tokenizer::Tokenizer, tokentypes::TokenType};
use rl_utils::errors::Error;

impl Tokenizer {
    /// Scans the source file and return token stream
    ///
    /// Iterates over every character in the source file, identifies lexemes,
    /// and returns a flat list of [`Token`]s terminated by [`TokenType::EOF`].
    ///
    /// For multi-character tokens (e.g. `==`, `!=`, `<=`) the scanner peeks
    /// ahead one character to decide which token to emit.
    ///
    /// # Tokens
    ///
    /// | category       | examples                                      |
    /// |----------------|-----------------------------------------------|
    /// | literals       | `1`, `3.14`, `'a'`, `"hello"`                 |
    /// | identifiers    | `foo`, `my_var`, `_private`                   |
    /// | arithmetic     | `+`, `-`, `*`, `/`                            |
    /// | comparison     | `==`, `!=`, `<`, `>`, `<=`, `>=`              |
    /// | assignment     | `=`, `+=`, `-=`, `*=`, `/=`                   |
    /// | delimiters     | `(`, `)`, `{`, `}`, `[`, `]`                  |
    /// | punctuation    | `,`, `;`, `.`, `..`, `:`, `::`                |
    /// | special        | `#`, `!#`, `->`, `//` (comment)               |
    /// | whitespace     | newlines (emitted), spaces/tabs (skipped)     |
    ///
    ///
    /// # Errors
    ///
    /// - unterminated character literal
    /// - unknown escape sequence
    /// - unterminated string
    /// - unexpected character
    pub fn scan_tokens(&mut self) -> Result<(), Error> {
        let current_character = self.advance();
        match current_character {
            // single character
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '#' => self.add_token(TokenType::Hash),
            ',' => self.add_token(TokenType::Comma),
            ';' => self.add_token(TokenType::Semicolon),
            '?' => self.add_token(TokenType::Question),

            // multi character
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
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                        let is_doc = self.peek() == '/';
                        if is_doc {
                            self.advance();
                        }

                        let start = self.current;
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                        let text: String = self.source[start..self.current].iter().collect();
                        let text = text.trim().to_string();

                        let trivia = if is_doc {
                            Trivia::DocComment(text)
                        } else {
                            Trivia::LineComment(text)
                        };

                        if let Some(last) = self.tokens.last_mut()
                            && last.line == self.line
                            && !is_doc
                            && !matches!(last.token, TokenType::Newline)
                        {
                            last.trailing_trivia.push(trivia);
                            return Ok(());
                        }
                        self.pending_trivia.push(trivia);
                        self.newlines_since_trivia = 0;
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
                } else if self.peek() == '>' {
                    self.advance();
                    self.add_token(TokenType::FatArrow);
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

            // whitespaces
            ' ' | '\t' | '\r' => {}

            '\n' => {
                self.line += 1;
                if !self.pending_trivia.is_empty() {
                    self.newlines_since_trivia += 1;
                    if self.newlines_since_trivia > 2 {
                        self.flush_orphaned_trivia();
                    }
                }
                self.add_token(TokenType::Newline)
            }

            // literals
            '\'' => self.character_literal()?,
            '"' => self.string_literal()?,

            '0'..='9' => self.number_literal(),

            '_' | 'a'..='z' | 'A'..='Z' => self.identifier(),

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
