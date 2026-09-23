use super::control::compile_return;
use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::statements::{Param, Statement, StatementKind, TypeAnnotation};
use rl_checker::structs::CheckType;
use rl_utils::errors::Error;

/// Compiles a function body: like the VM, a trailing expression is the
/// return value (except in `void` functions, where it is discarded).
fn compile_fn_body(cc: &mut CCodegen, c_ret: &str, body: &[Statement]) -> Result<(), Error> {
    let trailing = matches!(
        body.last().map(|s| &s.kind),
        Some(StatementKind::Expression(_))
    ) && c_ret != "void";
    let main = if trailing { &body[..body.len() - 1] } else { body };
    for s in main {
        cc.compile_statement(s)?;
    }
    if trailing {
        if let Some(StatementKind::Expression(expr_id)) = body.last().map(|s| &s.kind) {
            compile_return(cc, Some(*expr_id))?;
        }
    }
    Ok(())
}

/// `fn name(params): Ret { body }` — emits a top-level C function.
/// Parameters are mangled and declared in a fresh scope for the body.
pub(super) fn compile_function_decl(
    cc: &mut CCodegen,
    name: &str,
    params: &[Param],
    return_type: &TypeAnnotation,
    body: &[Statement],
) -> Result<(), Error> {
    let c_ret = type_to_c(return_type);
    let c_name = mangle(name);
    cc.writer.write_indent();
    cc.writer.write(&format!("{} {}(", c_ret, c_name));

    let c_params: Vec<String> = params
        .iter()
        .map(|p| {
            let c_type = type_to_c(&p.param_type);
            let c_name = mangle(&p.param_name);
            format!("{} {}", c_type, c_name)
        })
        .collect();
    cc.writer.write(&c_params.join(", "));
    cc.writer.write(") {\n");
    cc.writer.indent();

    cc.push_scope();
    // Locals must not leak into other functions' bodies.
    let saved_types = cc.var_types.clone();
    let saved_nullable = cc.nullable_vars.clone();
    let saved_closures = cc.closure_return_types.clone();
    let saved_return = cc.fn_return.clone();
    cc.fn_return = Some(return_type.clone());
    for p in params {
        cc.declare(&p.param_name, &mangle(&p.param_name));
        cc.var_types
            .insert(p.param_name.clone(), p.param_type.clone());
    }
    compile_fn_body(cc, &c_ret, body)?;
    cc.var_types = saved_types;
    cc.nullable_vars = saved_nullable;
    cc.closure_return_types = saved_closures;
    cc.fn_return = saved_return;
    cc.pop_scope();
    cc.writer.dedent();
    cc.writer.write_indent();
    cc.writer.write("}\n\n");
    Ok(())
}

/// `impl Record { fn method(...) { ... } }` — emits each method as a
/// top-level `impl_Record_method` C function.
pub(super) fn compile_impl_block(
    cc: &mut CCodegen,
    record: &str,
    methods: &[Statement],
) -> Result<(), Error> {
    for m in methods {
        if let rl_ast::statements::StatementKind::ResolvedFunctionDeclaration {
            name,
            params,
            return_type,
            body,
            ..
        } = &m.kind
        {
            // Prefer the checker-inferred method return over `Null`.
            let mut effective = return_type.clone();
            if effective == TypeAnnotation::Null {
                if let Some(CheckType::Function { return_type, .. }) =
                    cc.checker.methods.get(&(record.to_string(), name.clone()))
                {
                    if *return_type != TypeAnnotation::Null {
                        effective = return_type.clone();
                    }
                }
            }
            let return_type = &effective;
            let c_ret = type_to_c(return_type);
            let c_fn_name = format!("impl_{}_{}", record, name);
            cc.writer.write_indent();
            cc.writer.write(&format!("{} {}(", c_ret, c_fn_name));

            let c_params: Vec<String> = params
                .iter()
                .map(|p| {
                    let c_type = type_to_c(&p.param_type);
                    let c_name = mangle(&p.param_name);
                    format!("{} {}", c_type, c_name)
                })
                .collect();
            cc.writer.write(&c_params.join(", "));
            cc.writer.write(") {\n");
            cc.writer.indent();

            cc.push_scope();
            let saved_types = cc.var_types.clone();
            let saved_nullable = cc.nullable_vars.clone();
            let saved_closures = cc.closure_return_types.clone();
            let saved_return = cc.fn_return.clone();
            cc.fn_return = Some(return_type.clone());
            for p in params {
                cc.declare(&p.param_name, &mangle(&p.param_name));
                cc.var_types
                    .insert(p.param_name.clone(), p.param_type.clone());
            }
            compile_fn_body(cc, &c_ret, body)?;
            cc.var_types = saved_types;
            cc.nullable_vars = saved_nullable;
            cc.closure_return_types = saved_closures;
            cc.fn_return = saved_return;
            cc.pop_scope();
            cc.writer.dedent();
            cc.writer.write_indent();
            cc.writer.write("}\n\n");
        }
    }
    Ok(())
}
