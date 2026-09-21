use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::statements::{Param, Statement, TypeAnnotation};
use rl_utils::errors::Error;

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
    for p in params {
        cc.declare(&p.param_name, &mangle(&p.param_name));
    }
    for s in body {
        cc.compile_statement(s)?;
    }
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
            for p in params {
                cc.declare(&p.param_name, &mangle(&p.param_name));
            }
            for s in body {
                cc.compile_statement(s)?;
            }
            cc.pop_scope();
            cc.writer.dedent();
            cc.writer.write_indent();
            cc.writer.write("}\n\n");
        }
    }
    Ok(())
}
