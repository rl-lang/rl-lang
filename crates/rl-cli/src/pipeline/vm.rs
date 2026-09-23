use rl_ast::{Ast, statements::Statement};
use rl_resolver::Resolver;
use rl_utils::source::SourceFile;

#[cfg(feature = "vm")]
use rl_utils::line_index::LineIndex;

pub fn resolve(source: &SourceFile, ast: Ast, statements: Vec<Statement>) -> (Ast, Vec<Statement>) {
    let mut resolver = Resolver::new();
    resolver.current_dir = std::path::Path::new(source.name.as_ref())
        .parent()
        .unwrap_or(std::path::Path::new(""))
        .to_path_buf();

    let resolved = resolver.resolve_program(ast, statements);
    (resolver.ast_arena, resolved)
}

#[cfg(feature = "vm")]
pub fn compile_to_chunk(source: SourceFile, ast: Ast, statements: Vec<Statement>) -> rl_vm::Chunk {
    use rl_vm::Compiler;

    let (_arena, resolved) = resolve(&source, ast, statements);

    match Compiler::new(&_arena)
        .with_source_file(source.clone())
        .compile(&resolved)
    {
        Ok(c) => c,
        Err(e) => {
            e.report_to_stderr();
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "vm")]
pub fn run_chunk(chunk: &rl_vm::Chunk, source: Option<SourceFile>, line_index: Option<LineIndex>) {
    use rl_vm::Vm;

    let mut vm = Vm::new();
    if let Some(source) = source {
        vm = vm.with_source_file(source);
    } else if let Some(index) = line_index {
        vm = vm.with_line_index(index);
    }
    match vm.run(chunk) {
        Ok(_) => {}
        Err(e) => {
            e.report_to_stderr();
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "vm")]
pub fn vm_loop(source: SourceFile, ast: Ast, statements: Vec<Statement>) {
    let chunk = compile_to_chunk(source.clone(), ast, statements);
    run_chunk(&chunk, Some(source), None);
}

#[cfg(feature = "vm")]
pub fn run_rlc_bytes(bytes: &[u8], label: &str) {
    use rl_vm::{deserialize_chunk, stdlib};

    let (chunk, line_index) = match deserialize_chunk(bytes, &stdlib::root()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to load '{}': {}", label, e);
            std::process::exit(1);
        }
    };

    run_chunk(&chunk, None, line_index);
}

#[cfg(feature = "vm")]
pub fn run_rlc_file(path: &std::path::Path) {
    let bytes = std::fs::read(path).unwrap_or_else(|_| {
        eprintln!("error: could not read file '{}'", path.display());
        std::process::exit(1);
    });

    run_rlc_bytes(&bytes, &path.display().to_string());
}
