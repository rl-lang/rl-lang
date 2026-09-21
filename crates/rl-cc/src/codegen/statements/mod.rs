pub mod branches;
pub mod collections;
pub mod control;
pub mod declarations;
pub mod functions;
pub mod loops;
pub mod match_compile;
pub mod propagate;

use crate::codegen::CCodegen;
use rl_ast::statements::*;
use rl_utils::errors::Error;

impl<'a> CCodegen<'a> {
    pub fn compile_statement(&mut self, stmt: &Statement) -> Result<(), Error> {
        match &stmt.kind {
            StatementKind::ResolvedVariableDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => declarations::compile_var_decl(self, name, type_annotation, *value),
            StatementKind::ResolvedConstantDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => declarations::compile_const_decl(self, name, type_annotation, *value),
            StatementKind::ResolvedFunctionDeclaration {
                name,
                params,
                return_type,
                body,
                ..
            } => functions::compile_function_decl(self, name, params, return_type, body),
            StatementKind::Expression(expr_id) => control::compile_expr_stmt(self, *expr_id),
            StatementKind::Return(ret) => control::compile_return(self, *ret),
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => branches::write_conditional(self, if_branch, else_branch),
            StatementKind::While { condition, body } => {
                control::compile_while(self, *condition, body)
            }
            StatementKind::ResolvedFor {
                initializer,
                condition,
                increment,
                body,
            } => control::compile_for(self, initializer, *condition, *increment, body),
            StatementKind::Break => {
                self.writer.writeln("break;");
                Ok(())
            }
            StatementKind::Continue => {
                self.writer.writeln("continue;");
                Ok(())
            }
            StatementKind::Match { value, arms } => {
                match_compile::compile_match(self, *value, arms)
            }
            StatementKind::ResolvedArray {
                name,
                type_annotation,
                value,
                ..
            } => collections::compile_array_decl(self, name, type_annotation, *value, false),
            StatementKind::ResolvedConstantArray {
                name,
                type_annotation,
                value,
                ..
            } => collections::compile_array_decl(self, name, type_annotation, *value, true),
            StatementKind::ResolvedMap {
                name,
                type_annotation,
                value,
                ..
            } => collections::compile_map_decl(self, name, type_annotation, *value, false),
            StatementKind::ResolvedConstantMap {
                name,
                type_annotation,
                value,
                ..
            } => collections::compile_map_decl(self, name, type_annotation, *value, true),
            StatementKind::ResolvedSet {
                name,
                type_annotation,
                value,
                ..
            } => collections::compile_set_decl(self, name, type_annotation, *value, false),
            StatementKind::ResolvedConstantSet {
                name,
                type_annotation,
                value,
                ..
            } => collections::compile_set_decl(self, name, type_annotation, *value, true),
            StatementKind::ResolvedDestructureDeclaration {
                bindings,
                value,
                ..
            } => declarations::compile_destructure(self, bindings, *value),
            StatementKind::ResolvedImplBlock { record, methods } => {
                functions::compile_impl_block(self, record, methods)
            }
            StatementKind::ResolvedForEach {
                variable,
                iterable,
                body,
                ..
            } => loops::compile_foreach(self, variable, *iterable, body),
            StatementKind::ResolvedForRange {
                variable,
                range,
                body,
                ..
            } => loops::compile_for_range(self, variable, range, body),
            StatementKind::Loop(body) => control::compile_loop(self, body),
            StatementKind::ResolvedImportFile { body, .. } => {
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                Ok(())
            }
            StatementKind::Import {
                names,
                wildcard,
                path,
            } => {
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
                Ok(())
            }
            StatementKind::ImportFile { .. } | StatementKind::ImportFileNamed { .. } => Ok(()),
            _ => Ok(()),
        }
    }
}

/// Maps an RL value's C type to the `rl_result.data` union field holding it.
/// Used to unwrap a propagated `rl_result` into a plain C value.
pub(super) fn result_field_access(c_type: &str, temp: &str) -> String {
    match c_type {
        "int64_t" | "uint64_t" | "int32_t" | "uint32_t" | "int16_t" | "uint16_t" | "int8_t"
        | "uint8_t" | "char" => format!("{}.data.i64", temp),
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
