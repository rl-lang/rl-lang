mod assert;
mod c_ffi;
mod closure;
mod collections;
mod http;
mod io;
mod math;
mod network;
mod process;
mod random;
mod result;
mod string;
mod terminal;
mod types;

use crate::codegen::CCodegen;
use crate::codegen::ops::token_to_c_op;
use crate::name_mangle::{escape_c_char, escape_c_string, mangle};
use crate::types::type_to_c;
use crate::writer::CWriter;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_ast::statements::TypeAnnotation;
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl<'a> CCodegen<'a> {
    pub fn compile_expr(&mut self, id: ExprId) -> Result<(), Error> {
        let kind = self.ast.exprs.get(id).kind.clone();
        match &kind {
            ExpressionKind::Integer(v) => {
                self.writer.write(&format!("(int64_t){}", v));
            }
            ExpressionKind::SInt(v) => {
                self.writer.write(&format!("(int32_t){}", v));
            }
            ExpressionKind::UInt(v) => {
                self.writer.write(&format!("(uint64_t){}", v));
            }
            ExpressionKind::SUInt(v) => {
                self.writer.write(&format!("(uint32_t){}", v));
            }
            ExpressionKind::Float(v) => {
                self.writer.write(&format!("(double){}", v));
            }
            ExpressionKind::SFloat(v) => {
                self.writer.write(&format!("(float){}", v));
            }
            ExpressionKind::Bool(v) => {
                self.writer.write(if *v { "true" } else { "false" });
            }
            ExpressionKind::Character(v) => {
                self.writer.write(&escape_c_char(*v));
            }
            ExpressionKind::String(v) => {
                let escaped = escape_c_string(v);
                self.writer
                    .write(&format!("rl_str_literal(\"{}\", {})", escaped, escaped.len()));
            }
            ExpressionKind::Null => {
                self.writer.write("rl_ok_null()");
            }
            ExpressionKind::Byte(v) => {
                self.writer.write(&format!("(uint8_t){}", v));
            }
            ExpressionKind::BByte(v) => {
                self.writer.write(&format!("(uint16_t){}", v));
            }
            ExpressionKind::SByte(v) => {
                self.writer.write(&format!("(int8_t){}", v));
            }
            ExpressionKind::BSByte(v) => {
                self.writer.write(&format!("(int16_t){}", v));
            }
            ExpressionKind::Grouping(inner) => {
                self.compile_expr(*inner)?;
            }
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                self.compile_expr(*left)?;
                self.writer.write(&format!(" {} ", token_to_c_op(operator)));
                self.compile_expr(*right)?;
            }
            ExpressionKind::Unary { operator, operand } => {
                match operator {
                    TokenType::Minus => self.writer.write("-"),
                    TokenType::Bang => self.writer.write("!"),
                    _ => {}
                }
                self.compile_expr(*operand)?;
            }
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                let c_name = self.lookup(name);
                if self.nullable_vars.contains(name) {
                    match self.var_types.get(name) {
                        Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => {
                            self.writer.write(&format!("rl_unwrap_i64({})", c_name));
                        }
                        Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => {
                            self.writer.write(&format!("rl_unwrap_f64({})", c_name));
                        }
                        Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => {
                            self.writer.write(&format!("rl_unwrap_bool({})", c_name));
                        }
                        Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => {
                            self.writer.write(&format!("rl_unwrap_str({})", c_name));
                        }
                        Some(TypeAnnotation::Array(_)) | Some(TypeAnnotation::CArray(_)) => {
                            self.writer.write(&format!("rl_unwrap_arr({})", c_name));
                        }
                        _ => {
                            self.writer.write(&format!("rl_unwrap_i64({})", c_name));
                        }
                    }
                } else {
                    self.writer.write(&c_name);
                }
            }
            ExpressionKind::Identifier(name) => {
                let c_name = self.lookup(name);
                self.writer.write(&c_name);
            }
            ExpressionKind::Call { path, args } => {
                self.compile_func_call(path, args)?;
            }
            ExpressionKind::CallExpr { callee, args } => {
                let callee_expr = self.ast.exprs.get(*callee);
                let (is_closure, callee_name) = if let ExpressionKind::ResolvedIdentifier { name, .. } = &callee_expr.kind {
                    let is_fn = matches!(self.var_types.get(name), Some(TypeAnnotation::Fn) | Some(TypeAnnotation::Callback(_, _)));
                    (is_fn, Some(name.clone()))
                } else {
                    (false, None)
                };

                if is_closure {
                    let return_type = callee_name.as_ref().and_then(|n| self.closure_return_types.get(n));
                    let need_unwrap = !matches!(return_type, None | Some(TypeAnnotation::Result(_)));
                    if need_unwrap {
                        match return_type {
                            Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => self.writer.write("rl_unwrap_i64("),
                            Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => self.writer.write("rl_unwrap_f64("),
                            Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => self.writer.write("rl_unwrap_bool("),
                            Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => self.writer.write("rl_unwrap_str("),
                            Some(TypeAnnotation::Array(_)) | Some(TypeAnnotation::CArray(_)) => self.writer.write("rl_unwrap_arr("),
                            _ => {}
                        }
                    }
                    self.writer.write("rl_closure_call(");
                    self.compile_expr(*callee)?;
                    self.writer.write(", (rl_result[]){ ");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        self.write_arg_as_result(*arg)?;
                    }
                    self.writer.write(&format!(" }}, {})", args.len()));
                    if need_unwrap {
                        self.writer.write(")");
                    }
                } else {
                    self.compile_expr(*callee)?;
                    self.writer.write("(");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        self.compile_expr(*arg)?;
                    }
                    self.writer.write(")");
                }
            }
            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => {
                self.compile_method_call(*caller, method, args)?;
            }
            ExpressionKind::OkLiteral(inner) => {
                self.writer.write("rl_ok(");
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::ErrLiteral(inner) => {
                self.writer.write("rl_err(");
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::ErrorLiteral(inner) => {
                self.writer.write("rl_error(");
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::Propagate(inner) => {
                let temp = self.temp_var();
                self.writer.write_indent();
                self.writer.write(&format!("rl_result {} = ", temp));
                self.compile_expr(*inner)?;
                self.writer.write(";\n");
                self.writer.write_indent();
                self.writer.write(&format!("if (!{}.is_ok) {{\n", temp));
                self.writer.indent();
                self.writer.write_indent();
                self.writer.write(&format!("return {};\n", temp));
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
                self.writer.write(&format!("{}.data.i64", temp));
            }
            ExpressionKind::ArrayLiteral(elems) => {
                self.writer.write("rl_arr_from_vals(&(int64_t[]){");
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    self.compile_expr(*elem)?;
                }
                self.writer.write(&format!("}}, {}, (int32_t)sizeof(int64_t))", elems.len()));
            }
            ExpressionKind::MapLiteral(entries) => {
                let temp = self.temp_var();
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_map {} = rl_map_new();\n", temp));
                for (key_id, val_id) in entries {
                    let key_expr = self.ast.exprs.get(*key_id);
                    if let ExpressionKind::String(key_str) = &key_expr.kind {
                        let val_expr = self.ast.exprs.get(*val_id);
                        let val_type = match &val_expr.kind {
                            ExpressionKind::Integer(_) => TypeAnnotation::Int,
                            ExpressionKind::Float(_) => TypeAnnotation::Float,
                            ExpressionKind::Bool(_) => TypeAnnotation::Bool,
                            ExpressionKind::String(_) => TypeAnnotation::String,
                            ExpressionKind::ArrayLiteral(e) if !e.is_empty() => TypeAnnotation::Array(Box::new(TypeAnnotation::Infer)),
                            _ => TypeAnnotation::Int,
                        };
                        self.writer.write_indent();
                        self.writer.write(&format!(
                            "rl_map_set(&{}, \"{}\", ",
                            temp, key_str
                        ));
                        self.emit_value_wrapping(&val_type, *val_id)?;
                        self.writer.write(");\n");
                    }
                }
                self.writer.write(&temp);
            }
            ExpressionKind::SetLiteral(items) => {
                let temp = self.temp_var();
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_set {} = rl_set_new();\n", temp));
                for item_id in items {
                    let val_expr = self.ast.exprs.get(*item_id);
                    let val_type = match &val_expr.kind {
                        ExpressionKind::Integer(_) => TypeAnnotation::Int,
                        ExpressionKind::Float(_) => TypeAnnotation::Float,
                        ExpressionKind::Bool(_) => TypeAnnotation::Bool,
                        ExpressionKind::String(_) => TypeAnnotation::String,
                        _ => TypeAnnotation::Int,
                    };
                    self.writer.write_indent();
                    self.writer.write(&format!("rl_set_add(&{}, ", temp));
                    self.emit_value_wrapping(&val_type, *item_id)?;
                    self.writer.write(");\n");
                }
                self.writer.write(&temp);
            }
            ExpressionKind::TupleLiteral(elems) => {
                let field_types: Vec<TypeAnnotation> = elems.iter().map(|e| {
                    let expr = self.ast.exprs.get(*e);
                    match &expr.kind {
                        ExpressionKind::Integer(_) => TypeAnnotation::Int,
                        ExpressionKind::Float(_) => TypeAnnotation::Float,
                        ExpressionKind::Bool(_) => TypeAnnotation::Bool,
                        ExpressionKind::String(_) => TypeAnnotation::String,
                        ExpressionKind::Character(_) => TypeAnnotation::Char,
                        ExpressionKind::ResolvedIdentifier { name, .. } => {
                            self.var_types.get(name).cloned().unwrap_or(TypeAnnotation::Int)
                        }
                        _ => TypeAnnotation::Int,
                    }
                }).collect();
                let tuple_name = self.ensure_tuple_type(field_types);
                self.writer.write(&format!("({}){{ ", tuple_name));
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    self.writer.write(&format!(".field_{} = ", i));
                    self.compile_expr(*elem)?;
                }
                self.writer.write(" }");
            }
            ExpressionKind::StructLiteral { name, fields } => {
                let c_name = format!("rl_Record_{}", name);
                self.writer.write(&format!("({}){{ ", c_name));
                for (i, (field_name, field_val)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    self.writer.write(&format!(".{} = ", field_name));
                    self.compile_expr(*field_val)?;
                }
                self.writer.write(" }");
            }
            ExpressionKind::FieldAccess { target, field } => {
                self.compile_expr(*target)?;
                self.writer.write(&format!(".{}", field));
            }
            ExpressionKind::FieldAssign {
                target,
                field,
                value,
            } => {
                self.compile_expr(*target)?;
                self.writer.write(&format!(".{} = ", field));
                self.compile_expr(*value)?;
            }
            ExpressionKind::EnumVariant { enum_name, variant } => {
                let c_name = format!(
                    "RL_TAG_{}_{}",
                    enum_name.to_uppercase(),
                    variant.to_uppercase()
                );
                self.writer.write(&c_name);
            }
            ExpressionKind::ResolvedAssign { name, value, .. } => {
                let c_name = self.lookup(name);
                if self.nullable_vars.contains(name) {
                    let value_expr = self.ast.exprs.get(*value);
                    if let ExpressionKind::Null = &value_expr.kind {
                        self.writer.write(&format!("{} = rl_ok_null()", c_name));
                    } else {
                        match self.var_types.get(name) {
                            Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => {
                                self.writer.write(&format!("{} = rl_ok_i64(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                            Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => {
                                self.writer.write(&format!("{} = rl_ok_f64(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                            Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => {
                                self.writer.write(&format!("{} = rl_ok_bool(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                            Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => {
                                self.writer.write(&format!("{} = rl_ok_str(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                            _ => {
                                self.writer.write(&format!("{} = rl_ok(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                        }
                    }
                } else {
                    self.writer.write(&format!("{} = ", c_name));
                    self.compile_expr(*value)?;
                }
            }
            ExpressionKind::Index { target, index } => {
                let target_expr = self.ast.exprs.get(*target);
                if let ExpressionKind::ResolvedIdentifier { name, .. } = &target_expr.kind
                    && let Some(ta) = self.var_types.get(name)
                        && let TypeAnnotation::Array(inner) = ta {
                            let c_type = type_to_c(inner);
                            let c_name = self.lookup(name);
                            self.writer.write(&format!("(({}*){}.data)[", c_type, c_name));
                            self.compile_expr(*index)?;
                            self.writer.write("]");
                            return Ok(());
                        }
                self.compile_expr(*target)?;
                self.writer.write("[");
                self.compile_expr(*index)?;
                self.writer.write("]");
            }
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => {
                let target_expr = self.ast.exprs.get(*target);
                if let ExpressionKind::ResolvedIdentifier { name, .. } = &target_expr.kind
                    && let Some(ta) = self.var_types.get(name)
                        && let TypeAnnotation::Array(inner) = ta {
                            let c_type = type_to_c(inner);
                            let c_name = self.lookup(name);
                            self.writer.write(&format!("(({}*){}.data)[", c_type, c_name));
                            self.compile_expr(*index)?;
                            self.writer.write("] = ");
                            self.compile_expr(*value)?;
                            return Ok(());
                        }
                self.compile_expr(*target)?;
                self.writer.write("[");
                self.compile_expr(*index)?;
                self.writer.write("] = ");
                self.compile_expr(*value)?;
            }
            ExpressionKind::Cast { value, target_type } => {
                let c_type = type_to_c(target_type);
                self.writer.write(&format!("({})", c_type));
                self.compile_expr(*value)?;
            }
            ExpressionKind::ResolvedLambda { params, return_type, body, .. } => {
                self.compile_lambda(params, return_type, body)?;
            }
            _ => {
                self.writer.write("/* unhandled expr */");
            }
        }
        Ok(())
    }

    pub fn compile_func_call(&mut self, path: &[String], args: &[ExprId]) -> Result<(), Error> {
        let func_name = path.last().map(|s| s.as_str()).unwrap_or("");
        let _is_stdlib = path.first().map(|s| s.as_str()) == Some("std");

        match func_name {
            "println" | "print" => return self::io::compile_print(self, func_name, args),
            "read_file" => return self::io::compile_read_file(self, args),
            "read_lines" => return self::io::compile_read_lines(self, args),
            "read_bytes" => return self::io::compile_read_bytes(self, args),
            "read" => return self::io::compile_read(self),
            "read_int" => return self::io::compile_read_int(self),
            "read_float" => return self::io::compile_read_float(self),
            "write_file" => return self::io::compile_write_file(self, args),
            "append_file" => return self::io::compile_append_file(self, args),
            "delete_file" => return self::io::compile_delete_file(self, args),
            "eprint" => return self::io::compile_eprint(self, args),
            "eprintln" => return self::io::compile_eprintln(self, args),
            "to_upper" => return self::string::compile_to_upper(self, args),
            "to_lower" => return self::string::compile_to_lower(self, args),
            "trim" => return self::string::compile_trim(self, args),
            "trim_start" => return self::string::compile_trim_start(self, args),
            "trim_end" => return self::string::compile_trim_end(self, args),
            "contains" => return self::string::compile_contains(self, args),
            "starts_with" => return self::string::compile_starts_with(self, args),
            "ends_with" => return self::string::compile_ends_with(self, args),
            "replace" => return self::string::compile_replace(self, args),
            "repeat" => return self::string::compile_repeat(self, args),
            "index_of" => return self::string::compile_index_of(self, args),
            "count" => return self::string::compile_count(self, args),
            "pad_left" => return self::string::compile_pad_left(self, args),
            "pad_right" => return self::string::compile_pad_right(self, args),
            "slice" => return self::string::compile_slice(self, args),
            "reverse" => return self::string::compile_reverse(self, args),
            "bytes" => return self::string::compile_bytes(self, args),
            "chars" => return self::string::compile_chars(self, args),
            "char_at" => return self::string::compile_char_at(self, args),
            "join" => return self::string::compile_join(self, args),
            "split" => return self::string::compile_split(self, args),
            "concat" => return self::string::compile_concat(self, args),
            "format" => return self::string::compile_format(self, args),
            "string_is_empty" | "is_empty" => return self::string::compile_is_empty(self, args),
            "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "exp" => return self::math::compile_trig(self, func_name, args),
            "sqrt" | "log2" | "log10" | "ceil" | "floor" | "round" => return self::math::compile_single_ok(self, func_name, args),
            "abs" => return self::math::compile_abs(self, args),
            "hypot" => return self::math::compile_hypot(self, args),
            "atan2" => return self::math::compile_atan2(self, args),
            "pow" => return self::math::compile_pow(self, args),
            "log" => return self::math::compile_log(self, args),
            "radians" => return self::math::compile_radians(self, args),
            "degrees" => return self::math::compile_degrees(self, args),
            "sign" => return self::math::compile_sign(self, args),
            "lerp" => return self::math::compile_lerp(self, args),
            "map_range" => return self::math::compile_map_range(self, args),
            "mod" => return self::math::compile_mod(self, args),
            "max" | "min" | "clamp" => return self::math::compile_min_max_clamp(self, func_name, args),
            "factorial" | "gcd" | "lcm" | "is_prime" | "fibonacci" => return self::math::compile_math_runtime(self, func_name, args),
            "bit_and" | "bit_or" | "bit_xor" | "bit_not" | "bit_shift_left" | "bit_shift_right" => return self::math::compile_bitwise(self, func_name, args),
            "count_bits" => return self::math::compile_count_bits(self, args),
            "leading_zeros" => return self::math::compile_leading_zeros(self, args),
            "trailing_zeros" => return self::math::compile_trailing_zeros(self, args),
            "PI" | "E" | "TAU" | "PHI" | "INF" | "NAN" | "FRAC_1_PI" | "FRAC_1_SQRT_2" | "FRAC_2_PI" | "FRAC_2_SQRT_PI" | "FRAC_PI_2" | "FRAC_PI_3" | "FRAC_PI_4" | "FRAC_PI_6" | "FRAC_PI_8" | "SQRT_2" | "LN_2" | "LN_10" | "LOG2_E" | "LOG2_10" | "LOG10_2" | "LOG10_E" | "EULER_GAMMA" => return self::math::compile_constant(self, func_name),
            "is_inf" => return self::math::compile_is_inf(self, args),
            "is_nan" => return self::math::compile_is_nan(self, args),
            "ok" | "err" | "error" => return self::result::compile_result_wrap(self, func_name, args),
            "is_ok" | "is_err" => return self::result::compile_is_ok_err(self, func_name, args),
            "result_unwrap" | "result_unwrap_err" => return self::result::compile_unwrap(self, func_name, args),
            "result_unwrap_or" => return self::result::compile_unwrap_or(self, args),
            "arr_is_empty" | "set_is_empty" | "map_is_empty" => return self::collections::compile_is_empty(self, args),
            "arr_count" => return self::collections::compile_arr_count(self, args),
            "len" => return self::collections::compile_len(self, args),
            "map_len" => return self::collections::compile_map_len(self, args),
            "set_len" => return self::collections::compile_set_len(self, args),
            "exit" => return self::process::compile_exit(self, args),
            "pid" => return self::process::compile_pid(self),
            "sleep" => return self::process::compile_sleep(self, args),
            "env" => return self::process::compile_env(self, args),
            "cwd" => return self::process::compile_cwd(self),
            "set_cwd" => return self::process::compile_set_cwd(self, args),
            "exec" => return self::process::compile_exec(self, args),
            "exec_code" => return self::process::compile_exec_code(self, args),
            "exec_lines" => return self::process::compile_exec_lines(self, args),
            "with_exec" => return self::process::compile_with_exec(self, args),
            "with_exec_code" => return self::process::compile_with_exec_code(self, args),
            "with_exec_lines" => return self::process::compile_with_exec_lines(self, args),
            "args" => return self::process::compile_args(self),
            "time_now" => return self::process::compile_time_now(self),
            "time_now_ms" => return self::process::compile_time_now_ms(self),
            "time_add" => return self::process::compile_time_add(self, args),
            "time_diff" => return self::process::compile_time_diff(self, args),
            "format_time" => return self::process::compile_format_time(self, args),
            "format_date_str" => return self::process::compile_format_date_str(self, args),
            "format_time_str" => return self::process::compile_format_time_str(self, args),
            "time_parts" => return self::process::compile_time_parts(self, args),
            "path_exists" => return self::process::compile_path_exists(self, args),
            "path_extension" => return self::process::compile_path_extension(self, args),
            "path_filename" => return self::process::compile_path_filename(self, args),
            "path_parent" => return self::process::compile_path_parent(self, args),
            "path_stem" => return self::process::compile_path_stem(self, args),
            "path_pop" => return self::process::compile_path_pop(self, args),
            "path_join" | "path_push" => return self::process::compile_path_join(self, args),
            "path_set_extension" => return self::process::compile_path_set_extension(self, args),
            "path_is_dir" => return self::process::compile_path_is_dir(self, args),
            "path_is_file" => return self::process::compile_path_is_file(self, args),
            "mkdir" => return self::process::compile_mkdir(self, args),
            "rmdir" => return self::process::compile_rmdir(self, args),
            "move_file" | "rename_file" => return self::process::compile_move_file(self, args),
            "temp_dir" => return self::process::compile_temp_dir(self),
            "file_size" => return self::process::compile_file_size(self, args),
            "file_modified" => return self::process::compile_file_modified(self, args),
            "copy_file" => return self::process::compile_copy_file(self, args),
            "mkdir_all" => return self::process::compile_mkdir_all(self, args),
            "rmdir_all" => return self::process::compile_rmdir_all(self, args),
            "list_dir" => return self::process::compile_list_dir(self, args),
            "rand_int" | "rand_float" | "rand_bool" | "rand_char" | "rand_byte" => return self::random::compile_rand_simple(self, func_name),
            "rand_bool_weighted" => return self::random::compile_rand_bool_weighted(self, args),
            "rand_int_range" | "rand_float_range" | "rand_dice" | "rand_range" => return self::random::compile_rand_range(self, func_name, args),
            "rand_range_step" => return self::random::compile_rand_range_step(self, args),
            "rand_string" => return self::random::compile_rand_string(self, args),
            "rand_dices" | "rand_bytes" | "rand_choice" | "rand_shuffle" => return self::random::compile_rand_collection(self, func_name, args),
            "rand_choices" | "rand_sample" => return self::random::compile_rand_multi(self, func_name, args),
            "term_enter" | "term_leave" | "term_clear" | "term_clear_line" | "term_save_cursor" | "term_restore_cursor" | "term_hide_cursor" | "term_show_cursor" | "term_flush" | "term_reset_color" | "term_bold" | "term_dim" | "term_italic" | "term_underline" | "term_blink" | "term_reverse" | "term_crossed_out" | "term_reset_attr" | "term_enable_wrap" | "term_disable_wrap" | "term_begin_sync" | "term_end_sync" | "term_enable_mouse" | "term_disable_mouse" => return self::terminal::compile_term_no_args(self, func_name),
            "term_move" | "term_set_fg" | "term_set_bg" | "term_fg" | "term_bg" | "term_move_to_col" | "term_move_to_row" | "term_move_up" | "term_move_down" | "term_move_left" | "term_move_right" | "term_next_line" | "term_prev_line" | "term_scroll_up" | "term_scroll_down" | "term_set_size" | "term_poll" => return self::terminal::compile_term_with_args(self, func_name, args),
            "term_set_title" | "term_print" => return self::terminal::compile_term_str(self, func_name, args),
            "term_get_size" => return self::terminal::compile_term_get_size(self),
            "term_read_key" => return self::terminal::compile_term_read_key(self),
            "set_add" | "set_remove" | "set_contains" => return self::collections::compile_set_ops(self, func_name, args),
            "set_to_array" => return self::collections::compile_set_to_array(self, args),
            "map_contains" | "map_remove" | "map_get" => return self::collections::compile_map_ops(self, func_name, args),
            "map_keys" | "map_values" => return self::collections::compile_map_keys_values(self, func_name, args),
            "map_clear" => return self::collections::compile_map_clear(self, args),
            "map_merge" => return self::collections::compile_map_merge(self, args),
            "map_to_array" => return self::collections::compile_map_to_array(self, args),
            "arr_first" | "arr_last" => return self::collections::compile_arr_first_last(self, func_name, args),
            "arr_contains" | "arr_index_of" => return self::collections::compile_arr_contains(self, func_name, args),
            "arr_reverse" => return self::collections::compile_arr_reverse(self, args),
            "arr_concat" => return self::collections::compile_arr_concat(self, args),
            "arr_unique" => return self::collections::compile_arr_unique(self, args),
            "arr_slice" => return self::collections::compile_arr_slice(self, args),
            "arr_fill" => return self::collections::compile_arr_fill(self, args),
            "arr_range" => return self::collections::compile_arr_range(self, args),
            "arr_sum" | "arr_product" | "arr_max" | "arr_min" => return self::collections::compile_arr_agg(self, func_name, args),
            "arr_sort" => return self::collections::compile_arr_sort(self, args),
            "arr_flatten" => return self::collections::compile_arr_flatten(self, args),
            "arr_zip" => return self::collections::compile_arr_zip(self, args),
            "arr_push" | "arr_pop" | "arr_insert" | "arr_remove" => return self::collections::compile_arr_mut(self, func_name, args),
            "arr_filter" | "arr_map" | "arr_find" | "arr_reduce" | "arr_find_index" | "arr_all" | "arr_any" | "arr_for_each" | "arr_flat_map" | "arr_sort_by" => return self::closure::compile_arr_closure(self, func_name, args),
            "result_map" | "result_map_err" => return self::closure::compile_result_closure(self, func_name, args),
            "bench" if args.len() >= 2 => return self::closure::compile_bench(self, args),
            "to_string" | "to_bin" | "to_hex" | "to_oct" => return self::types::compile_to_string_bin_hex_oct(self, func_name, args),
            "to_int" | "to_float" | "to_bool" | "to_byte" | "to_char" => return self::types::compile_to_primitive(self, func_name, args),
            "error_unwrap" => return self::types::compile_error_unwrap(self, args),
            "is_bool" | "is_int" | "is_float" | "is_string" | "is_null" | "is_char" | "is_byte" | "is_error" => return self::types::compile_type_check(self, func_name, args),
            "type_of" => return self::types::compile_type_of(self, args),
            "dbg" => return self::types::compile_dbg(self, args),
            "assert" => return self::assert::compile_assert(self, args),
            "assert_eq" | "assert_ne" | "assert_lt" | "assert_le" | "assert_gt" | "assert_ge" => return self::assert::compile_assert_cmp(self, func_name, args),
            "assert_approx_eq" => return self::assert::compile_assert_approx_eq(self, args),
            "panic" => return self::assert::compile_panic(self, args),
            "unreachable" => return self::assert::compile_unreachable(self),
            "todo" => return self::assert::compile_todo(self),
            "tcp_listen" if self.std_net_imports.contains("tcp_listen") => return self::network::compile_tcp_listen(self, args),
            "tcp_accept" if self.std_net_imports.contains("tcp_accept") => return self::network::compile_tcp_accept(self, args),
            "tcp_connect" if self.std_net_imports.contains("tcp_connect") => return self::network::compile_tcp_connect(self, args),
            "tcp_read" if self.std_net_imports.contains("tcp_read") => return self::network::compile_tcp_read(self, args),
            "tcp_write" if self.std_net_imports.contains("tcp_write") => return self::network::compile_tcp_write(self, args),
            "tcp_peer_addr" if self.std_net_imports.contains("tcp_peer_addr") => return self::network::compile_tcp_peer_addr(self, args),
            "tcp_local_addr" if self.std_net_imports.contains("tcp_local_addr") => return self::network::compile_tcp_local_addr(self, args),
            "tcp_set_timeout" if self.std_net_imports.contains("tcp_set_timeout") => return self::network::compile_tcp_set_timeout(self, args),
            "tcp_set_nonblocking" if self.std_net_imports.contains("tcp_set_nonblocking") => return self::network::compile_tcp_set_nonblocking(self, args),
            "tcp_shutdown" if self.std_net_imports.contains("tcp_shutdown") => return self::network::compile_tcp_shutdown(self, args),
            "tcp_close" if self.std_net_imports.contains("tcp_close") => return self::network::compile_tcp_close(self, args),
            "udp_bind" if self.std_net_imports.contains("udp_bind") => return self::network::compile_udp_bind(self, args),
            "udp_connect" if self.std_net_imports.contains("udp_connect") => return self::network::compile_udp_connect(self, args),
            "udp_send" if self.std_net_imports.contains("udp_send") => return self::network::compile_udp_send(self, args),
            "udp_send_to" if self.std_net_imports.contains("udp_send_to") => return self::network::compile_udp_send_to(self, args),
            "udp_recv" if self.std_net_imports.contains("udp_recv") => return self::network::compile_udp_recv(self, args),
            "udp_recv_from" if self.std_net_imports.contains("udp_recv_from") => return self::network::compile_udp_recv_from(self, args),
            "udp_close" if self.std_net_imports.contains("udp_close") => return self::network::compile_udp_close(self, args),
            "resolve" if self.std_net_imports.contains("resolve") => return self::network::compile_resolve(self, args),
            "http_server_start" if self.std_http_imports.contains("http_server_start") => return self::http::compile_server_start(self, args),
            "http_server_recv" if self.std_http_imports.contains("http_server_recv") => return self::http::compile_server_recv(self, args),
            "http_server_try_recv" if self.std_http_imports.contains("http_server_try_recv") => return self::http::compile_server_try_recv(self, args),
            "http_server_stop" if self.std_http_imports.contains("http_server_stop") => return self::http::compile_server_stop(self, args),
            "http_request_method" if self.std_http_imports.contains("http_request_method") => return self::http::compile_request_method(self, args),
            "http_request_url" if self.std_http_imports.contains("http_request_url") => return self::http::compile_request_url(self, args),
            "http_request_header" if self.std_http_imports.contains("http_request_header") => return self::http::compile_request_header(self, args),
            "http_request_body" if self.std_http_imports.contains("http_request_body") => return self::http::compile_request_body(self, args),
            "http_respond" if self.std_http_imports.contains("http_respond") => return self::http::compile_respond(self, args),
            "http_get" if self.std_http_imports.contains("http_get") => return self::http::compile_get(self, args),
            "http_post" if self.std_http_imports.contains("http_post") => return self::http::compile_post(self, args),
            "http_request" if self.std_http_imports.contains("http_request") => return self::http::compile_request(self, args),
            "compile" if self.std_c_imports.contains("compile") => return self::c_ffi::compile_c_compile(self, args),
            "load" if self.std_c_imports.contains("load") => return self::c_ffi::compile_c_load(self, args),
            "has_symbol" if self.std_c_imports.contains("has_symbol") => return self::c_ffi::compile_c_has_symbol(self, args),
            "close" if self.std_c_imports.contains("close") => return self::c_ffi::compile_c_close(self, args),
            "clear_cache" if self.std_c_imports.contains("clear_cache") => return self::c_ffi::compile_c_clear_cache(self),
            "call" if self.std_c_imports.contains("call") => return self::c_ffi::compile_c_call(self, args),
            _ => {}
        }

        let c_name = mangle(&path.join("_"));
        self.writer.write(&format!("{}(", c_name));
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                self.writer.write(", ");
            }
            self.compile_expr(*arg)?;
        }
        self.writer.write(")");
        Ok(())
    }

    pub fn compile_method_call(
        &mut self,
        caller: ExprId,
        method: &[String],
        args: &[ExprId],
    ) -> Result<(), Error> {
        let method_name = method.first().map(|s| s.as_str()).unwrap_or("");

        match method_name {
            "len" => {
                self.writer.write("rl_str_len(");
                self.compile_expr(caller)?;
                self.writer.write(")");
                return Ok(());
            }
            "println" => {
                self.writer.write("rl_println(");
                self.compile_expr(caller)?;
                self.writer.write(")");
                return Ok(());
            }
            "print" => {
                self.writer.write("rl_print(");
                self.compile_expr(caller)?;
                self.writer.write(")");
                return Ok(());
            }
            _ => {}
        }

        let caller_expr = self.ast.exprs.get(caller);
        if let ExpressionKind::ResolvedIdentifier { name, .. } = &caller_expr.kind
            && let Some(ta) = self.var_types.get(name)
                && let TypeAnnotation::Record(rname) | TypeAnnotation::CRecord(rname) = ta {
                    let c_name = self.lookup(name);
                    let c_fn = format!("impl_{}_{}", rname, method_name);
                    self.writer.write(&format!("{}({}", c_fn, c_name));
                    for arg in args.iter() {
                        self.writer.write(", ");
                        self.compile_expr(*arg)?;
                    }
                    self.writer.write(")");
                    return Ok(());
                }

        self.compile_expr(caller)?;
        self.writer.write(&format!(".{}(", method_name));
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                self.writer.write(", ");
            }
            self.compile_expr(*arg)?;
        }
        self.writer.write(")");
        Ok(())
    }

    fn compile_lambda(
        &mut self,
        params: &[rl_ast::statements::Param],
        return_type: &Option<rl_ast::statements::TypeAnnotation>,
        body: &[rl_ast::statements::Statement],
    ) -> Result<(), Error> {
        let lambda_id = self.lambda_counter;
        self.lambda_counter += 1;
        let fn_name = format!("_rl_lambda_{}", lambda_id);

        let mut captured_names: Vec<String> = Vec::new();
        let param_names: std::collections::HashSet<String> =
            params.iter().map(|p| p.param_name.clone()).collect();
        self.collect_captures_from_statements(body, &param_names, &mut captured_names);
        captured_names.sort();
        captured_names.dedup();

        let _c_ret = match return_type {
            Some(ta) => type_to_c(ta),
            None => "rl_result".to_string(),
        };

        let mut func_code = String::new();
        func_code.push_str(&format!("static rl_result {}(rl_closure *_self, rl_result *_args, uint64_t _argc) {{\n", fn_name));

        for (i, p) in params.iter().enumerate() {
            let c_type = type_to_c(&p.param_type);
            let c_name = mangle(&p.param_name);
            func_code.push_str(&format!("    {} {} = ", c_type, c_name));
            match &p.param_type {
                TypeAnnotation::Int | TypeAnnotation::CInt => {
                    func_code.push_str(&format!("rl_unwrap_i64(_args[{}]);\n", i));
                }
                TypeAnnotation::Float | TypeAnnotation::CFloat => {
                    func_code.push_str(&format!("rl_unwrap_f64(_args[{}]);\n", i));
                }
                TypeAnnotation::Bool | TypeAnnotation::CBool => {
                    func_code.push_str(&format!("rl_unwrap_bool(_args[{}]);\n", i));
                }
                TypeAnnotation::String | TypeAnnotation::CString => {
                    func_code.push_str(&format!("rl_unwrap_str(_args[{}]);\n", i));
                }
                _ => {
                    func_code.push_str(&format!("rl_unwrap_i64(_args[{}]);\n", i));
                }
            }
        }

        for (i, name) in captured_names.iter().enumerate() {
            let c_name = mangle(name);
            let c_type = self.var_types.get(name).map(type_to_c).unwrap_or_else(|| "int64_t".to_string());
            func_code.push_str(&format!("    {} {} = ", c_type, c_name));
            match self.var_types.get(name) {
                Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => {
                    func_code.push_str(&format!("rl_unwrap_i64(_self->captures[{}]);\n", i));
                }
                Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => {
                    func_code.push_str(&format!("rl_unwrap_f64(_self->captures[{}]);\n", i));
                }
                Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => {
                    func_code.push_str(&format!("rl_unwrap_bool(_self->captures[{}]);\n", i));
                }
                Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => {
                    func_code.push_str(&format!("rl_unwrap_str(_self->captures[{}]);\n", i));
                }
                Some(TypeAnnotation::Array(_)) | Some(TypeAnnotation::CArray(_)) => {
                    func_code.push_str(&format!("rl_unwrap_arr(_self->captures[{}]);\n", i));
                }
                _ => {
                    func_code.push_str(&format!("rl_unwrap_i64(_self->captures[{}]);\n", i));
                }
            }
        }

        for s in body {
            self.compile_lambda_statement(s, &mut func_code)?;
        }

        func_code.push_str("    return rl_ok_null();\n");
        func_code.push_str("}\n\n");

        self.static_funcs.push(func_code);

        let mut captures_code = String::new();
        for (i, name) in captured_names.iter().enumerate() {
            if i > 0 { captures_code.push_str(", "); }
            let c_name = self.lookup(name);
            let c_type = self.var_types.get(name).cloned().unwrap_or(TypeAnnotation::Int);
            match c_type {
                TypeAnnotation::Int | TypeAnnotation::CInt => {
                    captures_code.push_str(&format!("rl_ok_i64({})", c_name));
                }
                TypeAnnotation::Float | TypeAnnotation::CFloat => {
                    captures_code.push_str(&format!("rl_ok_f64({})", c_name));
                }
                TypeAnnotation::Bool | TypeAnnotation::CBool => {
                    captures_code.push_str(&format!("rl_ok_bool({})", c_name));
                }
                TypeAnnotation::String | TypeAnnotation::CString => {
                    captures_code.push_str(&format!("rl_ok_str({})", c_name));
                }
                TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                    captures_code.push_str(&format!("rl_ok_arr({})", c_name));
                }
                _ => {
                    captures_code.push_str(&format!("rl_ok_i64({})", c_name));
                }
            }
        }

        let capture_count = captured_names.len();
        self.writer.write(&format!(
            "rl_closure_new({}, (rl_result[]){{ {} }}, {})",
            fn_name, captures_code, capture_count
        ));

        Ok(())
    }

    fn compile_lambda_statement(
        &mut self,
        stmt: &rl_ast::statements::Statement,
        func_code: &mut String,
    ) -> Result<(), Error> {
        use rl_ast::statements::StatementKind;
        match &stmt.kind {
            StatementKind::Return(Some(expr_id)) => {
                let expr = self.ast.exprs.get(*expr_id);
                if let ExpressionKind::Propagate(inner) = &expr.kind {
                    let temp = self.temp_var();
                    func_code.push_str(&format!("    rl_result {} = ", temp));
                    self.compile_expr_to_string(*inner, func_code)?;
                    func_code.push_str(";\n");
                    func_code.push_str(&format!("    if (!{}.is_ok) {{ return {}; }}\n", temp, temp));
                    func_code.push_str(&format!("    return {};\n", temp));
                } else {
                    func_code.push_str("    return rl_ok(");
                    self.compile_expr_to_string(*expr_id, func_code)?;
                    func_code.push_str(");\n");
                }
            }
            StatementKind::ResolvedVariableDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_type = type_to_c(type_annotation);
                let c_name = mangle(name);
                func_code.push_str(&format!("    {} {} = ", c_type, c_name));
                self.compile_expr_to_string(*value, func_code)?;
                func_code.push_str(";\n");
            }
            StatementKind::ResolvedConstantDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_type = type_to_c(type_annotation);
                let c_name = mangle(name);
                func_code.push_str(&format!("    const {} {} = ", c_type, c_name));
                self.compile_expr_to_string(*value, func_code)?;
                func_code.push_str(";\n");
            }
            StatementKind::Expression(expr_id) => {
                func_code.push_str("    ");
                self.compile_expr_to_string(*expr_id, func_code)?;
                func_code.push_str(";\n");
            }
            StatementKind::Conditional { if_branch, else_branch } => {
                if let StatementKind::ConditionalBranch { condition, body, .. } = &if_branch.kind {
                    func_code.push_str("    if (");
                    if let Some(cond) = condition {
                        self.compile_expr_to_string(*cond, func_code)?;
                    } else {
                        func_code.push('1');
                    }
                    func_code.push_str(") {\n");
                    for s in body {
                        self.compile_lambda_statement(s, func_code)?;
                    }
                }
                if let Some(else_b) = else_branch
                    && let StatementKind::ConditionalBranch { condition, body, .. } = &else_b.kind {
                        if condition.is_some() {
                            func_code.push_str("    } else if (");
                            self.compile_expr_to_string(condition.unwrap(), func_code)?;
                            func_code.push_str(") {\n");
                        } else {
                            func_code.push_str("    } else {\n");
                        }
                        for s in body {
                            self.compile_lambda_statement(s, func_code)?;
                        }
                    }
                func_code.push_str("    }\n");
            }
            StatementKind::ResolvedForRange {
                variable,
                range,
                body,
                ..
            } => {
                let items = match &range.kind {
                    StatementKind::Range(items) => items.clone(),
                    _ => vec![],
                };
                if !items.is_empty() {
                    let first = items[0];
                    let last = items[items.len() - 1];
                    let c_name = mangle(variable);
                    func_code.push_str(&format!(
                        "    for (int64_t {} = {}; {} < {}; {}++) {{\n",
                        c_name, first, c_name, last + 1, c_name
                    ));
                    for s in body {
                        self.compile_lambda_statement(s, func_code)?;
                    }
                    func_code.push_str("    }\n");
                }
            }
            StatementKind::Loop(body) => {
                func_code.push_str("    while (1) {\n");
                for s in body {
                    self.compile_lambda_statement(s, func_code)?;
                }
                func_code.push_str("    }\n");
            }
            _ => {
                func_code.push_str("    /* unhandled statement in lambda */\n");
            }
        }
        Ok(())
    }

    fn compile_expr_to_string(&mut self, id: ExprId, output: &mut String) -> Result<(), Error> {
        let old_source = std::mem::take(&mut self.writer);
        self.writer = CWriter::new();
        self.compile_expr(id)?;
        let generated = self.writer.source().to_string();
        self.writer = old_source;
        output.push_str(&generated);
        Ok(())
    }

    fn collect_captures_from_statements(
        &self,
        stmts: &[rl_ast::statements::Statement],
        param_names: &std::collections::HashSet<String>,
        captured: &mut Vec<String>,
    ) {
        use rl_ast::statements::StatementKind;
        for stmt in stmts {
            match &stmt.kind {
                StatementKind::ResolvedVariableDeclaration { name: _, value, .. } => {
                    self.collect_captures_from_expr(*value, param_names, captured);
                }
                StatementKind::ResolvedConstantDeclaration { name: _, value, .. } => {
                    self.collect_captures_from_expr(*value, param_names, captured);
                }
                StatementKind::Expression(expr_id) => {
                    self.collect_captures_from_expr(*expr_id, param_names, captured);
                }
                StatementKind::Return(Some(expr_id)) => {
                    self.collect_captures_from_expr(*expr_id, param_names, captured);
                }
                StatementKind::Conditional { if_branch, else_branch } => {
                    if let StatementKind::ConditionalBranch { condition, body, .. } = &if_branch.kind {
                        if let Some(cond) = condition {
                            self.collect_captures_from_expr(*cond, param_names, captured);
                        }
                        self.collect_captures_from_statements(body, param_names, captured);
                    }
                    if let Some(else_b) = else_branch
                        && let StatementKind::ConditionalBranch { condition, body, .. } = &else_b.kind {
                            if let Some(cond) = condition {
                                self.collect_captures_from_expr(*cond, param_names, captured);
                            }
                            self.collect_captures_from_statements(body, param_names, captured);
                        }
                }
                StatementKind::ResolvedForRange { body, range, .. } => {
                    self.collect_captures_from_statements(body, param_names, captured);
                    if let StatementKind::Range(_items) = &range.kind {}
                }
                StatementKind::Loop(body) => {
                    self.collect_captures_from_statements(body, param_names, captured);
                }
                _ => {}
            }
        }
    }

    fn collect_captures_from_expr(
        &self,
        id: ExprId,
        param_names: &std::collections::HashSet<String>,
        captured: &mut Vec<String>,
    ) {
        let kind = self.ast.exprs.get(id).kind.clone();
        match &kind {
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                if !param_names.contains(name) {
                    if (self.var_types.contains_key(name) || self.scopes.iter().rev().any(|s| s.contains_key(name)))
                        && !captured.contains(name) {
                            captured.push(name.clone());
                        }
                }
            }
            ExpressionKind::Binary { left, right, .. } => {
                self.collect_captures_from_expr(*left, param_names, captured);
                self.collect_captures_from_expr(*right, param_names, captured);
            }
            ExpressionKind::Unary { operand, .. } => {
                self.collect_captures_from_expr(*operand, param_names, captured);
            }
            ExpressionKind::Call { args, .. } => {
                for arg in args {
                    self.collect_captures_from_expr(*arg, param_names, captured);
                }
            }
            ExpressionKind::CallExpr { callee, args } => {
                self.collect_captures_from_expr(*callee, param_names, captured);
                for arg in args {
                    self.collect_captures_from_expr(*arg, param_names, captured);
                }
            }
            ExpressionKind::MethodCall { caller, args, .. } => {
                self.collect_captures_from_expr(*caller, param_names, captured);
                for arg in args {
                    self.collect_captures_from_expr(*arg, param_names, captured);
                }
            }
            ExpressionKind::ArrayLiteral(elems) => {
                for elem in elems {
                    self.collect_captures_from_expr(*elem, param_names, captured);
                }
            }
            ExpressionKind::MapLiteral(entries) => {
                for (_, v) in entries {
                    self.collect_captures_from_expr(*v, param_names, captured);
                }
            }
            ExpressionKind::SetLiteral(items) => {
                for item in items {
                    self.collect_captures_from_expr(*item, param_names, captured);
                }
            }
            ExpressionKind::TupleLiteral(elems) => {
                for elem in elems {
                    self.collect_captures_from_expr(*elem, param_names, captured);
                }
            }
            ExpressionKind::StructLiteral { fields, .. } => {
                for (_, v) in fields {
                    self.collect_captures_from_expr(*v, param_names, captured);
                }
            }
            ExpressionKind::Index { target, index } => {
                self.collect_captures_from_expr(*target, param_names, captured);
                self.collect_captures_from_expr(*index, param_names, captured);
            }
            ExpressionKind::IndexAssign { target, index, value } => {
                self.collect_captures_from_expr(*target, param_names, captured);
                self.collect_captures_from_expr(*index, param_names, captured);
                self.collect_captures_from_expr(*value, param_names, captured);
            }
            ExpressionKind::FieldAccess { target, .. } => {
                self.collect_captures_from_expr(*target, param_names, captured);
            }
            ExpressionKind::FieldAssign { target, field: _, value } => {
                self.collect_captures_from_expr(*target, param_names, captured);
                self.collect_captures_from_expr(*value, param_names, captured);
            }
            ExpressionKind::Grouping(inner) => {
                self.collect_captures_from_expr(*inner, param_names, captured);
            }
            ExpressionKind::OkLiteral(inner) | ExpressionKind::ErrLiteral(inner) | ExpressionKind::ErrorLiteral(inner) => {
                self.collect_captures_from_expr(*inner, param_names, captured);
            }
            ExpressionKind::Propagate(inner) => {
                self.collect_captures_from_expr(*inner, param_names, captured);
            }
            ExpressionKind::Cast { value, .. } => {
                self.collect_captures_from_expr(*value, param_names, captured);
            }
            ExpressionKind::ResolvedAssign { value, .. } => {
                self.collect_captures_from_expr(*value, param_names, captured);
            }
            _ => {}
        }
    }

    pub fn write_arg_as_result(&mut self, id: ExprId) -> Result<(), Error> {
        let kind = self.ast.exprs.get(id).kind.clone();
        match &kind {
            ExpressionKind::Integer(v) => {
                self.writer.write(&format!("rl_ok_i64((int64_t){})", v));
            }
            ExpressionKind::Float(v) => {
                self.writer.write(&format!("rl_ok_f64((double){})", v));
            }
            ExpressionKind::Bool(v) => {
                self.writer.write(if *v { "rl_ok_bool(true)" } else { "rl_ok_bool(false)" });
            }
            ExpressionKind::String(v) => {
                let escaped = escape_c_string(v);
                self.writer.write(&format!("rl_ok_str(rl_str_literal(\"{}\", {}))", escaped, escaped.len()));
            }
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                if let Some(ta) = self.var_types.get(name) {
                    let c_name = self.lookup(name);
                    match ta {
                        TypeAnnotation::Int | TypeAnnotation::CInt => {
                            self.writer.write(&format!("rl_ok_i64({})", c_name));
                        }
                        TypeAnnotation::Float | TypeAnnotation::CFloat => {
                            self.writer.write(&format!("rl_ok_f64({})", c_name));
                        }
                        TypeAnnotation::Bool | TypeAnnotation::CBool => {
                            self.writer.write(&format!("rl_ok_bool({})", c_name));
                        }
                        TypeAnnotation::String | TypeAnnotation::CString => {
                            self.writer.write(&format!("rl_ok_str({})", c_name));
                        }
                        TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                            self.writer.write(&format!("rl_ok_arr({})", c_name));
                        }
                        _ => {
                            self.writer.write(&format!("rl_ok_i64({})", c_name));
                        }
                    }
                } else {
                    self.compile_expr(id)?;
                }
            }
            _ => {
                self.writer.write("rl_ok(");
                self.compile_expr(id)?;
                self.writer.write(")");
            }
        }
        Ok(())
    }

    pub fn infer_expr_type(&self, kind: &ExpressionKind) -> TypeAnnotation {
        match kind {
            ExpressionKind::Integer(_) => TypeAnnotation::Int,
            ExpressionKind::Float(_) => TypeAnnotation::Float,
            ExpressionKind::Bool(_) => TypeAnnotation::Bool,
            ExpressionKind::String(_) => TypeAnnotation::String,
            ExpressionKind::Character(_) => TypeAnnotation::Char,
            ExpressionKind::ArrayLiteral(elems) if !elems.is_empty() => TypeAnnotation::Array(Box::new(TypeAnnotation::Infer)),
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                self.var_types.get(name).cloned().unwrap_or(TypeAnnotation::Int)
            }
            _ => TypeAnnotation::Int,
        }
    }
}
