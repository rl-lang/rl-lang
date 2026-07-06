//! Core data structures for the type checker.
//!
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{
    ast::{Ast, statements::TypeAnnotation},
    interpreter::native::Module,
    utils::{errors::Error, source::SourceFile, span::Span},
};

/// The stateful type checker, threaded through the entire AST walk.
pub struct TypeChecker {
    /// Stack of scopes, each mapping variable/function names to their [`ScopeItem`].
    pub scopes: Vec<HashMap<String, ScopeItem>>,
    /// Source file attached for Ariadne error rendering; `None` in LSP-less contexts.
    pub source_file: Option<SourceFile>,
    /// The stdlib module tree, used to resolve stdlib call paths.
    pub root_module: Module,
    /// All type errors accumulated during the check pass.
    pub errors: Vec<Error>,
    /// Stack of expected return types, pushed/popped on function and lambda entry/exit.
    pub return_type_stack: Vec<TypeAnnotation>,
    /// Nesting depth of loops - used to validate `break` and `continue`.
    pub loop_depth: u32,
    /// Flat set of all stdlib function names for fast single-name lookup.
    pub stdlib_fn_names: std::collections::HashSet<String>,
    /// `(span, markdown)` pairs collected at every declaration and usage site,
    /// consumed by the LSP hover provider.
    pub hovers: Vec<(Span, String)>,
    pub base_dir: Option<PathBuf>,
    pub importing: Vec<PathBuf>,
    pub imported: HashSet<PathBuf>,
    pub ast_arena: Ast,
    /// Maps `record` type names to their declared `(field name, field type)` list.
    pub records: HashMap<String, Vec<(String, TypeAnnotation)>>,
    /// Maps `tag` (enum) type names to their declared variant name list.
    pub tags: HashMap<String, Vec<String>>,
}

/// A single entry in a type checker scope.
pub struct ScopeItem {
    /// The static type of this variable or function.
    pub type_annotation: CheckType,
    /// Whether this binding is immutable (`CONST`).
    pub is_const: bool,
}

/// The type of a value as seen by the static checker.
#[derive(Debug, Clone, PartialEq)]
pub enum CheckType {
    /// A fully resolved type (e.g. `int`, `arr[string]`).
    Known(TypeAnnotation),
    /// A function type with known parameter and return types.
    Function {
        params: Vec<TypeAnnotation>,
        return_type: TypeAnnotation,
    },
    /// Type could not be determined statically (stdlib calls, unresolved names).
    /// Propagates silently to avoid cascading false errors.
    Unknown,
}
