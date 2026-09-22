//! Core data structures for the type checker.
//!
use crate::units::{ConversionTable, Unit};
use rl_ast::{
    Ast,
    statements::{Lint, TypeAnnotation},
};
use rl_commons::ModuleNames;
use rl_utils::{errors::Error, source::SourceFile, span::Span};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

/// The stateful type checker, threaded through the entire AST walk.
pub struct TypeChecker {
    /// Stack of scopes, each mapping variable/function names to their [`ScopeItem`].
    pub scopes: Vec<HashMap<String, ScopeItem>>,
    /// Source file attached for Ariadne error rendering; `None` in LSP-less contexts.
    pub source_file: Option<SourceFile>,
    /// The stdlib module tree, used to resolve stdlib call paths.
    pub root_module: ModuleNames,
    /// All type errors accumulated during the check pass.
    pub errors: Vec<Error>,
    /// Warnings accumulated during the check pass (e.g. unused variables).
    pub warnings: Vec<Error>,
    /// Stack of expected return types, pushed/popped on function and lambda entry/exit.
    pub return_type_stack: Vec<TypeAnnotation>,
    /// Nesting depth of loops - used to validate `break` and `continue`.
    pub loop_depth: u32,
    /// Flat map of all stdlib function names to their (possibly-untyped)
    /// signature, for fast single-name lookup (`print` vs `std::io::print`).
    pub stdlib_fn_names: HashMap<String, rl_commons::StdFn>,
    pub imported_std_fns: HashMap<String, rl_commons::StdFn>,
    /// Stdlib function deprecation map: full path -> deprecation message.
    pub deprecated_stdlib: HashMap<Vec<String>, String>,
    /// Canonical stdlib path per visible imported name (aliases resolve to
    /// their original path, wildcards to their namespace). Used to warn on
    /// deprecated functions called by bare name.
    pub imported_std_paths: HashMap<String, Vec<String>>,
    /// `(span, markdown)` pairs collected at every declaration and usage site,
    /// consumed by the LSP hover provider.
    pub hovers: Vec<(Span, String)>,
    /// `(usage_span, declaration_span)` pairs consumed by the LSP goto-defination provider
    pub definitions: Vec<(Span, Span)>,
    /// Declaration span of each `record` type name keyed by name
    pub record_spans: HashMap<String, Span>,
    /// Declaration span of each `tag` type name keyed by name
    pub tag_spans: HashMap<String, Span>,
    pub base_dir: Option<PathBuf>,
    pub importing: Vec<PathBuf>,
    pub imported: HashMap<PathBuf, Option<HashSet<String>>>,
    pub ast_arena: Ast,
    /// Maps `record` type names to their declared `(field name, field type)` list.
    pub records: HashMap<String, Vec<(String, TypeAnnotation)>>,
    /// Maps `tag` (enum) type names to their declared variant name list.
    pub tags: HashMap<String, Vec<String>>,
    /// Maps `(record name, method name)` to its checked function signature,
    /// populated during the pre-scan pass from `ImplBlock` statements.
    /// Instance methods (with a leading `self` param) include `self`'s
    /// record type as their first param, matching how `MethodCall` prepends
    /// the caller as arg 0.
    pub methods: HashMap<(String, String), CheckType>,
    /// Conversion registry built from `#![convert(symbol=factor(base))]`
    /// program attributes, used to treat convertible unit symbols as equal.
    pub conversions: ConversionTable,
    /// Stack of suppressed lints, pushed/popped around attributed statements.
    pub allow_stack: Vec<HashSet<Lint>>,
    /// Whether any top-level function is explicitly marked `!#[entry]`.
    pub has_explicit_entry: bool,
}

/// A single entry in a type checker scope.
pub struct ScopeItem {
    /// The static type of this variable or function.
    pub type_annotation: CheckType,
    /// The compile-time unit of measure attached to this binding, if any
    /// (`dec float speed: m/s = 12.5`). Only numeric bindings may carry one.
    pub unit: Option<Unit>,
    /// Whether this binding is immutable (`CONST`).
    pub is_const: bool,
    pub decl_span: Span,
    pub used: bool,
    /// Lints suppressed for this binding (e.g. `!#[allow(unused)]`).
    pub suppressed_lints: HashSet<Lint>,
    /// Deprecation message, if declared with `!#[deprecated("msg")]`.
    pub deprecated: Option<String>,
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

/// The static type and optional unit of a checked expression.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckedExpr {
    /// The static type of the expression.
    pub ty: CheckType,
    /// The compile-time unit of measure of the expression, if any.
    pub unit: Option<Unit>,
}

impl CheckedExpr {
    pub fn new(ty: CheckType, unit: Option<Unit>) -> Self {
        Self { ty, unit }
    }

    /// Converts the type to its constant variant, keeping the unit intact.
    pub fn into_const(self) -> Self {
        CheckedExpr::new(self.ty.into_const(), self.unit)
    }
}
