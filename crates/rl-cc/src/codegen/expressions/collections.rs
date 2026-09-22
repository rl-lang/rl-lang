use crate::codegen::CCodegen;
use rl_ast::statements::TypeAnnotation;
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
    use rl_ast::statements::TypeAnnotation;
    // VM `len` returns result[int] for arrays, tuples and strings,
    // Err for anything else.
    if args.is_empty() {
        cc.writer.write("rl_err(-1)");
        return Ok(());
    }
    match cc.inferred_expr_type(args[0]).as_ref() {
        Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => {
            cc.writer.write("rl_ok_i64((int64_t)rl_str_len(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        Some(TypeAnnotation::Array(_)) | Some(TypeAnnotation::CArray(_)) => {
            cc.writer.write("rl_ok_i64((int64_t)(");
            cc.compile_expr(args[0])?;
            cc.writer.write(").len)");
        }
        Some(TypeAnnotation::Map(_, _)) | Some(TypeAnnotation::CMap(_, _)) => {
            cc.writer.write("rl_ok_i64((int64_t)rl_map_len(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        Some(TypeAnnotation::Set(_)) | Some(TypeAnnotation::CSet(_)) => {
            cc.writer.write("rl_ok_i64((int64_t)rl_set_len(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        Some(TypeAnnotation::Tuple(elems)) | Some(TypeAnnotation::CTuple(elems)) => {
            cc.writer.write(&format!("rl_ok_i64({})", elems.len()));
        }
        Some(TypeAnnotation::Result(_)) => {
            cc.writer.write("rl_len_result(");
            cc.compile_expr(args[0])?;
            cc.writer.write(")");
        }
        Some(_) => {
            cc.writer.write("rl_err(-1)");
        }
        None => {
            // Fully dynamic: wrap and dispatch on the tag at runtime.
            cc.writer.write("rl_len_result(rl_ok(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
    }
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
/// Element type of an array-typed argument, for generic array ops.
fn array_arg_elem(cc: &CCodegen, id: &ExprId) -> Option<TypeAnnotation> {
    cc.array_arg_elem(*id)
}

/// Result-payload tag constant selecting the element wrapping.
fn elem_tag(elem: &TypeAnnotation) -> &'static str {
    match elem {
        TypeAnnotation::Float | TypeAnnotation::CFloat => "RL_TAG_F64",
        TypeAnnotation::Bool | TypeAnnotation::CBool => "RL_TAG_BOOL",
        TypeAnnotation::String | TypeAnnotation::CString => "RL_TAG_STR",
        TypeAnnotation::Char | TypeAnnotation::CChar => "RL_TAG_CHAR",
        _ => "RL_TAG_I64",
    }
}

pub(super) fn compile_arr_first_last(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    // Element-typed access when the array type is known; int fast path
    // otherwise.
    match func_name {
        "arr_first" => {
            if let Some(elem) = args.first().and_then(|id| array_arg_elem(cc, id)) {
                cc.writer
                    .write(&format!("rl_arr_first_t("));
                if !args.is_empty() { cc.compile_expr(args[0])?; }
                cc.writer.write(&format!(", {})", elem_tag(&elem)));
            } else {
                cc.writer.write("rl_arr_first(");
                if !args.is_empty() { cc.compile_expr(args[0])?; }
                cc.writer.write(")");
            }
        }
        "arr_last" => {
            if let Some(elem) = args.first().and_then(|id| array_arg_elem(cc, id)) {
                cc.writer.write("rl_arr_last_t(");
                if !args.is_empty() { cc.compile_expr(args[0])?; }
                cc.writer.write(&format!(", {})", elem_tag(&elem)));
            } else {
                cc.writer.write("rl_arr_last(");
                if !args.is_empty() { cc.compile_expr(args[0])?; }
                cc.writer.write(")");
            }
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_arr_contains(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "arr_contains" => {
            if let Some(elem) = args.first().and_then(|id| array_arg_elem(cc, id)) {
                cc.writer.write("rl_arr_contains_v(");
                if args.len() >= 1 { cc.compile_expr(args[0])?; }
                cc.writer.write(", ");
                if args.len() >= 2 {
                    cc.emit_value_wrapping(&elem, args[1])?;
                }
                cc.writer.write(")");
            } else {
                cc.writer.write("rl_arr_contains(");
                if args.len() >= 1 { cc.compile_expr(args[0])?; }
                cc.writer.write(", ");
                if args.len() >= 2 { cc.compile_expr(args[1])?; }
                cc.writer.write(")");
            }
        }
        "arr_index_of" => {
            if let Some(elem) = args.first().and_then(|id| array_arg_elem(cc, id)) {
                cc.writer.write("rl_arr_index_of_v(");
                if args.len() >= 1 { cc.compile_expr(args[0])?; }
                cc.writer.write(", ");
                if args.len() >= 2 {
                    cc.emit_value_wrapping(&elem, args[1])?;
                }
                cc.writer.write(")");
            } else {
                cc.writer.write("rl_arr_index_of(");
                if args.len() >= 1 { cc.compile_expr(args[0])?; }
                cc.writer.write(", ");
                if args.len() >= 2 { cc.compile_expr(args[1])?; }
                cc.writer.write(")");
            }
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
    cc.writer.write("rl_ok(rl_arr_unique_t(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if !args.is_empty() {
        cc.writer.write(cc.array_elem_tag(args[0]));
    } else {
        cc.writer.write("RL_TAG_I64");
    }
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
    // Boxed fill when the value type is known; int fast path otherwise.
    let wrappable = if !args.is_empty() {
        match cc.inferred_expr_type(args[0]).as_ref() {
            Some(ta) if !crate::codegen::CCodegen::needs_inference(ta) => Some(ta.clone()),
            _ => None,
        }
    } else {
        None
    };
    if let Some(ta) = wrappable {
        cc.writer.write("rl_arr_fill_v(");
        cc.emit_value_wrapping(&ta, args[0])?;
        cc.writer.write(", ");
        if args.len() >= 2 { cc.compile_expr(args[1])?; }
        cc.writer.write(")");
    } else {
        cc.writer.write("rl_arr_fill(");
        if args.len() >= 1 { cc.compile_expr(args[0])?; }
        cc.writer.write(", ");
        if args.len() >= 2 { cc.compile_expr(args[1])?; }
        cc.writer.write(")");
    }
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
    let c_func = match func_name {
        "arr_sum" => "rl_arr_sum_t(",
        "arr_product" => "rl_arr_product_t(",
        "arr_max" => "rl_arr_max_t(",
        "arr_min" => "rl_arr_min_t(",
        _ => return Ok(()),
    };
    cc.writer.write(c_func);
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if !args.is_empty() {
        cc.writer.write(cc.array_elem_tag(args[0]));
    } else {
        cc.writer.write("RL_TAG_I64");
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_arr_sort(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_arr_sort_t(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if !args.is_empty() {
        cc.writer.write(cc.array_elem_tag(args[0]));
    } else {
        cc.writer.write("RL_TAG_I64");
    }
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
    use crate::types::type_to_c;
    // Pairwise zip into real tuples when both element types are known.
    if args.len() >= 2 {
        let ea = cc.array_arg_elem(args[0]);
        let eb = cc.array_arg_elem(args[1]);
        if let (Some(ea), Some(eb)) = (ea, eb) {
            let tuple_name = cc.ensure_tuple_type(vec![ea.clone(), eb.clone()]);
            let ca = type_to_c(&ea);
            let cb = type_to_c(&eb);
            cc.writer.write("rl_arr_zip_t(");
            cc.compile_expr(args[0])?;
            cc.writer.write(", ");
            cc.compile_expr(args[1])?;
            cc.writer.write(&format!(
                ", sizeof({}), offsetof({}, field_1), sizeof({}), sizeof({})",
                tuple_name, tuple_name, ca, cb
            ));
            cc.writer.write(")");
            return Ok(());
        }
    }
    // Legacy flat interleave for dynamically-typed inputs, as a GNU
    // statement expression yielding an ok result like the VM.
    let arr_count = args.len();
    if arr_count == 0 {
        cc.writer.write("rl_ok(rl_arr_from_vals(NULL, 0, sizeof(int64_t)))");
        return Ok(());
    }

    let len_var = cc.temp_var();
    let src_vars: Vec<String> = (0..arr_count).map(|_| cc.temp_var()).collect();
    let i_var = cc.temp_var();
    let result_var = cc.temp_var();

    cc.writer.write("rl_ok(({{\n");
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

    cc.writer.write(&format!("{};\n", result_var));

    cc.writer.dedent();
    cc.writer.write("}))");
    Ok(())
}

pub(super) fn compile_arr_mut(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "arr_push" => {
            // Element-typed push when the array's element type is known
            // (strings, floats, ...); int fast path otherwise.
            let elem = args.first().and_then(|id| cc.array_arg_elem(*id));
            match elem {
                Some(elem_ta) => {
                    cc.writer.write("rl_arr_push_v(");
                    if args.len() >= 1 { cc.compile_expr(args[0])?; }
                    cc.writer.write(", ");
                    if args.len() >= 2 {
                        cc.emit_value_wrapping(&elem_ta, args[1])?;
                    }
                    cc.writer.write(")");
                }
                None => {
                    cc.writer.write("rl_arr_push(");
                    if args.len() >= 1 { cc.compile_expr(args[0])?; }
                    cc.writer.write(", ");
                    if args.len() >= 2 { cc.compile_expr(args[1])?; }
                    cc.writer.write(")");
                }
            }
        }
        "arr_pop" => {
            cc.writer.write("rl_arr_pop(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write(")");
        }
        "arr_insert" => {
            // RL order is (array, value, index); C order is (array, index, value).
            // Element-typed insert when the array type is known.
            let elem = args.first().and_then(|id| cc.array_arg_elem(*id));
            if let Some(elem_ta) = elem {
                cc.writer.write("rl_arr_insert_v(");
                if args.len() >= 1 { cc.compile_expr(args[0])?; }
                cc.writer.write(", ");
                if args.len() >= 3 { cc.compile_expr(args[2])?; }
                cc.writer.write(", ");
                if args.len() >= 2 {
                    cc.emit_value_wrapping(&elem_ta, args[1])?;
                }
                cc.writer.write(")");
            } else {
                cc.writer.write("rl_arr_insert(");
                if args.len() >= 1 { cc.compile_expr(args[0])?; }
                cc.writer.write(", ");
                if args.len() >= 3 { cc.compile_expr(args[2])?; }
                cc.writer.write(", ");
                if args.len() >= 2 { cc.compile_expr(args[1])?; }
                cc.writer.write(")");
            }
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
