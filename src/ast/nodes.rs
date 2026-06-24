//! Expression AST nodes.
//!
//! An [`Expression`] is any construct that produces a value. Every expression
//! node carries the source [`Span`] it was parsed from so that error reports
//! and the LSP can point at exact locations.
//!
//! # Resolved variants
//! After the [`Resolver`] pass runs, unresolved name-based variants
//! (`Identifier`, `Assign`, `Lambda`) are replaced with their `Resolved*`
//! counterparts that carry `(depth, slot)` integer pairs for direct
//! environment lookup, eliminating runtime name searches.
//!
//! [`Resolver`]: crate::resolver
use crate::ast::statements::{Param, Statement, TypeAnnotation};
use crate::lexer::tokentypes;
use crate::utils::span::Span;

/// An expression paired with its source span.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    /// The `null` literal.
    Null,
    /// A 64-bit signed integer literal.
    Integer(i64),
    /// A byte literal (`u8`).
    Byte(u8),
    /// A binary operation: `left operator right`.
    Binary {
        left: Box<Expression>,
        operator: tokentypes::TokenType,
        right: Box<Expression>,
    },
    /// A unary prefix operation: `operator operand`.
    Unary {
        operator: tokentypes::TokenType,
        operand: Box<Expression>,
    },
    /// A parenthesised expression `(expr)` - preserves grouping in the AST.
    Grouping(Box<Expression>),
    /// A string literal.
    String(String),
    /// A boolean literal.
    Bool(bool),
    /// A 64-bit float literal.
    Float(f64),
    /// A character literal.
    Character(char),
    /// An unresolved variable or function reference by name.
    /// Replaced by [`ResolvedIdentifier`] after the resolver pass.
    Identifier(String),
    /// A lexically-resolved variable reference.
    /// `depth` is the number of scopes to walk up; `slot` is the index within that scope.
    ResolvedIdentifier {
        name: String,
        depth: usize,
        slot: usize,
    },
    /// An array literal `[a, b, c]`.
    ArrayLiteral(Vec<Expression>),
    /// An unresolved variable assignment `name = value`.
    /// Replaced by [`ResolvedAssign`] after the resolver pass.
    Assign {
        name: String,
        value: Box<Expression>,
    },
    /// A lexically-resolved variable assignment.
    ResolvedAssign {
        name: String,
        depth: usize,
        slot: usize,
        value: Box<Expression>,
    },
    /// A function call via a module path: `std::io::println(args)` or `f(args)`.
    Call {
        path: Vec<String>,
        args: Vec<Expression>,
    },
    /// A method call chain: `expr.method(args)`.
    MethodCall {
        caller: Box<Expression>,
        method: Vec<String>,
        args: Vec<Expression>,
    },
    /// An index access: `target[index]`.
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },
    /// An index assignment: `target[index] = value`.
    IndexAssign {
        target: Box<Expression>,
        index: Box<Expression>,
        value: Box<Expression>,
    },
    /// An unresolved anonymous function (lambda) expression.
    /// Replaced by [`ResolvedLambda`] after the resolver pass.
    Lambda {
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Statement>,
    },
    /// A lexically-resolved lambda. `capture_depth` is the scope depth at the
    /// point of definition, used to correctly capture the enclosing environment.
    ResolvedLambda {
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Statement>,
        capture_depth: usize,
    },
    /// A call on an arbitrary callee expression, e.g. `fns[0](args)` or
    /// an immediately-invoked lambda.
    CallExpr {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },

    /// A type cast expression ` value as type `.
    /// Used for non-literal casts; literal casts are constant-folded in the parser.
    Cast {
        value: Box<Expression>,
        target_type: TypeAnnotation,
    },
}
