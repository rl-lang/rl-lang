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
}
