//! Statement AST nodes, type annotations, and parameter definitions.
//!
//! A [`Statement`] is any construct that does not directly produce a value
//! (declarations, control flow, imports). Every node carries the source [`Span`]
//! it was parsed from.
//!
//! # Resolved variants
//! The [`Resolver`] pass rewrites name-based declaration and loop variants into
//! their `Resolved*` counterparts, adding a `slot: usize` field that gives the
//! variable's index in its environment frame. This eliminates all runtime
//! HashMap lookups in the evaluator.
//!
//! # Type annotations
//! [`TypeAnnotation`] distinguishes mutable (`dec`) and constant (`const`)
//! bindings at the type level via separate variants (`Int` vs `CInt`, etc.).
//!
//! [`Resolver`]: crate::resolver
use std::rc::Rc;

use crate::ast::ExprId;
use crate::utils::span::Span;

/// A statement paired with its source span.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    /// A mutable variable declaration: `dec T name = value`.
    VariableDeclaration {
        name: String,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// Resolver-annotated mutable variable declaration. `slot` is the index
    /// in the current environment frame.
    ResolvedVariableDeclaration {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// An immutable constant declaration: `const T NAME = value`.
    ConstantDeclaration {
        name: String,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// Resolver-annotated constant declaration.
    ResolvedConstantDeclaration {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// A mutable array declaration with an inline literal: `dec array[T] name = [items]`.
    Array {
        name: String,
        type_annotation: TypeAnnotation,
        value: Vec<ExprId>,
    },
    /// An immutable array declaration with an inline literal: `const array[T] NAME = [items]`.
    ConstantArray {
        name: String,
        type_annotation: TypeAnnotation,
        value: Vec<ExprId>,
    },
    /// Resolver-annotated mutable array declaration. `value` is the
    /// initialiser expression (may be an [`ExpressionKind::ArrayLiteral`]).
    ///
    /// [`ExpressionKind::ArrayLiteral`]: crate::ast::nodes::ExpressionKind::ArrayLiteral
    ResolvedArray {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// Resolver-annotated constant array declaration.
    ResolvedConstantArray {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// A bare expression used as a statement (e.g. a function call whose
    /// return value is discarded, or a newline placeholder).
    Expression(ExprId),
    /// A `while condition { body }` loop.
    While {
        condition: ExprId,
        body: Vec<Statement>,
    },
    /// A C-style `for [init, cond, incr] { body }` loop.
    For {
        initializer: Box<Statement>,
        condition: ExprId,
        increment: ExprId,
        body: Vec<Statement>,
    },
    /// Resolver-annotated C-style for loop.
    ResolvedFor {
        initializer: Box<Statement>,
        condition: ExprId,
        increment: ExprId,
        body: Vec<Statement>,
    },
    /// A range-based `for x in N..M { body }` loop. The range is pre-evaluated
    /// at parse time into a [`Range`] statement.
    ForRange {
        variable: String,
        range: Box<Statement>,
        body: Vec<Statement>,
    },
    /// Resolver-annotated range-based for loop. `slot` is the loop variable's
    /// environment index.
    ResolvedForRange {
        slot: usize,
        variable: String,
        range: Box<Statement>,
        body: Vec<Statement>,
    },
    /// A foreach `for item in iterable { body }` loop over an array expression.
    ForEach {
        variable: String,
        iterable: ExprId,
        body: Vec<Statement>,
    },
    /// Resolver-annotated foreach loop.
    ResolvedForEach {
        slot: usize,
        variable: String,
        iterable: ExprId,
        body: Vec<Statement>,
    },
    /// A pre-evaluated integer range produced by the parser for `for x in N..M`.
    Range(Vec<i64>),
    /// A single branch of a conditional: either `if condition { body }` (`condition`
    /// is `Some`) or `else { body }` (`condition` is `None`).
    ConditionalBranch {
        condition: Option<ExprId>,
        body: Vec<Statement>,
    },
    /// A full if / else-if / else chain.
    Conditional {
        if_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    /// A named function declaration.
    FunctionDeclaration {
        name: String,
        params: Vec<Param>,
        return_type: TypeAnnotation,
        body: Vec<Statement>,
        /// `true` when the function is marked with `!#[entry]`.
        attribute: Option<FunctionAttribute>,
    },
    /// Resolver-annotated function declaration. `slot` is the function's
    /// index in the current environment frame.
    ResolvedFunctionDeclaration {
        name: String,
        slot: usize,
        params: Vec<Param>,
        return_type: TypeAnnotation,
        body: Vec<Statement>,
        attribute: Option<FunctionAttribute>,
    },
    /// A `return expr` or bare `return` statement.
    Return(Option<ExprId>),
    /// Breaks out of the nearest enclosing loop.
    Break,
    /// Skips to the next iteration of the nearest enclosing loop.
    Continue,
    /// A stdlib import: `get std::ns::fn` or `get fn from std::ns`.
    Import {
        /// names of the imported functions
        names: Vec<String>,
        /// module path (e.g. `["std", "math"]`)
        path: Vec<String>,
    },
    /// A file module import: `get mymodule` or `get mymodule::sub`.
    ImportFile { path: Vec<String> },
    /// A resolved file import - the imported file's statements are inlined here.
    ResolvedImportFile {
        path: Vec<String>,
        body: Vec<Statement>,
    },
    /// A named file import: `get fn, fn from mymodule::sub`.
    ImportFileNamed {
        path: Vec<String>,
        names: Vec<String>,
    },

    DestructureDeclaration {
        bindings: Vec<(TypeAnnotation, String)>,
        value: ExprId,
    },
    ResolvedDestructureDeclaration {
        bindings: Vec<(TypeAnnotation, String)>,
        slots: Vec<usize>,
        value: ExprId,
    },

    Match {
        value: ExprId,
        arms: Vec<(MatchPattern, Vec<Statement>)>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum MatchPattern {
    Literal(ExprId),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionAttribute {
    Entry,
    Init,
    Final,
    Test,
}

/// The type of a variable, constant, or parameter binding.
///
/// Mutable variants (`Int`, `Float`, etc.) are produced by `dec` declarations.
/// Constant variants (`CInt`, `CFloat`, etc.) are produced by `const` declarations.
/// `Array(T)` / `CArray(T)` are the mutable / constant array forms.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    /// Mutable 64-bit signed integer.
    Int,
    /// Mutable 64-bit float.
    Float,
    /// Mutable boolean.
    Bool,
    /// Mutable string.
    String,
    /// Mutable byte (`u8`).
    Byte,
    /// Mutable character.
    Char,
    /// Mutable array with a typed element.
    Array(Box<TypeAnnotation>),
    /// Constant 64-bit signed integer.
    CInt,
    /// Constant 64-bit float.
    CFloat,
    /// Constant boolean.
    CBool,
    /// Constant string.
    CString,
    /// Constant byte.
    CByte,
    /// Constant character.
    CChar,
    /// Constant array with a typed element.
    CArray(Box<TypeAnnotation>),
    /// A function value (used for `fn`-typed parameters and variables).
    Fn,
    /// Absence of a type - used as the default return type when none is annotated.
    Null,

    Tuple(Rc<Vec<TypeAnnotation>>),
    CTuple(Rc<Vec<TypeAnnotation>>),

    Error,
    CError,

    Result(Box<TypeAnnotation>),
    CResult(Box<TypeAnnotation>),
}

/// A single function or lambda parameter: a name and its type annotation.
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub param_name: String,
    pub param_type: TypeAnnotation,
}
