//! Identifier and keyword scanner.
//!
//! Consumes a run of alphanumeric/underscore characters and maps the result to
//! the appropriate keyword [`TokenType`] or falls back to [`TokenType::Identifier`].
use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};

impl Tokenizer {
    /// Scans an identifier or keyword starting at the current position.
    ///
    /// Consumes alphanumeric characters and underscores, then checks if the
    /// result is a reserved word. If not, emits [`TokenType::Identifier`].
    ///
    /// # Reserved Words
    ///
    /// | Category       | Keywords                                              |
    /// |----------------|-------------------------------------------------------|
    /// | Control flow   | `if`, `else`, `for`, `while`, `return`, `break`, `continue` |
    /// | Functions      | `fn`                                                  |
    /// | Imports        | `get`, `from`, `in`                                   |
    /// | Logical        | `and`, `or`                                           |
    /// | Types          | `int`, `float`, `bool`, `string`, `byte`, `char`, `arr`, `error` |
    /// | Declarations   | `dec`, `CONST`                                        |
    /// | Literals       | `true`, `false`, `null`                               |
    /// | Special        | `as`                                                  |
    ///
    /// `CONST` in uppercase in intentional
    pub fn identifier(&mut self) {
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
            "byte" => self.add_token(TokenType::Byte),
            "char" => self.add_token(TokenType::Char),
            "true" => self.add_token(TokenType::BoolLiteral(true)),
            "false" => self.add_token(TokenType::BoolLiteral(false)),
            "dec" => self.add_token(TokenType::Dec),
            "if" => self.add_token(TokenType::If),
            "else" => self.add_token(TokenType::Else),
            "arr" => self.add_token(TokenType::Array),
            "as" => self.add_token(TokenType::As),
            "error" => self.add_token(TokenType::Error),
            "result" => self.add_token(TokenType::Result),
            "ok" => self.add_token(TokenType::Ok),
            "err" => self.add_token(TokenType::Err),

            &_ => self.add_token(TokenType::Identifier(value)),
        }
    }
}
