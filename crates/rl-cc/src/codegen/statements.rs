use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::{ExprId, nodes::ExpressionKind, statements::*};
use rl_utils::errors::Error;

impl<'a> CCodegen<'a> {
    pub fn compile_statement(&mut self, stmt: &Statement) -> Result<(), Error> {
        match &stmt.kind {
            StatementKind::ResolvedVariableDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_type = type_to_c(type_annotation);
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                let expr = self.ast.exprs.get(*value);
                // Track closure return types for unwrapping at call sites
                if let ExpressionKind::ResolvedLambda { return_type, .. } = &expr.kind
                    && let Some(rt) = return_type {
                        self.closure_return_types.insert(name.clone(), rt.clone());
                    }
                if let ExpressionKind::Null = &expr.kind {
                    // Nullable vars stored as rl_result to preserve null tag
                    self.nullable_vars.insert(name.clone());
                    self.writer.write_indent();
                    self.writer.write(&format!("rl_result {} = rl_ok_null();\n", c_name));
                } else if let ExpressionKind::Propagate(inner) = &expr.kind {
                    let temp = self.temp_var();
                    self.writer.write_indent();
                    self.writer.write(&format!("rl_result {} = ", temp));
                    self.compile_expr(*inner)?;
                    self.writer.write(";\n");
                    self.writer.write_indent();
                    self.writer.write(&format!("if (!{}.is_ok) {{\n", temp));
                    self.writer.indent();
                    self.writer.write_indent();
                    if self.is_script_mode {
                        self.writer.write(&format!("rl_println_result({});\n", temp));
                        self.writer.write_indent();
                        self.writer.write("return 1;\n");
                    } else {
                        self.writer.write(&format!("return {};\n", temp));
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                    self.writer.write_indent();
                    self.writer.write(&format!("{} {} = {};\n", c_type, c_name, Self::result_field_access(&c_type, &temp)));
                } else {
                    self.writer.write_indent();
                    self.writer.write(&format!("{} {} = ", c_type, c_name));
                    self.compile_expr(*value)?;
                    self.writer.write(";\n");
                }
            }
            StatementKind::ResolvedConstantDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_type = type_to_c(type_annotation);
                let c_name = mangle(name);
                self.declare(name, &c_name);
                let expr = self.ast.exprs.get(*value);
                if let ExpressionKind::Propagate(inner) = &expr.kind {
                    let temp = self.temp_var();
                    self.writer.write_indent();
                    self.writer.write(&format!("rl_result {} = ", temp));
                    self.compile_expr(*inner)?;
                    self.writer.write(";\n");
                    self.writer.write_indent();
                    self.writer.write(&format!("if (!{}.is_ok) {{\n", temp));
                    self.writer.indent();
                    self.writer.write_indent();
                    if self.is_script_mode {
                        self.writer.write(&format!("rl_println_result({});\n", temp));
                        self.writer.write_indent();
                        self.writer.write("return 1;\n");
                    } else {
                        self.writer.write(&format!("return {};\n", temp));
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                    self.writer.write_indent();
                    self.writer
                        .write(&format!("const {} {} = {};\n", c_type, c_name, Self::result_field_access(&c_type, &temp)));
                } else {
                    self.writer.write_indent();
                    self.writer
                        .write(&format!("const {} {} = ", c_type, c_name));
                    self.compile_expr(*value)?;
                    self.writer.write(";\n");
                }
            }
            StatementKind::ResolvedFunctionDeclaration {
                name,
                params,
                return_type,
                body,
                ..
            } => {
                let c_ret = type_to_c(return_type);
                let c_name = mangle(name);
                self.writer.write_indent();
                self.writer.write(&format!("{} {}(", c_ret, c_name));

                let c_params: Vec<String> = params
                    .iter()
                    .map(|p| {
                        let c_type = type_to_c(&p.param_type);
                        let c_name = mangle(&p.param_name);
                        format!("{} {}", c_type, c_name)
                    })
                    .collect();
                self.writer.write(&c_params.join(", "));
                self.writer.write(") {\n");
                self.writer.indent();

                self.push_scope();
                for p in params {
                    self.declare(&p.param_name, &mangle(&p.param_name));
                }
                for s in body {
                    self.compile_statement(s)?;
                }
                self.pop_scope();
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n\n");
            }
            StatementKind::Expression(expr_id) => {
                let expr = self.ast.exprs.get(*expr_id);
                if let ExpressionKind::Propagate(inner) = &expr.kind {
                    let temp = self.temp_var();
                    self.writer.write_indent();
                    self.writer.write(&format!("rl_result {} = ", temp));
                    self.compile_expr(*inner)?;
                    self.writer.write(";\n");
                    self.writer.write_indent();
                    self.writer.write(&format!("if (!{}.is_ok) {{\n", temp));
                    self.writer.indent();
                    self.writer.write_indent();
                    if self.is_script_mode {
                        self.writer.write(&format!("rl_println_result({});\n", temp));
                        self.writer.write_indent();
                        self.writer.write("return 1;\n");
                    } else {
                        self.writer.write(&format!("return {};\n", temp));
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                } else {
                    self.writer.write_indent();
                    self.compile_expr(*expr_id)?;
                    self.writer.write(";\n");
                }
            }
            StatementKind::Return(Some(expr_id)) => {
                let expr = self.ast.exprs.get(*expr_id);
                if let ExpressionKind::Propagate(inner) = &expr.kind {
                    let temp = self.temp_var();
                    self.writer.write_indent();
                    self.writer.write(&format!("rl_result {} = ", temp));
                    self.compile_expr(*inner)?;
                    self.writer.write(";\n");
                    self.writer.write_indent();
                    self.writer.write(&format!("if (!{}.is_ok) {{\n", temp));
                    self.writer.indent();
                    self.writer.write_indent();
                    self.writer.write(&format!("return {};\n", temp));
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                    self.writer.write_indent();
                    self.writer.write(&format!("return {};\n", Self::result_field_access("rl_result", &temp)));
                } else {
                    self.writer.write_indent();
                    self.writer.write("return ");
                    self.compile_expr(*expr_id)?;
                    self.writer.write(";\n");
                }
            }
            StatementKind::Return(None) => {
                self.writer.writeln("return;");
            }
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                self.write_conditional(if_branch, else_branch)?;
            }
            StatementKind::While { condition, body } => {
                self.writer.write_indent();
                self.writer.write("while (");
                self.compile_expr(*condition)?;
                self.writer.write(") {\n");
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
            StatementKind::ResolvedFor {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.writer.write_indent();
                self.writer.write("for (");
                self.compile_for_init(initializer)?;
                self.writer.write("; ");
                self.compile_expr(*condition)?;
                self.writer.write("; ");
                self.compile_expr(*increment)?;
                self.writer.write(") {\n");
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
            StatementKind::Break => self.writer.writeln("break;"),
            StatementKind::Continue => self.writer.writeln("continue;"),
            StatementKind::Match { value, arms } => {
                self.compile_match(*value, arms)?;
            }
            StatementKind::ResolvedArray {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(
                    name.clone(),
                    TypeAnnotation::Array(Box::new(type_annotation.clone())),
                );
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_array {} = ", c_name));
                self.compile_expr(*value)?;
                self.writer.write(";\n");
            }
            StatementKind::ResolvedConstantArray {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(
                    name.clone(),
                    TypeAnnotation::CArray(Box::new(type_annotation.clone())),
                );
                self.writer.write_indent();
                self.writer
                    .write(&format!("const rl_array {} = ", c_name));
                self.compile_expr(*value)?;
                self.writer.write(";\n");
            }
            StatementKind::ResolvedMap {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_map {} = rl_map_new();\n", c_name));
                let map_expr = self.ast.exprs.get(*value);
                if let ExpressionKind::MapLiteral(entries) = &map_expr.kind {
                    let val_type = match type_annotation {
                        TypeAnnotation::Map(_, vt) | TypeAnnotation::CMap(_, vt) => (**vt).clone(),
                        _ => TypeAnnotation::Int,
                    };
                    for (key_id, val_id) in entries {
                        let key_expr = self.ast.exprs.get(*key_id);
                        if let ExpressionKind::String(key_str) = &key_expr.kind {
                            self.writer.write_indent();
                            self.writer.write(&format!(
                                "rl_map_set(&{}, \"{}\", ",
                                c_name, key_str
                            ));
                            self.emit_value_wrapping(&val_type, *val_id)?;
                            self.writer.write(");\n");
                        }
                    }
                }
            }
            StatementKind::ResolvedConstantMap {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                self.writer.write_indent();
                self.writer.write(&format!(
                    "const rl_map {} = rl_map_new();\n",
                    c_name
                ));
                let map_expr = self.ast.exprs.get(*value);
                if let ExpressionKind::MapLiteral(entries) = &map_expr.kind {
                    let val_type = match type_annotation {
                        TypeAnnotation::Map(_, vt) | TypeAnnotation::CMap(_, vt) => (**vt).clone(),
                        _ => TypeAnnotation::Int,
                    };
                    for (key_id, val_id) in entries {
                        let key_expr = self.ast.exprs.get(*key_id);
                        if let ExpressionKind::String(key_str) = &key_expr.kind {
                            self.writer.write_indent();
                            self.writer.write(&format!(
                                "rl_map_set((rl_map*)&{}, \"{}\", ",
                                c_name, key_str
                            ));
                            self.emit_value_wrapping(&val_type, *val_id)?;
                            self.writer.write(");\n");
                        }
                    }
                }
            }
            StatementKind::ResolvedSet {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_set {} = rl_set_new();\n", c_name));
                let set_expr = self.ast.exprs.get(*value);
                if let ExpressionKind::SetLiteral(items) = &set_expr.kind {
                    let elem_type = match type_annotation {
                        TypeAnnotation::Set(et) | TypeAnnotation::CSet(et) => (**et).clone(),
                        _ => TypeAnnotation::Int,
                    };
                    for item_id in items {
                        self.writer.write_indent();
                        self.writer
                            .write(&format!("rl_set_add(&{}, ", c_name));
                        self.emit_value_wrapping(&elem_type, *item_id)?;
                        self.writer.write(");\n");
                    }
                }
            }
            StatementKind::ResolvedConstantSet {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                self.writer.write_indent();
                self.writer.write(&format!(
                    "const rl_set {} = rl_set_new();\n",
                    c_name
                ));
                let set_expr = self.ast.exprs.get(*value);
                if let ExpressionKind::SetLiteral(items) = &set_expr.kind {
                    let elem_type = match type_annotation {
                        TypeAnnotation::Set(et) | TypeAnnotation::CSet(et) => (**et).clone(),
                        _ => TypeAnnotation::Int,
                    };
                    for item_id in items {
                        self.writer.write_indent();
                        self.writer.write(&format!(
                            "rl_set_add((rl_set*)&{}, ",
                            c_name
                        ));
                        self.emit_value_wrapping(&elem_type, *item_id)?;
                        self.writer.write(");\n");
                    }
                }
            }
            StatementKind::ResolvedDestructureDeclaration {
                bindings,
                value,
                ..
            } => {
                let temp = self.temp_var();
                let field_types: Vec<TypeAnnotation> = bindings.iter().map(|(ta, _)| ta.clone()).collect();
                let tuple_name = self.lookup_tuple_name(&field_types).to_string();
                self.writer.write_indent();
                self.writer.write(&format!("{} {} = ", tuple_name, temp));
                self.compile_expr(*value)?;
                self.writer.write(";\n");
                for (i, (type_annotation, name)) in bindings.iter().enumerate() {
                    let c_type = type_to_c(type_annotation);
                    let c_name = mangle(name);
                    self.declare(name, &c_name);
                    self.var_types.insert(name.clone(), type_annotation.clone());
                    self.writer.write_indent();
                    self.writer.write(&format!(
                        "{} {} = {}.field_{};\n",
                        c_type, c_name, temp, i
                    ));
                }
            }
            StatementKind::ResolvedImplBlock { record, methods } => {
                for m in methods {
                    if let StatementKind::ResolvedFunctionDeclaration {
                        name,
                        params,
                        return_type,
                        body,
                        ..
                    } = &m.kind
                    {
                        let c_ret = type_to_c(return_type);
                        let c_fn_name = format!("impl_{}_{}", record, name);
                        self.writer.write_indent();
                        self.writer
                            .write(&format!("{} {}(", c_ret, c_fn_name));

                        let c_params: Vec<String> = params
                            .iter()
                            .map(|p| {
                                let c_type = type_to_c(&p.param_type);
                                let c_name = mangle(&p.param_name);
                                format!("{} {}", c_type, c_name)
                            })
                            .collect();
                        self.writer.write(&c_params.join(", "));
                        self.writer.write(") {\n");
                        self.writer.indent();

                        self.push_scope();
                        for p in params {
                            self.declare(&p.param_name, &mangle(&p.param_name));
                        }
                        for s in body {
                            self.compile_statement(s)?;
                        }
                        self.pop_scope();
                        self.writer.dedent();
                        self.writer.write_indent();
                        self.writer.write("}\n\n");
                    }
                }
            }
            StatementKind::ResolvedForEach {
                variable,
                iterable,
                body,
                ..
            } => {
                let arr_temp = self.temp_var();
                let idx_temp = self.temp_var();
                self.writer.write_indent();
                self.writer.write(&format!("rl_array {} = ", arr_temp));
                self.compile_expr(*iterable)?;
                self.writer.write(";\n");

                // Resolve element type from the iterable
                let iter_expr = self.ast.exprs.get(*iterable);
                let elem_type = match &iter_expr.kind {
                    ExpressionKind::ResolvedIdentifier { name, .. } => {
                        match self.var_types.get(name) {
                            Some(TypeAnnotation::Array(inner)) => (**inner).clone(),
                            Some(ta) => ta.clone(),
                            None => TypeAnnotation::Int,
                        }
                    }
                    ExpressionKind::ArrayLiteral(elems) if !elems.is_empty() => {
                        let first = self.ast.exprs.get(elems[0]);
                        match &first.kind {
                            ExpressionKind::Integer(_) => TypeAnnotation::Int,
                            ExpressionKind::Float(_) => TypeAnnotation::Float,
                            ExpressionKind::Bool(_) => TypeAnnotation::Bool,
                            ExpressionKind::String(_) => TypeAnnotation::String,
                            _ => TypeAnnotation::Int,
                        }
                    }
                    _ => TypeAnnotation::Int,
                };
                let c_type = type_to_c(&elem_type);

                self.writer.write_indent();
                self.writer
                    .write(&format!("uint64_t {} = 0;\n", idx_temp));
                self.writer.write_indent();
                self.writer.write(&format!(
                    "for (; {} < {}.len; {}++) {{\n",
                    idx_temp, arr_temp, idx_temp
                ));
                self.writer.indent();
                let c_name = mangle(variable);
                self.declare(variable, &c_name);
                self.var_types.insert(variable.clone(), elem_type);
                self.writer.write_indent();
                self.writer.write(&format!(
                    "{} {} = (({}*){}.data)[{}];\n",
                    c_type, c_name, c_type, arr_temp, idx_temp
                ));
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
            StatementKind::ResolvedForRange {
                variable,
                range,
                body,
                ..
            } => {
                let items = match &range.kind {
                    StatementKind::Range(items) => items.clone(),
                    _ => vec![],
                };
                if !items.is_empty() {
                    let first = items[0];
                    let last = items[items.len() - 1];
                    let c_name = mangle(variable);
                    self.declare(variable, &c_name);
                    self.var_types
                        .insert(variable.clone(), TypeAnnotation::Int);
                    self.writer.write_indent();
                    self.writer.write(&format!(
                        "for (int64_t {} = {}; {} < {}; {}++) {{\n",
                        c_name, first, c_name, last + 1, c_name
                    ));
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                }
            }
            StatementKind::Loop(body) => {
                self.writer.write_indent();
                self.writer.write("while (1) {\n");
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
            StatementKind::ResolvedImportFile { body, .. } => {
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
            }
            StatementKind::Import { names, wildcard, path } => {
                if path.len() >= 2 && path[0] == "std" && path[1] == "c" {
                    if *wildcard {
                        self.std_c_imports.insert("*".to_string());
                    } else {
                        for (name, _alias) in names {
                            self.std_c_imports.insert(name.clone());
                        }
                    }
                }
                if path.len() >= 2 && path[0] == "std" && path[1] == "net" {
                    if *wildcard {
                        self.std_net_imports.insert("*".to_string());
                    } else {
                        for (name, _alias) in names {
                            self.std_net_imports.insert(name.clone());
                        }
                    }
                }
                if path.len() >= 2 && path[0] == "std" && path[1] == "http" {
                    if *wildcard {
                        self.std_http_imports.insert("*".to_string());
                    } else {
                        for (name, _alias) in names {
                            self.std_http_imports.insert(name.clone());
                        }
                    }
                }
            }
            StatementKind::ImportFile { .. } | StatementKind::ImportFileNamed { .. } => {}
            _ => {}
        }
        Ok(())
    }

    fn compile_for_init(&mut self, stmt: &Statement) -> Result<(), Error> {
        if let StatementKind::ResolvedVariableDeclaration {
            name,
            type_annotation,
            value,
            ..
        } = &stmt.kind
        {
            let c_type = type_to_c(type_annotation);
            let c_name = mangle(name);
            self.declare(name, &c_name);
            self.writer.write(&format!("{} {} = ", c_type, c_name));
            self.compile_expr(*value)?;
        }
        Ok(())
    }

    pub fn write_conditional(
        &mut self,
        if_branch: &Statement,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<(), Error> {
        if let StatementKind::ConditionalBranch {
            condition, body, ..
        } = &if_branch.kind
        {
            self.writer.write_indent();
            if let Some(cond) = condition {
                self.writer.write("if (");
                self.compile_expr(*cond)?;
                self.writer.write(") {\n");
            } else {
                self.writer.write("{\n");
            }
            self.writer.indent();
            for s in body {
                self.compile_statement(s)?;
            }
            self.writer.dedent();
            self.writer.write_indent();
            if else_branch.is_some() {
                self.writer.write("} ");
            } else {
                self.writer.write("}\n");
            }
        }

        if let Some(else_stmt) = else_branch {
            self.write_else_branch(else_stmt)?;
        }

        Ok(())
    }

    fn write_else_branch(&mut self, else_stmt: &Statement) -> Result<(), Error> {
        match &else_stmt.kind {
            StatementKind::ConditionalBranch {
                condition, body, ..
            } => {
                if let Some(cond) = condition {
                    self.writer.write("else if (");
                    self.compile_expr(*cond)?;
                    self.writer.write(") {\n");
                } else {
                    self.writer.write("else {\n");
                }
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                self.writer.write("else ");
                // Recurse but skip the indent since we're already on the } line
                if let StatementKind::ConditionalBranch {
                    condition, body, ..
                } = &if_branch.kind
                {
                    if let Some(cond) = condition {
                        self.writer.write("if (");
                        self.compile_expr(*cond)?;
                        self.writer.write(") {\n");
                    } else {
                        self.writer.write("{\n");
                    }
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    if else_branch.is_some() {
                        self.writer.write("} ");
                    } else {
                        self.writer.write("}\n");
                    }
                    if let Some(inner_else) = else_branch {
                        self.write_else_branch(inner_else)?;
                    }
                }
            }
            _ => {
                self.writer.write("else {\n");
                self.writer.indent();
                self.compile_statement(else_stmt)?;
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
        }
        Ok(())
    }

    pub fn compile_match(
        &mut self,
        value: ExprId,
        arms: &[(MatchPattern, Vec<Statement>)],
    ) -> Result<(), Error> {
        for (i, (pattern, body)) in arms.iter().enumerate() {
            match pattern {
                MatchPattern::Literal(lit_id) => {
                    self.writer.write_indent();
                    if i == 0 {
                        self.writer.write("if (");
                    } else {
                        self.writer.write("else if (");
                    }
                    self.compile_expr(value)?;
                    self.writer.write(" == ");
                    self.compile_expr(*lit_id)?;
                    self.writer.write(") {\n");
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                }
                MatchPattern::Wildcard => {
                    self.writer.write_indent();
                    self.writer.write("else {\n");
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                }
            }
        }
        Ok(())
    }

    fn result_field_access(c_type: &str, temp: &str) -> String {
        match c_type {
            "int64_t" | "uint64_t" | "int32_t" | "uint32_t" | "int16_t" | "uint16_t" | "int8_t" | "uint8_t" | "char" => format!("{}.data.i64", temp),
            "double" | "float" => format!("{}.data.f64", temp),
            "bool" => format!("{}.data.boolean", temp),
            "rl_string" => format!("{}.data.str", temp),
            "rl_array" => format!("{}.data.arr", temp),
            "rl_map" => format!("{}.data.map", temp),
            "rl_set" => format!("{}.data.set", temp),
            "rl_result" => temp.to_string(),
            _ => format!("{}.data.i64", temp),
        }
    }
}
