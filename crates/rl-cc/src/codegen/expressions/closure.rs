use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// Higher-order array functions: the callback is any expression
/// evaluating to a closure (inline lambda or closure variable),
/// mirroring the VM's `call_value` over any callable. The array's
/// static element tag tells the runtime how to box elements.
pub(super) fn compile_arr_closure(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(&format!("rl_{func_name}_closure("));
    // Reduce is (array, callback, init, tag); the rest are (array,
    // callback, tag). The init value wraps as a result.
    let inline = if func_name == "arr_reduce" && args.len() >= 3 {
        &args[..2]
    } else {
        args
    };
    for (i, arg) in inline.iter().enumerate() {
        if i > 0 {
            cc.writer.write(", ");
        }
        cc.compile_expr(*arg)?;
    }
    if func_name == "arr_reduce" {
        cc.writer.write(", ");
        if args.len() >= 3 {
            cc.writer.write("rl_ok(");
            cc.compile_expr(args[2])?;
            cc.writer.write(")");
        }
        cc.writer.write(", ");
    } else {
        cc.writer.write(", ");
    }
    if !args.is_empty() {
        cc.writer.write(cc.array_elem_tag(args[0]));
    } else {
        cc.writer.write("RL_TAG_I64");
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_result_closure(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    let c_func = match func_name {
        "result_map" => "rl_result_map_closure(",
        "result_map_err" => "rl_result_map_err_closure(",
        _ => unreachable!(),
    };
    cc.writer.write(c_func);
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            cc.writer.write(", ");
        }
        cc.compile_expr(*arg)?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_bench(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_bench_closure(");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    cc.writer.write(")");
    Ok(())
}
