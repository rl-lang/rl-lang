use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::{Statement, StatementKind, TypeAnnotation};
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// `for item in array_expr { body }` — lowers to an index loop over the
/// array's `.data` buffer. The element type is inferred from the iterable:
/// a known array variable's element type, an array literal's first item,
/// or `int` as a fallback.
pub(super) fn compile_foreach(
    cc: &mut CCodegen,
    variable: &str,
    iterable: ExprId,
    body: &[Statement],
) -> Result<(), Error> {
    let arr_temp = cc.temp_var();
    let idx_temp = cc.temp_var();
    cc.writer.write_indent();
    cc.writer.write(&format!("rl_array {} = ", arr_temp));
    cc.compile_expr(iterable)?;
    cc.writer.write(";\n");

    // Resolve element type from the iterable
    let iter_expr = cc.ast.exprs.get(iterable);
    let elem_type = match &iter_expr.kind {
        ExpressionKind::ResolvedIdentifier { name, .. } => match cc.var_types.get(name) {
            Some(TypeAnnotation::Array(inner)) => (**inner).clone(),
            Some(ta) => ta.clone(),
            None => TypeAnnotation::Int,
        },
        ExpressionKind::ArrayLiteral(elems) if !elems.is_empty() => {
            let first = cc.ast.exprs.get(elems[0]);
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

    cc.writer.write_indent();
    cc.writer.write(&format!("uint64_t {} = 0;\n", idx_temp));
    cc.writer.write_indent();
    cc.writer.write(&format!(
        "for (; {} < {}.len; {}++) {{\n",
        idx_temp, arr_temp, idx_temp
    ));
    cc.writer.indent();
    let c_name = mangle(variable);
    cc.declare(variable, &c_name);
    cc.var_types.insert(variable.to_string(), elem_type);
    cc.writer.write_indent();
    cc.writer.write(&format!(
        "{} {} = (({}*){}.data)[{}];\n",
        c_type, c_name, c_type, arr_temp, idx_temp
    ));
    for s in body {
        cc.compile_statement(s)?;
    }
    cc.writer.dedent();
    cc.writer.write_indent();
    cc.writer.write("}\n");
    Ok(())
}

/// `for i in N..M { body }` — the parser pre-evaluates the range into a
/// `Range(items)` statement; emits `for (int64_t i = first; i < last+1; i++)`.
pub(super) fn compile_for_range(
    cc: &mut CCodegen,
    variable: &str,
    range: &Statement,
    body: &[Statement],
) -> Result<(), Error> {
    let items = match &range.kind {
        StatementKind::Range(items) => items.clone(),
        _ => vec![],
    };
    if !items.is_empty() {
        let first = items[0];
        let last = items[items.len() - 1];
        let c_name = mangle(variable);
        cc.declare(variable, &c_name);
        cc.var_types
            .insert(variable.to_string(), TypeAnnotation::Int);
        cc.writer.write_indent();
        cc.writer.write(&format!(
            "for (int64_t {} = {}; {} < {}; {}++) {{\n",
            c_name,
            first,
            c_name,
            last + 1,
            c_name
        ));
        cc.writer.indent();
        for s in body {
            cc.compile_statement(s)?;
        }
        cc.writer.dedent();
        cc.writer.write_indent();
        cc.writer.write("}\n");
    }
    Ok(())
}
