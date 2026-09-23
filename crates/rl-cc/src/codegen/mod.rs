use crate::writer::CWriter;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::{Ast, statements::*};
use rl_checker::structs::{CheckType, TypeChecker};
use rl_utils::errors::{Error, Reason};
use rl_utils::span::Span;
use std::collections::{HashMap, HashSet};

pub mod expressions;
pub mod infer;
pub mod ops;
pub mod scope;
pub mod statements;
pub mod stdlib_names;

/// One `get ... from std::ns` import record, kept in source order.
/// Mirrors the VM's `stdlib_methods` table where later imports overwrite
/// earlier ones on name conflicts.
#[derive(Clone)]
pub enum StdImport {
    /// `get name as alias from std::ns` (`alias` defaults to `name`).
    Named {
        visible: String,
        namespace: String,
        original: String,
    },
    /// `get * from std::ns`.
    Wildcard { namespace: String },
}

pub struct CCodegen<'a> {
    pub ast: &'a Ast,
    pub checker: &'a TypeChecker,
    pub writer: CWriter,
    pub scopes: Vec<HashMap<String, String>>,
    pub var_types: HashMap<String, TypeAnnotation>,
    pub emitted_includes: bool,
    pub is_script_mode: bool,
    pub temp_counter: usize,
    pub lambda_counter: usize,
    pub static_funcs: Vec<String>,
    pub closure_params: Vec<String>,
    pub closure_return_types: HashMap<String, TypeAnnotation>,
    pub tuple_names: Vec<(Vec<TypeAnnotation>, String)>,
    pub nullable_vars: HashSet<String>,
    pub std_c_imports: HashSet<String>,
    pub std_net_imports: HashSet<String>,
    pub std_http_imports: HashSet<String>,
    pub std_fs_imports: HashSet<String>,
    pub std_imports: Vec<StdImport>,
    /// Lambdas that return a closure literal, mapped to the inner
    /// lambda's return type (`dec mk = fn(k) { return fn(x)->int... }`
    /// records `mk -> int`). Lets calls through factory results unwrap.
    pub closure_factories: HashMap<String, TypeAnnotation>,
    pub user_fns: HashSet<String>,
    /// Declared return types of top-level user functions, for inferring
    /// the type of `CallExpr` results (`dec x = get_filtered_notes()`).
    pub user_fn_returns: HashMap<String, TypeAnnotation>,
    /// Top-level variables, emitted at C file scope so every function
    /// body can read and assign them.
    pub global_names: HashSet<String>,
    /// True while emitting top-level initializers inside `main`: storage
    /// was already declared at file scope, so only assign.
    pub in_global_init: bool,
    /// True while compiling a lambda body: `return` values wrap with
    /// `rl_ok`, and `?` guards return the result instead of exiting.
    pub in_lambda_body: bool,
    /// Return type of the enclosing function, if any. Top-level code
    /// leaves it None.
    pub fn_return: Option<TypeAnnotation>,
    /// Disambiguates shadowed C names (`c`, `c_0`, ...).
    pub shadow_counter: usize,
    /// Scope snapshots for the flat type maps, pushed alongside scopes.
    pub type_snapshots: Vec<(
        HashMap<String, TypeAnnotation>,
        HashSet<String>,
        HashMap<String, TypeAnnotation>,
        HashMap<String, TypeAnnotation>,
    )>,
    /// Expected element type for an empty array literal (`[]`), set by
    /// the surrounding declaration or assignment context.
    pub array_elem_hint: Option<TypeAnnotation>,
    /// File-scope global definitions, emitted before `main`.
    pub globals_code: String,
    /// File-scope tuple typedefs, emitted before globals.
    pub tuple_defs: String,
    pub record_defs: String,
    /// Statement-shaped literal temps hoisted before the current
    /// statement (`dec m = {...}`, arrays holding map/set literals).
    /// `compile_expr` emits the recorded temp instead of rebuilding.
    pub hoisted_tmps: HashMap<rl_ast::ExprId, String>,
}

impl<'a> CCodegen<'a> {
    pub fn new(ast: &'a Ast, checker: &'a TypeChecker) -> Self {
        Self {
            ast,
            checker,
            writer: CWriter::new(),
            scopes: vec![HashMap::new()],
            var_types: HashMap::new(),
            emitted_includes: false,
            is_script_mode: false,
            temp_counter: 0,
            lambda_counter: 0,
            static_funcs: Vec::new(),
            closure_params: Vec::new(),
            closure_return_types: HashMap::new(),
            tuple_names: Vec::new(),
            nullable_vars: HashSet::new(),
            std_c_imports: HashSet::new(),
            std_imports: Vec::new(),
            closure_factories: HashMap::new(),
            user_fns: HashSet::new(),
            user_fn_returns: HashMap::new(),
            global_names: HashSet::new(),
            tuple_defs: String::new(),
            record_defs: String::new(),
            in_global_init: false,
            in_lambda_body: false,
            fn_return: None,
            shadow_counter: 0,
            type_snapshots: Vec::new(),
            array_elem_hint: None,
            globals_code: String::new(),
            std_net_imports: HashSet::new(),
            std_http_imports: HashSet::new(),
            std_fs_imports: HashSet::new(),
            hoisted_tmps: HashMap::new(),
        }
    }

    /// Records one `get ... from std::ns` import. Called both by the
    /// pre-scan (so hoisted function bodies resolve methods) and by the
    /// `Import` statement arm itself.
    pub fn record_import(
        &mut self,
        names: &[(String, Option<String>)],
        wildcard: bool,
        path: &[String],
    ) {
        if path.is_empty() || path[0] != "std" {
            return;
        }
        let namespace = path.join("::");
        if wildcard {
            self.std_imports.push(StdImport::Wildcard {
                namespace: namespace.clone(),
            });
        } else {
            for (name, alias) in names {
                self.std_imports.push(StdImport::Named {
                    visible: alias.clone().unwrap_or_else(|| name.clone()),
                    namespace: namespace.clone(),
                    original: name.clone(),
                });
            }
        }
        if path.len() >= 2 && path[1] == "c" {
            if wildcard {
                self.std_c_imports.insert("*".to_string());
            } else {
                for (name, _alias) in names {
                    self.std_c_imports.insert(name.clone());
                }
            }
        }
        if path.len() >= 2 && path[1] == "net" {
            if wildcard {
                self.std_net_imports.insert("*".to_string());
            } else {
                for (name, _alias) in names {
                    self.std_net_imports.insert(name.clone());
                }
            }
        }
        if path.len() >= 2 && path[1] == "http" {
            if wildcard {
                self.std_http_imports.insert("*".to_string());
            } else {
                for (name, _alias) in names {
                    self.std_http_imports.insert(name.clone());
                }
            }
        }
        if path.len() >= 2 && path[1] == "fs" {
            if wildcard {
                self.std_fs_imports.insert("*".to_string());
            } else {
                for (name, _alias) in names {
                    self.std_fs_imports.insert(name.clone());
                }
            }
        }
    }

    /// True for top-level declaration statements, which become C file-scope
    /// globals initialized inside `main`.
    pub(crate) fn is_global_decl(kind: &StatementKind) -> bool {
        matches!(
            kind,
            StatementKind::ResolvedVariableDeclaration { .. }
                | StatementKind::ResolvedConstantDeclaration { .. }
                | StatementKind::ResolvedArray { .. }
                | StatementKind::ResolvedConstantArray { .. }
                | StatementKind::ResolvedMap { .. }
                | StatementKind::ResolvedConstantMap { .. }
                | StatementKind::ResolvedSet { .. }
                | StatementKind::ResolvedConstantSet { .. }
                | StatementKind::ResolvedDestructureDeclaration { .. }
        )
    }

    /// Pre-scan pass declaring every top-level variable at file scope
    /// (storage only; initializers run inside `main`). Recurses into
    /// inlined file bodies. Function bodies compiled afterwards resolve
    /// globals by name and type.
    fn declare_globals(&mut self, statements: &[Statement]) {
        for stmt in statements {
            match &stmt.kind {
                StatementKind::ResolvedVariableDeclaration {
                    name,
                    type_annotation,
                    value,
                    ..
                } => {
                    crate::codegen::statements::declarations::declare_global_var(
                        self,
                        name,
                        type_annotation,
                        *value,
                    );
                }
                StatementKind::ResolvedConstantDeclaration {
                    name,
                    type_annotation,
                    value,
                    ..
                } => {
                    crate::codegen::statements::declarations::declare_global_const(
                        self,
                        name,
                        type_annotation,
                        *value,
                    );
                }
                StatementKind::ResolvedArray {
                    name, type_annotation, ..
                } => {
                    crate::codegen::statements::collections::declare_global_array(
                        self,
                        name,
                        type_annotation,
                        false,
                    );
                }
                StatementKind::ResolvedConstantArray {
                    name, type_annotation, ..
                } => {
                    crate::codegen::statements::collections::declare_global_array(
                        self,
                        name,
                        type_annotation,
                        true,
                    );
                }
                StatementKind::ResolvedMap {
                    name, type_annotation, ..
                }
                | StatementKind::ResolvedConstantMap {
                    name, type_annotation, ..
                }
                | StatementKind::ResolvedSet {
                    name, type_annotation, ..
                }
                | StatementKind::ResolvedConstantSet {
                    name, type_annotation, ..
                } => {
                    crate::codegen::statements::collections::declare_global_map_set(
                        self,
                        name,
                        type_annotation,
                    );
                }
                StatementKind::ResolvedDestructureDeclaration { bindings, .. } => {
                    crate::codegen::statements::declarations::declare_global_destructure(
                        self, bindings,
                    );
                }
                StatementKind::ResolvedImportFile { body, .. } => {
                    self.declare_globals(body);
                }
                _ => {}
            }
        }
    }

    /// Compile one top-level statement, using init-only emission for
    /// globals whose storage was declared at file scope.
    fn compile_top_level(&mut self, stmt: &Statement) -> Result<(), Error> {
        if Self::is_global_decl(&stmt.kind) {
            self.in_global_init = true;
            let result = self.compile_statement(stmt);
            self.in_global_init = false;
            result
        } else {
            self.compile_statement(stmt)
        }
    }

    /// Pre-scan pass over the whole program (including inlined file bodies)
    /// recording every import before any function body compiles. Hoisted
    /// functions and lambdas resolve methods and aliases against this.
    fn record_all_imports(&mut self, statements: &[Statement]) {
        for stmt in statements {
            match &stmt.kind {
                StatementKind::Import {
                    names,
                    wildcard,
                    path,
                } => {
                    self.record_import(names, *wildcard, path);
                }
                StatementKind::ResolvedImportFile { body, .. } => {
                    self.record_all_imports(body);
                }
                _ => {}
            }
        }
    }

    /// Resolves a visible function name (bare call or method) against the
    /// recorded stdlib imports, mirroring the VM's `stdlib_methods` table:
    /// later imports win. Returns (namespace, original name).
    pub fn resolve_std_name(&self, name: &str) -> Option<(String, String)> {
        let mut found = None;
        for import in &self.std_imports {
            match import {
                StdImport::Named {
                    visible,
                    namespace,
                    original,
                } => {
                    if visible == name {
                        found = Some((namespace.clone(), original.clone()));
                    }
                }
                StdImport::Wildcard { namespace } => {
                    if crate::codegen::stdlib_names::namespace_provides(namespace, name) {
                        found = Some((namespace.clone(), name.to_string()));
                    }
                }
            }
        }
        found
    }

    /// Scans top-level function declarations for the entry point (`!#[entry]`
    /// or `main`/`__entry__` fallback), tests, inits and finals. Mirrors the
    /// VM's `scan_entry_points`: numbered init/final priorities run first in
    /// ascending order, unnumbered ones last in declaration order.
    fn scan_entry(
        statements: &[Statement],
    ) -> Result<
        Option<(
            String,
            Vec<String>,
            Vec<String>,
            Vec<String>,
        )>,
        Error,
    > {
        let mut explicit_entry: Option<String> = None;
        let mut main_entry: Option<String> = None;
        let mut tests: Vec<String> = Vec::new();
        let mut inits: Vec<(String, Option<u32>, usize)> = Vec::new();
        let mut finals: Vec<(String, Option<u32>, usize)> = Vec::new();

        for (order, stmt) in statements.iter().enumerate() {
            let StatementKind::ResolvedFunctionDeclaration {
                name,
                attribute,
                ..
            } = &stmt.kind
            else {
                continue;
            };
            match attribute {
                Some(FunctionAttribute::Entry) => {
                    if explicit_entry.is_some() {
                        return Err(Error::at(
                            Reason::Compile,
                            "multiple !#[entry] functions found",
                            Span::dummy(),
                        ));
                    }
                    explicit_entry = Some(name.clone());
                }
                Some(FunctionAttribute::Test) => tests.push(name.clone()),
                Some(FunctionAttribute::Init(priority)) => {
                    inits.push((name.clone(), *priority, order));
                }
                Some(FunctionAttribute::Final(priority)) => {
                    finals.push((name.clone(), *priority, order));
                }
                None if name == "main" || name == "__entry__" => {
                    main_entry = Some(name.clone());
                }
                _ => {}
            }
        }

        let Some(entry) = explicit_entry.or(main_entry) else {
            return Ok(None);
        };
        // Numbered priorities first ascending, unnumbered last in order.
        // `sort_by_key` is stable, so declaration order is preserved.
        inits.sort_by_key(|(_, p, _)| (p.is_none(), p.unwrap_or(0)));
        finals.sort_by_key(|(_, p, _)| (p.is_none(), p.unwrap_or(0)));
        Ok(Some((
            entry,
            tests,
            inits.into_iter().map(|(n, _, _)| n).collect(),
            finals.into_iter().map(|(n, _, _)| n).collect(),
        )))
    }

    /// True for top-level statements that run in entry mode. Mirrors the
    /// VM's `is_program_setup_statement`: only declarations and imports
    /// execute; control flow and bare expressions are skipped.
    fn is_entry_setup(kind: &StatementKind) -> bool {
        matches!(
            kind,
            StatementKind::ResolvedImportFile { .. }
                | StatementKind::Import { .. }
                | StatementKind::ImportFile { .. }
                | StatementKind::ImportFileNamed { .. }
                | StatementKind::ResolvedVariableDeclaration { .. }
                | StatementKind::ResolvedConstantDeclaration { .. }
                | StatementKind::ResolvedArray { .. }
                | StatementKind::ResolvedConstantArray { .. }
                | StatementKind::ResolvedMap { .. }
                | StatementKind::ResolvedConstantMap { .. }
                | StatementKind::ResolvedSet { .. }
                | StatementKind::ResolvedConstantSet { .. }
                | StatementKind::ResolvedDestructureDeclaration { .. }
        )
    }

    pub fn emit_program(&mut self, statements: &[Statement]) -> Result<String, rl_utils::errors::Error> {
        self.emit_header(statements);

        // Top-level `?` fails the program the same way in both modes.
        self.is_script_mode = true;

        // Collect top-level user function names for the method-call fallback
        // (`x.double()` calls user `fn double(x)`), mirroring the VM's
        // `user_methods` table, plus their return types for inference.
        // Unannotated (`Null`) returns come from the checker's body
        // inference, which patched precise types into the root scope.
        for stmt in statements {
            if let StatementKind::ResolvedFunctionDeclaration {
                name, return_type, ..
            } = &stmt.kind
            {
                self.user_fns.insert(name.clone());
                let mut resolved_return = return_type.clone();
                if resolved_return == TypeAnnotation::Null {
                    if let Some(inferred) = self
                        .checker
                        .scopes
                        .first()
                        .and_then(|scope| scope.get(name))
                        .and_then(|item| match &item.type_annotation {
                            CheckType::Function { return_type, .. }
                                if *return_type != TypeAnnotation::Null =>
                            {
                                Some(return_type.clone())
                            }
                            _ => None,
                        })
                    {
                        resolved_return = inferred;
                    }
                }
                self.user_fn_returns
                    .insert(name.clone(), resolved_return);
            }
        }

        // Record imports before anything compiles so hoisted function
        // bodies resolve methods and aliases.
        self.record_all_imports(statements);

        // Declare top-level variables at file scope before hoisting so
        // function bodies see every global's name and type.
        self.declare_globals(statements);

        let entry = Self::scan_entry(statements)?;

        // Entry mode: tests, inits, entry, finals around the setup
        // statements. Script mode: everything runs top to bottom.
        if let Some((entry_name, tests, inits, finals)) = entry {
            let is_main_entry = entry_name == "main" || entry_name == "__entry__";
            // A `main` entry runs inline inside the C `main` wrapper, so
            // its definition is skipped during hoisting to avoid a clash.
            let mut entry_body: Option<Vec<Statement>> = None;
            for stmt in statements {
                match &stmt.kind {
                    StatementKind::ResolvedFunctionDeclaration {
                        name, body, params, ..
                    } if is_main_entry && name == &entry_name => {
                        if !params.is_empty() {
                            return Err(Error::at(
                                Reason::Compile,
                                "entry function `main` must take no arguments",
                                Span::dummy(),
                            ));
                        }
                        entry_body = Some(body.clone());
                    }
                    StatementKind::ResolvedFunctionDeclaration { .. } => {
                        self.compile_statement(stmt)?;
                    }
                    StatementKind::ResolvedImplBlock { .. } => {
                        self.compile_statement(stmt)?;
                    }
                    _ => {}
                }
            }
            self.writer.write("int main(int argc, char **argv) {\n");
            self.writer.indent();
            self.writer.writeln("rl_store_args(argc, argv);");
            for stmt in statements {
                if Self::is_entry_setup(&stmt.kind) {
                    self.compile_top_level(stmt)?;
                }
            }
            for name in &tests {
                self.writer.write_indent();
                self.writer.write(&format!("{}();\n", mangle(name)));
            }
            for name in &inits {
                self.writer.write_indent();
                self.writer.write(&format!("{}();\n", mangle(name)));
            }
            if is_main_entry {
                // Main body is function scope: its declarations are locals.
                for stmt in entry_body.unwrap_or_default() {
                    self.compile_statement(&stmt)?;
                }
            } else {
                self.writer.write_indent();
                self.writer.write(&format!("{}();\n", mangle(&entry_name)));
            }
            for name in &finals {
                self.writer.write_indent();
                self.writer.write(&format!("{}();\n", mangle(name)));
            }
            self.writer.writeln("return 0;");
            self.writer.dedent();
            self.writer.writeln("}");
        } else {
            self.is_script_mode = true;
            // Hoist function declarations and impl methods before main
            for stmt in statements {
                match &stmt.kind {
                    StatementKind::ResolvedFunctionDeclaration { .. } => {
                        self.compile_statement(stmt)?;
                    }
                    StatementKind::ResolvedImplBlock { .. } => {
                        self.compile_statement(stmt)?;
                    }
                    _ => {}
                }
            }
            self.writer.write("int main(int argc, char **argv) {\n");
            self.writer.indent();
            self.writer.writeln("rl_store_args(argc, argv);");

            for stmt in statements {
                if !matches!(&stmt.kind,
                    StatementKind::ResolvedFunctionDeclaration { .. }
                    | StatementKind::RecordDeclaration { .. }
                    | StatementKind::TagDeclaration { .. }
                    | StatementKind::ResolvedImplBlock { .. }
                    | StatementKind::ImplBlock { .. }
                ) {
                    self.compile_top_level(stmt)?;
                }
            }

            self.writer.writeln("return 0;");
            self.writer.dedent();
            self.writer.writeln("}");
        }

        // Combine: insert tuple typedefs, globals, then static lambda
        // functions at file scope, in that order.
        let mut output = self.writer.source().to_string();
        let mut file_scope = String::new();
        if !self.record_defs.is_empty() {
            file_scope.push_str(&format!("\n/* record types */\n{}\n", self.record_defs));
        }
        if !self.tuple_defs.is_empty() {
            file_scope.push_str(&format!("\n/* tuple types */\n{}\n", self.tuple_defs));
        }
        if !self.globals_code.is_empty() {
            file_scope.push_str(&format!("\n/* top-level globals */\n{}\n", self.globals_code));
        }
        if !self.static_funcs.is_empty() {
            let static_funcs_str: String = self.static_funcs.iter().cloned().collect();
            file_scope.push_str(&format!("\n{}\n", static_funcs_str));
        }
        if !file_scope.is_empty() {
            // Insert right after the #include "rl_runtime.h" line
            if let Some(pos) = output.find("#include \"rl_runtime.h\"") {
                let insert_pos = pos + "#include \"rl_runtime.h\"".len();
                // Skip past the newline after the include
                let insert_pos = if output.as_bytes().get(insert_pos) == Some(&b'\n') {
                    insert_pos + 1
                } else {
                    insert_pos
                };
                output.insert_str(insert_pos, &file_scope);
            } else {
                output.push_str(&file_scope);
            }
        }

        Ok(output)
    }

    fn emit_header(&mut self, statements: &[Statement]) {
        if self.emitted_includes {
            return;
        }
        self.emitted_includes = true;

        self.writer.writeln("#define _GNU_SOURCE");
        self.writer.writeln("#define _POSIX_C_SOURCE 200809L");
        self.writer.writeln("#include <stdint.h>");
        self.writer.writeln("#include <stdbool.h>");
        self.writer.writeln("#include <stdio.h>");
        self.writer.writeln("#include <stdlib.h>");
        self.writer.writeln("#include <string.h>");
        self.writer.writeln("#include \"rl_runtime.h\"");
        self.writer.blank_line();

        // Emit record typedefs and print functions into record_defs
        // (file scope, ahead of globals): self.writer content lands
        // after the file_scope insert.
        for (name, fields) in &self.checker.records.clone() {
            if self
                .record_defs
                .contains(&format!("}} rl_Record_{};", name))
            {
                continue;
            }
            let mut def = String::new();
            def.push_str("typedef struct { ");
            for (field_name, field_type) in fields {
                let c_type = type_to_c(field_type);
                def.push_str(&format!("{} {}; ", c_type, field_name));
            }
            def.push_str(&format!("}} rl_Record_{};\n", name));
            // Generate print function
            def.push_str(&format!(
                "void rl_print_rl_Record_{}(rl_Record_{} v) {{ ",
                name, name
            ));
            def.push_str("printf(\"Record(\");\n");
            // Render field printers into a side buffer via writer swap.
            let saved = std::mem::take(&mut self.writer);
            for (i, (field_name, field_type)) in fields.iter().enumerate() {
                if i > 0 {
                    self.writer.writeln("printf(\", \");");
                }
                self.writer.write_indent();
                self.writer.write(&format!("printf(\"{}: \");\n", field_name));
                self.emit_field_print(field_type, &format!("v.{}", field_name));
            }
            let rendered = std::mem::replace(&mut self.writer, saved).into_source();
            def.push_str(&rendered);
            def.push_str("printf(\")\");\n");
            def.push_str("}\n");
            def.push_str(&format!("void rl_println_rl_Record_{}(rl_Record_{} v) {{ rl_print_rl_Record_{}(v); printf(\"\\n\"); }}\n", name, name, name));
            self.record_defs.push_str(&def);
        }
        if !self.checker.records.is_empty() {
            self.writer.blank_line();
        }

        // Emit tag (enum) defines from checker's resolved tag info
        for (name, variants) in &self.checker.tags {
            for (i, variant) in variants.iter().enumerate() {
                let macro_name = format!(
                    "RL_TAG_{}_{}",
                    name.to_uppercase(),
                    variant.to_uppercase()
                );
                self.writer
                    .writeln(&format!("#define {} ((int64_t){})", macro_name, i));
            }
            // Generate string table and print function for this enum
            let count = variants.len();
            self.writer.write(&format!("static const char* _enum_{}_names[] = {{", name));
            for (i, variant) in variants.iter().enumerate() {
                if i > 0 { self.writer.write(", "); }
                self.writer.write(&format!("\"{}\"", variant));
            }
            self.writer.writeln("};");
            self.writer.write(&format!("void rl_print_Enum_{}(int64_t v) {{ ", name));
            self.writer.writeln(&format!("if (v >= 0 && v < (int64_t){}) printf(\"%s.%s\", \"{}\", _enum_{}_names[v]);", count, name, name));
            self.writer.writeln(&format!("else printf(\"{}(%ld)\", (long)v);", name));
            self.writer.writeln("}");
            self.writer.write(&format!("void rl_println_Enum_{}(int64_t v) {{ rl_print_Enum_{}(v); printf(\"\\n\"); }}\n", name, name));
        }
        if !self.checker.tags.is_empty() {
            self.writer.blank_line();
        }

        // Collect tuple types used in the program
        let mut tuple_types: Vec<Vec<TypeAnnotation>> = Vec::new();
        self.collect_tuple_types(statements, &mut tuple_types);
        // Dedup by full field-type layout, not just arity
        let mut unique_tuples: Vec<Vec<TypeAnnotation>> = Vec::new();
        for tt in &tuple_types {
            if !unique_tuples.contains(tt) {
                unique_tuples.push(tt.clone());
            }
        }
        // Assign names: one per arity gets rl_tuple_N, multiple get rl_tuple_N_K
        let mut arity_count: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for fields in &unique_tuples {
            *arity_count.entry(fields.len()).or_insert(0) += 1;
        }
        let mut arity_index: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut new_tuple_names: Vec<(Vec<TypeAnnotation>, String)> = Vec::new();
        for fields in &unique_tuples {
            let arity = fields.len();
            let count = arity_count[&arity];
            let idx = arity_index.entry(arity).or_insert(0);
            let name = if count == 1 {
                format!("rl_tuple_{}", arity)
            } else {
                format!("rl_tuple_{}_{}", arity, *idx)
            };
            *idx += 1;
            new_tuple_names.push((fields.clone(), name));
        }
        self.tuple_names = new_tuple_names;
        for (fields, name) in self.tuple_names.clone() {
            // Buffer into tuple_defs (file scope, ahead of globals):
            // self.writer content lands after the file_scope insert.
            if self
                .tuple_defs
                .contains(&format!("void rl_print_{}(", name))
            {
                continue;
            }
            let mut def = String::new();
            def.push_str("typedef struct { ");
            for (i, field_type) in fields.iter().enumerate() {
                let c_type = type_to_c(field_type);
                def.push_str(&format!("{} field_{}; ", c_type, i));
            }
            def.push_str(&format!("}} {};\n", name));
            // Generate print function for tuple
            def.push_str(&format!("void rl_print_{}({} v) {{ ", name, name));
            def.push_str("printf(\"(\");\n");
            // Render field printers into a side buffer via writer swap.
            let saved = std::mem::take(&mut self.writer);
            for (i, field_type) in fields.iter().enumerate() {
                if i > 0 {
                    self.writer.writeln("printf(\", \");");
                }
                self.emit_field_print(field_type, &format!("v.field_{}", i));
            }
            let rendered = std::mem::replace(&mut self.writer, saved).into_source();
            def.push_str(&rendered);
            def.push_str("printf(\")\");\n");
            def.push_str("}\n");
            def.push_str(&format!("void rl_println_{}({} v) {{ rl_print_{}(v); printf(\"\\n\"); }}\n", name, name, name));
            self.tuple_defs.push_str(&def);
        }
        if !tuple_types.is_empty() {
            self.writer.blank_line();
        }
    }

    fn collect_tuple_types(&self, statements: &[Statement], types: &mut Vec<Vec<TypeAnnotation>>) {
        for stmt in statements {
            match &stmt.kind {
                StatementKind::ResolvedVariableDeclaration {
                    type_annotation, ..
                } => {
                    self.collect_types_from_type(type_annotation, types);
                }
                StatementKind::ResolvedConstantDeclaration {
                    type_annotation, ..
                } => {
                    self.collect_types_from_type(type_annotation, types);
                }
                StatementKind::ResolvedFunctionDeclaration {
                    params,
                    return_type,
                    body,
                    ..
                } => {
                    for param in params {
                        self.collect_types_from_type(&param.param_type, types);
                    }
                    self.collect_types_from_type(return_type, types);
                    self.collect_tuple_types(body, types);
                }
                _ => {}
            }
        }
    }

    pub fn emit_value_wrapping(&mut self, ta: &TypeAnnotation, expr_id: rl_ast::ExprId) -> Result<(), rl_utils::errors::Error> {
        match ta {
            TypeAnnotation::Int | TypeAnnotation::CInt
            | TypeAnnotation::UInt | TypeAnnotation::CUInt
            | TypeAnnotation::SInt | TypeAnnotation::CSInt
            | TypeAnnotation::SUInt | TypeAnnotation::CSUInt
            | TypeAnnotation::Byte | TypeAnnotation::CByte
            | TypeAnnotation::SByte | TypeAnnotation::CSByte
            | TypeAnnotation::BByte | TypeAnnotation::CBByte
            | TypeAnnotation::BSByte | TypeAnnotation::CBSByte => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_I64, .data.i64 = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Float | TypeAnnotation::CFloat
            | TypeAnnotation::SFloat | TypeAnnotation::CSFloat => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_F64, .data.f64 = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Bool | TypeAnnotation::CBool => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_BOOL, .data.boolean = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::String | TypeAnnotation::CString => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_STR, .data.str = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Char | TypeAnnotation::CChar => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_I64, .data.i64 = (int64_t)(unsigned char)");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_ARR, .data.arr = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _) => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_MAP, .data.map = &");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Set(_) | TypeAnnotation::CSet(_) => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_SET, .data.set = &");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Fn | TypeAnnotation::Callback(_, _) => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_CLOSURE, .data.closure = &");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Null => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_NULL, .data.i64 = 0 }");
            }
            _ => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_I64, .data.i64 = (int64_t)");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
        }
        Ok(())
    }

    fn emit_field_print(&mut self, field_type: &TypeAnnotation, accessor: &str) {
        match field_type {
            TypeAnnotation::Int | TypeAnnotation::CInt
            | TypeAnnotation::UInt | TypeAnnotation::CUInt
            | TypeAnnotation::SInt | TypeAnnotation::CSInt
            | TypeAnnotation::SUInt | TypeAnnotation::CSUInt
            | TypeAnnotation::Byte | TypeAnnotation::CByte
            | TypeAnnotation::SByte | TypeAnnotation::CSByte
            | TypeAnnotation::BByte | TypeAnnotation::CBByte
            | TypeAnnotation::BSByte | TypeAnnotation::CBSByte => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%ld\", (long){});", accessor));
            }
            TypeAnnotation::Float | TypeAnnotation::CFloat
            | TypeAnnotation::SFloat | TypeAnnotation::CSFloat => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%g\", (double){});", accessor));
            }
            TypeAnnotation::Bool | TypeAnnotation::CBool => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%s\", {} ? \"true\" : \"false\");", accessor));
            }
            TypeAnnotation::String | TypeAnnotation::CString => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%.*s\", (int){}.len, {}.data);", accessor, accessor));
            }
            TypeAnnotation::Char | TypeAnnotation::CChar => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%c\", (int){});", accessor));
            }
            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_rl_array({});", accessor));
            }
            TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_rl_map({});", accessor));
            }
            TypeAnnotation::Set(_) | TypeAnnotation::CSet(_) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_rl_set({});", accessor));
            }
            TypeAnnotation::Result(_) | TypeAnnotation::CResult(_) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_result({});", accessor));
            }
            TypeAnnotation::Fn | TypeAnnotation::Callback(_, _) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_closure({});", accessor));
            }
            TypeAnnotation::Record(rname) | TypeAnnotation::CRecord(rname) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_rl_Record_{}({});", rname, accessor));
            }
            TypeAnnotation::Tuple(elems) | TypeAnnotation::CTuple(elems) => {
                let tuple_name = self.lookup_tuple_name(elems).to_string();
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_{}({});", tuple_name, accessor));
            }
            TypeAnnotation::Enum(ename) | TypeAnnotation::CEnum(ename) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_Enum_{}({});", ename, accessor));
            }
            _ => {
                self.writer.write_indent();
                self.writer.writeln("printf(\"?\");");
            }
        }
    }

    fn collect_types_from_type(&self, ta: &TypeAnnotation, types: &mut Vec<Vec<TypeAnnotation>>) {
        match ta {
            TypeAnnotation::Tuple(elems) | TypeAnnotation::CTuple(elems) => {
                types.push(elems.as_ref().clone());
                for elem in elems.iter() {
                    self.collect_types_from_type(elem, types);
                }
            }
            TypeAnnotation::Array(inner) | TypeAnnotation::CArray(inner) => {
                self.collect_types_from_type(inner, types);
            }
            TypeAnnotation::Result(inner) | TypeAnnotation::CResult(inner) => {
                self.collect_types_from_type(inner, types);
            }
            _ => {}
        }
    }

    pub fn lookup_tuple_name(&self, field_types: &[TypeAnnotation]) -> &str {
        for (fields, name) in &self.tuple_names {
            if fields == field_types {
                return name;
            }
        }
        "rl_tuple_2"
    }

    /// Ensures the element printer plus an array printer looping over it.
    /// Returns `(print_fn, println_fn)` for `rl_array` values of tuples.
    pub fn ensure_tuple_array_printer(
        &mut self,
        field_types: Vec<TypeAnnotation>,
    ) -> (String, String) {
        let tuple_name = self.ensure_tuple_type(field_types);
        let print_fn = format!("rl_print_{}_arr", tuple_name);
        let println_fn = format!("rl_println_{}_arr", tuple_name);
        if !self.tuple_defs.contains(&format!("void {}(", print_fn)) {
            let mut def = String::new();
            def.push_str(&format!(
                "void {}(rl_array v) {{ printf(\"[\"); for (uint64_t _i = 0; _i < v.len; _i++) {{ if (_i > 0) printf(\", \"); rl_print_{}((({}*)v.data)[_i]); }} printf(\"]\"); }}\n",
                print_fn, tuple_name, tuple_name
            ));
            def.push_str(&format!(
                "void {}(rl_array v) {{ {}(v); printf(\"\\n\"); }}\n",
                println_fn, print_fn
            ));
            self.tuple_defs.push_str(&def);
        }
        (print_fn, println_fn)
    }

    pub fn ensure_tuple_type(&mut self, field_types: Vec<TypeAnnotation>) -> String {
        for (fields, name) in &self.tuple_names {
            if *fields == field_types {
                return name.clone();
            }
        }
        let arity = field_types.len();
        let count = self.tuple_names.iter().filter(|(f, _)| f.len() == arity).count();
        let name = if count == 0 {
            format!("rl_tuple_{}", arity)
        } else {
            format!("rl_tuple_{}_{}", arity, count)
        };
        // Buffered for file scope: emitting here would land mid-expression.
        let mut def = String::new();
        def.push_str("typedef struct { ");
        for (i, field_type) in field_types.iter().enumerate() {
            let c_type = type_to_c(field_type);
            def.push_str(&format!("{} field_{}; ", c_type, i));
        }
        def.push_str(&format!("}} {};\n", name));
        def.push_str(&format!("void rl_print_{}({} v) {{ ", name, name));
        def.push_str("printf(\"(\");\n");
        // Render field printers into a side buffer via writer swap.
        let saved = std::mem::take(&mut self.writer);
        for (i, field_type) in field_types.iter().enumerate() {
            if i > 0 {
                self.writer.writeln("printf(\", \");");
            }
            self.emit_field_print(field_type, &format!("v.field_{}", i));
        }
        let rendered = std::mem::replace(&mut self.writer, saved).into_source();
        def.push_str(&rendered);
        def.push_str("printf(\")\");\n");
        def.push_str("}\n");
        def.push_str(&format!("void rl_println_{}({} v) {{ rl_print_{}(v); printf(\"\\n\"); }}\n", name, name, name));
        self.tuple_defs.push_str(&def);
        self.tuple_names.push((field_types, name.clone()));
        name
    }

}
