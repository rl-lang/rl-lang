use crate::{
    ast::nodes::{Expression, ExpressionKind},
    resolver::Resolver,
};

impl Resolver {
    pub fn resolve_expression(&mut self, expression: Expression) -> Expression {
        let span = expression.span;
        let kind = match expression.kind {
            ExpressionKind::Identifier(name) => match self.resolve_name(&name) {
                Some((depth, slot)) => ExpressionKind::ResolvedIdentifier { name, depth, slot },
                None => ExpressionKind::Identifier(name),
            },

            ExpressionKind::Assign { name, value } => {
                let value = Box::new(self.resolve_expression(*value));
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
                left: Box::new(self.resolve_expression(*left)),
                operator,
                right: Box::new(self.resolve_expression(*right)),
            },
            ExpressionKind::Unary { operator, operand } => ExpressionKind::Unary {
                operator,
                operand: Box::new(self.resolve_expression(*operand)),
            },

            ExpressionKind::Grouping(inner) => {
                ExpressionKind::Grouping(Box::new(self.resolve_expression(*inner)))
            }
            ExpressionKind::ArrayLiteral(items) => ExpressionKind::ArrayLiteral(
                items
                    .into_iter()
                    .map(|item| self.resolve_expression(item))
                    .collect(),
            ),

            ExpressionKind::Index { target, index } => ExpressionKind::Index {
                target: Box::new(self.resolve_expression(*target)),
                index: Box::new(self.resolve_expression(*index)),
            },
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => ExpressionKind::IndexAssign {
                target: Box::new(self.resolve_expression(*target)),
                index: Box::new(self.resolve_expression(*index)),
                value: Box::new(self.resolve_expression(*value)),
            },

            ExpressionKind::Call { path, args } => {
                let args = args
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                if path.len() == 1
                    && let Some((depth, slot)) = self.resolve_name(&path[0])
                {
                    return Expression::new(
                        ExpressionKind::CallExpr {
                            callee: Box::new(Expression::new(
                                ExpressionKind::ResolvedIdentifier {
                                    name: path[0].clone(),
                                    depth,
                                    slot,
                                },
                                span,
                            )),
                            args,
                        },
                        span,
                    );
                }
                // stdlib path - leave as Call
                ExpressionKind::Call { path, args }
            }
            ExpressionKind::CallExpr { callee, args } => ExpressionKind::CallExpr {
                callee: Box::new(self.resolve_expression(*callee)),
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
                caller: Box::new(self.resolve_expression(*caller)),
                method,
                args: args
                    .into_iter()
                    .map(|arg| self.resolve_expression(arg))
                    .collect(),
            },

            other => other,
        };

        Expression::new(kind, span)
    }
}
