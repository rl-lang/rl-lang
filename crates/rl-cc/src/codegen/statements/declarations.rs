use super::propagate::{emit_propagate_assign, emit_propagate_guard};
use super::result_field_access;
use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::TypeAnnotation;
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// `x: T = value` — declares a mutable C local and tracks its RL type.
/// A `null` initializer is stored as `rl_result` to preserve the null tag;
/// a `?expr` initializer emits the early-return guard before unwrapping.
pub(super) fn compile_var_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
) -> Result<(), Error> {
    let c_type = type_to_c(type_annotation);
    let c_name = mangle(name);
    cc.declare(name, &c_name);
    cc.var_types
        .insert(name.to_string(), type_annotation.clone());
    let expr = cc.ast.exprs.get(value);
    // Track closure return types for unwrapping at call sites
    if let ExpressionKind::ResolvedLambda { return_type, .. } = &expr.kind
        && let Some(rt) = return_type {
            cc.closure_return_types.insert(name.to_string(), rt.clone());
        }
    if let ExpressionKind::Null = &expr.kind {
        // Nullable vars stored as rl_result to preserve null tag
        cc.nullable_vars.insert(name.to_string());
        cc.writer.write_indent();
        cc.writer
            .write(&format!("rl_result {} = rl_ok_null();\n", c_name));
    } else if let ExpressionKind::Propagate(inner) = &expr.kind {
        let temp = emit_propagate_assign(cc, *inner)?;
        emit_propagate_guard(cc, &temp, true)?;
        cc.writer.write_indent();
        cc.writer.write(&format!(
            "{} {} = {};\n",
            c_type,
            c_name,
            result_field_access(&c_type, &temp)
        ));
    } else {
        cc.writer.write_indent();
        cc.writer.write(&format!("{} {} = ", c_type, c_name));
        cc.compile_expr(value)?;
        cc.writer.write(";\n");
    }
    Ok(())
}

/// `const x: T = value` — like a variable declaration but the C local
/// is `const`. `?expr` initializers get the same early-return guard.
pub(super) fn compile_const_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
) -> Result<(), Error> {
    let c_type = type_to_c(type_annotation);
    let c_name = mangle(name);
    cc.declare(name, &c_name);
    let expr = cc.ast.exprs.get(value);
    if let ExpressionKind::Propagate(inner) = &expr.kind {
        let temp = emit_propagate_assign(cc, *inner)?;
        emit_propagate_guard(cc, &temp, true)?;
        cc.writer.write_indent();
        cc.writer.write(&format!(
            "const {} {} = {};\n",
            c_type,
            c_name,
            result_field_access(&c_type, &temp)
        ));
    } else {
        cc.writer.write_indent();
        cc.writer
            .write(&format!("const {} {} = ", c_type, c_name));
        cc.compile_expr(value)?;
        cc.writer.write(";\n");
    }
    Ok(())
}

/// `(a, b) = tuple_expr` — evaluates the tuple once into a temp, then
/// binds each `.field_N` to a fresh C local.
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
        let c_name = mangle(name);
        cc.declare(name, &c_name);
        cc.var_types
            .insert(name.clone(), type_annotation.clone());
        cc.writer.write_indent();
        cc.writer.write(&format!(
            "{} {} = {}.field_{};\n",
            c_type, c_name, temp, i
        ));
    }
    Ok(())
}
