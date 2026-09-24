//! Double-quoted string literal scanner.
//!
//! Consumes everything between `"..."`, handling escape sequences, and emits
//! [`TokenType::StringLiteral`].
use crate::{tokenizer::Tokenizer, tokentypes::TokenType};
use rl_utils::errors::Error;

impl Tokenizer {
    /// Scans a double-quoted string literal and emits [`TokenType::StringLiteral`].
    ///
    /// Supports multi-line strings and the following escape sequences:
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
    /// - `unterminated string` -> if EOF is reached before the closing `"`
    /// - `unknown escape sequence` -> if `\` is followed by an unrecognized character
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
                match escaped_ch {
                    'n' => { value.push('\n'); }
                    't' => { value.push('\t'); }
                    'r' => { value.push('\r'); }
                    '0' => { value.push('\0'); }
                    '\\' => { value.push('\\'); }
                    '"' => { value.push('"'); }
                    '\'' => { value.push('\''); }
                    'a' => { value.push('\x07'); }
                    'b' => { value.push('\x08'); }
                    'f' => { value.push('\x0C'); }
                    'v' => { value.push('\x0B'); }
                    'e' => { value.push('\x1B'); }
                    'x' => {
                        self.advance();
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
                        value.push(byte as char);
                        continue;
                    }
                    'u' => {
                        self.advance();
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
                            let ch = char::from_u32(codepoint).ok_or_else(|| {
                                self.err(
                                    format!(
                                        "invalid unicode codepoint '\\u{{{}}}'",
                                        hex
                                    ),
                                    self.current_span(),
                                )
                            })?;
                            value.push(ch);
                            continue;
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
                            let ch = char::from_u32(codepoint).ok_or_else(|| {
                                self.err(
                                    format!("invalid unicode codepoint '\\u{}'", hex),
                                    self.current_span(),
                                )
                            })?;
                            value.push(ch);
                            continue;
                        }
                    }
                    other => {
                        return Err(self.err(
                            format!("unknown escape sequence '\\{}'", other),
                            self.current_span(),
                        ));
                    }
                }

                // single-character escapes: advance past the escape character
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
