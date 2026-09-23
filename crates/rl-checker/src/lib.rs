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
pub mod units;

use crate::structs::CheckType;
use rl_ast::{
    Ast,
    statements::{FunctionAttribute, Lint, ProgramAttribute, Statement, StatementKind},
};
use rl_docs::find_fn_doc;
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
        let root_module = rl_commons::stdlib_names();
        let mut stdlib_fn_names = HashMap::new();
        root_module.collect_fn_names(&mut stdlib_fn_names);

        Self {
            scopes: vec![HashMap::new()],
            source_file: None,
            root_module: rl_commons::stdlib_names(),
            errors: Vec::new(),
            warnings: Vec::new(),
            return_type_stack: Vec::new(),
            loop_depth: 0,
            stdlib_fn_names,
            imported_std_fns: HashMap::new(),
            imported_std_paths: HashMap::new(),
            hovers: Vec::new(),
            definitions: Vec::new(),
            record_spans: HashMap::new(),
            tag_spans: HashMap::new(),
            base_dir: None,
            importing: Vec::new(),
            imported: HashMap::new(),
            ast_arena: Ast::new(),
            records: HashMap::new(),
            tags: HashMap::new(),
            methods: HashMap::new(),
            conversions: crate::units::ConversionTable::default(),
            allow_stack: Vec::new(),
            has_explicit_entry: false,
            deprecated_stdlib: Self::build_deprecated_stdlib_map(),
        }
    }

    /// Builds the map of deprecated stdlib function paths to their messages.
    fn build_deprecated_stdlib_map() -> HashMap<Vec<String>, String> {
        let mut m = HashMap::new();
        m.insert(
            vec!["std".into(), "array".into(), "len".into()],
            "use std::len instead".into(),
        );
        // std::rl metaprogramming is deprecated
        for name in [
            "lex",
            "eval",
            "eval_isolated",
            "check",
            "rl_version",
            "source_name",
        ] {
            m.insert(
                vec!["std".into(), "rl".into(), name.into()],
                "std::rl is deprecated and may be removed in a future version".into(),
            );
        }
        // fs reorg: io file ops -> fs
        m.insert(
            vec!["std".into(), "io".into(), "read_file".into()],
            "use std::fs::read_file instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "read_lines".into()],
            "use std::fs::read_lines instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "read_bytes".into()],
            "use std::fs::read_bytes instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "write_file".into()],
            "use std::fs::write_file instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "append_file".into()],
            "use std::fs::append_file instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "delete_file".into()],
            "use std::fs::delete_file instead".into(),
        );
        // fs reorg: path syscall ops -> fs
        m.insert(
            vec!["std".into(), "path".into(), "path_exists".into()],
            "use std::fs::path_exists instead".into(),
        );
        m.insert(
            vec!["std".into(), "path".into(), "path_is_dir".into()],
            "use std::fs::path_is_dir instead".into(),
        );
        m.insert(
            vec!["std".into(), "path".into(), "path_is_file".into()],
            "use std::fs::path_is_file instead".into(),
        );
        m.insert(
            vec!["std".into(), "path".into(), "path_canonicalize".into()],
            "use std::fs::path_canonicalize instead".into(),
        );
        m.insert(
            vec!["std".into(), "path".into(), "path_absolute".into()],
            "use std::fs::path_absolute instead".into(),
        );
        m.insert(
            vec!["std".into(), "path".into(), "path_expand_home".into()],
            "use std::fs::path_expand_home instead".into(),
        );
        m.insert(
            vec!["std".into(), "path".into(), "path_relative".into()],
            "use std::fs::path_relative instead".into(),
        );
        // fs reorg: io handle ops -> fs
        m.insert(
            vec!["std".into(), "io".into(), "open".into()],
            "use std::fs::open instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "close".into()],
            "use std::fs::close instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "read_handle".into()],
            "use std::fs::read_handle instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "write_handle".into()],
            "use std::fs::write_handle instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "seek".into()],
            "use std::fs::seek instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "flush".into()],
            "use std::fs::flush instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "read_all".into()],
            "use std::fs::read_all instead".into(),
        );
        m.insert(
            vec!["std".into(), "io".into(), "readline".into()],
            "use std::fs::readline instead".into(),
        );
        m
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
        for attr in &self.ast_arena.program_attributes {
            // Custom `define` markers need no checker setup; only unit
            // conversions register here.
            let ProgramAttribute::Convert {
                symbol,
                factor,
                base_symbol,
            } = attr
            else {
                continue;
            };
            self.conversions.insert(symbol, *factor, base_symbol);
        }
        for statement in statements {
            if let StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                attribute,
                ..
            } = &statement.kind
            {
                if matches!(attribute, Some(FunctionAttribute::Entry)) {
                    self.has_explicit_entry = true;
                }
                let fn_type = CheckType::Function {
                    params: params.iter().map(|p| p.param_type.clone()).collect(),
                    return_type: return_type.clone(),
                };
                self.declare(name.clone(), fn_type, false, statement.span);
                // Runtime-called attributes: these are invoked by the runtime,
                // not user code - mark as used to suppress unused warnings.
                if matches!(
                    attribute,
                    Some(FunctionAttribute::Entry | FunctionAttribute::Init(_) | FunctionAttribute::Final(_) | FunctionAttribute::Test)
                )
                    && let Some(scope) = self.scopes.last_mut()
                        && let Some(item) = scope.get_mut(name) {
                            item.used = true;
                        }
            }
            if let StatementKind::RecordDeclaration { name, fields } = &statement.kind {
                self.records.insert(name.clone(), fields.clone());
                self.record_spans.insert(name.clone(), statement.span);
            }
            if let StatementKind::TagDeclaration { name, variants } = &statement.kind {
                self.tags.insert(name.clone(), variants.clone());
                self.tag_spans.insert(name.clone(), statement.span);
            }
            if let StatementKind::ImplBlock { record, methods } = &statement.kind {
                for m in methods {
                    if let StatementKind::FunctionDeclaration {
                        name,
                        params,
                        return_type,
                        ..
                    } = &m.kind
                    {
                        let fn_type = CheckType::Function {
                            params: params.iter().map(|p| p.param_type.clone()).collect(),
                            return_type: return_type.clone(),
                        };
                        self.methods.insert((record.clone(), name.clone()), fn_type);
                    }
                }
            }
        }
        for statement in statements {
            self.check_statement(statement);
        }
        self.report_unused_in_root_scope();
        &self.errors
    }

    pub fn warn(&mut self, message: impl Into<String>, span: Span) {
        self.warnings
            .push(self.err(message.into(), span).as_warning());
    }

    /// Emits a warning only if the given `lint` is not suppressed by an
    /// enclosing `!#[allow(...)]` attribute.
    pub fn warn_lint(&mut self, lint: Lint, message: impl Into<String>, span: Span) {
        let suppressed = self.allow_stack.iter().any(|set| set.contains(&lint));
        if !suppressed {
            self.warn(message, span);
        }
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
            Some((std_entry, func)) => {
                // `core` is top-level, not under `std::`.
                let path = if std_entry.name == "core" {
                    format!("core::{}", func.signature)
                } else {
                    format!("std::{}::{}", std_entry.name, func.signature)
                };
                format!("```rl\n{}\n```\n{}", path, func.description)
            }
            None => format!("```rl\nfn {}(..)\n```\nstdlib function", fn_name),
        };

        self.push_hover(span, text);
    }
}
