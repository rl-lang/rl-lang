use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// `std::test` runtime helpers: thin calls into the C test backend
/// (`rl_test_*` in the runtime). Assertion values box through `rl_ok`
/// (which passes `rl_result` through); messages and closures pass raw.
pub(super) fn compile_test_fn(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "test_assert_eq" | "test_assert_ne" => {
            let c_fn = if func_name == "test_assert_eq" {
                "rl_test_assert_eq"
            } else {
                "rl_test_assert_ne"
            };
            cc.writer.write(&format!("{c_fn}(rl_ok("));
            if !args.is_empty() {
                cc.compile_expr(args[0])?;
            }
            cc.writer.write("), rl_ok(");
            if args.len() >= 2 {
                cc.compile_expr(args[1])?;
            }
            cc.writer.write("), ");
            if args.len() >= 3 {
                cc.compile_expr(args[2])?;
            }
            cc.writer.write(")");
            return Ok(());
        }
        _ => {}
    }
    let c_fn = match func_name {
        "test_skip" => "rl_test_skip",
        "test_skip_if" => "rl_test_skip_if",
        "test_assert_panics" => "rl_test_assert_panics",
        "test_assert_no_panic" => "rl_test_assert_no_panic",
        "test_run_registered" => "rl_test_run_registered",
        _ => unreachable!(),
    };
    cc.writer.write(c_fn);
    cc.writer.write("(");
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            cc.writer.write(", ");
        }
        cc.compile_expr(*arg)?;
    }
    cc.writer.write(")");
    Ok(())
}
