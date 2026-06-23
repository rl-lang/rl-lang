use crate::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::{Statement, StatementKind},
    },
    lexer::tokenizer::Tokenizer,
    parser::parser_logic::Parser,
    resolver::Resolver,
    utils::source::SourceFile,
};

impl Resolver {
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
                let initializer = Box::new(self.resolve_statement(*initializer));
                let condition = self.resolve_expression(condition);
                let increment = self.resolve_expression(increment);
                let body = self.resolve_statements(body);
                StatementKind::ResolvedFor {
                    initializer,
                    condition,
                    increment,
                    body,
                }
            }
            StatementKind::While { condition, body } => {
                let condition = self.resolve_expression(condition);
                let body = self.resolve_statements(body);
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
                let body = self.resolve_statements(body);
                StatementKind::ConditionalBranch { condition, body }
            }
            StatementKind::Return(expr) => {
                StatementKind::Return(expr.map(|e| self.resolve_expression(e)))
            }
            StatementKind::Expression(expr) => {
                StatementKind::Expression(self.resolve_expression(expr))
            }

            StatementKind::ImportFile { path } => {
                let import_name = format!("{}.rl", path.join("/"));
                let Ok(source_text) = std::fs::read_to_string(&import_name) else {
                    return Statement::new(StatementKind::ImportFile { path }, span);
                };
                let source_file = SourceFile::new(import_name, source_text);
                let Ok(tokens) = Tokenizer::lex(source_file.clone()) else {
                    return Statement::new(StatementKind::ImportFile { path }, span);
                };
                let Ok(stmts) = Parser::parse(tokens, source_file) else {
                    return Statement::new(StatementKind::ImportFile { path }, span);
                };
                // Resolve in current scope — imported names get slots here
                let resolved = self.resolve_statements(stmts);
                StatementKind::ResolvedImportFile {
                    path,
                    body: resolved,
                }
            }
            other => other,
        };
        Statement::new(kind, span)
    }
}
