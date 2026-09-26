use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_ast::statements::TypeAnnotation;
use rl_utils::errors::Error;

// Emitters for `core::` intrinsics. Typed getters are selected from the
// container's static element/value type (standard 64-bit layouts only);
// unknown layouts go through the boxed forms with declaration-driven
// unboxing (`rl_unbox_*`). Abort-on-misuse matches the VM.

// Suffix for a typed getter, or None for the boxed form.
fn getter_suffix(ta: &TypeAnnotation) -> Option<&'static str> {
    match ta {
        TypeAnnotation::Int | TypeAnnotation::CInt => Some("i64"),
        TypeAnnotation::Float | TypeAnnotation::CFloat => Some("f64"),
        TypeAnnotation::Bool | TypeAnnotation::CBool => Some("bool"),
        TypeAnnotation::String | TypeAnnotation::CString => Some("str"),
        TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => Some("arr"),
        TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _) => Some("map"),
        _ => None,
    }
}

fn array_elem_ta(cc: &CCodegen, id: &ExprId) -> Option<TypeAnnotation> {
    cc.array_arg_elem(*id)
}

fn one_arg(cc: &mut CCodegen, name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(name);
    cc.writer.write("(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(")");
    Ok(())
}

fn two_args(cc: &mut CCodegen, name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(name);
    cc.writer.write("(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(")");
    Ok(())
}

fn three_args(cc: &mut CCodegen, name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(name);
    cc.writer.write("(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(", ");
    if args.len() >= 3 {
        cc.compile_expr(args[2])?;
    }
    cc.writer.write(")");
    Ok(())
}

fn map_value_ta(cc: &CCodegen, id: &ExprId) -> Option<TypeAnnotation> {
    match cc.inferred_expr_type(*id)? {
        TypeAnnotation::Map(_, vt) | TypeAnnotation::CMap(_, vt) => {
            let v = *vt;
            if CCodegen::needs_inference(&v) {
                None
            } else {
                Some(v)
            }
        }
        _ => None,
    }
}

// VMs maps/sets are shared references: a bare `__map_set(m, k, v)`
// mutates visibly. C structs are values, so the emitter assigns the
// result back when the target is a plain variable (`(m = f(m, ...))`,
// valid in statement and expression positions alike). Anything else
// (literals, call results) keeps the plain call and loses the
// mutation - loud only in the sense that it compiles; documented.
fn mutating_target(cc: &CCodegen, id: &ExprId) -> Option<String> {
    use rl_ast::nodes::ExpressionKind;
    let expr = cc.ast.exprs.get(*id);
    if let ExpressionKind::ResolvedIdentifier { name, .. } = &expr.kind {
        Some(cc.lookup(name))
    } else {
        None
    }
}

pub(super) fn compile_arr_new(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_core_arr_new()");
    Ok(())
}

pub(super) fn compile_arr_push(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_core_arr_push(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", rl_box(");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_get(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if args.is_empty() {
        cc.writer.write("rl_core_arr_get_boxed(rl_arr_new(8), 0)");
        return Ok(());
    }
    match array_elem_ta(cc, &args[0]).as_ref().and_then(getter_suffix) {
        Some(suffix) => {
            cc.writer.write(&format!("rl_core_arr_get_{}(", suffix));
            cc.compile_expr(args[0])?;
            cc.writer.write(", ");
            if args.len() >= 2 {
                cc.compile_expr(args[1])?;
            }
            cc.writer.write(")");
            Ok(())
        }
        None => {
            cc.writer.write("rl_core_arr_get_boxed(");
            cc.compile_expr(args[0])?;
            cc.writer.write(", ");
            if args.len() >= 2 {
                cc.compile_expr(args[1])?;
            }
            cc.writer.write(")");
            Ok(())
        }
    }
}

pub(super) fn compile_arr_set(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_core_arr_set(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(", rl_box(");
    if args.len() >= 3 {
        cc.compile_expr(args[2])?;
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_map_new(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_map_new()");
    Ok(())
}

pub(super) fn compile_map_get(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if args.is_empty() {
        cc.writer.write("rl_core_map_get_boxed(rl_map_new(), rl_str_literal(\"\", 0))");
        return Ok(());
    }
    match map_value_ta(cc, &args[0]).as_ref().and_then(getter_suffix) {
        Some(suffix) => {
            cc.writer.write(&format!("rl_core_map_get_{}(", suffix));
            cc.compile_expr(args[0])?;
            cc.writer.write(", ");
            if args.len() >= 2 {
                cc.compile_expr(args[1])?;
            }
            cc.writer.write(")");
            Ok(())
        }
        None => {
            cc.writer.write("rl_core_map_get_boxed(");
            cc.compile_expr(args[0])?;
            cc.writer.write(", ");
            if args.len() >= 2 {
                cc.compile_expr(args[1])?;
            }
            cc.writer.write(")");
            Ok(())
        }
    }
}

pub(super) fn compile_map_set(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // Assigns back into plain-variable targets (shared-map semantics);
    // other shapes keep the plain call (documented limitation).
    if let Some(var) = args.first().and_then(|id| mutating_target(cc, id)) {
        cc.writer.write(&format!("({} = rl_core_map_set(", var));
        cc.compile_expr(args[0])?;
        cc.writer.write(", ");
        if args.len() >= 2 {
            cc.compile_expr(args[1])?;
        }
        cc.writer.write(", rl_box(");
        if args.len() >= 3 {
            cc.compile_expr(args[2])?;
        }
        cc.writer.write(")))");
        return Ok(());
    }
    cc.writer.write("rl_core_map_set(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(", rl_box(");
    if args.len() >= 3 {
        cc.compile_expr(args[2])?;
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_map_keys(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_core_map_keys(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_set_new(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_set_new()");
    Ok(())
}

pub(super) fn compile_set_add(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if let Some(var) = args.first().and_then(|id| mutating_target(cc, id)) {
        cc.writer.write(&format!("({} = rl_core_set_add(", var));
        cc.compile_expr(args[0])?;
        cc.writer.write(", rl_box(");
        if args.len() >= 2 {
            cc.compile_expr(args[1])?;
        }
        cc.writer.write(")))");
        return Ok(());
    }
    cc.writer.write("rl_core_set_add(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", rl_box(");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_set_has(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_core_set_has(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", rl_box(");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_abort(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_panic(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_arr_remove(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_core_arr_remove", args)
}

pub(super) fn compile_map_remove(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if let Some(var) = args.first().and_then(|id| mutating_target(cc, id)) {
        cc.writer.write(&format!("({} = rl_core_map_remove(", var));
        cc.compile_expr(args[0])?;
        cc.writer.write(", ");
        if args.len() >= 2 {
            cc.compile_expr(args[1])?;
        }
        cc.writer.write("))");
        return Ok(());
    }
    two_args(cc, "rl_core_map_remove", args)
}

pub(super) fn compile_map_has(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_core_map_has", args)
}

pub(super) fn compile_set_remove(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if let Some(var) = args.first().and_then(|id| mutating_target(cc, id)) {
        cc.writer.write(&format!("({} = rl_core_set_remove(", var));
        cc.compile_expr(args[0])?;
        cc.writer.write(", rl_box(");
        if args.len() >= 2 {
            cc.compile_expr(args[1])?;
        }
        cc.writer.write(")))");
        return Ok(());
    }
    cc.writer.write("rl_core_set_remove(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", rl_box(");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_arr_len(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_core_arr_len", args)
}

pub(super) fn compile_map_len(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_core_map_len", args)
}

pub(super) fn compile_set_len(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_core_set_len", args)
}

pub(super) fn compile_str_len(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_core_str_len", args)
}

pub(super) fn compile_str_get_byte(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_core_str_get_byte", args)
}

pub(super) fn compile_str_slice(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    three_args(cc, "rl_core_str_slice", args)
}

pub(super) fn compile_str_concat(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_core_str_concat", args)
}

// Buffers are int64 ids into a C-side table, exactly like the VM side:
// no reassignment needed, the table mutates behind the id.
pub(super) fn compile_buf_new(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_buf_new()");
    Ok(())
}

pub(super) fn compile_buf_len(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_buf_len", args)
}

pub(super) fn compile_buf_push_byte(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_buf_push_byte", args)
}

pub(super) fn compile_buf_get_byte(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_buf_get_byte", args)
}

pub(super) fn compile_buf_set_byte(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    three_args(cc, "rl_buf_set_byte", args)
}

pub(super) fn compile_buf_append(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_buf_append", args)
}

pub(super) fn compile_buf_slice(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    three_args(cc, "rl_buf_slice", args)
}

pub(super) fn compile_buf_clear(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_buf_clear", args)
}

pub(super) fn compile_buf_to_string(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_buf_to_string", args)
}

pub(super) fn compile_buf_free(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_buf_free", args)
}

pub(super) fn compile_buf_addr(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_buf_addr", args)
}

pub(super) fn compile_buf_resize(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    two_args(cc, "rl_buf_resize", args)
}

pub(super) fn compile_syscall6(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_core_syscall6(");
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            cc.writer.write(", ");
        }
        cc.compile_expr(*arg)?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_type_of(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_core_type_of(rl_ok(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write("))");
    Ok(())
}

/// `__result_ok_value(r)`: trust-and-verify unwrap, aborting on err -
/// mirrors `result_unwrap` (tuple payloads travel as one element arrays).
pub(super) fn compile_result_ok_value(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if args.is_empty() {
        cc.writer.write("rl_result_unwrap_i64(rl_err(-1))");
        return Ok(());
    }
    if let Some(fields) = cc.tuple_payload_fields(args[0]) {
        let tname = cc.ensure_tuple_type(fields);
        cc.writer.write(&format!("(({0}*)rl_result_unwrap_arr(", tname));
        cc.compile_expr(args[0])?;
        cc.writer.write(").data)[0]");
        return Ok(());
    }
    let unwrap_fn = cc.unwrap_fn_for_result(args[0]);
    cc.writer.write(&format!("{unwrap_fn}("));
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

/// `__result_err_value(r)`: the err payload, aborting on ok - mirrors
/// `result_unwrap_err`'s mapping exactly (including no tuple branch).
pub(super) fn compile_result_err_value(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if args.is_empty() {
        cc.writer.write("rl_result_unwrap_err_i64(rl_ok(0))");
        return Ok(());
    }
    let unwrap_fn = match cc.unwrap_fn_for_result(args[0]) {
        "rl_result_unwrap_str" => "rl_result_unwrap_err_str",
        "rl_result_unwrap_f64" => "rl_result_unwrap_err_f64",
        "rl_result_unwrap_bool" => "rl_result_unwrap_err_bool",
        _ => "rl_result_unwrap_err_i64",
    };
    cc.writer.write(&format!("{unwrap_fn}("));
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}
