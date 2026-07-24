//! Statement resolution - transforms declarations and control flow into
//! slot-indexed variants and resolves import statements at compile time.
//!
//! Key behaviors:
//!
//! - Variable/constant/array declarations: resolve the initializer expression
//!   first, *then* declare the name so the initializer cannot reference itself
//! - Function declarations: declare the name in the outer scope first (enabling
//!   recursion), then push a new scope for parameters and resolve the body
//! - `ForEach`/`ForRange`: the loop variable is declared inside its own scope
//!   so it does not leak into the surrounding scope after the loop ends
//! - `ImportFile` / `ImportFileNamed`: reads the file from disk, lexes, parses,
//!   and resolves it inline - the result replaces the import statement with
//!   `ResolvedImportFile { body }` containing the fully resolved statements.
//!   `ImportFileNamed` additionally filters to only the requested names before resolving.
//!   Both silently return the original unresolved statement on any IO/parse failure

use crate::Resolver;
use rl_ast::{
    Ast,
    nodes::ExpressionKind,
    statements::{Statement, StatementKind},
};
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;

impl Resolver {
    pub fn resolve_program(&mut self, ast: Ast, statements: Vec<Statement>) -> Vec<Statement> {
        let statements = self.ast_arena.merge_statements(ast, statements);
        self.resolve_statements(statements)
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
                unit_annotation,
                value,
            } => {
                let value = self.resolve_expression(value);
                let slot = self.declare(name.clone());
                StatementKind::ResolvedVariableDeclaration {
                    name,
                    slot,
                    type_annotation,
                    unit_annotation,
                    value,
                }
            }
            StatementKind::ConstantDeclaration {
                name,
                type_annotation,
                unit_annotation,
                value,
            } => {
                let value = self.resolve_expression(value);
                let slot = self.declare(name.clone());
                StatementKind::ResolvedConstantDeclaration {
                    name,
                    slot,
                    type_annotation,
                    unit_annotation,
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
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::ArrayLiteral(value), span);

                StatementKind::ResolvedArray {
                    name,
                    slot,
                    type_annotation,
                    value,
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
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::ArrayLiteral(value), span);

                StatementKind::ResolvedConstantArray {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }

            StatementKind::Set {
                name,
                type_annotation,
                items,
            } => {
                let value = items
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::SetLiteral(value), span);

                StatementKind::ResolvedSet {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            StatementKind::ConstantSet {
                name,
                type_annotation,
                items,
            } => {
                let value = items
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::SetLiteral(value), span);

                StatementKind::ResolvedConstantSet {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }

            StatementKind::Map {
                name,
                type_annotation,
                entries,
            } => {
                let entries = entries
                    .into_iter()
                    .map(|(k, v)| (self.resolve_expression(k), self.resolve_expression(v)))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::MapLiteral(entries), span);

                StatementKind::ResolvedMap {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            StatementKind::ConstantMap {
                name,
                type_annotation,
                entries,
            } => {
                let entries = entries
                    .into_iter()
                    .map(|(k, v)| (self.resolve_expression(k), self.resolve_expression(v)))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::MapLiteral(entries), span);

                StatementKind::ResolvedConstantMap {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }

            StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
                attribute,
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
                    attribute,
                }
            }
            StatementKind::ImplBlock { record, methods } => {
                let methods = methods
                    .into_iter()
                    .map(|m| {
                        let m_span = m.span;
                        match m.kind {
                            StatementKind::FunctionDeclaration {
                                name,
                                params,
                                return_type,
                                body,
                                attribute,
                            } => {
                                self.push_scope();
                                for p in &params {
                                    self.declare(p.param_name.clone());
                                }
                                let body = self.resolve_statements(body);
                                self.pop_scope();
                                Statement::new(
                                    StatementKind::ResolvedFunctionDeclaration {
                                        name,
                                        // impl methods are dispatched by
                                        // `record::method` name, not by
                                        // lexical address - this slot is
                                        // never read.
                                        slot: usize::MAX,
                                        params,
                                        return_type,
                                        body,
                                        attribute,
                                    },
                                    m_span,
                                )
                            }
                            other => Statement::new(other, m_span),
                        }
                    })
                    .collect();
                StatementKind::ResolvedImplBlock { record, methods }
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
                self.push_scope();
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::While { condition, body }
            }
            StatementKind::Loop(body) => {
                self.push_scope();
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::Loop(body)
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

            StatementKind::ConditionalBranch {
                condition, body, ..
            } => {
                let condition = condition.map(|e| self.resolve_expression(e));
                let needs_scope = body.iter().any(|s| {
                    matches!(
                        s.kind,
                        StatementKind::VariableDeclaration { .. }
                            | StatementKind::ConstantDeclaration { .. }
                            | StatementKind::Array { .. }
                            | StatementKind::ConstantArray { .. }
                            | StatementKind::Map { .. }
                            | StatementKind::ConstantMap { .. }
                            | StatementKind::FunctionDeclaration { .. }
                    )
                });
                if needs_scope {
                    self.push_scope();
                }
                let body = self.resolve_statements(body);
                if needs_scope {
                    self.pop_scope();
                }
                StatementKind::ConditionalBranch {
                    condition,
                    body,
                    needs_scope,
                }
            }

            StatementKind::Return(expr) => {
                StatementKind::Return(expr.map(|e| self.resolve_expression(e)))
            }
            StatementKind::Expression(expr) => {
                StatementKind::Expression(self.resolve_expression(expr))
            }

            StatementKind::ImportFile { path } => {
                let import_name = format!("{}.rl", path.join("/"));
                let file_path = self.current_dir.join(&import_name);
                let Ok(source_text) = std::fs::read_to_string(&file_path) else {
                    return Statement::new(StatementKind::ImportFile { path }, span);
                };
                let source_file =
                    SourceFile::new(file_path.to_string_lossy().as_ref(), source_text);
                let Ok(tokens) = Tokenizer::lex(source_file.clone()) else {
                    return Statement::new(StatementKind::ImportFile { path }, span);
                };
                let Ok((imported_ast, stmts)) = Parser::parse(tokens, source_file) else {
                    return Statement::new(StatementKind::ImportFile { path }, span);
                };

                let stmts = self.ast_arena.merge_statements(imported_ast, stmts);

                let imported_dir = file_path
                    .parent()
                    .unwrap_or(std::path::Path::new(""))
                    .to_path_buf();
                let prev_dir = std::mem::replace(&mut self.current_dir, imported_dir);
                let resolved = self.resolve_statements(stmts);
                self.current_dir = prev_dir;

                StatementKind::ResolvedImportFile {
                    path,
                    body: resolved,
                }
            }

            StatementKind::ImportFileNamed { path, names } => {
                let import_name = format!("{}.rl", path.join("/"));
                let file_path = self.current_dir.join(&import_name);
                let Ok(source_text) = std::fs::read_to_string(&file_path) else {
                    return Statement::new(StatementKind::ImportFileNamed { path, names }, span);
                };
                let source_file =
                    SourceFile::new(file_path.to_string_lossy().as_ref(), source_text);
                let Ok(tokens) = Tokenizer::lex(source_file.clone()) else {
                    return Statement::new(StatementKind::ImportFileNamed { path, names }, span);
                };
                let Ok((imported_ast, stmts)) = Parser::parse(tokens, source_file) else {
                    return Statement::new(StatementKind::ImportFileNamed { path, names }, span);
                };
                let stmts: Vec<_> = stmts
                    .into_iter()
                    .filter(|s| match &s.kind {
                        StatementKind::FunctionDeclaration { name, .. }
                        | StatementKind::VariableDeclaration { name, .. }
                        | StatementKind::ConstantDeclaration { name, .. } => names.contains(name),
                        StatementKind::Array { name, .. }
                        | StatementKind::ConstantArray { name, .. } => names.contains(name),
                        StatementKind::Map { name, .. }
                        | StatementKind::ConstantMap { name, .. } => names.contains(name),
                        StatementKind::Set { name, .. }
                        | StatementKind::ConstantSet { name, .. } => names.contains(name),
                        StatementKind::RecordDeclaration { name, .. }
                        | StatementKind::TagDeclaration { name, .. } => names.contains(name),
                        _ => false,
                    })
                    .collect();

                let stmts = self.ast_arena.merge_statements(imported_ast, stmts);

                let imported_dir = file_path
                    .parent()
                    .unwrap_or(std::path::Path::new(""))
                    .to_path_buf();
                let prev_dir = std::mem::replace(&mut self.current_dir, imported_dir);
                let body = self.resolve_statements(stmts);
                self.current_dir = prev_dir;

                StatementKind::ResolvedImportFile { path, body }
            }

            StatementKind::DestructureDeclaration { bindings, value } => {
                let value = self.resolve_expression(value);
                let slots = bindings
                    .iter()
                    .map(|(_, name)| self.declare(name.clone()))
                    .collect();
                StatementKind::ResolvedDestructureDeclaration {
                    bindings,
                    slots,
                    value,
                }
            }

            StatementKind::Match { value, arms } => {
                let value = self.resolve_expression(value);
                let arms = arms
                    .into_iter()
                    .map(|(pattern, body)| {
                        self.push_scope();
                        let body = self.resolve_statements(body);
                        self.pop_scope();
                        (pattern, body)
                    })
                    .collect();
                StatementKind::Match { value, arms }
            }

            other => other,
        };
        Statement::new(kind, span)
    }
}
