//! Core [`Parser`] struct and its cursor primitives.
//!
//! Every other parser sub-module is an `impl Parser` block that depends on
//! the methods defined here. Nothing in this file produces AST nodes directly;
//! it only provides the machinery for navigating the token stream.
use crate::{
    ast::{Ast, statements::Statement},
    lexer::tokentypes::Token,
    utils::{
        errors::{Error, Reason},
        source::SourceFile,
        span::Span,
    },
};

/// Parses a flat list of tokens and produces a [`Vec<Statement>`].
///
/// The parser owns the token stream and advances through it with a single
/// `current` index cursor. All sub-parsers borrow `self` mutably and call
/// the cursor primitives ([`peek`], [`advance`], [`match_type`], etc.) defined
/// in this module.
///
/// Construct via [`Parser::parse`] - there is no public `new`.
///
/// [`peek`]: Parser::peek
/// [`advance`]: Parser::advance
/// [`match_type`]: Parser::match_type
pub struct Parser {
    /// the source file (text + name) carried for error reports
    pub source_file: SourceFile,
    /// the full token list produced by the lexer, including the terminal [`TokenType::Eof`]
    pub tokens: Vec<Token>,
    /// index of the token currently being examined (the "read head")
    pub current: usize,
    pub ast_arena: Ast,
}

impl Parser {
    /// Entry point: consumes `tokens` and returns a fully-parsed statement list.
    ///
    /// Drives the top-level parse loop, calling [`parse_statement_to_ast`] until
    /// [`TokenType::Eof`] is reached.
    ///
    /// # Errors
    /// Returns the first [`Error`] encountered; parsing stops immediately.
    ///
    /// [`parse_statement_to_ast`]: Parser::parse_statement_to_ast
    pub fn parse(
        tokens: Vec<Token>,
        source_file: SourceFile,
    ) -> Result<(Ast, Vec<Statement>), Error> {
        let mut parser = Parser {
            source_file,
            tokens,
            current: 0,
            ast_arena: Ast::new(),
        };

        #[cfg(feature = "debug")]
        log::info!("parser initialized");
        let mut statements = Vec::new();

        while !parser.is_at_end() {
            statements.push(parser.parse_statement_to_ast()?);
        }

        #[cfg(feature = "debug")]
        log::info!("parsing complete");
        Ok((parser.ast_arena, statements))
    }

    /// Constructs a [`Reason::Parse`] error anchored at `span` with the source
    /// file already attached, ready to be returned from any parse method.
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        Error::at(Reason::Parse, message, span).with_source_file(&self.source_file)
    }
}
