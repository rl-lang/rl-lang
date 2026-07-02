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

use crate::{
    ast::{
        arena::{Arena, Id},
        nodes::{Expression, ExpressionKind},
        statements::{Statement, StatementKind},
    },
    utils::span::Span,
};
pub mod nodes;
pub mod statements;
// pub mod variables;
pub type ScopeMap = Vec<Vec<String>>;
mod arena;
pub mod macros;

pub struct Ast {
    pub exprs: Arena<Expression>,
    pub stmts: Arena<Statement>,
}

pub type ExprId = Id<Expression>;
pub type StmtId = Id<Statement>;

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            exprs: Arena::new(),
            stmts: Arena::new(),
        }
    }
    pub fn alloc_expr(&mut self, kind: ExpressionKind, span: Span) -> ExprId {
        self.exprs.alloc(Expression { kind, span })
    }
    pub fn alloc_stmt(&mut self, kind: StatementKind, span: Span) -> StmtId {
        self.stmts.alloc(Statement { kind, span })
    }
}
