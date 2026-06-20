use crate::{
    checker::TypeChecker,
    lexer::{tokenizer::Tokenizer, tokentypes::TokenType},
    lsp::utils::{offset_to_position, position_to_offset},
    parser::parser_logic::Parser,
    utils::{source::SourceFile, span::Span},
};
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Range};

/// lex -> parse -> type-check `source` then resolve a hover for the
/// given cursor `position`
/// mirrors `pipeline::run_pipeline` but instead
/// of diagnostics it reads `checker.hovers` the side table of
/// (span, markdown) pairs the checker records at every declaration
/// identifier usage and stdlib call site
pub fn run_hover(source: &str, position: Position) -> Option<Hover> {
    let offset = position_to_offset(source, position);
    let file = SourceFile::new("buffer", source.to_string());

    let tokens = Tokenizer::lex(file.clone()).ok()?;

    // only identifiers carry hover info
    // if not then return
    let token_span = find_identifier_span_at(&tokens, offset)?;

    let statements = Parser::parse(tokens, file.clone()).ok()?;
    let mut checker = TypeChecker::new().with_source_file(file);
    checker.check(&statements);

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
