//! Hover resolution: lex -> parse -> type-check -> pick the smallest matching span.
//!
//! The [`TypeChecker`] populates a `hovers` side-table of `(Span, markdown)`
//! pairs at every declaration, identifier usage, and stdlib call site.
//! [`run_hover`] finds the entry whose span contains the cursor offset and
//! whose span is the smallest (to prefer the most specific match when multiple
//! spans overlap on the same line).
use crate::{
    checker::TypeChecker,
    lexer::{tokenizer::Tokenizer, tokentypes::TokenType},
    lsp::utils::{offset_to_position, position_to_offset},
    parser::parser_logic::Parser,
    utils::{source::SourceFile, span::Span},
};
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Range, Url};

/// Runs the full pipeline on `source` and returns a [`Hover`] for `position`,
/// or `None` if the cursor is not over a hoverable token.
///
/// Only [`TokenType::Identifier`] tokens carry hover info - non-identifier
/// positions short-circuit early before running the parser and type-checker.
pub fn run_hover(source: &str, position: Position, uri: &Url) -> Option<Hover> {
    let offset = position_to_offset(source, position);
    let file = SourceFile::new("buffer", source.to_string());

    let tokens = Tokenizer::lex(file.clone()).ok()?;

    // only identifiers carry hover info
    // if not then return
    let token_span = find_identifier_span_at(&tokens, offset)?;

    let statements = Parser::parse(tokens, file.clone()).ok()?;
    let (ast, statements) = statements;
    let mut checker = TypeChecker::new().with_source_file(file);
    if let Ok(doc_path) = uri.to_file_path()
        && let Some(doc_dir) = doc_path.parent()
    {
        checker = checker.with_base_dir(doc_dir.to_path_buf());
    }
    checker.check(ast, &statements);

    // since several spans can exists on same line
    // pick the smallest span
    let (smallest_span, text) = checker
        .hovers
        .iter()
        .filter(|(span, _)| span.start <= offset && offset <= span.end)
        .min_by_key(|(span, _)| span.end - span.start)?;

    let range_span =
        if token_span.start >= smallest_span.start && token_span.end <= smallest_span.end {
            token_span
        } else {
            *smallest_span
        };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: text.clone(),
        }),
        range: Some(Range::new(
            offset_to_position(source, range_span.start),
            offset_to_position(source, range_span.end),
        )),
    })
}

/// Returns the [`Span`] of the identifier token that contains `offset`,
/// or `None` if no identifier token covers that position.
fn find_identifier_span_at(
    tokens: &[crate::lexer::tokentypes::Token],
    offset: usize,
) -> Option<Span> {
    tokens
        .iter()
        .find(|t| {
            matches!(t.token, TokenType::Identifier(_))
                && t.span.start <= offset
                && offset <= t.span.end
        })
        .map(|t| t.span)
}
