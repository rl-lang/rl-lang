use std::rc::Rc;

use crate::parser_logic::Parser;
use rl_ast::statements::TypeAnnotation;
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl Parser {
    /// Parses an `any[T, ...]` member list (the `[` is consumed here).
    ///
    /// Members parse like array elements (always mutable inside, matching
    /// `array[T]`). Normalization: nested `any[...]` flatten, duplicates
    /// drop out, trailing commas are allowed. Empty and single-member
    /// lists are errors (`any[]` means nothing, `any[int]` is just `int`).
    ///
    /// Shared by `parse_type` (both mutabilities) and `parse_param_type`.
    pub fn parse_any_members(&mut self) -> Result<Vec<TypeAnnotation>, Error> {
        if !self.match_type(&[TokenType::LeftBracket]) {
            return Err(self.err("`any` needs a member list: any[T, ...]", self.peek_span()));
        }
        let mut members: Vec<TypeAnnotation> = Vec::new();
        // newlines are fine everywhere: the formatter may break long
        // member lists vertical, like arrays
        while self.match_type(&[TokenType::Newline]) {}
        loop {
            if self.peek() == TokenType::RightBracket {
                break;
            }
            match self.parse_type(true)? {
                TypeAnnotation::Any(inner) | TypeAnnotation::CAny(inner) => {
                    members.extend(inner.iter().cloned());
                }
                other => members.push(other),
            }
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Comma]) {
                break;
            }
            while self.match_type(&[TokenType::Newline]) {}
        }
        if !self.match_type(&[TokenType::RightBracket]) {
            return Err(self.err("expected `]` after any members", self.peek_span()));
        }
        let mut unique: Vec<TypeAnnotation> = Vec::with_capacity(members.len());
        for m in members {
            if !unique.contains(&m) {
                unique.push(m);
            }
        }
        match unique.len() {
            0 => Err(self.err("any[...] needs at least one member type", self.peek_span())),
            1 => Err(self.err(
                "any[...] with one member is just that type - write it directly",
                self.peek_span(),
            )),
            _ => Ok(unique),
        }
    }
    /// Parses a type keyword into a [`TypeAnnotation`].
    ///
    /// The `is_mut` flag controls which annotation variant is produced:
    ///
    /// | `is_mut` | produced variant |
    /// |---|---|
    /// | `true` | `Int`, `Float`, `Bool`, `String`, `Byte`, `Char`, `Fn`, `Array(T)` |
    /// | `false` | `CInt`, `CFloat`, `CBool`, `CString`, `CByte`, `CChar`, `Fn`, `CArray(T)` |
    ///
    /// The `C*` variants represent constant (immutable) bindings and are used by
    /// the `const` declaration path.
    ///
    /// For `array[T]` the inner element type is parsed recursively.
    ///
    /// # Errors
    /// Returns an error if the current token is not a recognised type keyword.
    pub fn parse_type(&mut self, is_mut: bool) -> Result<TypeAnnotation, Error> {
        let span = self.peek_span();
        Ok(match is_mut {
            true => match self.peek() {
                TokenType::Int => {
                    self.advance();
                    TypeAnnotation::Int
                }
                TokenType::UInt => {
                    self.advance();
                    TypeAnnotation::UInt
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
                TokenType::Byte => {
                    self.advance();
                    TypeAnnotation::Byte
                }
                TokenType::SByte => {
                    self.advance();
                    TypeAnnotation::SByte
                }
                TokenType::Big => {
                    self.advance();
                    match self.peek() {
                        TokenType::Byte => {
                            self.advance();
                            TypeAnnotation::BByte
                        }
                        TokenType::SByte => {
                            self.advance();
                            TypeAnnotation::BSByte
                        }
                        _ => return Err(self.err("`big` only applies to byte types", span)),
                    }
                }
                TokenType::Small => {
                    self.advance();
                    match self.peek() {
                        TokenType::Int => {
                            self.advance();
                            TypeAnnotation::SInt
                        }
                        TokenType::UInt => {
                            self.advance();
                            TypeAnnotation::SUInt
                        }
                        TokenType::Float => {
                            self.advance();
                            TypeAnnotation::SFloat
                        }
                        _ => return Err(self.err("`small` only applies to int/float types", span)),
                    }
                }
                TokenType::Char => {
                    self.advance();
                    TypeAnnotation::Char
                }
                TokenType::Fn => {
                    self.advance();
                    TypeAnnotation::Fn
                }
                TokenType::Array => {
                    self.advance();
                    if !self.match_type(&[TokenType::LeftBracket]) {
                        return Err(self.err("expected `[` after `array`", self.peek_span()));
                    }
                    let inner = self.parse_type(true)?;
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after array type", self.peek_span()));
                    }
                    TypeAnnotation::Array(Box::new(inner))
                }
                TokenType::Map => {
                    self.advance();
                    if !self.match_type(&[TokenType::LeftBracket]) {
                        return Err(self.err("expected `[` after `map`", self.peek_span()));
                    }
                    let key_type = self.parse_type(true)?;
                    if !self.match_type(&[TokenType::Comma]) {
                        return Err(self.err(
                            "expected `,` between map key and value types",
                            self.peek_span(),
                        ));
                    }
                    let value_type = self.parse_type(true)?;
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after map type", self.peek_span()));
                    }
                    TypeAnnotation::Map(Box::new(key_type), Box::new(value_type))
                }
                TokenType::Set => {
                    self.advance();
                    if !self.match_type(&[TokenType::LeftBracket]) {
                        return Err(self.err("expected `[` after `set`", self.peek_span()));
                    }
                    let inner = self.parse_type(true)?;
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after set type", self.peek_span()));
                    }
                    TypeAnnotation::Set(Box::new(inner))
                }
                TokenType::LeftParen => {
                    self.advance();
                    let mut inner = vec![];
                    while self.match_type(&[TokenType::Newline]) {}
                    if !self.match_type(&[TokenType::RightParen]) {
                        inner.push(self.parse_type(true)?);
                        while self.match_type(&[TokenType::Newline]) {}
                        while self.match_type(&[TokenType::Comma]) {
                            while self.match_type(&[TokenType::Newline]) {}
                            if self.peek() == TokenType::RightParen {
                                break;
                            }
                            inner.push(self.parse_type(true)?);
                            while self.match_type(&[TokenType::Newline]) {}
                        }
                        if !self.match_type(&[TokenType::RightParen]) {
                            return Err(
                                self.err("expected `,` or `)` in tuple type", self.peek_span())
                            );
                        }
                    }
                    TypeAnnotation::Tuple(Rc::new(inner))
                }
                TokenType::Result => {
                    self.advance();
                    if !self.match_type(&[TokenType::LeftBracket]) {
                        return Err(self.err("expected `[` after `result`", self.peek_span()));
                    }
                    let inner = self.parse_type(true)?;
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after result type", self.peek_span()));
                    }
                    TypeAnnotation::Result(Box::new(inner))
                }
                TokenType::Error => {
                    self.advance();
                    TypeAnnotation::Error
                }
                TokenType::Handle => {
                    self.advance();
                    TypeAnnotation::HandleInfer
                }
                TokenType::Identifier(name) if name == "any" => {
                    self.advance();
                    let members = self.parse_any_members()?;
                    TypeAnnotation::Any(Rc::new(members))
                }
                TokenType::Identifier(name) => {
                    let use_span = self.peek_span();
                    self.advance();
                    // Type aliases substitute eagerly: the table is
                    // complete for everything declared above, the target
                    // is stored resolved, and the use is recorded for
                    // deprecation warnings. Nothing alias-flavored flows
                    // downstream - backends only ever see concrete types.
                    if let Some(target) = self.ast_arena.type_aliases.get(&name).cloned() {
                        self.ast_arena.alias_uses.push((name.clone(), use_span));
                        target
                    } else if self.tag_names.contains(&name) {
                        TypeAnnotation::Enum(name)
                    } else {
                        TypeAnnotation::Record(name)
                    }
                }
                _ => return Err(self.err("expected a type", span)),
            },
            false => match self.peek() {
                TokenType::Int => {
                    self.advance();
                    TypeAnnotation::CInt
                }
                TokenType::UInt => {
                    self.advance();
                    TypeAnnotation::CUInt
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
                TokenType::Byte => {
                    self.advance();
                    TypeAnnotation::CByte
                }
                TokenType::SByte => {
                    self.advance();
                    TypeAnnotation::CSByte
                }
                TokenType::Big => {
                    self.advance();
                    match self.peek() {
                        TokenType::Byte => {
                            self.advance();
                            TypeAnnotation::CBByte
                        }
                        TokenType::SByte => {
                            self.advance();
                            TypeAnnotation::CBSByte
                        }
                        _ => return Err(self.err("`big` only applies to byte types", span)),
                    }
                }
                TokenType::Small => {
                    self.advance();
                    match self.peek() {
                        TokenType::Int => {
                            self.advance();
                            TypeAnnotation::CSInt
                        }
                        TokenType::UInt => {
                            self.advance();
                            TypeAnnotation::CSUInt
                        }
                        TokenType::Float => {
                            self.advance();
                            TypeAnnotation::CSFloat
                        }
                        _ => return Err(self.err("`small` only applies to int/float types", span)),
                    }
                }
                TokenType::Char => {
                    self.advance();
                    TypeAnnotation::CChar
                }
                TokenType::Fn => {
                    self.advance();
                    TypeAnnotation::Fn
                }
                TokenType::Array => {
                    self.advance();
                    if !self.match_type(&[TokenType::LeftBracket]) {
                        return Err(self.err("expected `[` after `array`", self.peek_span()));
                    }
                    let inner = self.parse_type(false)?;
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after array type", self.peek_span()));
                    }
                    TypeAnnotation::CArray(Box::new(inner))
                }
                TokenType::Map => {
                    self.advance();
                    if !self.match_type(&[TokenType::LeftBracket]) {
                        return Err(self.err("expected `[` after `map`", self.peek_span()));
                    }
                    let key_type = self.parse_type(false)?;
                    if !self.match_type(&[TokenType::Comma]) {
                        return Err(self.err(
                            "expected `,` between map key and value types",
                            self.peek_span(),
                        ));
                    }
                    let value_type = self.parse_type(false)?;
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after map type", self.peek_span()));
                    }
                    TypeAnnotation::CMap(Box::new(key_type), Box::new(value_type))
                }
                TokenType::Set => {
                    self.advance();
                    if !self.match_type(&[TokenType::LeftBracket]) {
                        return Err(self.err("expected `[` after `set`", self.peek_span()));
                    }
                    let inner = self.parse_type(false)?;
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after set type", self.peek_span()));
                    }
                    TypeAnnotation::CSet(Box::new(inner))
                }
                TokenType::LeftParen => {
                    self.advance();
                    let mut inner = vec![];
                    while self.match_type(&[TokenType::Newline]) {}
                    if !self.match_type(&[TokenType::RightParen]) {
                        inner.push(self.parse_type(false)?);
                        while self.match_type(&[TokenType::Newline]) {}
                        while self.match_type(&[TokenType::Comma]) {
                            while self.match_type(&[TokenType::Newline]) {}
                            if self.peek() == TokenType::RightParen {
                                break;
                            }
                            inner.push(self.parse_type(false)?);
                            while self.match_type(&[TokenType::Newline]) {}
                        }
                        if !self.match_type(&[TokenType::RightParen]) {
                            return Err(
                                self.err("expected `,` or `)` in tuple type", self.peek_span())
                            );
                        }
                    }
                    TypeAnnotation::CTuple(Rc::new(inner))
                }
                TokenType::Result => {
                    self.advance();
                    if !self.match_type(&[TokenType::LeftBracket]) {
                        return Err(self.err("expected `[` after `result`", self.peek_span()));
                    }
                    let inner = self.parse_type(false)?;
                    if !self.match_type(&[TokenType::RightBracket]) {
                        return Err(self.err("expected `]` after result type", self.peek_span()));
                    }
                    TypeAnnotation::CResult(Box::new(inner))
                }
                TokenType::Error => {
                    self.advance();
                    TypeAnnotation::CError
                }
                TokenType::Identifier(name) if name == "any" => {
                    self.advance();
                    let members = self.parse_any_members()?;
                    TypeAnnotation::CAny(Rc::new(members))
                }
                TokenType::Identifier(name) => {
                    let use_span = self.peek_span();
                    self.advance();
                    if let Some(target) = self.ast_arena.type_aliases.get(&name).cloned() {
                        self.ast_arena.alias_uses.push((name.clone(), use_span));
                        target
                    } else if self.tag_names.contains(&name) {
                        TypeAnnotation::CEnum(name)
                    } else {
                        TypeAnnotation::CRecord(name)
                    }
                }
                _ => return Err(self.err("expected a type", span)),
            },
        })
    }
}
