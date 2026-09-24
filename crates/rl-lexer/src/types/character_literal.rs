//! Single-quoted character literal scanner.
//!
//! Handles `'x'` and simple escape sequences, emitting [`TokenType::CharacterLiteral`].
use crate::{tokenizer::Tokenizer, tokentypes::TokenType};
use rl_utils::errors::Error;

impl Tokenizer {
    /// Scans a single-quoted character literal and emits [`TokenType::CharacterLiteral`].
    ///
    /// Only a single character is allowed between the quotes.
    /// Supports the following escape sequences:
    ///
    /// | Sequence   | Meaning              |
    /// |------------|----------------------|
    /// | `\n`       | newline              |
    /// | `\t`       | tab                  |
    /// | `\r`       | carriage return      |
    /// | `\0`       | null                 |
    /// | `\\`       | backslash            |
    /// | `\"`       | double quote         |
    /// | `\'`       | single quote         |
    /// | `\a`       | bell                 |
    /// | `\b`       | backspace            |
    /// | `\f`       | form feed            |
    /// | `\v`       | vertical tab         |
    /// | `\e`       | escape (ESC)         |
    /// | `\xHH`     | hex byte (1-2 digits)|
    /// | `\uHHHH`   | unicode (4 digits)   |
    /// | `\u{HHHH}` | unicode (braced)     |
    ///
    /// # Errors
    ///
    /// - `unterminated character literal` -> if EOF is reached or no closing `'` is found
    /// - `unknown escape sequence` -> if `\` is followed by an unrecognized character
    pub fn character_literal(&mut self) -> Result<(), Error> {
        self.advance();

        if self.is_at_end() {
            return Err(self.err("unterminated character literal", self.current_span()));
        }

        let character = self.source[self.current - 1];
        let value: char = if character == '\\' {
            // escape sequence
            if self.is_at_end() {
                return Err(self.err("unterminated character literal", self.current_span()));
            }
            let escaped = self.source[self.current];
            self.advance();
            match escaped {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\"' => '\"',
                '\'' => '\'',
                '0' => '\0',
                'a' => '\x07',
                'b' => '\x08',
                'f' => '\x0C',
                'v' => '\x0B',
                'e' => '\x1B',
                'x' => {
                    let hex = self.read_hex_digits(2).map_err(|_| {
                        self.err(
                            "expected hex digits after '\\x'",
                            self.current_span(),
                        )
                    })?;
                    let byte = u8::from_str_radix(&hex, 16).map_err(|_| {
                        self.err(
                            format!("invalid hex escape '\\x{}'", hex),
                            self.current_span(),
                        )
                    })?;
                    byte as char
                }
                'u' => {
                    if self.is_at_end() {
                        return Err(self.err(
                            "unterminated unicode escape",
                            self.current_span(),
                        ));
                    }
                    if self.peek() == '{' {
                        self.advance();
                        let mut hex = String::new();
                        while !self.is_at_end() && self.peek() != '}' {
                            if self.peek().is_ascii_hexdigit() {
                                hex.push(self.advance());
                            } else {
                                let invalid_ch = self.peek();
                                return Err(self.err(
                                    format!(
                                        "invalid character in unicode escape: '{}'",
                                        invalid_ch
                                    ),
                                    self.current_span(),
                                ));
                            }
                        }
                        if self.is_at_end() {
                            return Err(self.err(
                                "unterminated unicode escape",
                                self.current_span(),
                            ));
                        }
                        self.advance();
                        if hex.is_empty() {
                            return Err(self.err(
                                "expected hex digits in unicode escape",
                                self.current_span(),
                            ));
                        }
                        let codepoint = u32::from_str_radix(&hex, 16).map_err(|_| {
                            self.err(
                                format!("invalid unicode escape '\\u{{{}}}'", hex),
                                self.current_span(),
                            )
                        })?;
                        char::from_u32(codepoint).ok_or_else(|| {
                            self.err(
                                format!(
                                    "invalid unicode codepoint '\\u{{{}}}'",
                                    hex
                                ),
                                self.current_span(),
                            )
                        })?
                    } else {
                        let hex = self.read_hex_digits(4).map_err(|_| {
                            self.err(
                                "expected 4 hex digits after '\\u'",
                                self.current_span(),
                            )
                        })?;
                        if hex.len() != 4 {
                            return Err(self.err(
                                format!(
                                    "expected 4 hex digits after '\\u', found {}",
                                    hex.len()
                                ),
                                self.current_span(),
                            ));
                        }
                        let codepoint = u32::from_str_radix(&hex, 16).map_err(|_| {
                            self.err(
                                format!("invalid unicode escape '\\u{}'", hex),
                                self.current_span(),
                            )
                        })?;
                        char::from_u32(codepoint).ok_or_else(|| {
                            self.err(
                                format!("invalid unicode codepoint '\\u{}'", hex),
                                self.current_span(),
                            )
                        })?
                    }
                }
                _ => {
                    return Err(self.err(
                        format!("unknown escape sequence `\\{}`", escaped),
                        self.current_span(),
                    ));
                }
            }
        } else {
            character
        };
        if self.peek() != '\'' {
            return Err(self.err("unterminated character literal", self.current_span()));
        }

        self.advance();

        self.add_token(TokenType::CharacterLiteral(value));
        Ok(())
    }
}
