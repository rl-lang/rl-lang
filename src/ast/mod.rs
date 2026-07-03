//! Abstract Syntax Tree (AST) node definitions.
//!
//! # Pipeline position
//! ```text
//! source -> Lexer -> [Token] -> Parser -> [Statement] -> Checker -> Evaluator
//! ```
//!
//! # Module layout
//! - [`nodes`] - [`Expression`] and [`ExpressionKind`]: the expression AST
//! - [`statements`] - [`Statement`], [`StatementKind`], [`TypeAnnotation`], [`Param`]
//!
//! [`Expression`]: nodes::Expression
//! [`Statement`]: statements::Statement
use crate::ast::{
    arena::{Arena, Id},
    nodes::Expression,
};

pub mod arena;
pub mod nodes;
pub mod statements;
// pub mod variables;
pub type ScopeMap = Vec<Vec<String>>;
pub mod macros;

/// Owns every `Expression` allocated during a single parse/resolve/eval
/// session. `Statement` stays `Box`-owned for now - this migrates the
/// leaf/expression side first, on its own, before statements follow.
pub struct Ast {
    pub exprs: Arena<Expression>,
}

/// Handle to an `Expression` living in `Ast::exprs`.
pub type ExprId = Id<Expression>;

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            exprs: Arena::new(),
        }
    }

    pub fn alloc_expr(
        &mut self,
        kind: nodes::ExpressionKind,
        span: crate::utils::span::Span,
    ) -> ExprId {
        self.exprs.alloc(Expression { kind, span })
    }
}
