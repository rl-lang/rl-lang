use crate::utils::{offset_to_position, position_to_offset};
use rl_checker::TypeChecker;
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::{source::SourceFile, span::Span};

use tower_lsp::lsp_types::{Location, Position, Range, Url};

pub fn run_references(
    source: &str,
    position: Position,
    uri: &Url,
    include_declaration: bool,
) -> Option<Vec<Location>> {
    let offset = position_to_offset(source, position);
    let file_name = uri
        .to_file_path()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "buffer".to_string());
    let file = SourceFile::new(file_name, source.to_string());

    let tokens = Tokenizer::lex(file.clone()).ok()?;
    let (ast, statements) = Parser::parse(tokens, file.clone()).ok()?;

    let base_dir = uri
        .to_file_path()
        .ok()
        .and_then(|p| p.parent().map(std::path::Path::to_path_buf))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let mut checker = TypeChecker::new()
        .with_source_file(file)
        .with_ast_arena(ast)
        .with_base_dir(base_dir);
    checker.check(&statements);

    // cursor can be on a usage OR right on the declaration itself - check both
    let target_decl: Span = checker
        .definitions
        .iter()
        .find(|(usage, _)| usage.start <= offset && offset <= usage.end)
        .map(|(_, decl)| *decl)
        .or_else(|| {
            checker
                .definitions
                .iter()
                .find(|(_, decl)| decl.start <= offset && offset <= decl.end)
                .map(|(_, decl)| *decl)
        })?;

    let mut spans: Vec<Span> = checker
        .definitions
        .iter()
        .filter(|(_, decl)| *decl == target_decl)
        .map(|(usage, _)| *usage)
        .collect();

    if include_declaration {
        spans.push(target_decl);
    }

    spans.sort_by_key(|s| s.start);
    spans.dedup();

    Some(
        spans
            .into_iter()
            .map(|span| Location {
                uri: uri.clone(),
                range: Range::new(
                    offset_to_position(source, span.start),
                    offset_to_position(source, span.end),
                ),
            })
            .collect(),
    )
}
