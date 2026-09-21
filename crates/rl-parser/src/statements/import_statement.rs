//! Import statement parser (`get`).
//!
//! Handles all import forms in rl-lang:
//!
//! ```text
//! // 1. single file module
//! get mymodule
//!
//! // 2. file module with path
//! get mymodule::utils
//!
//! // 3. stdlib function (fully qualified)
//! get std::math::sin
//!
//! // 4. named imports from a module or stdlib
//! get sin, cos from std::math
//! get add, sub from mymodule::utils
//!
//! // 5. aliased imports
//! get sin as sine from std::math
//!
//! // 6. wildcard import (all functions from a module)
//! get * from std::math
//! ```
//!
//! The first token after `get` and whether `::` or `from` follows determines
//! which [`StatementKind`] variant is produced:
//!
//! | syntax | kind |
//! |---|---|
//! | `get mod` | [`StatementKind::ImportFile`] |
//! | `get mod::sub` | [`StatementKind::ImportFile`] |
//! | `get std::ns::fn` | [`StatementKind::Import`] |
//! | `get fn, fn from std::ns` | [`StatementKind::Import`] |
//! | `get fn, fn from mod::sub` | [`StatementKind::ImportFileNamed`] |
//! | `get * from std::ns` | [`StatementKind::Import`] (wildcard) |
//! | `get fn as alias from std::ns` | [`StatementKind::Import`] (aliased) |

use crate::parser_logic::Parser;
use rl_ast::statements::{Statement, StatementKind};
use rl_lexer::{tokenizer::Tokenizer, tokentypes::TokenType};
use rl_utils::{errors::Error, source::SourceFile, span::Span};

impl Parser {
    fn get_imported_type_names(&mut self, path: &[String], only: Option<&[String]>) {
        let import_name = format!("{}.rl", path.join("/"));
        let base_dir = std::path::Path::new(self.source_file.name.as_ref())
            .parent()
            .unwrap_or_else(|| std::path::Path::new(""));

        // try direct file first
        let mut file_path = base_dir.join(&import_name);
        if !file_path.exists() {
            // try deps/ directory
            if let Some(first) = path.first() {
                let dep_path = base_dir.join("deps").join(first).join("lib.rl");
                if dep_path.exists() {
                    file_path = dep_path;
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        let Ok(source_text) = std::fs::read_to_string(&file_path) else {
            return;
        };
        let source_file = SourceFile::new(
            file_path.to_string_lossy().as_ref().to_string(),
            source_text,
        );
        let Ok(tokens) = Tokenizer::lex(source_file.clone()) else {
            return;
        };
        let Ok((_, stmts)) = Parser::parse(tokens, source_file) else {
            return;
        };

        for stmt in &stmts {
            let (name, set): (&String, &mut std::collections::HashSet<String>) = match &stmt.kind {
                StatementKind::RecordDeclaration { name, .. } => (name, &mut self.record_names),
                StatementKind::TagDeclaration { name, .. } => (name, &mut self.tag_names),
                _ => continue,
            };
            let wanted = match only {
                Some(names) => names.contains(name),
                None => true,
            };
            if wanted {
                set.insert(name.clone());
            }
        }
    }

    /// Parses a `get` import statement.
    ///
    /// Called after `get` has been consumed. Dispatches on the tokens that
    /// follow the first identifier:
    ///
    /// - **`get mod`** (no `::`, no `from`) - single-segment file import.
    ///   Produces [`StatementKind::ImportFile`]`{ path: [mod] }`.
    ///
    /// - **`get mod::sub::…`** - multi-segment path. If the first segment is
    ///   `std`, the last segment is treated as the function name and the rest
    ///   as the namespace path -> [`StatementKind::Import`]. Otherwise the whole
    ///   path is a file module -> [`StatementKind::ImportFile`].
    ///
    /// - **`get name, name from path`** - named imports. If `path` starts with
    ///   `std` -> [`StatementKind::Import`]`{ names, path }`. Otherwise ->
    ///   [`StatementKind::ImportFileNamed`]`{ path, names }`.
    ///
    /// - **`get * from path`** - wildcard import. Imports all functions from the
    ///   module.
    ///
    /// # Errors
    /// Returns an error if an identifier is missing after `get`, `::`, `,`, or
    /// `from`, or if `from` itself is absent in the named-import form.
    pub fn parse_import(&mut self, start: Span) -> Result<Statement, Error> {
        // Wildcard import: get * from std::math
        if self.match_type(&[TokenType::Star]) {
            if !self.match_type(&[TokenType::From]) {
                return Err(self.err("expected 'from' after '*'", self.peek_span()));
            }
            let mut path = Vec::new();
            while let TokenType::Identifier(segment) = self.peek() {
                self.advance();
                path.push(segment);
                if !self.match_type(&[TokenType::ColonColon]) {
                    break;
                }
            }
            let span = start.join(self.previous_span());
            return Ok(Statement::new(
                StatementKind::Import {
                    names: vec![],
                    wildcard: true,
                    path,
                },
                span,
            ));
        }

        let first = match self.peek() {
            TokenType::Identifier(name) => name,
            _ => return Err(self.err("expected identifier after 'get'", self.peek_span())),
        };
        self.advance();

        // multi-segment path: get mod::sub  OR  get std::math::sin
        if self.match_type(&[TokenType::ColonColon]) {
            let mut segments = vec![first];
            loop {
                match self.peek() {
                    TokenType::Identifier(seg) => {
                        self.advance();
                        segments.push(seg);
                    }
                    _ => return Err(self.err("expected identifier after '::'", self.peek_span())),
                }
                if !self.match_type(&[TokenType::ColonColon]) {
                    break;
                }
            }
            let span = start.join(self.previous_span());
            let is_std = segments[0] == "std";
            return if is_std {
                // last segment is the function name; everything before it is the path
                let name = segments
                    .pop()
                    .ok_or_else(|| self.err("expected function name after '::'", start))?;
                Ok(Statement::new(
                    StatementKind::Import {
                        names: vec![(name, None)],
                        wildcard: false,
                        path: segments,
                    },
                    span,
                ))
            } else {
                self.get_imported_type_names(&segments, None);
                Ok(Statement::new(
                    StatementKind::ImportFile { path: segments },
                    span,
                ))
            };
        }

        // single-segment file import: get mymodule
        if !matches!(self.peek(), TokenType::Comma | TokenType::From | TokenType::As) {
            let span = start.join(self.previous_span());
            let path = vec![first];
            self.get_imported_type_names(&path, None);
            return Ok(Statement::new(StatementKind::ImportFile { path }, span));
        }

        // named imports: get add, sub from …  (with optional `as alias`)
        // `first` is already consumed as the first name.
        let first_entry = self.parse_name_entry(first)?;
        let mut names = vec![first_entry];

        if self.match_type(&[TokenType::Comma]) {
            loop {
                self.match_type(&[TokenType::Newline]);
                match self.peek() {
                    TokenType::Identifier(name) => {
                        self.advance();
                        if name == "*" {
                            return Err(self.err(
                                "wildcard '*' cannot be mixed with named imports",
                                self.peek_span(),
                            ));
                        }
                        let entry = self.parse_name_entry(name)?;
                        names.push(entry);
                    }
                    _ => return Err(self.err("expected identifier after ','", self.peek_span())),
                }
                if !self.match_type(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        if !self.match_type(&[TokenType::From]) {
            return Err(self.err("expected 'from' after names", self.peek_span()));
        }

        let mut path = Vec::new();
        loop {
            match self.peek() {
                TokenType::Identifier(segment) => {
                    self.advance();
                    path.push(segment);
                }
                TokenType::Set => {
                    self.advance();
                    path.push("set".to_string());
                }
                TokenType::Map => {
                    self.advance();
                    path.push("map".to_string());
                }
                _ => return Err(self.err("expected path after 'from'", self.peek_span())),
            }
            if !self.match_type(&[TokenType::ColonColon]) {
                break;
            }
        }

        let span = start.join(self.previous_span());
        let is_std = path.first().map(|s| s == "std").unwrap_or(false);

        if is_std {
            Ok(Statement::new(
                StatementKind::Import {
                    names,
                    wildcard: false,
                    path,
                },
                span,
            ))
        } else {
            let name_strs: Vec<String> = names.into_iter().map(|(n, _)| n).collect();
            self.get_imported_type_names(&path, Some(&name_strs));
            Ok(Statement::new(
                StatementKind::ImportFileNamed {
                    path,
                    names: name_strs,
                },
                span,
            ))
        }
    }

    /// Parses a name entry, checking for `as alias`.
    /// `name` is the already-consumed identifier token value.
    fn parse_name_entry(&mut self, name: String) -> Result<(String, Option<String>), Error> {
        if self.match_type(&[TokenType::As]) {
            let alias = match self.peek() {
                TokenType::Identifier(a) => {
                    self.advance();
                    a
                }
                _ => return Err(self.err("expected alias after 'as'", self.peek_span())),
            };
            Ok((name, Some(alias)))
        } else {
            Ok((name, None))
        }
    }
}
