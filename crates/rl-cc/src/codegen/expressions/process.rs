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

pub(super) fn compile_set_env(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_set_env(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_remove_env(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_remove_env(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_env_keys(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_process_env_keys()");
    Ok(())
}

pub(super) fn compile_arch(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_process_arch()");
    Ok(())
}

pub(super) fn compile_num_cpus(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_process_num_cpus()");
    Ok(())
}

pub(super) fn compile_parent_pid(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_process_parent_pid()");
    Ok(())
}

pub(super) fn compile_process_exists(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exists(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_with_exec_fg(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_with_exec_fg(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_exec_with_stdin(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec_with_stdin(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_with_exec_with_stdin(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_with_exec_with_stdin(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(", ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_exec_with_env(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec_with_env(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_with_exec_with_env(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_with_exec_with_env(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(", ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_exec_with_cwd(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec_with_cwd(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_with_exec_with_cwd(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_with_exec_with_cwd(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(", ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_exec_with_timeout(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_exec_with_timeout(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_with_exec_background(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_with_exec_background(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_pipe(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_pipe(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_pipe_all(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_process_pipe_all(");
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

pub(super) fn compile_monotonic_now(cc: &mut CCodegen) -> Result<(), Error> {
    // VM returns bare int nanos since first call; the runtime mirrors it.
    cc.writer.write("rl_time_monotonic_now()");
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

pub(super) fn compile_path_is_absolute(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_is_absolute(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_is_relative(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_is_relative(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_starts_with(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_starts_with(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_ends_with(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_ends_with(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_normalize(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_normalize(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_absolute(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_absolute(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_canonicalize(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_canonicalize(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_expand_home(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_expand_home(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_split(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_split(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_split_extension(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_split_extension(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_components(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_components(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_with_file_name(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_with_file_name(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_relative(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_relative(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_path_join_many(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_path_join_many(");
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

pub(super) fn compile_list_dir_names(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_list_dir_names(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_file_accessed(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_file_accessed(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_file_permissions(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_file_permissions(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_set_permissions(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_set_permissions(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_temp_file(cc: &mut CCodegen) -> Result<(), Error> {
    // VM returns result[string]; the runtime builds ok or error.
    cc.writer.write("rl_fs_temp_file()");
    Ok(())
}

pub(super) fn compile_temp_file_in(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_temp_file_in(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_truncate_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_truncate_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_glob(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_glob(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_walk_dir(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_walk_dir(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_symlink(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_symlink(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_readlink(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_readlink(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_hardlink(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_hardlink(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_realpath(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_realpath(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_lock_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_lock_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_unlock_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_unlock_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_copy_dir(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_copy_dir(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_dir_size(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_dir_size(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_is_symlink(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_is_symlink(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_open(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // VM returns result[handle(File)]; the runtime holds an int64 id.
    cc.writer.write("rl_fs_open(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_fs_close(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // FS close takes a file handle id; c close takes a c handle id.
    cc.writer.write("rl_fs_close(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_read_handle(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_read_handle(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_write_handle(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_write_handle(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_seek(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_seek(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(", ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_flush(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_flush(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_read_all(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_fs_read_all(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_readline(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // VM trims trailing whitespace and returns empty at EOF.
    cc.writer.write("rl_fs_readline(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}
