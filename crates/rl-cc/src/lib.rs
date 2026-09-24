pub mod codegen;
pub mod name_mangle;
pub mod runtime;
pub mod types;
pub mod writer;

use rl_ast::{Ast, statements::Statement};
use rl_checker::structs::TypeChecker;
use rl_utils::{
    errors::{Error, Reason},
    span::Span,
};

pub struct TranspileConfig {
    pub embed_runtime: bool,
    pub output_dir: std::path::PathBuf,
    pub output_name: String,
}

pub struct TranspileResult {
    pub c_source: String,
    pub c_path: std::path::PathBuf,
    pub runtime_paths: Option<(std::path::PathBuf, std::path::PathBuf)>,
}

pub fn transpile(
    ast: &Ast,
    statements: &[Statement],
    checker: &TypeChecker,
    config: &TranspileConfig,
) -> Result<TranspileResult, Vec<Error>> {
    let mut codegen = codegen::CCodegen::new(ast, checker);

    let c_source = codegen.emit_program(statements).map_err(|e| vec![e])?;

    let c_path = config.output_dir.join(format!("{}.c", config.output_name));

    std::fs::write(&c_path, &c_source)
        .map_err(|e| vec![Error::at(Reason::Compile, e.to_string(), Span::dummy())])?;

    let runtime_paths = if config.embed_runtime {
        let h_path = config.output_dir.join("rl_runtime.h");
        let c_path = config.output_dir.join("rl_runtime.c");
        std::fs::write(&h_path, runtime::RUNTIME_H)
            .map_err(|e| vec![Error::at(Reason::Compile, e.to_string(), Span::dummy())])?;
        std::fs::write(&c_path, runtime::RUNTIME_C)
            .map_err(|e| vec![Error::at(Reason::Compile, e.to_string(), Span::dummy())])?;
        // Single-header audio backend the runtime includes; emitted next
        // to rl_runtime.c so `#include "miniaudio.h"` resolves when the
        // transpiled program compiles.
        std::fs::write(config.output_dir.join("miniaudio.h"), runtime::RUNTIME_MINIAUDIO)
            .map_err(|e| vec![Error::at(Reason::Compile, e.to_string(), Span::dummy())])?;
        Some((h_path, c_path))
    } else {
        None
    };

    Ok(TranspileResult {
        c_source,
        c_path,
        runtime_paths,
    })
}
