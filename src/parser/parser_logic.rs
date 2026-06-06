use crate::{
    ast::statements::Statement,
    lexer::tokentypes::Token,
    utils::{
        errors::{Error, Reason},
        source::SourceFile,
        span::Span,
    },
};

/// parses list of tokens and produce [`Vec<Statement>`]
pub struct Parser {
    /// the source file (text + name) for error reports
    pub source_file: SourceFile,
    /// the full tokens list from the lexer
    pub tokens: Vec<Token>,
    /// index of current token that being parsed
    pub current: usize,
}

impl Parser {
    /// consumes tokens to return a list of [`Statement`]s
    pub fn parse(tokens: Vec<Token>, source_file: SourceFile) -> Result<Vec<Statement>, Error> {
        let mut parser = Parser {
            source_file,
            tokens,
            current: 0,
        };
        log::info!("parser initialized");
        let mut statements = Vec::new();

        while !parser.is_at_end() {
            statements.push(parser.parse_statement_to_ast()?);
        }

        log::info!("parsing complete");
        Ok(statements)
    }

    /// build a [`Reason::Parse`] error anchored at `span`, with the source attached.
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        Error::at(Reason::Parse, message, span).with_source_file(&self.source_file)
    }
}
