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
use rl_utils::errors::{Error, Reason};
use rl_utils::span::Span;

impl<'a> CCodegen<'a> {
    pub fn compile_statement(&mut self, stmt: &Statement) -> Result<(), Error> {
        // Hoisted literal temps never cross statement boundaries.
        self.hoisted_tmps.clear();
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
                // Inlined module body: its declarations were pre-declared
                // as globals, so initialize them without redeclaring.
                let was_init = self.in_global_init;
                self.in_global_init = true;
                for stmt in body {
                    // Only declarations need init mode; anything else
                    // compiles normally.
                    if crate::codegen::CCodegen::is_global_decl(&stmt.kind) {
                        self.compile_statement(stmt)?;
                    } else {
                        self.in_global_init = false;
                        let result = self.compile_statement(stmt);
                        self.in_global_init = true;
                        result?;
                    }
                }
                self.in_global_init = was_init;
                Ok(())
            }
            StatementKind::Import {
                names,
                wildcard,
                path,
            } => {
                // Recorded again here for any path that compiles statements
                // without the emit_program pre-scan; duplicates are harmless.
                self.record_import(names, *wildcard, path);
                Ok(())
            }
            StatementKind::ImportFile { .. } | StatementKind::ImportFileNamed { .. } => Ok(()),
            // Record and tag declarations emit their C types in the header.
            StatementKind::RecordDeclaration { .. } | StatementKind::TagDeclaration { .. } => Ok(()),
            other => Err(Error::at(
                Reason::Compile,
                format!(
                    "statement kind not supported by the C transpiler: {:?}",
                    other
                ),
                Span::dummy(),
            )),
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
