//! Statement evaluation - the main dispatch loop and control flow primitives.

use crate::{
    ast::statements::{FunctionAttribute, MatchPattern, Statement, StatementKind, TypeAnnotation},
    interpreter::{evaluator::Evaluator, values::Value},
    lexer::tokenizer::Tokenizer,
    parser::parser_logic::Parser,
    utils::{errors::Error, source::SourceFile, span::Span},
};
use std::sync::Arc;
use std::{path::Path, rc::Rc};

impl Evaluator {
    /// Evaluates a single statement, mutating the environment and control-flow flags.
    ///
    /// Loop control (`break`, `continue`) and function return (`return`) are signalled
    /// via `is_breaking`, `is_continuing`, and `return_value` flags on [`Evaluator`]
    /// rather than exceptions, so callers must check these flags after each statement.
    pub fn evaluate_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match &statement.kind {
            StatementKind::ResolvedVariableDeclaration {
                slot,
                value,
                type_annotation,
                ..
            } => {
                let val = self.evaluate(value)?;

                let val_type = Self::infer_type(&val, false);
                if !Self::types_compatible(&val_type, type_annotation)
                    && val_type != *type_annotation
                    && val_type != TypeAnnotation::Null
                {
                    return Err(self.err(
                        format!(
                            "type mismatch: expected {:?}, got {:?}",
                            type_annotation, val_type
                        ),
                        statement.span,
                    ));
                }
                self.insert_value(*slot, val, type_annotation.clone(), statement.span)?;
            }

            StatementKind::ResolvedConstantDeclaration {
                slot,
                value,
                type_annotation,
                ..
            } => {
                let val = self.evaluate(value)?;

                let val_type = Self::infer_type(&val, true);
                if !Self::types_compatible(&val_type, type_annotation)
                    && val_type != *type_annotation
                    && val_type != TypeAnnotation::Null
                {
                    return Err(self.err(
                        format!(
                            "type mismatch: expected {:?}, got {:?}",
                            type_annotation, val_type
                        ),
                        statement.span,
                    ));
                }
                self.insert_const(*slot, val, type_annotation.clone(), statement.span)?;
            }

            StatementKind::ResolvedArray {
                slot,
                value,
                type_annotation,
                ..
            } => {
                let val = self.evaluate(value)?;
                let val = match val {
                    Value::Values { items, .. } => {
                        for item in &items {
                            let actual = Self::infer_type(item, false);
                            if !Self::types_compatible(&actual, type_annotation) {
                                return Err(self.err(
                                    format!(
                                        "array element type mismatch: expected {:?}, found {:?}",
                                        type_annotation, actual
                                    ),
                                    statement.span,
                                ));
                            }
                        }
                        Value::Values {
                            items_type: type_annotation.clone(),
                            items,
                        }
                    }
                    other => {
                        return Err(self.err(
                            format!("expected array value found {}", other.type_name()),
                            statement.span,
                        ));
                    }
                };
                let declared_type = TypeAnnotation::Array(Box::new(type_annotation.clone()));
                self.insert_value(*slot, val, declared_type, statement.span)?;
            }

            StatementKind::ResolvedConstantArray {
                slot,
                value,
                type_annotation,
                ..
            } => {
                let val = self.evaluate(value)?;
                let val = match val {
                    Value::Values { items, .. } => {
                        for item in &items {
                            let actual = Self::infer_type(item, false);
                            if !Self::types_compatible(&actual, type_annotation) {
                                return Err(self.err(
                                    format!(
                                        "array element type mismatch: expected {:?}, found {:?}",
                                        type_annotation, actual
                                    ),
                                    statement.span,
                                ));
                            }
                        }
                        Value::Values {
                            items_type: type_annotation.clone(),
                            items,
                        }
                    }
                    other => {
                        return Err(self.err(
                            format!("expected array value found {}", other.type_name()),
                            statement.span,
                        ));
                    }
                };
                let declared_type = TypeAnnotation::CArray(Box::new(type_annotation.clone()));
                self.insert_value(*slot, val, declared_type, statement.span)?;
            }

            StatementKind::Expression(expr) => {
                self.evaluate(expr)?;
            }

            StatementKind::While { condition, body } => loop {
                let v = self.evaluate(condition)?;
                match v {
                    Value::Bool(true) => {}
                    Value::Bool(false) => break,
                    other => {
                        return Err(self
                            .err("while condition must be a bool", statement.span)
                            .with_label(
                                condition.span,
                                format!("this is {}, expected bool", other.type_name()),
                            ));
                    }
                }
                self.push_scope();
                for statement in body {
                    self.evaluate_statement(statement)?;
                    if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                        break;
                    }
                }
                self.pop_scope();
                if self.is_breaking {
                    self.is_breaking = false;
                    break;
                }

                if self.is_continuing {
                    self.is_continuing = false;
                }

                if self.return_value.is_some() {
                    break;
                }
            },

            StatementKind::Range(..) => {}

            StatementKind::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.push_scope();
                self.evaluate_statement(initializer)?;
                loop {
                    let v = self.evaluate(condition)?;
                    match v {
                        Value::Bool(true) => {}
                        Value::Bool(false) => break,
                        other => {
                            return Err(self
                                .err("for condition must be a bool", statement.span)
                                .with_label(
                                    condition.span,
                                    format!("this is {}, expected bool", other.type_name()),
                                ));
                        }
                    }

                    for statement in body {
                        self.evaluate_statement(statement)?;
                        if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                            break;
                        }
                    }

                    if self.is_breaking {
                        self.is_breaking = false;
                        break;
                    }

                    if self.is_continuing {
                        self.is_continuing = false;
                        self.evaluate(increment)?;
                        continue;
                    }

                    if self.return_value.is_some() {
                        break;
                    }

                    self.evaluate(increment)?;
                }
                self.pop_scope();
            }

            StatementKind::ResolvedFor {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.evaluate_statement(initializer)?;
                loop {
                    let v = self.evaluate(condition)?;
                    match v {
                        Value::Bool(true) => {}
                        Value::Bool(false) => break,
                        other => {
                            return Err(self
                                .err("for condition must be a bool", statement.span)
                                .with_label(
                                    condition.span,
                                    format!("this is {}, expected bool", other.type_name()),
                                ));
                        }
                    }

                    for stmt in body {
                        self.evaluate_statement(stmt)?;
                        if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                            break;
                        }
                    }

                    if self.is_breaking {
                        self.is_breaking = false;
                        break;
                    }
                    if self.is_continuing {
                        self.is_continuing = false;
                        self.evaluate(increment)?;
                        continue;
                    }
                    if self.return_value.is_some() {
                        break;
                    }

                    self.evaluate(increment)?;
                }
            }
            StatementKind::Import { names, path } => {
                let module_path = path.join("::");
                let mut module = &self.root_module;
                for seg in path {
                    module = module.submodules.get(seg).ok_or_else(|| {
                        self.err(format!("unknown module '{}'", seg), statement.span)
                    })?;
                }
                let fns: Vec<_> = names
                    .iter()
                    .map(|name| {
                        let f = module.functions.get(name).ok_or_else(|| {
                            self.err(
                                format!("'{}' is not defined in '{}'", name, module_path),
                                statement.span,
                            )
                        })?;
                        Ok((name.clone(), Arc::clone(f)))
                    })
                    .collect::<Result<_, Error>>()?;
                for (name, f) in fns {
                    self.root_module.functions.insert(name, f);
                }
            }

            StatementKind::ResolvedImportFile { body, .. } => {
                for stmt in body {
                    self.evaluate_statement(stmt)?;
                }
            }
            // require adding resolved version in resolver
            StatementKind::ImportFileNamed { path, names } => {
                let import_name = format!("{}.rl", path.join("/"));
                let file_path = if let Some(ref source_file) = self.source_file {
                    let current_file_dir = Path::new(source_file.name.as_ref())
                        .parent()
                        .unwrap_or_else(|| Path::new(""));
                    current_file_dir.join(&import_name)
                } else {
                    import_name.clone().into()
                };

                let source_text = std::fs::read_to_string(&file_path).map_err(|_| {
                    self.err(
                        format!("could not read file '{}'", import_name),
                        statement.span,
                    )
                })?;
                let source_file =
                    SourceFile::new(file_path.to_string_lossy().as_ref(), source_text);
                let tokens = Tokenizer::lex(source_file.clone())?;
                let stmts = Parser::parse(tokens, source_file.clone())?;
                let stmts = self.resolver.resolve_statements(stmts);

                let previous_source = self.source_file.clone();
                self.source_file = Some(source_file);

                for stmt in &stmts {
                    self.evaluate_statement(stmt)?;
                }

                let exported = self.environment.last().cloned().unwrap_or_default();

                self.source_file = previous_source;

                // ImportFileNamed can't filter by name anymore without a name->slot map
                // For now merge all exported slots - named filtering requires ScopeMap
                let _ = names; // TODO: filter by name once ScopeMap is threaded through
                let no_scope_err = self.err("no active scope", statement.span);
                let frame = self.environment.last_mut().ok_or(no_scope_err)?;
                frame.extend(exported);
            }

            StatementKind::ResolvedForRange {
                slot, range, body, ..
            } => {
                let items = match &range.kind {
                    StatementKind::Range(items) => items.clone(),
                    _ => {
                        return Err(
                            self.err("for-range: expected a range statement", statement.span)
                        );
                    }
                };

                for item in items {
                    self.push_scope();
                    self.insert_value(
                        *slot,
                        Value::Integer(item),
                        crate::ast::statements::TypeAnnotation::Int,
                        statement.span,
                    )?;

                    for statement in body {
                        self.evaluate_statement(statement)?;
                        if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                            break;
                        }
                    }

                    self.pop_scope();

                    if self.is_breaking {
                        self.is_breaking = false;
                        break;
                    }

                    if self.is_continuing {
                        self.is_continuing = false;
                    }

                    if self.return_value.is_some() {
                        break;
                    }
                }
            }

            StatementKind::ResolvedForEach {
                slot,
                iterable,
                body,
                ..
            } => {
                let arr = self.evaluate(iterable)?;
                let items = match arr {
                    Value::Values { items, .. } => items,
                    other => {
                        return Err(self
                            .err("for-each: expected an array", statement.span)
                            .with_label(
                                iterable.span,
                                format!("this is {}, expected array", other.type_name()),
                            ));
                    }
                };
                for item in items {
                    let item_type = Evaluator::infer_type(&item, false);
                    self.push_scope();
                    self.insert_value(*slot, item, item_type, statement.span)?;

                    for statement in body {
                        self.evaluate_statement(statement)?;
                        if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                            break;
                        }
                    }
                    self.pop_scope();

                    if self.is_breaking {
                        self.is_breaking = false;
                        break;
                    }

                    if self.is_continuing {
                        self.is_continuing = false;
                    }

                    if self.return_value.is_some() {
                        break;
                    }
                }
            }

            StatementKind::ConditionalBranch { condition, body } => match condition {
                Some(condition) => {
                    let v = self.evaluate(condition)?;
                    match v {
                        Value::Bool(true) => {}
                        Value::Bool(false) => return Ok(()),
                        other => {
                            return Err(self
                                .err("condition must be a bool", statement.span)
                                .with_label(
                                    condition.span,
                                    format!("this is {}, expected bool", other.type_name()),
                                ));
                        }
                    }
                    self.push_scope();
                    for statement in body {
                        self.evaluate_statement(statement)?;
                        if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                            break;
                        }
                    }
                    self.pop_scope();
                }
                _ => {
                    for statement in body {
                        self.evaluate_statement(statement)?;
                        if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                            break;
                        }
                    }
                }
            },

            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                if !self.evaluate_branch(if_branch)?
                    && let Some(branch) = else_branch
                {
                    self.evaluate_branch(branch)?;
                }
            }

            StatementKind::ResolvedFunctionDeclaration {
                slot,
                params,
                return_type,
                body,
                name,
                ..
            } => {
                let func = Value::Function {
                    params: Rc::new(params.clone()),
                    body: Rc::new(body.clone()),
                    return_type: Some(return_type.clone()),
                    captured_env: vec![],
                };
                self.fn_names.insert(name.clone(), *slot);
                self.insert_value(
                    *slot,
                    func,
                    crate::ast::statements::TypeAnnotation::Fn,
                    statement.span,
                )?;
            }

            StatementKind::Return(expr) => {
                let value = match expr {
                    Some(e) => self.evaluate(e)?,
                    None => Value::Null,
                };

                self.return_value = Some(value);
            }

            StatementKind::Break => {
                self.is_breaking = true;
            }

            StatementKind::Continue => {
                self.is_continuing = true;
            }

            StatementKind::ResolvedDestructureDeclaration {
                bindings,
                slots,
                value,
            } => {
                let val = self.evaluate(value)?;
                let items = match val {
                    Value::Tuple(items) => items,
                    other => {
                        return Err(self.err(
                            format!(
                                "expected tuple on right side of destructure, got {}",
                                other.type_name()
                            ),
                            statement.span,
                        ));
                    }
                };
                if items.len() != bindings.len() {
                    return Err(self.err(
                        format!(
                            "destructure mismatch: {} bindings but tuple has {} elements",
                            bindings.len(),
                            items.len()
                        ),
                        statement.span,
                    ));
                }
                for ((type_annotation, _name), (slot, val)) in
                    bindings.iter().zip(slots.iter().zip(items))
                {
                    let val = match (type_annotation, &val) {
                        (TypeAnnotation::Int | TypeAnnotation::CInt, Value::Byte(b)) => {
                            Value::Integer(*b as i64)
                        }
                        _ => val,
                    };
                    let val_type = Self::infer_type(&val, false);
                    if !Self::types_compatible(&val_type, type_annotation)
                        && val_type != *type_annotation
                        && val_type != TypeAnnotation::Null
                    {
                        return Err(self.err(
                            format!(
                                "tuple element type mismatch: expected {:?}, got {:?}",
                                type_annotation, val_type
                            ),
                            statement.span,
                        ));
                    }
                    self.insert_value(*slot, val, type_annotation.clone(), statement.span)?;
                }
            }

            StatementKind::Match { value, arms } => {
                let val = self.evaluate(value)?;
                for (pattern, body) in arms {
                    let matched = match pattern {
                        MatchPattern::Wildcard => true,
                        MatchPattern::Literal(expr) => {
                            let pat_val = self.evaluate(expr)?;
                            val == pat_val
                        }
                    };
                    if matched {
                        self.evaluate_block(body)?;
                        break;
                    }
                }
            }

            _ => {}
        }
        Ok(())
    }

    /// Evaluates a [`ConditionalBranch`] or [`Conditional`] and returns `true` if the
    /// branch was taken (condition was true or it was an `else`).
    fn evaluate_branch(&mut self, statement: &Statement) -> Result<bool, Error> {
        match &statement.kind {
            StatementKind::ConditionalBranch { condition, body } => match condition {
                Some(condition) => {
                    let v = self.evaluate(condition)?;
                    match v {
                        Value::Bool(true) => {
                            self.push_scope();
                            for statement in body {
                                self.evaluate_statement(statement)?;
                                if self.return_value.is_some()
                                    || self.is_breaking
                                    || self.is_continuing
                                {
                                    break;
                                }
                            }
                            self.pop_scope();
                            Ok(true)
                        }
                        Value::Bool(false) => Ok(false),
                        other => Err(self
                            .err("condition must be a bool", statement.span)
                            .with_label(
                                condition.span,
                                format!("this is {}, expected bool", other.type_name()),
                            )),
                    }
                }
                None => {
                    self.push_scope();
                    for statement in body {
                        self.evaluate_statement(statement)?;
                        if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                            break;
                        }
                    }
                    self.pop_scope();
                    Ok(true)
                }
            },
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                if !self.evaluate_branch(if_branch)?
                    && let Some(branch) = else_branch
                {
                    self.evaluate_branch(branch)?;
                }

                Ok(true)
            }
            _ => Err(self.err("expected conditional branch", statement.span)),
        }
    }

    /// The top-level entry point for a full rl program.
    ///
    /// If a function is annotated `!#[entry]` or named `main`, only declarations
    /// and imports are evaluated first, then that function is called with no arguments.
    /// If no entry point exists, all statements are evaluated top-to-bottom (script mode).
    ///
    /// Returns an error if multiple `!#[entry]` functions are found.
    pub fn evaluate_program(&mut self, statements: &[Statement]) -> Result<(), Error> {
        let mut explicit_entry: Option<(Span, usize)> = None;
        let mut main_entry: Option<(Span, usize)> = None;

        for statement in statements {
            if let StatementKind::FunctionDeclaration {
                name, attribute, ..
            }
            | StatementKind::ResolvedFunctionDeclaration {
                name, attribute, ..
            } = &statement.kind
            {
                let slot = match &statement.kind {
                    StatementKind::ResolvedFunctionDeclaration { slot, .. } => Some(*slot),
                    _ => None,
                };

                match attribute {
                    Some(FunctionAttribute::Entry) => {
                        if explicit_entry.is_some() {
                            return Err(
                                self.err("multiple !#[entry] functions found", statement.span)
                            );
                        }
                        if let Some(s) = slot {
                            explicit_entry = Some((statement.span, s));
                        }
                    }

                    Some(FunctionAttribute::Init) => {}
                    Some(FunctionAttribute::Final) => {}
                    Some(FunctionAttribute::Test) => {}

                    &None => {
                        if name == "main"
                            && let Some(s) = slot
                        {
                            main_entry = Some((statement.span, s));
                        }
                    }
                }
            }
        }

        let entry = explicit_entry.or(main_entry);
        let Some((entry_span, entry_slot)) = entry else {
            for statement in statements {
                self.evaluate_statement(statement)?;
            }
            return Ok(());
        };

        for statement in statements {
            match &statement.kind {
                StatementKind::ResolvedImportFile { .. }
                | StatementKind::ResolvedFunctionDeclaration { .. }
                | StatementKind::FunctionDeclaration { .. }
                | StatementKind::Import { .. }
                | StatementKind::ImportFile { .. }
                | StatementKind::ImportFileNamed { .. }
                | StatementKind::ResolvedVariableDeclaration { .. }
                | StatementKind::ResolvedConstantDeclaration { .. }
                | StatementKind::ResolvedArray { .. }
                | StatementKind::ResolvedConstantArray { .. } => {
                    self.evaluate_statement(statement)?
                }
                _ => {}
            }
        }
        let func = self.get_value(0, entry_slot, entry_span)?;
        self.call_value(func, vec![], entry_span)?;
        Ok(())
    }

    /// Evaluates a list of statements inside a fresh scope, used for inline blocks.
    pub fn evaluate_block(&mut self, statements: &[Statement]) -> Result<(), Error> {
        self.push_scope();
        for statement in statements {
            self.evaluate_statement(statement)?;
            if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                break;
            }
        }
        self.pop_scope();
        Ok(())
    }
}
