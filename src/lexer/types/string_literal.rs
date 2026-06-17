use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};
use crate::utils::errors::Error;

impl Tokenizer {
    /// scans a double quoted literal
    ///
    /// supports multi-line strings by incrementing the line counter
    /// when hitting \n and now it supports escape sequences
    /// returns TokenType::StringLiteral
    pub fn string_literal(&mut self) -> Result<(), Error> {
        // construct new string
        let mut value = String::new();

        // while not the end of string which is determinated by "
        while !self.is_at_end() && self.peek() != '"' {
            // cache next character in ch
            let ch = self.peek();
            // if it is new line (e.g. pressed enter)
            // increase line count and push escaped sequence into string
            // then advance
            if ch == '\n' {
                self.line += 1;
                value.push(ch);
                self.advance();
                continue;
            }

            // is it escape sequence?
            if ch == '\\' {
                // consume the first \
                self.advance();
                // safety check for end of file
                if self.is_at_end() {
                    return Err(self.err("unterminated string", self.current_span()));
                }

                // cache the next escaped character
                let escaped_ch = self.peek();
                // is it recognized escaped sequence?
                let resolved_escape = match escaped_ch {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '0' => '\0',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    other => {
                        return Err(self.err(
                            format!("unknown escape sequence '\\{}'", other),
                            self.current_span(),
                        ));
                    }
                };

                // add the escaped sequence into value then advance
                value.push(resolved_escape);
                self.advance();
                continue;
            }

            // if not escape sequence nor " then add to value and advance
            value.push(ch);
            self.advance();
        }

        // are we at end of file or there is "?
        if self.is_at_end() {
            return Err(self.err("unterminated string", self.current_span()));
        }
        // consume the "
        self.advance();

        // add the constructed string value
        self.add_token(TokenType::StringLiteral(value));
        Ok(())
    }
}
