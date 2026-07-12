//! Import statement parser (`get`).
//!
//! Handles all four import forms in rl-lang:
//!
//! ```text
//! // 1. single file module
//! get mymodule
//!
//! // 2. file module with path
//! get mymodule::utils
//!
//! // 3. stdlib function
//! get std::math::sin
//!
//! // 4. named imports from a module or stdlib
//! get sin, cos from std::math
//! get add, sub from mymodule::utils
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

use crate::parser_logic::Parser;
use rl_ast::statements::{Statement, StatementKind};
use rl_lexer::{tokenizer::Tokenizer, tokentypes::TokenType};
use rl_utils::{errors::Error, source::SourceFile, span::Span};

impl Parser {
    fn get_imported_type_names(&mut self, path: &[String], only: Option<&[String]>) {
        let import_name = format!("{}.rl", path.join("/"));
        let file_path = std::path::Path::new(self.source_file.name.as_ref())
            .parent()
            .unwrap_or_else(|| std::path::Path::new(""))
            .join(&import_name);

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
    /// # Errors
    /// Returns an error if an identifier is missing after `get`, `::`, `,`, or
    /// `from`, or if `from` itself is absent in the named-import form.
    pub fn parse_import(&mut self, start: Span) -> Result<Statement, Error> {
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
                        names: vec![name],
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
        if !matches!(self.peek(), TokenType::Comma | TokenType::From) {
            let span = start.join(self.previous_span());
            let path = vec![first];
            self.get_imported_type_names(&path, None);
            return Ok(Statement::new(StatementKind::ImportFile { path }, span));
        }

        // named imports: get add, sub from …
        let mut names = vec![first];
        if self.match_type(&[TokenType::Comma]) {
            loop {
                match self.peek() {
                    TokenType::Identifier(name) => {
                        self.advance();
                        names.push(name);
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
                _ => return Err(self.err("expected path after 'from'", self.peek_span())),
            }
            if !self.match_type(&[TokenType::ColonColon]) {
                break;
            }
        }

        let span = start.join(self.previous_span());
        let is_std = path.first().map(|s| s == "std").unwrap_or(false);

        if is_std {
            Ok(Statement::new(StatementKind::Import { names, path }, span))
        } else {
            self.get_imported_type_names(&path, Some(&names));
            Ok(Statement::new(
                StatementKind::ImportFileNamed { path, names },
                span,
            ))
        }
    }
}
