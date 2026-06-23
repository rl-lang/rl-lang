use crate::{
    ast::nodes::{Expression, ExpressionKind},
    resolver::Resolver,
};

impl Resolver {
    pub fn collect_capture_expression(
        &self,
        expression: &Expression,
        params: &[&str],
        out: &mut Vec<(usize, usize)>,
    ) {
        match &expression.kind {
            ExpressionKind::Identifier(name) => {
                if !params.contains(&name.as_str())
                    && let Some(addr) = self.resolve_name(name)
                {
                    out.push(addr);
                }
            }
            ExpressionKind::Assign { value, .. } => {
                self.collect_capture_expression(value, params, out)
            }
            ExpressionKind::Binary { left, right, .. } => {
                self.collect_capture_expression(left, params, out);
                self.collect_capture_expression(right, params, out);
            }
            ExpressionKind::Unary { operand, .. } => {
                self.collect_capture_expression(operand, params, out)
            }
            ExpressionKind::Grouping(inner) => self.collect_capture_expression(inner, params, out),
            ExpressionKind::ArrayLiteral(items) => {
                for item in items {
                    self.collect_capture_expression(item, params, out);
                }
            }
            ExpressionKind::Call { args, .. } | ExpressionKind::CallExpr { args, .. } => {
                for arg in args {
                    self.collect_capture_expression(arg, params, out);
                }
            }
            ExpressionKind::MethodCall { caller, args, .. } => {
                self.collect_capture_expression(caller, params, out);
                for arg in args {
                    self.collect_capture_expression(arg, params, out);
                }
            }
            ExpressionKind::Index { target, index } => {
                self.collect_capture_expression(target, params, out);
                self.collect_capture_expression(index, params, out);
            }
            _ => {}
        }
    }

    pub fn resolve_expression(&mut self, expression: Expression) -> Expression {
        let span = expression.span;
        let kind = match expression.kind {
            ExpressionKind::Identifier(name) => {
                let (depth, slot) = self
                    .resolve_name(&name)
                    .expect(&format!("undefined variable '{}'", name));
                ExpressionKind::ResolvedIdentifier { name, depth, slot }
            }

            ExpressionKind::Assign { name, value } => {
                let value = Box::new(self.resolve_expression(*value));
                let (depth, slot) = self
                    .resolve_name(&name)
                    .expect(&format!("undefined variable '{}'", name));
                ExpressionKind::ResolvedAssign {
                    name,
                    depth,
                    slot,
                    value,
                }
            }

            ExpressionKind::Lambda {
                params,
                return_type,
                body,
            } => {
                let captured_slots = self.collect_captures(&params, &body);
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
                    captured_slots,
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
                if path.len() == 1 {
                    if let Some((depth, slot)) = self.resolve_name(&path[0]) {
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
                }
                // stdlib path — leave as Call
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
