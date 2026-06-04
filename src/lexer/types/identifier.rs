use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};

impl Tokenizer {
    /// scans an identifier or keyword starting at the current position
    ///
    /// it consumes underscore and alphanumeric characters
    /// then checks if the result is reserverd word if not
    /// it returns TokenType::Identifier instead
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
            "char" => self.add_token(TokenType::Char),
            "true" => self.add_token(TokenType::BoolLiteral(true)),
            "false" => self.add_token(TokenType::BoolLiteral(false)),
            "dec" => self.add_token(TokenType::Dec),
            "if" => self.add_token(TokenType::If),
            "else" => self.add_token(TokenType::Else),
            "arr" => self.add_token(TokenType::Array),

            &_ => self.add_token(TokenType::Identifier(value)),
        }
    }
}
