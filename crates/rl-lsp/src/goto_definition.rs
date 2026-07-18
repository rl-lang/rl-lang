use crate::utils::{offset_to_position, position_to_offset};
use rl_checker::TypeChecker;
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;

use tower_lsp::lsp_types::{GotoDefinitionResponse, Location, Position, Range, Url};

pub fn run_goto_definition(
    source: &str,
    position: Position,
    uri: &Url,
) -> Option<GotoDefinitionResponse> {
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

    let (_, decl_span) = checker
        .definitions
        .iter()
        .filter(|(usage, _)| usage.start <= offset && offset <= usage.end)
        .min_by_key(|(usage, _)| usage.end - usage.start)?;

    Some(GotoDefinitionResponse::Scalar(Location {
        uri: uri.clone(),
        range: Range::new(
            offset_to_position(source, decl_span.start),
            offset_to_position(source, decl_span.end),
        ),
    }))
}
