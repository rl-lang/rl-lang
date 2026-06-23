use crate::{ast::{ statements::{Statement, StatementKind}}, resolver::Resolver};

impl Resolver {
    pub fn collect_captures_statements(&self, statements: &[Statement], params: &[&str], out: &mut Vec<(usize,usize)>) {
        for statement in statements {
            self.collect_captures_statement(statement, params, out);
        }
    }

    fn collect_captures_statement(&self,statement: &Statement, params: &[&str], out: &mut Vec<(usize,usize)>) {
        match &statement.kind {
            StatementKind::Expression(e) => self.collect_capture_expression(e, params, out),
            StatementKind::Return(Some(e)) => self.collect_capture_expression(e, params, out),
            StatementKind::VariableDeclaration { value, .. } |
            StatementKind::ConstantDeclaration {   value, .. } => self.collect_capture_expression(value, params, out),

            StatementKind::While { condition, body } => {
                self.collect_capture_expression(condition, params, out);
                self.collect_captures_statements(body, params, out);
            }
            StatementKind::ConditionalBranch { condition, body } => {
                if let Some(condition) = condition {self.collect_capture_expression(condition, params, out);};
                self.collect_captures_statements(body, params, out);
            }
            StatementKind::Conditional { if_branch, else_branch } => {
                self.collect_captures_statement(if_branch, params, out);
                if let Some(e) = else_branch {self.collect_captures_statement(e, params, out);}
            }
            StatementKind::ForEach {    iterable, body, .. } => {
                self.collect_capture_expression(iterable, params, out);
                self.collect_captures_statements(body, params, out);
            }

            _ => {}
        }
    }

    pub fn resolve_statements(&mut self, statements: Vec<Statement>) -> Vec<Statement> {
        statements.into_iter().map(|statement| self.resolve_statement(statement)).collect()
    }

    fn resolve_statement(&mut self, statement: Statement) -> Statement {
        let span = statement.span;
        let kind = match statement.kind {
            StatementKind::VariableDeclaration { name, type_annotation, value }
        }
    }
}
