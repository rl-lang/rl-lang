use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_is_empty(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(".len == 0)");
    Ok(())
}

pub(super) fn compile_arr_count(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(".len");
    Ok(())
}

pub(super) fn compile_len(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_str_len(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_map_len(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok((int64_t)rl_map_len(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_set_len(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok((int64_t)rl_set_len(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_set_ops(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "set_add" => {
            cc.writer.write("rl_ok(rl_set_add_s(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 {
                let expr = cc.ast.exprs.get(args[1]);
                let ta = cc.infer_expr_type(&expr.kind);
                cc.emit_value_wrapping(&ta, args[1])?;
            }
            cc.writer.write("))");
        }
        "set_remove" => {
            cc.writer.write("rl_ok(rl_set_remove_s(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 {
                let expr = cc.ast.exprs.get(args[1]);
                let ta = cc.infer_expr_type(&expr.kind);
                cc.emit_value_wrapping(&ta, args[1])?;
            }
            cc.writer.write("))");
        }
        "set_contains" => {
            cc.writer.write("rl_set_contains_s(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 {
                let expr = cc.ast.exprs.get(args[1]);
                let ta = cc.infer_expr_type(&expr.kind);
                cc.emit_value_wrapping(&ta, args[1])?;
            }
            cc.writer.write(")");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_set_to_array(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_set_to_array(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_map_ops(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "map_contains" => {
            cc.writer.write("rl_map_contains_s(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        "map_remove" => {
            cc.writer.write("rl_ok(rl_map_remove_s(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write("))");
        }
        "map_get" => {
            cc.writer.write("rl_ok(rl_map_get_s(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write("))");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_map_keys_values(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "map_keys" => {
            cc.writer.write("rl_ok(rl_map_keys(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write("))");
        }
        "map_values" => {
            cc.writer.write("rl_ok(rl_map_values(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write("))");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_map_clear(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_map_clear(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_map_merge(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_map_merge(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_map_to_array(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_map_to_array_s(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_first_last(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "arr_first" => {
            cc.writer.write("rl_arr_first(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write(")");
        }
        "arr_last" => {
            cc.writer.write("rl_arr_last(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write(")");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_arr_contains(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "arr_contains" => {
            cc.writer.write("rl_arr_contains(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        "arr_index_of" => {
            cc.writer.write("rl_arr_index_of(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_arr_reverse(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_arr_reverse(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_concat(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_arr_concat(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_unique(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_arr_unique(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_slice(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_arr_slice(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(", ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_fill(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_arr_fill(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_arr_range(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_arr_range(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(", ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_arr_agg(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "arr_sum" => {
            cc.writer.write("rl_arr_sum(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write(")");
        }
        "arr_product" => {
            cc.writer.write("rl_arr_product(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write(")");
        }
        "arr_max" => {
            cc.writer.write("rl_arr_max(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write(")");
        }
        "arr_min" => {
            cc.writer.write("rl_arr_min(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write(")");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_arr_sort(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_arr_sort(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_flatten(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_arr_flatten(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_zip(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    let arr_count = args.len();
    if arr_count == 0 {
        cc.writer.write("rl_arr_from_vals(NULL, 0, sizeof(int64_t))");
        return Ok(());
    }

    let len_var = cc.temp_var();
    let src_vars: Vec<String> = (0..arr_count).map(|_| cc.temp_var()).collect();
    let i_var = cc.temp_var();
    let result_var = cc.temp_var();

    cc.writer.write("{\n");
    cc.writer.indent();

    for (i, arg) in args.iter().enumerate() {
        cc.writer.write(&format!("rl_array {} = ", src_vars[i]));
        cc.compile_expr(*arg)?;
        cc.writer.write(";\n");
    }

    cc.writer.write(&format!("int64_t {} = ", len_var));
    cc.writer.write(&src_vars[0]);
    cc.writer.write(".len;\n");

    cc.writer.write(&format!("rl_array {} = rl_arr_from_vals(NULL, (size_t)({} * {}), sizeof(int64_t));\n", result_var, arr_count, len_var));

    cc.writer.write(&format!("for (int64_t {} = 0; {} < {}; {}++) {{\n", i_var, i_var, len_var, i_var));
    cc.writer.indent();

    for (j, src) in src_vars.iter().enumerate() {
        cc.writer.write(&format!(
            "((int64_t*){}.data)[{} * {} + {}] = ((int64_t*){}.data)[{}];\n",
            result_var, i_var, arr_count, j, src, i_var
        ));
    }

    cc.writer.dedent();
    cc.writer.write("}\n");

    cc.writer.write(&format!("{}\n", result_var));

    cc.writer.dedent();
    cc.writer.write("}");
    Ok(())
}

pub(super) fn compile_arr_mut(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "arr_push" => {
            cc.writer.write("rl_arr_push(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        "arr_pop" => {
            cc.writer.write("rl_arr_pop(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write(")");
        }
        "arr_insert" => {
            cc.writer.write("rl_arr_insert(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(", ");
            if args.len() >= 3 { cc.compile_expr(args[2])?; }
            cc.writer.write(")");
        }
        "arr_remove" => {
            cc.writer.write("rl_arr_remove(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        _ => {}
    }
    Ok(())
}
