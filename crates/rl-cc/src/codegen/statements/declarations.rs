use super::propagate::{emit_propagate_assign, emit_propagate_guard};
use super::result_field_access;
use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::TypeAnnotation;
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// File-scope storage for one global plus scope registration. The
/// initializer runs later inside `main` (see the `in_global_init` paths
/// below). `const` is dropped: C file-scope initializers must be constant.
fn declare_global(
    cc: &mut CCodegen,
    name: &str,
    effective: &TypeAnnotation,
) {
    if cc.global_names.contains(name) {
        return;
    }
    let c_type = type_to_c(effective);
    let c_name = mangle(name);
    cc.globals_code.push_str(&format!("{} {};\n", c_type, c_name));
    cc.declare(name, &c_name);
    cc.var_types.insert(name.to_string(), effective.clone());
    cc.global_names.insert(name.to_string());
}

/// Track closure/null metadata shared by the declare and init paths.
fn track_initializer(cc: &mut CCodegen, name: &str, value: ExprId) {
    let expr = cc.ast.exprs.get(value);
    // Track closure return types for unwrapping at call sites
    if let ExpressionKind::ResolvedLambda { return_type, .. } = &expr.kind
        && let Some(rt) = return_type {
            cc.closure_return_types.insert(name.to_string(), rt.clone());
        }
    if let ExpressionKind::Null = &expr.kind {
        // Nullable vars stored as rl_result to preserve null tag
        cc.nullable_vars.insert(name.to_string());
    }
    // Factory calls (`dec add7 = mk(7)` where mk returns a lambda):
    // record the inner lambda's return type so calls through the new
    // binding unwrap correctly.
    if let ExpressionKind::CallExpr { callee, .. } = &expr.kind {
        let callee_expr = cc.ast.exprs.get(*callee);
        if let ExpressionKind::ResolvedIdentifier { name: callee_name, .. } =
            &callee_expr.kind
            && let Some(ret) = cc.closure_factories.get(callee_name).cloned()
        {
            cc.closure_return_types.insert(name.to_string(), ret);
        }
    }
    // Factory definitions (`dec mk = fn(k) { return fn(x)->int... }`):
    // a returned lambda literal gives the factory's output type, via
    // explicit `return` or a trailing expression.
    if let ExpressionKind::ResolvedLambda { body, .. } = &expr.kind
        && let Some(last) = body.last()
    {
        let yielded = match &last.kind {
            rl_ast::statements::StatementKind::Return(Some(ret_id)) => Some(*ret_id),
            rl_ast::statements::StatementKind::Expression(expr_id) => Some(*expr_id),
            _ => None,
        };
        if let Some(yield_id) = yielded {
            let ret_expr = cc.ast.exprs.get(yield_id);
            if let ExpressionKind::ResolvedLambda { return_type, .. } = &ret_expr.kind
                && let Some(rt) = return_type
            {
                cc.closure_factories.insert(name.to_string(), rt.clone());
            }
        }
    }
}

/// Declare a top-level variable at file scope (pre-pass; no code emitted
/// to the current writer).
pub(crate) fn declare_global_var(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
) {
    let effective = cc.effective_decl_type(type_annotation, value);
    declare_global(cc, name, &effective);
    track_initializer(cc, name, value);
}

/// Declare a top-level constant at file scope (pre-pass).
pub(crate) fn declare_global_const(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
) {
    declare_global_var(cc, name, type_annotation, value);
}

/// Declare destructured top-level bindings at file scope (pre-pass).
pub(crate) fn declare_global_destructure(
    cc: &mut CCodegen,
    bindings: &[(TypeAnnotation, String)],
) {
    for (type_annotation, name) in bindings {
        declare_global(cc, name, type_annotation);
    }
}

/// `x: T = value` — declares a mutable C local and tracks its RL type.
/// A `null` initializer is stored as `rl_result` to preserve the null tag;
/// a `?expr` initializer emits the early-return guard before unwrapping.
/// During global init the storage already exists: only assign.
/// Registers a declared name after its initializer compiled, so the
/// initializer still sees an outer binding of the same name
/// (`dec int c = c.result_unwrap()`). Lambdas register first to allow
/// recursion (`dec f = fn... f() ...`).
fn register_after(
    cc: &mut CCodegen,
    name: &str,
    c_name: &str,
    effective: TypeAnnotation,
    value: ExprId,
    early: bool,
) {
    if cc.in_global_init || !early {
        return;
    }
    cc.declare(name, c_name);
    cc.var_types.insert(name.to_string(), effective);
    track_initializer(cc, name, value);
}

/// Computes the emission name for a declaration: peeked (unregistered) so
/// a shadowed initializer still resolves to the outer variable.
fn emission_name(cc: &mut CCodegen, name: &str) -> String {
    if cc.in_global_init {
        mangle(name)
    } else {
        cc.peek_unique_name(name)
    }
}

pub(super) fn compile_var_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
) -> Result<(), Error> {
    let effective = cc.effective_decl_type(type_annotation, value);
    let c_type = type_to_c(&effective);
    let c_name = emission_name(cc, name);
    let expr = cc.ast.exprs.get(value);
    let early = matches!(&expr.kind, ExpressionKind::ResolvedLambda { .. });
    register_after(cc, name, &c_name, effective.clone(), value, early);
    if let ExpressionKind::Null = &expr.kind {
        cc.writer.write_indent();
        if cc.in_global_init {
            cc.writer.write(&format!("{} = rl_ok_null();\n", c_name));
        } else {
            cc.writer
                .write(&format!("rl_result {} = rl_ok_null();\n", c_name));
        }
    } else if let ExpressionKind::Propagate(inner) = &expr.kind {
        let temp = emit_propagate_assign(cc, *inner)?;
        emit_propagate_guard(cc, &temp, true)?;
        cc.writer.write_indent();
        if cc.in_global_init {
            cc.writer.write(&format!(
                "{} = {};\n",
                c_name,
                result_field_access(&c_type, &temp)
            ));
        } else {
            cc.writer.write(&format!(
                "{} {} = {};\n",
                c_type,
                c_name,
                result_field_access(&c_type, &temp)
            ));
        }
    } else {
        // Closure storage always unboxes through a result round-trip:
        // bare closures box and back, call results pass through and
        // unbox, so `dec fn x = ...` works for literals, variables and
        // calls alike.
        let unbox = matches!(
            effective,
            TypeAnnotation::Fn | TypeAnnotation::Callback(_, _)
        );
        // Concrete scalar storage over a dynamically-typed closure call
        // (`dec int z = add7(1)`): unwrap by the storage type.
        let dyn_unwrap = if !unbox && cc.is_dynamic_closure_call(value) {
            match &effective {
                TypeAnnotation::Float | TypeAnnotation::CFloat => {
                    Some("rl_result_unwrap_f64")
                }
                TypeAnnotation::Bool | TypeAnnotation::CBool => {
                    Some("rl_result_unwrap_bool")
                }
                TypeAnnotation::String | TypeAnnotation::CString => {
                    Some("rl_result_unwrap_str")
                }
                TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                    Some("rl_result_unwrap_arr")
                }
                TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _) => {
                    Some("rl_result_unwrap_map")
                }
                TypeAnnotation::Set(_) | TypeAnnotation::CSet(_) => {
                    Some("rl_result_unwrap_set")
                }
                _ if CCodegen::needs_inference(&effective) => None,
                TypeAnnotation::Result(_)
                | TypeAnnotation::CResult(_)
                | TypeAnnotation::Error
                | TypeAnnotation::CError => None,
                _ => Some("rl_result_unwrap_i64"),
            }
        } else {
            None
        };
        cc.writer.write_indent();
        if cc.in_global_init {
            cc.writer.write(&format!("{} = ", c_name));
        } else {
            cc.writer.write(&format!("{} {} = ", c_type, c_name));
        }
        if unbox {
            cc.writer.write("rl_result_unwrap_closure(rl_ok(");
            cc.compile_expr(value)?;
            cc.writer.write("))");
        } else if let Some(unwrap_fn) = dyn_unwrap {
            cc.writer.write(&format!("{unwrap_fn}("));
            cc.compile_expr(value)?;
            cc.writer.write(")");
        } else {
            cc.compile_expr(value)?;
        }
        cc.writer.write(";\n");
    }
    register_after(cc, name, &c_name, effective, value, !early);
    Ok(())
}

/// `const x: T = value` — like a variable declaration but the C local
/// is `const`. `?expr` initializers get the same early-return guard.
/// Global consts lose `const` at file scope (see `declare_global`).
pub(super) fn compile_const_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
) -> Result<(), Error> {
    let effective = cc.effective_decl_type(type_annotation, value);
    let c_type = type_to_c(&effective);
    let c_name = emission_name(cc, name);
    let expr = cc.ast.exprs.get(value);
    let early = matches!(&expr.kind, ExpressionKind::ResolvedLambda { .. });
    if !cc.in_global_init && early {
        cc.declare(name, &c_name);
    }
    if let ExpressionKind::Propagate(inner) = &expr.kind {
        let temp = emit_propagate_assign(cc, *inner)?;
        emit_propagate_guard(cc, &temp, true)?;
        cc.writer.write_indent();
        if cc.in_global_init {
            cc.writer.write(&format!(
                "{} = {};\n",
                c_name,
                result_field_access(&c_type, &temp)
            ));
        } else {
            cc.writer.write(&format!(
                "const {} {} = {};\n",
                c_type,
                c_name,
                result_field_access(&c_type, &temp)
            ));
        }
    } else {
        let unbox = matches!(
            effective,
            TypeAnnotation::Fn | TypeAnnotation::Callback(_, _)
        );
        cc.writer.write_indent();
        if cc.in_global_init {
            cc.writer.write(&format!("{} = ", c_name));
        } else {
            cc.writer
                .write(&format!("const {} {} = ", c_type, c_name));
        }
        if unbox {
            cc.writer.write("rl_result_unwrap_closure(rl_ok(");
            cc.compile_expr(value)?;
            cc.writer.write("))");
        } else {
            cc.compile_expr(value)?;
        }
        cc.writer.write(";\n");
    }
    if !cc.in_global_init && !early {
        cc.declare(name, &c_name);
    }
    Ok(())
}

/// `(a, b) = tuple_expr` — evaluates the tuple once into a temp, then
/// binds each `.field_N` to a fresh C local. During global init the
/// temp is main-local and each global is assigned.
pub(super) fn compile_destructure(
    cc: &mut CCodegen,
    bindings: &[(TypeAnnotation, String)],
    value: ExprId,
) -> Result<(), Error> {
    let temp = cc.temp_var();
    let field_types: Vec<TypeAnnotation> =
        bindings.iter().map(|(ta, _)| ta.clone()).collect();
    let tuple_name = cc.lookup_tuple_name(&field_types).to_string();
    cc.writer.write_indent();
    cc.writer.write(&format!("{} {} = ", tuple_name, temp));
    cc.compile_expr(value)?;
    cc.writer.write(";\n");
    for (i, (type_annotation, name)) in bindings.iter().enumerate() {
        let c_type = type_to_c(type_annotation);
        let c_name = emission_name(cc, name);
        if !cc.in_global_init {
            cc.declare(name, &c_name);
            cc.var_types
                .insert(name.clone(), type_annotation.clone());
        }
        cc.writer.write_indent();
        if cc.in_global_init {
            cc.writer.write(&format!(
                "{} = {}.field_{};\n",
                c_name, temp, i
            ));
        } else {
            cc.writer.write(&format!(
                "{} {} = {}.field_{};\n",
                c_type, c_name, temp, i
            ));
        }
    }
    Ok(())
}
