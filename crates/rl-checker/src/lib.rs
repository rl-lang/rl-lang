//! Static type checker for rl - runs after parsing, before evaluation.
//!
//! The checker walks the AST and verifies:
//! - Variable and constant declarations match their declared types
//! - Binary/unary operators receive compatible operand types
//! - Function calls receive the correct number and types of arguments
//! - `return` types match the enclosing function's declared return type
//! - `break` and `continue` only appear inside loops
//! - Array elements are all the same type
//!
//! It also populates [`TypeChecker::hovers`] - a side-table of
//! `(Span, markdown)` pairs used by the LSP hover provider.
//!
//! # Two-pass function checking
//!
//! [`TypeChecker::check`] does two passes over the statement list:
//! first it pre-declares all top-level `FunctionDeclaration`s so they
//! are visible to each other regardless of order, then it checks every
//! statement body. This allows mutual recursion at the top level.

pub mod operators;
pub mod scope;
pub mod statements;
pub mod structs;
pub mod types;

use crate::structs::CheckType;
use rl_ast::{
    Ast,
    statements::{Statement, StatementKind},
};
use rl_docs::find_fn_doc;
use rl_interpreter::evaluator::Evaluator;
use rl_interpreter::native::Module;
use rl_utils::{
    errors::{Error, Reason},
    source::SourceFile,
    span::Span,
};
use std::{collections::HashMap, path::PathBuf};
pub use structs::TypeChecker;

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        // getting all stdlib modules
        let root_module = Evaluator::default().with_stdlib().root_module;
        let mut stdlib_fn_names = std::collections::HashSet::new();
        collect_fn_names(&root_module, &mut stdlib_fn_names);

        Self {
            scopes: vec![HashMap::new()],
            source_file: None,
            root_module: Evaluator::default().with_stdlib().root_module,
            errors: Vec::new(),
            return_type_stack: Vec::new(),
            loop_depth: 0,
            stdlib_fn_names,
            hovers: Vec::new(),
            base_dir: None,
            importing: Vec::new(),
            imported: HashMap::new(),
            ast_arena: Ast::new(),
            records: HashMap::new(),
            tags: HashMap::new(),
        }
    }

    // functions for source file for ariadne
    pub fn with_source_file(mut self, file: SourceFile) -> Self {
        self.source_file = Some(file);
        self
    }
    pub fn set_source_file(&mut self, file: SourceFile) {
        self.source_file = Some(file);
    }

    pub fn with_base_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.base_dir = Some(dir.into());
        self
    }
    pub fn with_ast_arena(mut self, arena: Ast) -> Self {
        self.ast_arena = arena;
        self
    }

    // runs check on every ast statement in the list and returns errors as list
    pub fn check(&mut self, statements: &[Statement]) -> &[Error] {
        for statement in statements {
            if let StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                ..
            } = &statement.kind
            {
                let fn_type = CheckType::Function {
                    params: params.iter().map(|p| p.param_type.clone()).collect(),
                    return_type: return_type.clone(),
                };
                self.declare(name.clone(), fn_type, false, statement.span);
            }
            if let StatementKind::RecordDeclaration { name, fields } = &statement.kind {
                self.records.insert(name.clone(), fields.clone());
            }
            if let StatementKind::TagDeclaration { name, variants } = &statement.kind {
                self.tags.insert(name.clone(), variants.clone());
            }
        }
        for statement in statements {
            self.check_statement(statement);
        }
        &self.errors
    }

    // transforms arguments into Error type
    // for message it accepts str and String types
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        let err = Error::at(Reason::Compile, message, span);
        match &self.source_file {
            Some(file) => err.with_source_file(file),
            None => err,
        }
    }

    // transforms the arguments into Error type via err() functions
    // and pushes the error to the errors field
    pub fn error(&mut self, message: impl Into<String>, span: Span) {
        let err = self.err(message, span);
        self.errors.push(err);
    }
    // same as error() but with helper
    pub fn error_with_help(&mut self, message: impl Into<String>, span: Span, help: Option<&str>) {
        let mut err = self.err(message, span);
        if let Some(h) = help {
            err = err.with_help(format!("did you mean `{}`?", h));
        }
        self.errors.push(err);
    }

    // adds markdown hover text for a source span
    pub fn push_hover(&mut self, span: Span, text: impl Into<String>) {
        self.hovers.push((span, text.into()));
    }

    // using find_fn_doc() in `crate::docs` find the current docs
    // for the function and add the markdown hover for the span of fn
    fn push_stdlib_hover(&mut self, path: &[String], span: Span) {
        let fn_name = match path.last() {
            Some(n) => n.as_str(),
            None => return,
        };
        // get the module to handle std::io::print()
        // and get print from std::io
        let module = if path.len() >= 2 {
            Some(path[path.len() - 2].as_str())
        } else {
            None
        };

        let text = match find_fn_doc(module, fn_name).or_else(|| find_fn_doc(None, fn_name)) {
            Some((std_entry, func)) => format!(
                "```rl\nstd::{}::{}\n```\n{}",
                std_entry.name, func.signature, func.description
            ),
            None => format!("```rl\nfn {}(..)\n```\nstdlib function", fn_name),
        };

        self.push_hover(span, text);
    }
}

/// Collects all function names from a stdlib [`Module`] tree into `out`.
fn collect_fn_names(module: &Module, out: &mut std::collections::HashSet<String>) {
    for name in module.functions.keys() {
        out.insert(name.clone());
    }
    for sub in module.submodules.values() {
        collect_fn_names(sub, out);
    }
}
