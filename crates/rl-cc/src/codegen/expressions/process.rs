use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_exit(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("exit(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_pid(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("getpid()");
    Ok(())
}

pub(super) fn compile_sleep(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("usleep(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(" * 1000)");
    Ok(())
}

pub(super) fn compile_env(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // VM returns a bare string when set, null when missing.
    cc.writer.write("rl_process_env(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_cwd(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_process_cwd()");
    Ok(())
}

pub(super) fn compile_set_cwd(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_set_cwd(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_exec(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_exec_fg(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec_fg(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_exec_code(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec_code(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_exec_lines(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec_lines(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_with_exec(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_with_exec(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_with_exec_code(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_with_exec_code(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_with_exec_lines(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_with_exec_lines(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_args(cc: &mut CCodegen) -> Result<(), Error> {
    // VM returns a bare array of strings.
    cc.writer.write("rl_process_args()");
    Ok(())
}

pub(super) fn compile_os_name(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_process_os_name()");
    Ok(())
}

pub(super) fn compile_exec_background(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec_background(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_process_running(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_running(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_term_pid(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_term_pid(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_kill_pid(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_kill_pid(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_wait_pid(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_wait_pid(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_time_now(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("time(NULL)");
    Ok(())
}

pub(super) fn compile_time_now_ms(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_time_now_ms()");
    Ok(())
}

pub(super) fn compile_time_add(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(" + ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    Ok(())
}

pub(super) fn compile_time_diff(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(" - ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    Ok(())
}

pub(super) fn compile_format_time(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_time_format_time(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_format_date_str(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_time_format_date_str(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_format_time_str(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_time_format_time_str(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_time_parts(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_time_parts(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_exists(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("((bool)(access(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
        cc.writer.write(".data");
    }
    cc.writer.write(", F_OK) == 0))");
    Ok(())
}

pub(super) fn compile_path_extension(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_extension(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_filename(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_filename(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_parent(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_parent(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_stem(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_stem(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_pop(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_pop(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_join(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_join(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_set_extension(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_set_extension(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_is_dir(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_is_dir(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_is_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_is_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_mkdir(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_mkdir(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_rmdir(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_rmdir(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_move_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_move_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_rename_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_rename_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_file_created(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_file_created(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_touch(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_touch(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_temp_dir(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_str_literal(\"/tmp\", 4)");
    Ok(())
}

pub(super) fn compile_file_size(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_file_size(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_file_modified(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_file_modified(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_copy_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_copy_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_mkdir_all(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_mkdir_all(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_rmdir_all(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_rmdir_all(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_list_dir(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_list_dir(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}
