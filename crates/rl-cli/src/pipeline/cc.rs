use rl_checker::TypeChecker;
use rl_resolver::Resolver;
use rl_utils::source::SourceFile;

use super::lex::lex;
use super::parse::parse;

/// Transpile a .rl source file to C via the rl-cc crate.
///
/// Returns the path to the emitted `.c` file.
pub fn transpile_loop(
    file: &std::path::Path,
    output: Option<std::path::PathBuf>,
    embed_runtime: bool,
    test_mode: bool,
    match_pattern: Option<String>,
) -> std::path::PathBuf {
    let path = file
        .to_str()
        .unwrap_or_else(|| {
            eprintln!("error: invalid file path");
            std::process::exit(1);
        })
        .to_string();

    let source_text = std::fs::read_to_string(file).unwrap_or_else(|_| {
        eprintln!("error: could not read file '{}'", file.display());
        std::process::exit(1);
    });

    let source = SourceFile::new(&*path, source_text);
    let tokens = lex(source.clone());
    let (ast, statements) = parse(source.clone(), tokens);

    // Resolve
    let mut resolver = Resolver::new();
    resolver.current_dir = std::path::Path::new(source.name.as_ref())
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .to_path_buf();
    let resolved_statements = resolver.resolve_program(ast, statements);

    // Type-check
    let checker_tokens = lex(source.clone());
    let (checker_ast, checker_statements) = parse(source.clone(), checker_tokens);
    let base_dir = file
        .parent()
        .map(std::path::Path::to_path_buf)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let mut checker = TypeChecker::new()
        .with_source_file(source.clone())
        .with_ast_arena(checker_ast)
        .with_base_dir(base_dir.clone());
    checker.check(&checker_statements);
    for w in &checker.warnings {
        w.report_to_stderr();
    }
    if !checker.errors.is_empty() {
        for e in &checker.errors {
            e.report_to_stderr();
        }
        std::process::exit(1);
    }

    // Transpile
    let output_dir = output
        .as_ref()
        .and_then(|p| p.parent().map(std::path::Path::to_path_buf))
        .unwrap_or_else(|| base_dir.clone());

    let output_name = output
        .as_ref()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| {
            file.file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "output".to_string())
        });

    let config = rl_cc::TranspileConfig {
        embed_runtime,
        output_dir,
        output_name,
        test_mode,
        match_pattern,
    };

    let result = rl_cc::transpile(&resolver.ast_arena, &resolved_statements, &checker, &config).unwrap_or_else(|e| {
        for err in e {
            err.report_to_stderr();
        }
        std::process::exit(1);
    });

    println!("transpiled '{}' -> '{}'", file.display(), result.c_path.display());
    if let Some((h, c)) = &result.runtime_paths {
        println!("runtime   '{}' + '{}'", h.display(), c.display());
    }

    result.c_path
}
