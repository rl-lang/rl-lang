use crate::lexer::tokentypes::{Token, TokenType};

pub struct Tokenizer {
    pub source: Vec<char>,
    pub tokens: Vec<super::tokentypes::Token>,
    pub current: usize,
    pub start: usize,
    pub line: usize,
}

impl Tokenizer {
    pub fn lex(source: &str) -> Vec<super::tokentypes::Token> {
        let mut lexer = Tokenizer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        };

        // if not end of file scan those characters into tokens and add them to the holder
        while !lexer.is_at_end() {
            // reset lexer position to the current character
            lexer.start = lexer.current;

            lexer.scan_tokens();
        }

        // to mark the end of file when parsing those tokens
        lexer
            .tokens
            .push(Token::new(TokenType::Eof, String::new(), lexer.line));

        println!("Recognized {} token(s)", lexer.tokens.len());
        lexer.tokens
    }
}

impl Tokenizer {
    pub fn scan_tokens(&mut self) {
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

            '\'' => self.character_literal(),
            '"' => self.string_literal(),

            '0'..='9' => self.number_literal(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            _ => {}
        }
    }
}

impl Tokenizer {
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();

        match value.as_str() {
            "fn" => self.add_token(TokenType::Fn),
            "for" => self.add_token(TokenType::For),
            "while" => self.add_token(TokenType::While),
            "return" => self.add_token(TokenType::Return),
            "continue" => self.add_token(TokenType::Continue),
            "break" => self.add_token(TokenType::Break),
            "get" => self.add_token(TokenType::Get),
            "from" => self.add_token(TokenType::From),
            "in" => self.add_token(TokenType::In),
            "or" => self.add_token(TokenType::Or),
            "and" => self.add_token(TokenType::And),
            "null" => self.add_token(TokenType::Null),
            "int" => self.add_token(TokenType::Int),
            "CONST" => self.add_token(TokenType::Const),
            "float" => self.add_token(TokenType::Float),
            "bool" => self.add_token(TokenType::Bool),
            "string" => self.add_token(TokenType::String),
            "char" => self.add_token(TokenType::Char),
            "true" => self.add_token(TokenType::BoolLiteral(true)),
            "false" => self.add_token(TokenType::BoolLiteral(false)),
            "dec" => self.add_token(TokenType::Dec),
            "if" => self.add_token(TokenType::If),
            "else" => self.add_token(TokenType::Else),

            &_ => self.add_token(TokenType::Identifier(value)),
        }
    }

    fn number_literal(&mut self) {
        let mut is_float = false;
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_float = true;
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();

        if is_float {
            let parsed_value: f64 = value.parse().unwrap();
            self.add_token(TokenType::FloatLiteral(parsed_value));
        } else {
            let parsed_value: i64 = value.parse().unwrap();
            self.add_token(TokenType::NumberLiteral(parsed_value));
        }
    }

    fn string_literal(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            crate::utils::errors::Error::init(
                "Unterminated String".to_string(),
                Some(self.line),
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Lexer,
                    None,
                )),
            )
            .print_error();
            return;
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::StringLiteral(value));
    }

    fn character_literal(&mut self) {
        self.advance();

        if self.is_at_end() {
            crate::utils::errors::Error::init(
                "unterminated character literal".to_string(),
                Some(self.line),
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Lexer,
                    None,
                )),
            )
            .print_error();
            return;
        }
        let value: char = self.source[self.current];

        if self.peek() != '\'' {
            crate::utils::errors::Error::init(
                "unterminated character literal".to_string(),
                Some(self.line),
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Lexer,
                    None,
                )),
            )
            .print_error();
            return;
        }

        self.advance();

        self.add_token(TokenType::CharacterLiteral(value));
    }
}
