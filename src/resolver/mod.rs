//! Variable resolution pass - runs between parsing and evaluation.
//!
//! The resolver walks the AST and transforms unresolved name references
//! into slot-indexed lookups (`depth`, `slot`), eliminating string-based
//! name lookups at runtime.
//!
//! - `depth` - how many scopes up from the current scope the variable lives
//! - `slot`  - the index of the variable within that scope's slot array
//!
//! Unresolved `Identifier` nodes become `ResolvedIdentifier { depth, slot }`.
//! Unresolved `Assign` nodes become `ResolvedAssign { depth, slot, value }`.
//! Function and lambda bodies are resolved in their own pushed scope.
//! Import statements are read from disk, lexed, parsed, and resolved inline.

use crate::{
    ast::{Ast, ExprId, StmtId, nodes::ExpressionKind, statements::StatementKind},
    utils::span::Span,
};

mod expressions;
mod statements;

/// Walks the AST and resolves all name references to `(depth, slot)` pairs.
pub struct Resolver {
    /// Stack of scopes, each scope being an ordered list of declared names.
    /// Index in the list is the slot number; distance from the top is the depth.
    scopes: Vec<Vec<String>>,
    pub current_dir: std::path::PathBuf,
    pub ast: Ast,
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Resolver {
    /// Creates a new [`Resolver`] with a single empty global scope.
    pub fn new() -> Self {
        Self {
            scopes: vec![vec![]],
            current_dir: std::path::PathBuf::new(),
            ast: Ast::new(),
        }
    }

    pub fn with_ast(mut self, ast: Ast) -> Self {
        self.ast = ast;
        self
    }

    /// Pushes a new empty scope onto the scope stack.
    pub fn push_scope(&mut self) {
        self.scopes.push(vec![]);
    }

    /// Pops the innermost scope from the stack.
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Declares a name in the current scope and returns its slot index.
    ///
    /// The slot is the position of the name within the current scope frame,
    /// used later by the evaluator for direct indexed access.
    pub fn declare(&mut self, name: String) -> usize {
        let frame = self.scopes.last_mut().unwrap();
        let slot = frame.len();
        frame.push(name);
        slot
    }

    /// Searches for `name` by walking scopes from innermost to outermost.
    ///
    /// Returns `Some((depth, slot))` where `depth` is the number of scopes
    /// above the current one (0 = current), and `slot` is the index within
    /// that scope. Returns `None` if the name is not declared in any scope.
    pub fn resolve_name(&self, name: &str) -> Option<(usize, usize)> {
        for (depth, frame) in self.scopes.iter().rev().enumerate() {
            if let Some(slot) = frame.iter().position(|n| n == name) {
                return Some((depth, slot));
            }
        }
        None
    }

    pub fn expr_span(&self, id: ExprId) -> Span {
        self.ast.exprs.get(id).span
    }
    pub fn expr_kind(&self, id: ExprId) -> ExpressionKind {
        self.ast.exprs.get(id).kind.clone()
    }

    pub fn stmt_span(&self, id: StmtId) -> Span {
        self.ast.stmts.get(id).span
    }
    pub fn stmt_kind(&self, id: StmtId) -> StatementKind {
        self.ast.stmts.get(id).kind.clone()
    }

    pub fn into_ast(&mut self) -> Ast {
        std::mem::take(&mut self.ast)
    }
}
