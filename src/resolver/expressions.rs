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
use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    resolver::Resolver,
};

impl Resolver {
    pub fn resolve_expression(&mut self, expression: ExprId) -> ExprId {
        let span = self.expr_span(expression);
        let kind = self.expr_kind(expression);
        let kind = match kind {
            ExpressionKind::Identifier(name) => match self.resolve_name(&name) {
                Some((depth, slot)) => ExpressionKind::ResolvedIdentifier { name, depth, slot },
                None => ExpressionKind::Identifier(name),
            },

            ExpressionKind::Assign { name, value } => {
                let value = self.resolve_expression(value);
                match self.resolve_name(&name) {
                    Some((depth, slot)) => ExpressionKind::ResolvedAssign {
                        name,
                        depth,
                        slot,
                        value,
                    },
                    None => ExpressionKind::Assign { name, value },
                }
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

                ExpressionKind::ResolvedLambda {
                    params,
                    return_type,
                    body,
                    capture_depth,
                }
            }

            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => ExpressionKind::Binary {
                left: self.resolve_expression(left),
                operator,
                right: self.resolve_expression(right),
            },
            ExpressionKind::Unary { operator, operand } => ExpressionKind::Unary {
                operator,
                operand: self.resolve_expression(operand),
            },

            ExpressionKind::Grouping(inner) => {
                ExpressionKind::Grouping(self.resolve_expression(inner))
            }
            ExpressionKind::ArrayLiteral(items) => ExpressionKind::ArrayLiteral(
                items
                    .into_iter()
                    .map(|item| self.resolve_expression(item))
                    .collect(),
            ),

            ExpressionKind::Index { target, index } => ExpressionKind::Index {
                target: self.resolve_expression(target),
                index: self.resolve_expression(index),
            },
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => ExpressionKind::IndexAssign {
                target: self.resolve_expression(target),
                index: self.resolve_expression(index),
                value: self.resolve_expression(value),
            },

            ExpressionKind::Call { path, args } => {
                let args = args
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                if path.len() == 1
                    && let Some((depth, slot)) = self.resolve_name(&path[0])
                {
                    let callee = self.ast.alloc_expr(
                        ExpressionKind::ResolvedIdentifier {
                            name: path[0].clone(),
                            depth,
                            slot,
                        },
                        span,
                    );

                    return self
                        .ast
                        .alloc_expr(ExpressionKind::CallExpr { callee, args }, span);
                }
                // stdlib path - leave as Call
                ExpressionKind::Call { path, args }
            }
            ExpressionKind::CallExpr { callee, args } => ExpressionKind::CallExpr {
                callee: self.resolve_expression(callee),
                args: args
                    .into_iter()
                    .map(|arg| self.resolve_expression(arg))
                    .collect(),
            },

            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => ExpressionKind::MethodCall {
                caller: self.resolve_expression(caller),
                method,
                args: args
                    .into_iter()
                    .map(|arg| self.resolve_expression(arg))
                    .collect(),
            },

            ExpressionKind::Cast { value, target_type } => ExpressionKind::Cast {
                value: self.resolve_expression(value),
                target_type,
            },

            ExpressionKind::ErrorLiteral(inner) => {
                ExpressionKind::ErrorLiteral(self.resolve_expression(inner))
            }

            ExpressionKind::TupleLiteral(items) => ExpressionKind::TupleLiteral(
                items
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect(),
            ),

            ExpressionKind::OkLiteral(inner) => {
                ExpressionKind::OkLiteral(self.resolve_expression(inner))
            }

            ExpressionKind::ErrLiteral(inner) => {
                ExpressionKind::ErrLiteral(self.resolve_expression(inner))
            }

            ExpressionKind::Propagate(inner) => {
                ExpressionKind::Propagate(self.resolve_expression(inner))
            }

            other => other,
        };

        self.ast.alloc_expr(kind, span)
    }
}
