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
    nodes::{Expression, ExpressionKind},
    statements::{Statement, StatementKind},
};

pub mod arena;
pub mod nodes;
pub mod statements;
// pub mod variables;
pub type ScopeMap = Vec<Vec<String>>;

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

    pub fn alloc_expr(&mut self, kind: ExpressionKind, span: crate::utils::span::Span) -> ExprId {
        self.exprs.alloc(Expression { kind, span })
    }

    /// Merges `other`'s expression arena into `self`, and rewrites every
    /// `ExprId` in `statements` - both ids referenced directly from a
    /// `StatementKind` field, and ids nested *inside* the moved expressions
    /// themselves (e.g. `Binary`'s `left`/`right`) - to point into `self`
    /// under its numbering instead of `other`'s.
    pub fn merge_statements(
        &mut self,
        mut other: Ast,
        mut statements: Vec<Statement>,
    ) -> Vec<Statement> {
        let offset = self.exprs.raw_parts_mut().1.len() as u32;
        let target_arena_id = self.exprs.raw_parts_mut().0;

        let (_, other_items) = other.exprs.raw_parts_mut();
        for expr in other_items.iter_mut() {
            remap_expr_kind(&mut expr.kind, offset, target_arena_id);
        }
        let other_items = std::mem::take(other_items);

        self.exprs.raw_parts_mut().1.extend(other_items);

        for stmt in &mut statements {
            remap_stmt_kind(&mut stmt.kind, offset, target_arena_id);
        }
        statements
    }
}

fn remap_id<T>(id: &mut Id<T>, offset: u32, target_arena_id: u32) {
    *id = id.rebase(offset, target_arena_id);
}

fn remap_expr_kind(kind: &mut ExpressionKind, offset: u32, target_arena_id: u32) {
    use ExpressionKind::*;
    match kind {
        Null
        | Integer(_)
        | Byte(_)
        | String(_)
        | Bool(_)
        | Float(_)
        | Character(_)
        | Identifier(_)
        | ResolvedIdentifier { .. } => {}

        Binary { left, right, .. } => {
            remap_id(left, offset, target_arena_id);
            remap_id(right, offset, target_arena_id);
        }
        Unary { operand, .. } => remap_id(operand, offset, target_arena_id),
        Grouping(id) | ErrorLiteral(id) | OkLiteral(id) | ErrLiteral(id) | Propagate(id) => {
            remap_id(id, offset, target_arena_id)
        }
        ArrayLiteral(items) | TupleLiteral(items) => {
            for id in items {
                remap_id(id, offset, target_arena_id);
            }
        }
        MapLiteral(entries) => {
            for (k, v) in entries {
                remap_id(k, offset, target_arena_id);
                remap_id(v, offset, target_arena_id);
            }
        }
        SetLiteral(items) => {
            for id in items {
                remap_id(id, offset, target_arena_id);
            }
        }
        Assign { value, .. } | ResolvedAssign { value, .. } => {
            remap_id(value, offset, target_arena_id)
        }
        Call { args, .. } => {
            for id in args {
                remap_id(id, offset, target_arena_id);
            }
        }
        MethodCall { caller, args, .. } => {
            remap_id(caller, offset, target_arena_id);
            for id in args {
                remap_id(id, offset, target_arena_id);
            }
        }
        Index { target, index } => {
            remap_id(target, offset, target_arena_id);
            remap_id(index, offset, target_arena_id);
        }
        IndexAssign {
            target,
            index,
            value,
        } => {
            remap_id(target, offset, target_arena_id);
            remap_id(index, offset, target_arena_id);
            remap_id(value, offset, target_arena_id);
        }
        CallExpr { callee, args } => {
            remap_id(callee, offset, target_arena_id);
            for id in args {
                remap_id(id, offset, target_arena_id);
            }
        }
        Cast { value, .. } => remap_id(value, offset, target_arena_id),
        Lambda { body, .. } | ResolvedLambda { body, .. } => {
            for stmt in body {
                remap_stmt_kind(&mut stmt.kind, offset, target_arena_id);
            }
        }

        StructLiteral { fields, .. } => {
            for (_, id) in fields {
                remap_id(id, offset, target_arena_id);
            }
        }
        EnumVariant { .. } => {}
        FieldAccess { target, .. } => remap_id(target, offset, target_arena_id),
        FieldAssign { target, value, .. } => {
            remap_id(target, offset, target_arena_id);
            remap_id(value, offset, target_arena_id);
        }
    }
}

fn remap_stmt_kind(kind: &mut StatementKind, offset: u32, target_arena_id: u32) {
    use StatementKind::*;
    match kind {
        Break
        | Continue
        | Range(_)
        | Import { .. }
        | ImportFile { .. }
        | ImportFileNamed { .. }
        | RecordDeclaration { .. }
        | TagDeclaration { .. } => {}

        VariableDeclaration { value, .. }
        | ResolvedVariableDeclaration { value, .. }
        | ConstantDeclaration { value, .. }
        | ResolvedConstantDeclaration { value, .. }
        | ResolvedArray { value, .. }
        | ResolvedMap { value, .. }
        | ResolvedConstantMap { value, .. }
        | ResolvedConstantArray { value, .. }
        | ResolvedSet { value, .. }
        | ResolvedConstantSet { value, .. }
        | Expression(value)
        | ResolvedDestructureDeclaration { value, .. }
        | DestructureDeclaration { value, .. } => remap_id(value, offset, target_arena_id),

        Array { value, .. } | ConstantArray { value, .. } => {
            for id in value {
                remap_id(id, offset, target_arena_id);
            }
        }

        Return(expr) => {
            if let Some(id) = expr {
                remap_id(id, offset, target_arena_id);
            }
        }

        While { condition, body } => {
            remap_id(condition, offset, target_arena_id);
            remap_stmts(body, offset, target_arena_id);
        }
        For {
            initializer,
            condition,
            increment,
            body,
        }
        | ResolvedFor {
            initializer,
            condition,
            increment,
            body,
        } => {
            remap_stmt_kind(&mut initializer.kind, offset, target_arena_id);
            remap_id(condition, offset, target_arena_id);
            remap_id(increment, offset, target_arena_id);
            remap_stmts(body, offset, target_arena_id);
        }
        ForRange { range, body, .. } | ResolvedForRange { range, body, .. } => {
            remap_stmt_kind(&mut range.kind, offset, target_arena_id);
            remap_stmts(body, offset, target_arena_id);
        }
        ForEach { iterable, body, .. } | ResolvedForEach { iterable, body, .. } => {
            remap_id(iterable, offset, target_arena_id);
            remap_stmts(body, offset, target_arena_id);
        }

        ConditionalBranch {
            condition, body, ..
        } => {
            if let Some(id) = condition {
                remap_id(id, offset, target_arena_id);
            }
            remap_stmts(body, offset, target_arena_id);
        }
        Conditional {
            if_branch,
            else_branch,
        } => {
            remap_stmt_kind(&mut if_branch.kind, offset, target_arena_id);
            if let Some(branch) = else_branch {
                remap_stmt_kind(&mut branch.kind, offset, target_arena_id);
            }
        }

        FunctionDeclaration { body, .. }
        | ResolvedFunctionDeclaration { body, .. }
        | ResolvedImportFile { body, .. } => {
            remap_stmts(body, offset, target_arena_id);
        }

        Match { value, arms } => {
            remap_id(value, offset, target_arena_id);
            for (pattern, body) in arms {
                if let crate::ast::statements::MatchPattern::Literal(id) = pattern {
                    remap_id(id, offset, target_arena_id);
                }
                remap_stmts(body, offset, target_arena_id);
            }
        }

        Map { entries, .. } | ConstantMap { entries, .. } => {
            for (k, v) in entries {
                remap_id(k, offset, target_arena_id);
                remap_id(v, offset, target_arena_id);
            }
        }

        Set { items, .. } | ConstantSet { items, .. } => {
            for id in items {
                remap_id(id, offset, target_arena_id);
            }
        }
    }
}

fn remap_stmts(body: &mut [Statement], offset: u32, target_arena_id: u32) {
    for stmt in body {
        remap_stmt_kind(&mut stmt.kind, offset, target_arena_id);
    }
}
