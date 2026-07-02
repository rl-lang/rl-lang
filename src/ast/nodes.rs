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
use crate::ast::statements::{Param, TypeAnnotation};
use crate::ast::{ExprId, StmtId};
use crate::lexer::tokentypes::TokenType;
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
        left: ExprId,
        operator: TokenType,
        right: ExprId,
    },
    /// A unary prefix operation: `operator operand`.
    Unary {
        operator: TokenType,
        operand: ExprId,
    },
    /// A parenthesised expression `(expr)` - preserves grouping in the AST.
    Grouping(ExprId),
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
    ArrayLiteral(Vec<ExprId>),
    /// An unresolved variable assignment `name = value`.
    /// Replaced by [`ResolvedAssign`] after the resolver pass.
    Assign {
        name: String,
        value: ExprId,
    },
    /// A lexically-resolved variable assignment.
    ResolvedAssign {
        name: String,
        depth: usize,
        slot: usize,
        value: ExprId,
    },
    /// A function call via a module path: `std::io::println(args)` or `f(args)`.
    Call {
        path: Vec<String>,
        args: Vec<ExprId>,
    },
    /// A method call chain: `expr.method(args)`.
    MethodCall {
        caller: ExprId,
        method: Vec<String>,
        args: Vec<ExprId>,
    },
    /// An index access: `target[index]`.
    Index {
        target: ExprId,
        index: ExprId,
    },
    /// An index assignment: `target[index] = value`.
    IndexAssign {
        target: ExprId,
        index: ExprId,
        value: ExprId,
    },
    /// An unresolved anonymous function (lambda) expression.
    /// Replaced by [`ResolvedLambda`] after the resolver pass.
    Lambda {
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: Vec<StmtId>,
    },
    /// A lexically-resolved lambda. `capture_depth` is the scope depth at the
    /// point of definition, used to correctly capture the enclosing environment.
    ResolvedLambda {
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: Vec<StmtId>,
        capture_depth: usize,
    },
    /// A call on an arbitrary callee expression, e.g. `fns[0](args)` or
    /// an immediately-invoked lambda.
    CallExpr {
        callee: ExprId,
        args: Vec<ExprId>,
    },

    /// A type cast expression ` value as type `.
    /// Used for non-literal casts; literal casts are constant-folded in the parser.
    Cast {
        value: ExprId,
        target_type: TypeAnnotation,
    },

    TupleLiteral(Vec<ExprId>),
    ErrorLiteral(ExprId),

    OkLiteral(ExprId),
    ErrLiteral(ExprId),

    Propagate(ExprId),
}
