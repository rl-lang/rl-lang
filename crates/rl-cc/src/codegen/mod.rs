use crate::writer::CWriter;
use crate::name_mangle::{escape_c_string, mangle};
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

/// One pushed scope: the flat type maps saved alongside it, restored on pop.
type ScopeSnapshot = (
    HashMap<String, TypeAnnotation>,
    HashSet<String>,
    HashMap<String, TypeAnnotation>,
    HashMap<String, TypeAnnotation>,
);

/// One discovered `!#[test]` function: name plus the attribute params
/// (group/register/cases) and the declared parameter count for arity
/// and property checks.
#[derive(Debug, Clone)]
pub struct ScannedTest {
    pub name: String,
    pub params: rl_ast::statements::TestParams,
    pub param_count: usize,
    /// Declared function parameters (types drive `cases(N)` generation).
    pub fn_params: Vec<Param>,
    /// Declared return type (for forward declarations; the effective
    /// type prefers `user_fn_returns` inference like definitions do).
    pub ret: TypeAnnotation,
}

/// Entry scan result: optional entry name, tests, setups, teardowns,
/// inits, finals (each in run order).
#[derive(Debug, Clone, Default)]
pub struct ScannedEntry {
    pub entry: Option<String>,
    pub tests: Vec<ScannedTest>,
    pub setups: Vec<ScannedTest>,
    pub teardowns: Vec<ScannedTest>,
    pub inits: Vec<String>,
    pub finals: Vec<String>,
}

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
    /// `is`-refined variable types active in the current branch body
    /// (`x is int` maps `x` to `Int` until the body ends). Identifier
    /// emission unboxes `rl_value` storage through these; mirrored from
    /// the checker's branch refinement (which gates all programs, so an
    /// entry always reflects a taken test).
    pub refined_vars: HashMap<String, TypeAnnotation>,
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
    /// Declared parameter types of top-level user functions, for boxing
    /// concrete arguments into `any` parameters at call sites.
    pub user_fn_params: HashMap<String, Vec<TypeAnnotation>>,
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
    pub type_snapshots: Vec<ScopeSnapshot>,
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
    /// Test-driver mode (`rlt --test`): emit the test runner main
    /// instead of the program main. Transpiled mains never call tests.
    pub test_mode: bool,
    /// `--match` filter for test mode (substring over name/group/register).
    pub match_pattern: Option<String>,
}

/// Matches `--match` against a test name, group, register, or full name,
/// mirroring the `rl test` runner filter.
fn test_matches(name: &str, params: &rl_ast::statements::TestParams, pattern: &str) -> bool {
    if name.contains(pattern) {
        return true;
    }
    if let Some(group) = params.group.as_deref() {
        let full = format!("{group}::{name}");
        if group.contains(pattern) || full.contains(pattern) {
            return true;
        }
    }
    if let Some(register) = params.register.as_deref()
        && register.contains(pattern)
    {
        return true;
    }
    false
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
            refined_vars: HashMap::new(),
            std_c_imports: HashSet::new(),
            std_imports: Vec::new(),
            closure_factories: HashMap::new(),
            user_fns: HashSet::new(),
            user_fn_returns: HashMap::new(),
            user_fn_params: HashMap::new(),
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
            test_mode: false,
            match_pattern: None,
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
    /// or `main`/`__entry__` fallback), tests, setups, teardowns, inits and
    /// finals. Mirrors the VM's `scan_entry_points`: numbered init/final
    /// priorities run first in ascending order, unnumbered ones last in
    /// declaration order. Returns `None` when there is no entry point.
    fn scan_entry(statements: &[Statement]) -> Result<ScannedEntry, Error> {
        let mut explicit_entry: Option<String> = None;
        let mut main_entry: Option<String> = None;
        let mut tests: Vec<ScannedTest> = Vec::new();
        let mut setups: Vec<ScannedTest> = Vec::new();
        let mut teardowns: Vec<ScannedTest> = Vec::new();
        let mut inits: Vec<(String, Option<u32>, usize)> = Vec::new();
        let mut finals: Vec<(String, Option<u32>, usize)> = Vec::new();

        for (order, stmt) in statements.iter().enumerate() {
            let StatementKind::ResolvedFunctionDeclaration {
                name,
                attribute,
                params,
                return_type,
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
                Some(FunctionAttribute::Test(test_params)) => tests.push(ScannedTest {
                    name: name.clone(),
                    params: test_params.clone(),
                    param_count: params.len(),
                    fn_params: params.clone(),
                    ret: return_type.clone(),
                }),
                Some(FunctionAttribute::Setup) => setups.push(ScannedTest {
                    name: name.clone(),
                    params: rl_ast::statements::TestParams::default(),
                    param_count: params.len(),
                    fn_params: params.clone(),
                    ret: return_type.clone(),
                }),
                Some(FunctionAttribute::Teardown) => teardowns.push(ScannedTest {
                    name: name.clone(),
                    params: rl_ast::statements::TestParams::default(),
                    param_count: params.len(),
                    fn_params: params.clone(),
                    ret: return_type.clone(),
                }),
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

        // Numbered priorities first ascending, unnumbered last in order.
        // `sort_by_key` is stable, so declaration order is preserved.
        inits.sort_by_key(|(_, p, _)| (p.is_none(), p.unwrap_or(0)));
        finals.sort_by_key(|(_, p, _)| (p.is_none(), p.unwrap_or(0)));
        Ok(ScannedEntry {
            entry: explicit_entry.or(main_entry),
            tests,
            setups,
            teardowns,
            inits: inits.into_iter().map(|(n, _, _)| n).collect(),
            finals: finals.into_iter().map(|(n, _, _)| n).collect(),
        })
    }

    /// Emits registry population for `test_run_registered` (all modes):
    /// every zero-arg test/setup/teardown registers its address. Parameterized
    /// functions are skipped (they cannot be invoked without arguments).
    fn emit_test_registrations(&mut self, entry: &ScannedEntry) {
        for test in &entry.tests {
            if test.param_count > 0 {
                continue;
            }
            let group = test.params.group.as_deref().unwrap_or_default();
            let register = test.params.register.as_deref().unwrap_or_default();
            self.writer.write_indent();
            self.writer.write(&format!(
                "rl_test_register(\"case\", \"{}\", \"{}\", \"{}\", {});\n",
                escape_c_string(&test.name),
                escape_c_string(group),
                escape_c_string(register),
                mangle(&test.name)
            ));
        }
        for hook in &entry.setups {
            if hook.param_count > 0 {
                continue;
            }
            self.writer.write_indent();
            self.writer.write(&format!(
                "rl_test_register(\"setup\", \"{}\", \"\", \"\", {});\n",
                escape_c_string(&hook.name),
                mangle(&hook.name)
            ));
        }
        for hook in &entry.teardowns {
            if hook.param_count > 0 {
                continue;
            }
            self.writer.write_indent();
            self.writer.write(&format!(
                "rl_test_register(\"teardown\", \"{}\", \"\", \"\", {});\n",
                escape_c_string(&hook.name),
                mangle(&hook.name)
            ));
        }
    }

    /// C type for a generatable parameter, or `None` when the type is
    /// outside the C property scope (maps, sets, tuples, records, chars,
    /// non-string numerics beyond int/float/bool, functions, handles).
    /// Arrays recurse one level into scalar elements only.
    fn prop_c_type(ty: &TypeAnnotation) -> Option<&'static str> {
        match ty {
            TypeAnnotation::Int | TypeAnnotation::CInt => Some("int64_t"),
            TypeAnnotation::Float | TypeAnnotation::CFloat => Some("double"),
            TypeAnnotation::Bool | TypeAnnotation::CBool => Some("bool"),
            TypeAnnotation::String | TypeAnnotation::CString => Some("rl_string"),
            TypeAnnotation::Array(inner) | TypeAnnotation::CArray(inner) => {
                match inner.as_ref() {
                    TypeAnnotation::Int
                    | TypeAnnotation::CInt
                    | TypeAnnotation::Float
                    | TypeAnnotation::CFloat
                    | TypeAnnotation::Bool
                    | TypeAnnotation::CBool
                    | TypeAnnotation::String
                    | TypeAnnotation::CString => Some("rl_array"),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Why a property test cannot run under `rlt --test`, for the skip
    /// message. `None` means generatable.
    fn prop_unsupported_reason(test: &ScannedTest) -> Option<String> {
        for param in &test.fn_params {
            if Self::prop_c_type(&param.param_type).is_none() {
                return Some(format!(
                    "cannot generate `{}`",
                    match &param.param_type {
                        TypeAnnotation::Array(inner) | TypeAnnotation::CArray(inner) => {
                            format!("array[{:?}]", inner.as_ref())
                        }
                        other => format!("{other:?}"),
                    }
                ));
            }
        }
        None
    }


    /// Emits file-scope support for one property test: the boxed-argument
    /// buffer, the generator, and the zero-arg trampoline (setups, test
    /// call with unboxed args, teardowns). The driver invokes the
    /// trampoline through `rl_test_run_property`, which owns iteration,
    /// shrinking, and reporting.
    fn emit_prop_support(
        &mut self,
        entry: &ScannedEntry,
        test: &ScannedTest,
    ) -> Result<(), Error> {
        let base = mangle(&test.name);
        let nargs = test.fn_params.len();
        // Boxed argument buffer shared by generator, trampoline, and the
        // runtime shrinker (never empty: strict C99 has no zero arrays).
        self.globals_code.push_str(&format!(
            "\nstatic rl_result _tp_args_{base}[{}];\n",
            nargs.max(1)
        ));
        // Forward declaration: the definition hoists later in file
        // order. Mirrors the definition-site effective type
        // (inferred returns over the unresolved `Null` default).
        let effective = match self.user_fn_returns.get(&test.name) {
            Some(rt) if test.ret == TypeAnnotation::Null => rt.clone(),
            _ => test.ret.clone(),
        };
        let proto_params: Vec<String> = test
            .fn_params
            .iter()
            .map(|p| {
                format!("{} {}", type_to_c(&p.param_type), mangle(&p.param_name))
            })
            .collect();
        self.static_funcs.push(format!(
            "{} {}({});\n",
            type_to_c(&effective),
            base,
            proto_params.join(", ")
        ));
        // Generator: one statement block per parameter (later bounds read
        // earlier locals), then boxing into the shared buffer.
        let mut gen_fn = format!("static void prop_gen_{base}(void) {{\n");
        for (i, _param) in test.fn_params.iter().enumerate() {
            gen_fn.push_str(&self.prop_gen_stmts(test, i, &base)?);
        }
        gen_fn.push_str("}\n");
        self.static_funcs.push(gen_fn);
        // Trampoline: setups, unboxed call, teardowns, boxed return.
        let mut tramp = format!("static rl_result prop_invoke_{base}(void) {{\n");
        for hook in &entry.setups {
            if hook.param_count > 0 {
                continue;
            }
            tramp.push_str(&format!("    {}();\n", mangle(&hook.name)));
        }
        let call = format!(
            "{}({})",
            base,
            (0..nargs)
                .map(|i| {
                    let unbox = match &test.fn_params[i].param_type {
                        TypeAnnotation::String | TypeAnnotation::CString => "rl_unwrap_str",
                        TypeAnnotation::Float | TypeAnnotation::CFloat => "rl_unwrap_f64",
                        TypeAnnotation::Bool | TypeAnnotation::CBool => "rl_unwrap_bool",
                        TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => "rl_unwrap_arr",
                        _ => "rl_unwrap_i64",
                    };
                    format!("{unbox}(_tp_args_{base}[{i}])")
                })
                .collect::<Vec<_>>()
                .join(", ")
        );
        match self.user_fn_returns.get(&test.name) {
            None | Some(TypeAnnotation::Null) => {
                tramp.push_str(&format!("    {call};\n    return rl_ok_null();\n"));
            }
            Some(TypeAnnotation::Result(_)) | Some(TypeAnnotation::CResult(_)) => {
                tramp.push_str(&format!("    return {call};\n"));
            }
            Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => {
                tramp.push_str(&format!("    return rl_ok_i64({call});\n"));
            }
            Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => {
                tramp.push_str(&format!("    return rl_ok_f64({call});\n"));
            }
            Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => {
                tramp.push_str(&format!("    return rl_ok_bool({call});\n"));
            }
            Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => {
                tramp.push_str(&format!("    return rl_ok_str({call});\n"));
            }
            Some(_) => {
                // Exotic returns: verdicts come from asserts/aborts only.
                tramp.push_str(&format!("    {call};\n    return rl_ok_null();\n"));
            }
        }
        for hook in &entry.teardowns {
            if hook.param_count > 0 {
                continue;
            }
            tramp.push_str(&format!("    {}();\n", mangle(&hook.name)));
        }
        tramp.push_str("}\n");
        self.static_funcs.push(tramp);
        Ok(())
    }

    /// Boxing expression for a generated local of the given type.
    fn prop_box_expr(&self, ty: &TypeAnnotation, local: &str) -> Result<String, Error> {
        Ok(match ty {
            TypeAnnotation::String | TypeAnnotation::CString => format!("rl_ok_str({local})"),
            TypeAnnotation::Float | TypeAnnotation::CFloat => format!("rl_ok_f64({local})"),
            TypeAnnotation::Bool | TypeAnnotation::CBool => format!("rl_ok_bool({local})"),
            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => format!("rl_ok_arr({local})"),
            _ => format!("rl_ok_i64({local})"),
        })
    }

    /// Generation statements for parameter `i`: declares the unboxed
    /// local `_tp_a{i}`, fills it (narrowed bounds, pinned equality,
    /// retry loops), then boxes it into the shared buffer. Cross-parameter
    /// bounds read earlier locals; later or unknown parameters fall back
    /// to defaults (the runtime guard still enforces correctness).
    fn prop_gen_stmts(&self, test: &ScannedTest, i: usize, base: &str) -> Result<String, Error> {
        use rl_ast::statements::{RefineOp, RefineOperand};
        // Unresolvable equality pins (later/unknown parameters) degrade
        // to plain generation; the runtime guard still enforces them.
        let owned;
        let param = match &test.fn_params[i].refinement {
            Some(r) if r.op == RefineOp::Eq && matches!(&r.operand, RefineOperand::Param(p) if test.fn_params[..i].iter().position(|q| &q.param_name == p).is_none()) => {
                owned = rl_ast::statements::Param {
                    refinement: None,
                    ..test.fn_params[i].clone()
                };
                &owned
            }
            _ => &test.fn_params[i],
        };
        let c_ty = Self::prop_c_type(&param.param_type).expect("checked");
        let mut out = String::new();
        // Equality pins the value outright.
        if let Some(r) = param.refinement.as_ref()
            && r.op == RefineOp::Eq
        {
            let value = match &r.operand {
                RefineOperand::Integer(v) => v.to_string(),
                RefineOperand::Bool(b) => b.to_string(),
                RefineOperand::Str(s) => {
                    // Box string literals directly; no local needed.
                    let lit = format!("rl_str_literal(\"{}\", {})", escape_c_string(s), s.len());
                    out.push_str(&format!("    _tp_args_{base}[{i}] = rl_ok_str({lit});\n"));
                    return Ok(out);
                }
                RefineOperand::Param(p) => match test.fn_params[..i]
                    .iter()
                    .position(|q| &q.param_name == p)
                {
                    Some(j) => format!("_tp_a{j}"),
                    // Unreachable: normalized above.
                    None => unreachable!(),
                },
            };
            out.push_str(&format!("    {c_ty} _tp_a{i} = {value};\n"));
            out.push_str(&format!("    _tp_args_{base}[{i}] = "));
            out.push_str(&self.prop_box_expr(&param.param_type, &format!("_tp_a{i}"))?);
            out.push_str(";\n");
            return Ok(out);
        }
        match &param.param_type {
            TypeAnnotation::String | TypeAnnotation::CString => {
                if let Some(r) = param.refinement.as_ref()
                    && r.op == RefineOp::Ne
                    && let RefineOperand::Str(s) = &r.operand
                {
                    out.push_str(&format!(
                        "    rl_string _tp_a{i} = rl_test_rand_string_ne(\"{e}\", {n});\n",
                        n = s.len(),
                        e = escape_c_string(s),
                    ));
                } else {
                    out.push_str(&format!("    rl_string _tp_a{i} = rl_test_rand_string();\n"));
                }
                out.push_str(&format!("    _tp_args_{base}[{i}] = rl_ok_str(_tp_a{i});\n"));
                return Ok(out);
            }
            TypeAnnotation::Bool | TypeAnnotation::CBool => {
                out.push_str(&format!("    bool _tp_a{i} = (rl_test_rand_below(2) == 0);\n"));
                out.push_str(&format!("    _tp_args_{base}[{i}] = rl_ok_bool(_tp_a{i});\n"));
                return Ok(out);
            }
            TypeAnnotation::Float | TypeAnnotation::CFloat => {
                // Integer bounds apply numerically (the grammar only
                // expresses integer refinement operands).
                let (lo_s, hi_s) = self.prop_int_range(test, i)?;
                out.push_str(&format!(
                    "    double _tp_a{i} = ((double)rl_test_rand_range(({lo_s}) * 100, ({hi_s}) * 100) / 100.0);\n"
                ));
                out.push_str(&format!("    _tp_args_{base}[{i}] = rl_ok_f64(_tp_a{i});\n"));
                return Ok(out);
            }
            TypeAnnotation::Array(inner) | TypeAnnotation::CArray(inner) => {
                let (c_elem, sizeof_elem, tag, elem_gen) = match inner.as_ref() {
                    TypeAnnotation::Int | TypeAnnotation::CInt => (
                        "int64_t", "sizeof(int64_t)", "RL_TAG_I64",
                        "rl_test_rand_range(-100, 100)".to_string(),
                    ),
                    TypeAnnotation::Float | TypeAnnotation::CFloat => (
                        "double", "sizeof(double)", "RL_TAG_F64",
                        "((double)rl_test_rand_range(-10000, 10000) / 100.0)".to_string(),
                    ),
                    TypeAnnotation::Bool | TypeAnnotation::CBool => (
                        "bool", "sizeof(bool)", "RL_TAG_BOOL",
                        "(rl_test_rand_below(2) == 0)".to_string(),
                    ),
                    TypeAnnotation::String | TypeAnnotation::CString => (
                        "rl_string", "sizeof(rl_string)", "RL_TAG_STR",
                        "rl_test_rand_string()".to_string(),
                    ),
                    _ => unreachable!(),
                };
                out.push_str(&format!("    {c_elem} _tp_e{i}[3];\n"));
                out.push_str(&format!("    int _tp_n{i} = (int)rl_test_rand_below(4);\n"));
                out.push_str(&format!(
                    "    for (int _tp_k{i} = 0; _tp_k{i} < _tp_n{i}; _tp_k{i}++) _tp_e{i}[_tp_k{i}] = {elem_gen};\n"
                ));
                out.push_str(&format!(
                    "    rl_array _tp_a{i} = rl_arr_from_vals_tag(_tp_e{i}, (uint64_t)_tp_n{i}, (int32_t){sizeof_elem}, {tag});\n"
                ));
                out.push_str(&format!("    _tp_args_{base}[{i}] = rl_ok_arr(_tp_a{i});\n"));
                return Ok(out);
            }
            _ => {}
        }
        // Integer family (default -100..100, narrowed below).
        let mut ne_lit: Option<String> = None;
        if let Some(r) = param.refinement.as_ref()
            && r.op == RefineOp::Ne
            && let RefineOperand::Integer(v) = &r.operand
        {
            ne_lit = Some(v.to_string());
        }
        let (lo_s, hi_s) = self.prop_int_range(test, i)?;
        if let Some(lit) = ne_lit {
            out.push_str(&format!(
                "    int64_t _tp_a{i} = rl_test_rand_range_ne({lo_s}, {hi_s}, {lit});\n"
            ));
        } else {
            out.push_str(&format!("    int64_t _tp_a{i} = rl_test_rand_range({lo_s}, {hi_s});\n"));
        }
        out.push_str(&format!("    _tp_args_{base}[{i}] = rl_ok_i64(_tp_a{i});\n"));
        Ok(out)
    }

    /// Integer bound range as C expressions `(lo, hi)`, narrowed by the
    /// parameter's refinement. Literals embed directly; earlier
    /// parameters read their locals; later or unknown parameters keep
    /// defaults (the runtime guard still enforces correctness).
    /// Contradictory static bounds are a transpile error.
    fn prop_int_range(&self, test: &ScannedTest, i: usize) -> Result<(String, String), Error> {
        use rl_ast::statements::{RefineOp, RefineOperand};
        let param = &test.fn_params[i];
        let (mut lo, mut hi) = ("-100".to_string(), "100".to_string());
        let mut lo_num: Option<i64> = Some(-100);
        let mut hi_num: Option<i64> = Some(100);
        if let Some(r) = param.refinement.as_ref()
            && r.op != RefineOp::Eq
            && r.op != RefineOp::Ne
        {
            // Static narrowing only when both sides are literals (so
            // contradictions are provable); cross-parameter bounds emit
            // locals and skip the static check.
            let bound_num = match &r.operand {
                RefineOperand::Integer(v) => Some(*v),
                _ => None,
            };
            let bound_s = match &r.operand {
                RefineOperand::Integer(v) => v.to_string(),
                RefineOperand::Param(p) => match test.fn_params[..i]
                    .iter()
                    .position(|q| &q.param_name == p)
                {
                    Some(j) => format!("(int64_t)_tp_a{j}"),
                    None => return Ok((lo, hi)),
                },
                _ => return Ok((lo, hi)),
            };
            match r.op {
                RefineOp::Gt => {
                    if let Some(v) = bound_num {
                        lo_num = Some(v.saturating_add(1));
                        lo = lo_num.unwrap().to_string();
                    } else {
                        lo = format!("{bound_s}+1");
                    }
                }
                RefineOp::Ge => {
                    lo = bound_s.clone();
                    if let Some(v) = bound_num {
                        lo_num = Some(v);
                        lo = v.to_string();
                    }
                }
                RefineOp::Lt => {
                    if let Some(v) = bound_num {
                        hi_num = Some(v.saturating_sub(1));
                        hi = hi_num.unwrap().to_string();
                    } else {
                        hi = format!("{bound_s}-1");
                    }
                }
                RefineOp::Le => {
                    hi = bound_s.clone();
                    if let Some(v) = bound_num {
                        hi_num = Some(v);
                        hi = v.to_string();
                    }
                }
                RefineOp::Eq | RefineOp::Ne => unreachable!(),
            }
            if let (Some(lo_n), Some(hi_n)) = (lo_num, hi_num)
                && lo_n > hi_n
            {
                return Err(Error::at(
                    Reason::Compile,
                    format!(
                        "property test `{}`: contradictory bounds for `{}`",
                        test.name, param.param_name
                    ),
                    Span::dummy(),
                ));
            }
        }
        Ok((lo, hi))
    }
    /// Emits the `rlt --test` driver main: setup statements, inits, then
    /// one setjmp-guarded block per test (setups, test, teardowns),
    /// finals, a summary, and a non-zero exit on failure. Property cases
    /// (`cases(N)`) report a runtime skip (generation lives in `rl test`);
    /// tests with parameters but no `cases` are a transpile error.
    fn emit_test_main(&mut self, statements: &[Statement], entry: &ScannedEntry) -> Result<(), Error> {
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
        // Test registry for `test_run_registered`.
        self.emit_test_registrations(entry);
        for stmt in statements {
            if Self::is_entry_setup(&stmt.kind) {
                self.compile_top_level(stmt)?;
            }
        }
        for name in &entry.inits {
            self.writer.write_indent();
            self.writer.write(&format!("{}();\n", mangle(name)));
        }
        self.writer.writeln("uint64_t t_run = 0, t_ok = 0, t_fail = 0, t_skip = 0;");
        for test in &entry.tests {
            if let Some(pat) = self.match_pattern.as_deref()
                && !test_matches(&test.name, &test.params, pat)
            {
                continue;
            }
            let c_name = mangle(&test.name);
            // Property cases generate inputs in C: emit support code
            // once, then drive iterations through the runtime engine.
            // Unsupported parameter types report a runtime skip.
            if let Some(n) = test.params.cases {
                if let Some(reason) = Self::prop_unsupported_reason(test) {
                    self.writer.write_indent();
                    self.writer.write(&format!(
                        "t_run++; t_skip++; printf(\"SKIP {} ({})\\n\");\n",
                        test.name, reason
                    ));
                    continue;
                }
                self.emit_prop_support(entry, test)?;
                let base = mangle(&test.name);
                let nargs = test.fn_params.len();
                self.writer.write_indent();
                self.writer.write(&format!("{{ /* property {} */\n", test.name));
                self.writer.indent();
                self.writer.writeln("t_run++;");
                self.writer.writeln("uint64_t p0 = rl_test_state.passed, f0 = rl_test_state.failed;");
                self.writer.writeln("size_t s0 = rl_test_state.skipped_len;");
                self.writer.write(&format!("rl_test_state.current = \"{}\";\n", test.name));
                self.writer.write(&format!(
                    "rl_test_run_property(\"{}\", prop_gen_{base}, prop_invoke_{base}, _tp_args_{base}, {nargs}, {n}ULL);\n",
                    test.name
                ));
                self.writer.writeln("rl_test_state.current = NULL;");
                self.writer.writeln("if (rl_test_state.skipped_len > s0) {");
                self.writer.indent();
                self.writer.writeln("t_skip++;");
                self.writer.writeln("for (size_t i = s0; i < rl_test_state.skipped_len; i++) printf(\"SKIP %s\\n\", rl_test_state.skipped[i]);");
                self.writer.dedent();
                self.writer.writeln("} else if (rl_test_state.failed > f0) {");
                self.writer.indent();
                self.writer.writeln("t_fail++;");
                self.writer.write(&format!("printf(\"FAIL {}\\n\");\n", test.name));
                self.writer.dedent();
                self.writer.writeln("} else {");
                self.writer.indent();
                self.writer.writeln("t_ok++;");
                self.writer.write(&format!("printf(\"ok {}\\n\");\n", test.name));
                self.writer.dedent();
                self.writer.writeln("}");
                self.writer.writeln("(void)p0;");
                self.writer.dedent();
                self.writer.writeln("}");
                continue;
            }
            if test.param_count > 0 {
                return Err(Error::at(
                    Reason::Compile,
                    format!(
                        "test `{}` takes parameters: add cases(N) and run under `rl test`, or use zero arguments for `rlt --test`",
                        test.name
                    ),
                    Span::dummy(),
                ));
            }
            self.writer.write_indent();
            self.writer.write(&format!("{{ /* test {} */\n", test.name));
            self.writer.indent();
            self.writer.writeln("t_run++;");
            self.writer.writeln("uint64_t p0 = rl_test_state.passed, f0 = rl_test_state.failed;");
            self.writer.writeln("size_t s0 = rl_test_state.skipped_len;");
            self.writer.write(&format!("rl_test_state.current = \"{}\";\n", test.name));
            self.writer.writeln("int code = setjmp(rl_abort_frames[rl_abort_depth++]);");
            self.writer.writeln("if (code == 0) {");
            self.writer.indent();
            for hook in &entry.setups {
                self.writer.write_indent();
                self.writer.write(&format!("{}();\n", mangle(&hook.name)));
            }
            self.writer.write_indent();
            self.writer.write(&format!("{c_name}();\n"));
            for hook in &entry.teardowns {
                self.writer.write_indent();
                self.writer.write(&format!("{}();\n", mangle(&hook.name)));
            }
            self.writer.writeln("rl_abort_depth--;");
            self.writer.dedent();
            self.writer.writeln("}");
            self.writer.writeln("rl_test_state.current = NULL;");
            self.writer.writeln("if (rl_test_state.skipped_len > s0) {");
            self.writer.indent();
            self.writer.writeln("t_skip++;");
            self.writer.writeln("for (size_t i = s0; i < rl_test_state.skipped_len; i++) printf(\"SKIP %s\\n\", rl_test_state.skipped[i]);");
            self.writer.dedent();
            self.writer.writeln("} else if (rl_test_state.failed > f0) {");
            self.writer.indent();
            self.writer.writeln("t_fail++;");
            self.writer.write(&format!("printf(\"FAIL {}\\n\");\n", test.name));
            self.writer.dedent();
            self.writer.writeln("} else {");
            self.writer.indent();
            self.writer.writeln("t_ok++;");
            self.writer.write(&format!("printf(\"ok {}\\n\");\n", test.name));
            self.writer.dedent();
            self.writer.writeln("}");
            // Silence unused-variable warnings when a test has no asserts.
            self.writer.writeln("(void)p0;");
            self.writer.dedent();
            self.writer.writeln("}");
        }
        for name in &entry.finals {
            self.writer.write_indent();
            self.writer.write(&format!("{}();\n", mangle(name)));
        }
        self.writer.writeln("printf(\"ran %llu tests: %llu ok, %llu failed, %llu skipped (%llu assertions passed, %llu failed)\\n\", (unsigned long long)t_run, (unsigned long long)t_ok, (unsigned long long)t_fail, (unsigned long long)t_skip, (unsigned long long)rl_test_state.passed, (unsigned long long)rl_test_state.failed);");
        self.writer.writeln("for (size_t i = 0; i < rl_test_state.failures_len; i++) printf(\"%s\\n\", rl_test_state.failures[i]);");
        self.writer.writeln("return t_fail ? 1 : 0;");
        self.writer.dedent();
        self.writer.writeln("}");
        Ok(())
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
                name,
                return_type,
                params,
                ..
            } = &stmt.kind
            {
                self.user_fns.insert(name.clone());
                self.user_fn_params.insert(
                    name.clone(),
                    params.iter().map(|p| p.param_type.clone()).collect(),
                );
                let mut resolved_return = return_type.clone();
                if resolved_return == TypeAnnotation::Null
                    && let Some(inferred) = self
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

        // Test-driver mode (`rlt --test`): inits, per-test setup/test/
        // teardown drivers, finals. Transpiled mains never call tests.
        // Falls through to the shared file-scope tail below.
        if self.test_mode {
            self.emit_test_main(statements, &entry)?;
        } else if let Some(entry_name) = entry.entry.clone() {
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
            // Test registry for `test_run_registered`.
            self.emit_test_registrations(&entry);
            for stmt in statements {
                if Self::is_entry_setup(&stmt.kind) {
                    self.compile_top_level(stmt)?;
                }
            }
            for name in &entry.inits {
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
            for name in &entry.finals {
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
            // Test registry for `test_run_registered`.
            self.emit_test_registrations(&entry);

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

    /// Box a value into `any` (`rl_value`) storage. Returns `Ok(true)`
    /// when emitted, `Ok(false)` when the storage isn't dynamic (the
    /// caller emits normally). Members with no dynamic box (bytes,
    /// chars, records, tags, tuples, closures, results) fail loudly
    /// instead of generating mistyped C. Shared by declarations and
    /// assignments into union storage.
    pub fn box_for_any_storage(
        &mut self,
        effective: &TypeAnnotation,
        value: rl_ast::ExprId,
    ) -> Result<bool, Error> {
        use rl_ast::nodes::ExpressionKind;
        if !matches!(
            effective,
            TypeAnnotation::Any(_) | TypeAnnotation::CAny(_)
        ) {
            return Ok(false);
        }
        let expr = self.ast.exprs.get(value);
        if matches!(&expr.kind, ExpressionKind::Null) {
            self.writer.write("rl_value_null()");
            return Ok(true);
        }
        // identifier reads out of dynamic storage (union params and
        // variables) are already boxes: assign directly. inference
        // reports None for these, which must not be mistaken for a
        // concrete value needing a box. refined reads are excluded:
        // they unbox on emission, so the value needs re-boxing below.
        if let ExpressionKind::Identifier(n) | ExpressionKind::ResolvedIdentifier { name: n, .. } =
            &expr.kind
            && !self.refined_vars.contains_key(n) && matches!(
                self.var_types.get(n),
                Some(
                    TypeAnnotation::Any(_)
                        | TypeAnnotation::CAny(_)
                        | TypeAnnotation::Infer
                        | TypeAnnotation::Generic(_)
                )
            ) {
                return Ok(false);
            }
        // name the value for loud errors below (struct literals infer
        // to None, which would otherwise report as `None`)
        let value_desc = match &expr.kind {
            ExpressionKind::StructLiteral { name, .. } => format!("record {}", name),
            _ => format!(
                "{:?}",
                self.inferred_expr_type(value).unwrap_or(TypeAnnotation::Infer)
            ),
        };
        match self.inferred_expr_type(value) {
            Some(
                TypeAnnotation::Int
                | TypeAnnotation::CInt
                | TypeAnnotation::UInt
                | TypeAnnotation::CUInt
                | TypeAnnotation::SInt
                | TypeAnnotation::CSInt
                | TypeAnnotation::SUInt
                | TypeAnnotation::CSUInt
                | TypeAnnotation::Float
                | TypeAnnotation::CFloat
                | TypeAnnotation::SFloat
                | TypeAnnotation::CSFloat
                | TypeAnnotation::Bool
                | TypeAnnotation::CBool
                | TypeAnnotation::String
                | TypeAnnotation::CString
                | TypeAnnotation::Array(_)
                | TypeAnnotation::CArray(_)
                | TypeAnnotation::Map(_, _)
                | TypeAnnotation::CMap(_, _)
                | TypeAnnotation::Set(_)
                | TypeAnnotation::CSet(_)
                | TypeAnnotation::Handle(_)
                | TypeAnnotation::HandleInfer
                | TypeAnnotation::Enum(_)
                | TypeAnnotation::CEnum(_),
            ) => {
                self.writer.write("rl_box(");
                self.compile_expr(value)?;
                self.writer.write(")");
                Ok(true)
            }
            // already boxed/dynamic: assign directly
            Some(
                TypeAnnotation::Infer
                | TypeAnnotation::Generic(_)
                | TypeAnnotation::Any(_)
                | TypeAnnotation::CAny(_),
            ) => Ok(false),
            _ => Err(Error::at(
                Reason::Compile,
                format!(
                    "any[...] value cannot be stored on the C backend yet: {}",
                    value_desc
                ),
                Span::dummy(),
            )),
        }
    }

    /// Records an `is`-refinement for a branch body, returning the
    /// previous entry for restore. Mirrors the checker's branch
    /// refinement (which gates all programs, so an entry always
    /// reflects a taken test).
    pub fn refine_var(
        &mut self,
        name: String,
        refined: TypeAnnotation,
    ) -> Option<TypeAnnotation> {
        self.refined_vars.insert(name, refined)
    }

    /// Restores a refinement saved by [`CCodegen::refine_var`].
    pub fn unrefine_var(&mut self, name: &str, prev: Option<TypeAnnotation>) {
        match prev {
            Some(t) => {
                self.refined_vars.insert(name.to_string(), t);
            }
            None => {
                self.refined_vars.remove(name);
            }
        }
    }

    /// C unboxer for a dynamic (`rl_value`) source flowing into
    /// `target` storage, or None when no conversion applies. Numerics
    /// convert (mirroring the VM's `as`); everything else asserts the
    /// exact tag. Shared by casts and `return` positions.
    pub fn dynamic_unboxer(target: &TypeAnnotation) -> Option<&'static str> {
        use TypeAnnotation as T;
        match target {
            T::Int | T::CInt | T::UInt | T::CUInt | T::SInt | T::CSInt | T::SUInt | T::CSUInt => {
                Some("rl_unbox_num_i64")
            }
            T::Float | T::CFloat | T::SFloat | T::CSFloat => Some("rl_unbox_num_f64"),
            T::Bool | T::CBool => Some("rl_unbox_bool"),
            T::String | T::CString => Some("rl_unbox_str"),
            T::Array(_) | T::CArray(_) => Some("rl_unbox_arr"),
            T::Map(_, _) | T::CMap(_, _) => Some("rl_unbox_map"),
            T::Set(_) | T::CSet(_) => Some("rl_unbox_set"),
            T::Handle(_) | T::HandleInfer | T::Enum(_) | T::CEnum(_) => {
                Some("rl_unbox_i64")
            }
            _ => None,
        }
    }

    /// Detects `name is Type` conditions for branch refinement, plus
    /// `!(name is Type)` when the operand is a union with exactly one
    /// member left after removing the tested type. Mirrors the
    /// checker's branch refinement (which gates all programs).
    /// A `!(name is Type)` test refines when the operand is a union
    /// with exactly one member left after removing the tested type.
    pub fn detect_is_refinement(&self, cond: rl_ast::ExprId) -> Option<(String, TypeAnnotation)> {
        use rl_ast::nodes::ExpressionKind;
        let expr = self.ast.exprs.get(cond);
        match &expr.kind {
            ExpressionKind::Is { value, target_type } => {
                let v = self.ast.exprs.get(*value);
                match &v.kind {
                    ExpressionKind::Identifier(n) | ExpressionKind::ResolvedIdentifier { name: n, .. } => {
                        Some((n.to_string(), target_type.clone()))
                    }
                    _ => None,
                }
            }
            ExpressionKind::Unary { operator, operand } => {
                use rl_lexer::tokentypes::TokenType;
                if *operator != TokenType::Bang {
                    return None;
                }
                // peel one grouping layer: `!(x is T)`
                let inner = self.ast.exprs.get(*operand);
                let (value, target_type) = match &inner.kind {
                    ExpressionKind::Is { value, target_type } => (*value, target_type.clone()),
                    ExpressionKind::Grouping(inner) => {
                        let g = self.ast.exprs.get(*inner);
                        match &g.kind {
                            ExpressionKind::Is { value, target_type } => {
                                (*value, target_type.clone())
                            }
                            _ => return None,
                        }
                    }
                    _ => return None,
                };
                let v = self.ast.exprs.get(value);
                let name = match &v.kind {
                    ExpressionKind::Identifier(n) | ExpressionKind::ResolvedIdentifier { name: n, .. } => {
                        n.to_string()
                    }
                    _ => return None,
                };
                // negate: the remainder must be exactly one member
                let operand_ty = self.inferred_expr_type(value);
                match operand_ty {
                    Some(TypeAnnotation::Any(members) | TypeAnnotation::CAny(members)) => {
                        let mut rest = members.iter().filter(|m| *m != &target_type);
                        match (rest.next(), rest.next()) {
                            (Some(only), None) => Some((name, only.clone())),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            ExpressionKind::Grouping(inner) => self.detect_is_refinement(*inner),
            _ => None,
        }
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
