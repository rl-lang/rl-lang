use crate::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::{Statement, StatementKind},
    },
    resolver::Resolver,
};

impl Resolver {
    pub fn collect_captures_statements(
        &self,
        statements: &[Statement],
        params: &[&str],
        out: &mut Vec<(usize, usize)>,
    ) {
        for statement in statements {
            self.collect_captures_statement(statement, params, out);
        }
    }

    fn collect_captures_statement(
        &self,
        statement: &Statement,
        params: &[&str],
        out: &mut Vec<(usize, usize)>,
    ) {
        match &statement.kind {
            StatementKind::Expression(e) => self.collect_capture_expression(e, params, out),
            StatementKind::Return(Some(e)) => self.collect_capture_expression(e, params, out),
            StatementKind::VariableDeclaration { value, .. }
            | StatementKind::ConstantDeclaration { value, .. } => {
                self.collect_capture_expression(value, params, out)
            }

            StatementKind::While { condition, body } => {
                self.collect_capture_expression(condition, params, out);
                self.collect_captures_statements(body, params, out);
            }
            StatementKind::ConditionalBranch { condition, body } => {
                if let Some(condition) = condition {
                    self.collect_capture_expression(condition, params, out);
                };
                self.collect_captures_statements(body, params, out);
            }
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                self.collect_captures_statement(if_branch, params, out);
                if let Some(e) = else_branch {
                    self.collect_captures_statement(e, params, out);
                }
            }
            StatementKind::ForEach { iterable, body, .. } => {
                self.collect_capture_expression(iterable, params, out);
                self.collect_captures_statements(body, params, out);
            }

            _ => {}
        }
    }

    pub fn resolve_statements(&mut self, statements: Vec<Statement>) -> Vec<Statement> {
        statements
            .into_iter()
            .map(|statement| self.resolve_statement(statement))
            .collect()
    }

    fn resolve_statement(&mut self, stmt: Statement) -> Statement {
        let span = stmt.span;
        let kind = match stmt.kind {
            StatementKind::VariableDeclaration {
                name,
                type_annotation,
                value,
            } => {
                let value = self.resolve_expression(value);
                let slot = self.declare(name.clone());
                StatementKind::ResolvedVariableDeclaration {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            StatementKind::ConstantDeclaration {
                name,
                type_annotation,
                value,
            } => {
                let value = self.resolve_expression(value);
                let slot = self.declare(name.clone());
                StatementKind::ResolvedConstantDeclaration {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            // new arrays variant
            StatementKind::Array {
                name,
                type_annotation,
                value,
            } => {
                let value = value
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                let slot = self.declare(name.clone());
                StatementKind::ResolvedArray {
                    name,
                    slot,
                    type_annotation,
                    value: Expression::new(ExpressionKind::ArrayLiteral(value), span),
                }
            }
            StatementKind::ConstantArray {
                name,
                type_annotation,
                value,
            } => {
                let value = value
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                let slot = self.declare(name.clone());
                StatementKind::ResolvedConstantArray {
                    name,
                    slot,
                    type_annotation,
                    value: Expression::new(ExpressionKind::ArrayLiteral(value), span),
                }
            }
            StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
                is_entry,
            } => {
                let slot = self.declare(name.clone());
                self.push_scope();
                for p in &params {
                    self.declare(p.param_name.clone());
                }
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::ResolvedFunctionDeclaration {
                    name,
                    slot,
                    params,
                    return_type,
                    body,
                    is_entry,
                }
            }
            StatementKind::ForEach {
                variable,
                iterable,
                body,
            } => {
                let iterable = self.resolve_expression(iterable);
                self.push_scope();
                let slot = self.declare(variable.clone());
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::ResolvedForEach {
                    variable,
                    slot,
                    iterable,
                    body,
                }
            }
            StatementKind::ForRange {
                variable,
                range,
                body,
            } => {
                let range = Box::new(self.resolve_statement(*range));
                self.push_scope();
                let slot = self.declare(variable.clone());
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::ResolvedForRange {
                    variable,
                    slot,
                    range,
                    body,
                }
            }
            StatementKind::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.push_scope();
                let initializer = Box::new(self.resolve_statement(*initializer));
                let condition = self.resolve_expression(condition);
                let increment = self.resolve_expression(increment);
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::For {
                    initializer,
                    condition,
                    increment,
                    body,
                }
            }
            StatementKind::While { condition, body } => {
                let condition = self.resolve_expression(condition);
                self.push_scope();
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::While { condition, body }
            }
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                let if_branch = Box::new(self.resolve_statement(*if_branch));
                let else_branch = else_branch.map(|e| Box::new(self.resolve_statement(*e)));
                StatementKind::Conditional {
                    if_branch,
                    else_branch,
                }
            }
            StatementKind::ConditionalBranch { condition, body } => {
                let condition = condition.map(|e| self.resolve_expression(e));
                self.push_scope();
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::ConditionalBranch { condition, body }
            }
            StatementKind::Return(expr) => {
                StatementKind::Return(expr.map(|e| self.resolve_expression(e)))
            }
            StatementKind::Expression(expr) => {
                StatementKind::Expression(self.resolve_expression(expr))
            }

            other => other,
        };
        Statement::new(kind, span)
    }
}
