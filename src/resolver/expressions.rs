//! Expression resolution - transforms name references into indexed lookups.
//!
//! Each expression variant is handled in [`Resolver::resolve_expression`]:
//!
//! - `Identifier` -> `ResolvedIdentifier { depth, slot }` if found in scope,
//!   left as `Identifier` if not (stdlib names, unresolved at this stage)
//! - `Assign` -> `ResolvedAssign { depth, slot, value }`
//! - `Lambda` -> `ResolvedLambda { capture_depth, ... }` with a fresh scope
//!   for its parameters; `capture_depth` is the scope stack length at the
//!   point of definition, used by the evaluator to capture the environment
//! - `Call` with a single-segment path -> `CallExpr` with a `ResolvedIdentifier`
//!   callee if the name is in scope; left as `Call` for stdlib paths
//! - All other variants recurse into their sub-expressions unchanged
//!
//! All resolution mutates nodes in place via `ExprId` + `Arena::get_mut`.
//! The node's `ExprId` never changes - only its `kind` does. Child `ExprId`s
//! embedded in a parent's kind stay valid across the rewrite since resolving
//! a child mutates the arena slot it already occupies, not its address.
use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    resolver::Resolver,
};

impl Resolver {
    /// Resolves the expression at `id` in place and returns the same `id`.
    /// The return value exists so call sites can write
    /// `field: self.resolve_expression(field)` without changing shape.
    pub fn resolve_expression(&mut self, id: ExprId) -> ExprId {
        let span = self.ast_arena.exprs.get(id).span;
        // One clone to release the immutable borrow before recursing -
        // matches the `.kind.clone()` pattern already in evaluator.rs.
        let kind = self.ast_arena.exprs.get(id).kind.clone();

        let new_kind = match kind {
            ExpressionKind::Identifier(name) => self
                .resolve_name(&name)
                .map(|(depth, slot)| ExpressionKind::ResolvedIdentifier { name, depth, slot }),

            ExpressionKind::Assign { name, value } => {
                self.resolve_expression(value);
                self.resolve_name(&name)
                    .map(|(depth, slot)| ExpressionKind::ResolvedAssign {
                        name,
                        depth,
                        slot,
                        value,
                    })
            }

            ExpressionKind::Lambda {
                params,
                return_type,
                body,
            } => {
                let capture_depth = self.scopes.len();

                self.push_scope();
                for p in &params {
                    self.declare(p.param_name.clone());
                }
                let body = self.resolve_statements(body);
                self.pop_scope();

                Some(ExpressionKind::ResolvedLambda {
                    params,
                    return_type,
                    body,
                    capture_depth,
                })
            }

            ExpressionKind::Binary { left, right, .. } => {
                self.resolve_expression(left);
                self.resolve_expression(right);
                None
            }
            ExpressionKind::Unary { operand, .. } => {
                self.resolve_expression(operand);
                None
            }

            ExpressionKind::Grouping(inner) => {
                self.resolve_expression(inner);
                None
            }
            ExpressionKind::ArrayLiteral(items) => {
                for item in &items {
                    self.resolve_expression(*item);
                }
                None
            }

            ExpressionKind::Index { target, index } => {
                self.resolve_expression(target);
                self.resolve_expression(index);
                None
            }
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => {
                self.resolve_expression(target);
                self.resolve_expression(index);
                self.resolve_expression(value);
                None
            }

            ExpressionKind::Call { path, args } => {
                for arg in &args {
                    self.resolve_expression(*arg);
                }
                if path.len() == 1
                    && let Some((depth, slot)) = self.resolve_name(&path[0])
                {
                    let callee = self.ast_arena.alloc_expr(
                        ExpressionKind::ResolvedIdentifier {
                            name: path[0].clone(),
                            depth,
                            slot,
                        },
                        span,
                    );
                    Some(ExpressionKind::CallExpr { callee, args })
                } else {
                    // stdlib path - leave as Call
                    None
                }
            }
            ExpressionKind::CallExpr { callee, args } => {
                self.resolve_expression(callee);
                for arg in &args {
                    self.resolve_expression(*arg);
                }
                None
            }

            ExpressionKind::MethodCall { caller, args, .. } => {
                self.resolve_expression(caller);
                for arg in &args {
                    self.resolve_expression(*arg);
                }
                None
            }

            ExpressionKind::Cast { value, .. } => {
                self.resolve_expression(value);
                None
            }

            ExpressionKind::ErrorLiteral(inner)
            | ExpressionKind::OkLiteral(inner)
            | ExpressionKind::ErrLiteral(inner)
            | ExpressionKind::Propagate(inner) => {
                self.resolve_expression(inner);
                None
            }

            ExpressionKind::TupleLiteral(items) => {
                for item in &items {
                    self.resolve_expression(*item);
                }
                None
            }

            ExpressionKind::StructLiteral { fields, .. } => {
                for (_, value) in &fields {
                    self.resolve_expression(*value);
                }
                None
            }
            ExpressionKind::FieldAccess { target, .. } => {
                self.resolve_expression(target);
                None
            }
            ExpressionKind::FieldAssign { target, value, .. } => {
                self.resolve_expression(target);
                self.resolve_expression(value);
                None
            }
            ExpressionKind::MapLiteral(entries) => {
                for (key, value) in &entries {
                    self.resolve_expression(*key);
                    self.resolve_expression(*value);
                }
                None
            }

            _ => None,
        };

        if let Some(new_kind) = new_kind {
            self.ast_arena.exprs.get_mut(id).kind = new_kind;
        }

        id
    }
}
