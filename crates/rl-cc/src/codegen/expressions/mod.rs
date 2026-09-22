mod assert;
mod audio;
mod c_ffi;
mod cli;
mod closure;
mod collections;
mod crypto;
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
use rl_utils::errors::{Error, Reason};
use rl_utils::span::Span;

impl<'a> CCodegen<'a> {
    pub fn compile_expr(&mut self, id: ExprId) -> Result<(), Error> {
        // A statement-shaped literal hoisted before this statement
        // reuses its temp instead of emitting statements mid-expression.
        if let Some(temp) = self.hoisted_tmps.get(&id).cloned() {
            self.writer.write(&temp);
            return Ok(());
        }
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
                // Decoded length is the original byte length: escapes only
                // change the source spelling, never the byte count.
                self.writer
                    .write(&format!("rl_str_literal(\"{}\", {})", escaped, v.len()));
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
                // String equality compares contents; C `==` cannot
                // compare string structs.
                if matches!(
                    operator,
                    TokenType::Compare | TokenType::BangEqual
                ) && Self::is_string_valued(self, *left)
                    && Self::is_string_valued(self, *right)
                {
                    if matches!(operator, TokenType::BangEqual) {
                        self.writer.write("!rl_str_eq(");
                    } else {
                        self.writer.write("rl_str_eq(");
                    }
                    self.compile_expr(*left)?;
                    self.writer.write(", ");
                    self.compile_expr(*right)?;
                    self.writer.write(")");
                    return Ok(());
                }
                // Comparisons and logic yield RL bools: cast to C `bool`
                // so generic printing says `true`, not `1`.
                let is_bool_op = matches!(
                    operator,
                    TokenType::Compare
                        | TokenType::BangEqual
                        | TokenType::Less
                        | TokenType::LessEqual
                        | TokenType::Greater
                        | TokenType::GreaterEqual
                        | TokenType::And
                        | TokenType::Or
                );
                if is_bool_op {
                    self.writer.write("(bool)(");
                }
                self.compile_expr(*left)?;
                self.writer.write(&format!(" {} ", token_to_c_op(operator)?));
                self.compile_expr(*right)?;
                if is_bool_op {
                    self.writer.write(")");
                }
            }
            ExpressionKind::Unary { operator, operand } => {
                match operator {
                    TokenType::Minus => self.writer.write("-"),
                    // Parenthesized bool: `!(a == b)` keeps grouping and
                    // yields an RL bool for printing.
                    TokenType::Bang => self.writer.write("(bool)(!("),
                    _ => {
                        return Err(Error::at(
                            Reason::Compile,
                            format!("unsupported unary operator {:?}", operator),
                            Span::dummy(),
                        ));
                    }
                }
                self.compile_expr(*operand)?;
                if matches!(operator, TokenType::Bang) {
                    self.writer.write("))");
                }
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
                // Immediately-invoked lambda: call the literal through the
                // closure machinery like the VM does.
                if matches!(&callee_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                    self.writer.write("rl_closure_call(");
                    self.compile_expr(*callee)?;
                    self.writer.write(", (rl_result[]){ ");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        self.write_arg_as_result(*arg)?;
                    }
                    self.writer.write(&format!(" }}, {})", args.len()));
                    return Ok(());
                }
                let (is_closure, callee_name) = if let ExpressionKind::ResolvedIdentifier { name, .. } = &callee_expr.kind {
                    let is_fn = matches!(self.var_types.get(name), Some(TypeAnnotation::Fn) | Some(TypeAnnotation::Callback(_, _)));
                    (is_fn, Some(name.clone()))
                } else {
                    (false, None)
                };

                // Aliased or bare imported name (`sine(1.0)` for
                // `get sin as sine from std::math`): rewrite to the
                // canonical stdlib path. User functions keep the direct
                // C call below, as do plain variables shadowing a stdlib
                // name (`dec args = args()`).
                let is_variable = callee_name.as_ref().is_some_and(|name| {
                    self.var_types.contains_key(name)
                        || self.scopes.iter().any(|s| s.contains_key(name))
                });
                if !is_closure
                    && !is_variable
                    && let Some(name) = &callee_name
                    && !self.user_fns.contains(name)
                    && let Some((namespace, original)) = self.resolve_std_name(name) {
                        let mut path: Vec<String> =
                            namespace.split("::").map(|s| s.to_string()).collect();
                        path.push(original);
                        self.compile_func_call(&path, args)?;
                        return Ok(());
                    }

                // Dynamically-typed callee (e.g. `dec add7 = mk(7)` without
                // a `fn` annotation stores an opaque result): unbox and
                // call, aborting loudly when it holds no closure. Goes
                // beyond the VM, which fails these calls at runtime.
                let is_boxed_callee = callee_name.as_ref().is_some_and(|name| {
                    matches!(
                        self.var_types.get(name.as_str()),
                        Some(TypeAnnotation::Result(_))
                            | Some(TypeAnnotation::CResult(_))
                            | Some(TypeAnnotation::Infer)
                            | Some(TypeAnnotation::Generic(_))
                    )
                });
                if !is_closure && is_boxed_callee {
                    self.writer.write("rl_closure_call_checked(");
                    self.compile_expr(*callee)?;
                    self.writer.write(", (rl_result[]){ ");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        self.write_arg_as_result(*arg)?;
                    }
                    self.writer.write(&format!(" }}, {})", args.len()));
                    return Ok(());
                }

                if is_closure {
                    let return_type = callee_name.as_ref().and_then(|n| self.closure_return_types.get(n));
                    // Unknown (unannotated) stays wrapped; only known
                    // non-result returns unwrap.
                    let need_unwrap = !matches!(
                        return_type,
                        None | Some(TypeAnnotation::Result(_)) | Some(TypeAnnotation::CResult(_))
                    );
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
                // String messages wrap with rl_err_msg, codes with rl_err.
                let is_string = matches!(
                    self.inferred_expr_type(*inner).as_ref(),
                    Some(TypeAnnotation::String) | Some(TypeAnnotation::CString)
                );
                self.writer.write(if is_string {
                    "rl_err_msg("
                } else {
                    "rl_err("
                });
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::ErrorLiteral(inner) => {
                self.writer.write("rl_error(");
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::Propagate(inner) => {
                // `?` in expression position (conditions, indexes, call
                // arguments, nested operands): unwrap inline, aborting on
                // error like the checked unwraps do. Statement-level `?`
                // (declarations, bare `?;`, `return ?`) keeps precise
                // early-return propagation in its own arms above.
                // Tuple payloads travel as one element arrays.
                if let Some(fields) = self.tuple_payload_fields(*inner) {
                    let tname = self.ensure_tuple_type(fields);
                    self.writer.write(&format!("(({0}*)rl_result_unwrap_arr(", tname));
                    self.compile_expr(*inner)?;
                    self.writer.write(").data)[0]");
                    return Ok(());
                }
                let unwrap_fn = self.unwrap_fn_for_result(*inner);
                self.writer.write(&format!("{unwrap_fn}("));
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::ArrayLiteral(elems) => {
                // Element type: first element wins, else the contextual
                // hint (e.g. `dec arr[string] x = []`), else int64. The
                // payload tag travels along so later ops dispatch right.
                // Tuple elements use the specific struct name so mixed
                // layouts (e.g. headers vs responses) do not collide.
                let mut elem_ta = elems
                    .first()
                    .and_then(|e| self.inferred_expr_type(*e))
                    .or_else(|| self.array_elem_hint.clone())
                    .unwrap_or(TypeAnnotation::Int);
                if Self::needs_inference(&elem_ta) {
                    elem_ta = TypeAnnotation::Int;
                }
                let c_elem = match &elem_ta {
                    TypeAnnotation::Tuple(fields) | TypeAnnotation::CTuple(fields) => {
                        self.ensure_tuple_type(fields.as_ref().clone())
                    }
                    _ => type_to_c(&elem_ta),
                };
                let tag = Self::array_elem_tag_from_type(&elem_ta);
                // Map/set elements are statement-shaped, which cannot sit
                // inside the `&(...[]){...}` initializer below. Hoist each
                // into a temp first (valid at statement level, where array
                // literals overwhelmingly appear; nested-in-expression
                // arrays of literals stay unsupported, as before).
                let mut hoisted: Vec<Option<String>> = Vec::with_capacity(elems.len());
                for elem in elems.iter() {
                    // A declaration-level pre-pass may already have built
                    // this element; otherwise hoist it now.
                    if let Some(temp) = self.hoisted_tmps.get(elem).cloned() {
                        hoisted.push(Some(temp));
                        continue;
                    }
                    let kind = self.ast.exprs.get(*elem).kind.clone();
                    match kind {
                        ExpressionKind::MapLiteral(entries) => {
                            hoisted.push(Some(self.emit_map_lit(&entries)?));
                        }
                        ExpressionKind::SetLiteral(items) => {
                            hoisted.push(Some(self.emit_set_lit(&items)?));
                        }
                        _ => hoisted.push(None),
                    }
                }
                self.writer.write(&format!("rl_arr_from_vals_tag(&({}[]){{", c_elem));
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    if let Some(temp) = &hoisted[i] {
                        self.writer.write(temp);
                    } else {
                        self.compile_expr(*elem)?;
                    }
                }
                self.writer.write(&format!("}}, {}, (int32_t)sizeof({}), {})", elems.len(), c_elem, tag));
            }
            ExpressionKind::MapLiteral(entries) => {
                let temp = self.emit_map_lit(&entries)?;
                self.writer.write(&temp);
            }
            ExpressionKind::SetLiteral(items) => {
                let temp = self.emit_set_lit(&items)?;
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
                    // Hint empty literals (`x = []`) with the element type.
                    let saved_hint = self.array_elem_hint.clone();
                    if let Some(TypeAnnotation::Array(elem))
                    | Some(TypeAnnotation::CArray(elem)) =
                        self.var_types.get(name).cloned()
                    {
                        self.array_elem_hint = Some(*elem);
                    }
                    self.compile_expr(*value)?;
                    self.array_elem_hint = saved_hint;
                }
            }
            ExpressionKind::Index { target, index } => {
                // Arrays index through the data buffer with the element
                // type; tuples use `.field_N` for literal indexes; maps
                // look up string keys. Anything else is a compile error
                // instead of an invalid C subscript.
                match self.inferred_expr_type(*target).as_ref() {
                    Some(TypeAnnotation::Array(inner))
                    | Some(TypeAnnotation::CArray(inner)) => {
                        let c_type = type_to_c(&inner);
                        self.writer.write(&format!("(({}*)(", c_type));
                        self.compile_expr(*target)?;
                        self.writer.write(").data)[");
                        self.compile_expr(*index)?;
                        self.writer.write("]");
                    }
                    Some(TypeAnnotation::Tuple(_)) | Some(TypeAnnotation::CTuple(_)) => {
                        let index_expr = self.ast.exprs.get(*index);
                        if let ExpressionKind::Integer(n) = &index_expr.kind {
                            self.writer.write("(");
                            self.compile_expr(*target)?;
                            self.writer.write(&format!(").field_{}", n));
                        } else {
                            return Err(Error::at(
                                Reason::Compile,
                                "tuple index must be an integer literal",
                                Span::dummy(),
                            ));
                        }
                    }
                    Some(TypeAnnotation::Map(_, vt)) | Some(TypeAnnotation::CMap(_, vt)) => {
                        // Checked lookup aborts on a missing key, like the VM.
                        let unwrap = match vt.as_ref() {
                            TypeAnnotation::String | TypeAnnotation::CString => {
                                "rl_result_unwrap_str"
                            }
                            TypeAnnotation::Float | TypeAnnotation::CFloat => {
                                "rl_result_unwrap_f64"
                            }
                            TypeAnnotation::Bool | TypeAnnotation::CBool => {
                                "rl_result_unwrap_bool"
                            }
                            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                                "rl_result_unwrap_arr"
                            }
                            TypeAnnotation::Map(_, _)
                            | TypeAnnotation::CMap(_, _) => "rl_result_unwrap_map",
                            TypeAnnotation::Set(_) | TypeAnnotation::CSet(_) => {
                                "rl_result_unwrap_set"
                            }
                            _ => "rl_result_unwrap_i64",
                        };
                        self.writer.write(&format!("{}(rl_map_get_s(", unwrap));
                        self.compile_expr(*target)?;
                        self.writer.write(", ");
                        self.compile_expr(*index)?;
                        self.writer.write("))");
                    }
                    _ => {
                        return Err(Error::at(
                            Reason::Compile,
                            "indexing a value that is not an array, tuple or map",
                            Span::dummy(),
                        ));
                    }
                }
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
            other => {
                return Err(Error::at(
                    Reason::Compile,
                    format!("expression kind not supported by the C transpiler: {:?}", other),
                    Span::dummy(),
                ));
            }
        }
        Ok(())
    }

    pub fn compile_func_call(&mut self, path: &[String], args: &[ExprId]) -> Result<(), Error> {
        let func_name = path.last().map(|s| s.as_str()).unwrap_or("");
        let _is_stdlib = path.first().map(|s| s.as_str()) == Some("std");

        // A user function shadows any stdlib arm on a bare call.
        if path.len() == 1 && self.user_fns.contains(func_name) {
            return self.compile_user_call(func_name, args);
        }

        // Aliased import on a bare call (`sine(1.0)` for
        // `get sin as sine from std::math`): redispatch on the canonical
        // path. Only single-segment paths rewrite, so this terminates.
        if path.len() == 1
            && let Some((namespace, original)) = self.resolve_std_name(func_name)
                && original != func_name {
                    let mut canonical: Vec<String> =
                        namespace.split("::").map(|s| s.to_string()).collect();
                    canonical.push(original);
                    return self.compile_func_call(&canonical, args);
                }

        // Namespaces with no C backend: fail loudly at transpile time
        // instead of emitting a dangling C call.
        if path.len() >= 2 && path[0] == "std" {
            match path[1].as_str() {
                "gui" => {
                    return Err(Error::at(
                        Reason::Compile,
                        format!(
                            "std::gui::{} is not supported by the C transpiler",
                            func_name
                        ),
                        Span::dummy(),
                    ));
                }
                "rl" => match func_name {
                    "rl_version" => {
                        self.writer.write(&format!(
                            "rl_str_literal(\"{}\", {})",
                            env!("CARGO_PKG_VERSION"),
                            env!("CARGO_PKG_VERSION").len()
                        ));
                        return Ok(());
                    }
                    // No source file exists inside a transpiled binary.
                    "source_name" => {
                        self.writer.write("rl_ok_null()");
                        return Ok(());
                    }
                    _ => {
                        return Err(Error::at(
                            Reason::Compile,
                            format!(
                                "std::rl::{} needs the compiler pipeline and cannot run in transpiled programs",
                                func_name
                            ),
                            Span::dummy(),
                        ));
                    }
                },
                _ => {}
            }
        }

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
            "isatty" => return self::io::compile_isatty(self),
            "read_all_stdin" => return self::io::compile_read_all_stdin(self),
            "decode_utf8" => return self::io::compile_decode_utf8(self, args),
            "encode_utf8" => return self::io::compile_encode_utf8(self, args),
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
            "strip_prefix" => return self::string::compile_strip_prefix(self, args),
            "strip_suffix" => return self::string::compile_strip_suffix(self, args),
            "last_index_of" => return self::string::compile_last_index_of(self, args),
            "split_once" => return self::string::compile_split_once(self, args),
            "lines" => return self::string::compile_lines(self, args),
            "wrap" => return self::string::compile_wrap(self, args),
            "indent" => return self::string::compile_indent(self, args),
            "dedent" => return self::string::compile_dedent(self, args),
            "diff_lines" => return self::string::compile_diff_lines(self, args),
            "is_alpha" => return self::string::compile_is_alpha(self, args),
            "is_numeric" => return self::string::compile_is_numeric(self, args),
            "is_whitespace" => return self::string::compile_is_whitespace(self, args),
            "unicode_category" => return self::string::compile_unicode_category(self, args),
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
            "rotate_left" | "rotate_right" | "bit_set" | "bit_clear" | "bit_toggle" | "bit_is_set" => return self::math::compile_bitwise(self, func_name, args),
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
            "result_unwrap_or_else" => return self::result::compile_unwrap_or_else(self, args),
            "result_and_then" => return self::result::compile_and_then(self, args),
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
            "exec_fg" => return self::process::compile_exec_fg(self, args),
            "exec_background" => return self::process::compile_exec_background(self, args),
            "process_running" => return self::process::compile_process_running(self, args),
            "term_pid" => return self::process::compile_term_pid(self, args),
            "kill_pid" => return self::process::compile_kill_pid(self, args),
            "wait_pid" => return self::process::compile_wait_pid(self, args),
            "os_name" => return self::process::compile_os_name(self),
            "exec_code" => return self::process::compile_exec_code(self, args),
            "exec_lines" => return self::process::compile_exec_lines(self, args),
            "with_exec" => return self::process::compile_with_exec(self, args),
            "with_exec_code" => return self::process::compile_with_exec_code(self, args),
            "with_exec_lines" => return self::process::compile_with_exec_lines(self, args),
            "set_env" => return self::process::compile_set_env(self, args),
            "remove_env" => return self::process::compile_remove_env(self, args),
            "env_keys" => return self::process::compile_env_keys(self),
            "arch" => return self::process::compile_arch(self),
            "num_cpus" => return self::process::compile_num_cpus(self),
            "parent_pid" => return self::process::compile_parent_pid(self),
            "process_exists" => return self::process::compile_process_exists(self, args),
            "with_exec_fg" => return self::process::compile_with_exec_fg(self, args),
            "exec_with_stdin" => return self::process::compile_exec_with_stdin(self, args),
            "with_exec_with_stdin" => return self::process::compile_with_exec_with_stdin(self, args),
            "exec_with_env" => return self::process::compile_exec_with_env(self, args),
            "with_exec_with_env" => return self::process::compile_with_exec_with_env(self, args),
            "exec_with_cwd" => return self::process::compile_exec_with_cwd(self, args),
            "with_exec_with_cwd" => return self::process::compile_with_exec_with_cwd(self, args),
            "exec_with_timeout" => return self::process::compile_exec_with_timeout(self, args),
            "with_exec_background" => return self::process::compile_with_exec_background(self, args),
            "parse_args" => return self::cli::compile_parse_args(self, args),
            "parse_args_or_exit" => return self::cli::compile_parse_args_or_exit(self, args),
            "usage_string" => return self::cli::compile_usage_string(self, args),
            "prompt" => return self::cli::compile_prompt(self, args),
            "prompt_password" => return self::cli::compile_prompt_password(self, args),
            "prompt_confirm" => return self::cli::compile_prompt_confirm(self, args),
            "prompt_choice" => return self::cli::compile_prompt_choice(self, args),
            "shell_split" => return self::cli::compile_shell_split(self, args),
            "shell_join" => return self::cli::compile_shell_join(self, args),
            "read_line_editable" => return self::cli::compile_read_line_editable(self, args),
            "read_line_with_history" => return self::cli::compile_read_line_with_history(self, args),
            "progress_bar" => return self::cli::compile_progress_bar(self, args),
            "spinner_tick" => return self::cli::compile_spinner_tick(self, args),
            "sha256" => return self::crypto::compile_sha256(self, args),
            "sha512" => return self::crypto::compile_sha512(self, args),
            "sha1" => return self::crypto::compile_sha1(self, args),
            "md5" => return self::crypto::compile_md5(self, args),
            "hmac_sha256" => return self::crypto::compile_hmac_sha256(self, args),
            "hmac_sha512" => return self::crypto::compile_hmac_sha512(self, args),
            "constant_time_eq" => return self::crypto::compile_constant_time_eq(self, args),
            "secure_random_bytes" => return self::crypto::compile_secure_random_bytes(self, args),
            "secure_token" => return self::crypto::compile_secure_token(self, args),
            "secure_token_hex" => return self::crypto::compile_secure_token_hex(self, args),
            "secure_token_urlsafe" => return self::crypto::compile_secure_token_urlsafe(self, args),
            "base64_encode" => return self::crypto::compile_base64_encode(self, args),
            "base64_decode" => return self::crypto::compile_base64_decode(self, args),
            "base64_url_encode" => return self::crypto::compile_base64_url_encode(self, args),
            "base64_url_decode" => return self::crypto::compile_base64_url_decode(self, args),
            "hex_encode" => return self::crypto::compile_hex_encode(self, args),
            "hex_decode" => return self::crypto::compile_hex_decode(self, args),
            "uuid_v4" => return self::crypto::compile_uuid_v4(self),
            "uuid_v7" => return self::crypto::compile_uuid_v7(self),
            "uuid_parse" => return self::crypto::compile_uuid_parse(self, args),
            "password_hash" => return self::crypto::compile_password_hash(self, args),
            "password_verify" => return self::crypto::compile_password_verify(self, args),
            "pipe" => return self::process::compile_pipe(self, args),
            "pipe_all" => return self::process::compile_pipe_all(self, args),
            "args" => return self::process::compile_args(self),
            "time_now" => return self::process::compile_time_now(self),
            "time_now_ms" => return self::process::compile_time_now_ms(self),
            "time_add" => return self::process::compile_time_add(self, args),
            "time_diff" => return self::process::compile_time_diff(self, args),
            "format_time" => return self::process::compile_format_time(self, args),
            "format_date_str" => return self::process::compile_format_date_str(self, args),
            "format_time_str" => return self::process::compile_format_time_str(self, args),
            "time_parts" => return self::process::compile_time_parts(self, args),
            "monotonic_now" => return self::process::compile_monotonic_now(self),
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
            "path_is_absolute" => return self::process::compile_path_is_absolute(self, args),
            "path_is_relative" => return self::process::compile_path_is_relative(self, args),
            "path_starts_with" => return self::process::compile_path_starts_with(self, args),
            "path_ends_with" => return self::process::compile_path_ends_with(self, args),
            "path_normalize" => return self::process::compile_path_normalize(self, args),
            "path_absolute" => return self::process::compile_path_absolute(self, args),
            "path_canonicalize" => return self::process::compile_path_canonicalize(self, args),
            "path_expand_home" => return self::process::compile_path_expand_home(self, args),
            "path_split" => return self::process::compile_path_split(self, args),
            "path_split_extension" => return self::process::compile_path_split_extension(self, args),
            "path_components" => return self::process::compile_path_components(self, args),
            "path_with_file_name" => return self::process::compile_path_with_file_name(self, args),
            "path_relative" => return self::process::compile_path_relative(self, args),
            "path_join_many" => return self::process::compile_path_join_many(self, args),
            "mkdir" => return self::process::compile_mkdir(self, args),
            "rmdir" => return self::process::compile_rmdir(self, args),
            "move_file" => return self::process::compile_move_file(self, args),
            "rename_file" => return self::process::compile_rename_file(self, args),
            "file_created" => return self::process::compile_file_created(self, args),
            "touch" => return self::process::compile_touch(self, args),
            "temp_dir" => return self::process::compile_temp_dir(self),
            "file_size" => return self::process::compile_file_size(self, args),
            "file_modified" => return self::process::compile_file_modified(self, args),
            "copy_file" => return self::process::compile_copy_file(self, args),
            "mkdir_all" => return self::process::compile_mkdir_all(self, args),
            "rmdir_all" => return self::process::compile_rmdir_all(self, args),
            "list_dir" => return self::process::compile_list_dir(self, args),
            "list_dir_names" => return self::process::compile_list_dir_names(self, args),
            "file_accessed" => return self::process::compile_file_accessed(self, args),
            "file_permissions" => return self::process::compile_file_permissions(self, args),
            "set_permissions" => return self::process::compile_set_permissions(self, args),
            "temp_file" => return self::process::compile_temp_file(self),
            "temp_file_in" => return self::process::compile_temp_file_in(self, args),
            "truncate_file" => return self::process::compile_truncate_file(self, args),
            "glob" => return self::process::compile_glob(self, args),
            "walk_dir" => return self::process::compile_walk_dir(self, args),
            "symlink" => return self::process::compile_symlink(self, args),
            "readlink" => return self::process::compile_readlink(self, args),
            "hardlink" => return self::process::compile_hardlink(self, args),
            "realpath" => return self::process::compile_realpath(self, args),
            "lock_file" => return self::process::compile_lock_file(self, args),
            "unlock_file" => return self::process::compile_unlock_file(self, args),
            "copy_dir" => return self::process::compile_copy_dir(self, args),
            "dir_size" => return self::process::compile_dir_size(self, args),
            "is_symlink" => return self::process::compile_is_symlink(self, args),
            "open" => return self::process::compile_open(self, args),
            "read_handle" => return self::process::compile_read_handle(self, args),
            "write_handle" => return self::process::compile_write_handle(self, args),
            "seek" => return self::process::compile_seek(self, args),
            "flush" => return self::process::compile_flush(self, args),
            "read_all" => return self::process::compile_read_all(self, args),
            "readline" => return self::process::compile_readline(self, args),
            "rand_int" | "rand_float" | "rand_bool" | "rand_char" | "rand_byte" => return self::random::compile_rand_simple(self, func_name),
            "rand_bool_weighted" => return self::random::compile_rand_bool_weighted(self, args),
            "rand_int_range" | "rand_float_range" | "rand_dice" | "rand_range" => return self::random::compile_rand_range(self, func_name, args),
            "rand_range_step" => return self::random::compile_rand_range_step(self, args),
            "rand_string" => return self::random::compile_rand_string(self, args),
            "rand_dices" | "rand_bytes" | "rand_choice" | "rand_shuffle" => return self::random::compile_rand_collection(self, func_name, args),
            "rand_choices" | "rand_sample" => return self::random::compile_rand_multi(self, func_name, args),
            "rand_seed" => return self::random::compile_rand_seed(self, args),
            "term_enter" | "term_leave" | "term_clear" | "term_clear_line" | "term_save_cursor" | "term_restore_cursor" | "term_hide_cursor" | "term_show_cursor" | "term_flush" | "term_reset_color" | "term_bold" | "term_dim" | "term_italic" | "term_underline" | "term_blink" | "term_reverse" | "term_crossed_out" | "term_reset_attr" | "term_enable_wrap" | "term_disable_wrap" | "term_begin_sync" | "term_end_sync" | "term_enable_mouse" | "term_disable_mouse" => return self::terminal::compile_term_no_args(self, func_name),
            "term_move" | "term_set_fg" | "term_set_bg" | "term_fg" | "term_bg" | "term_move_to_col" | "term_move_to_row" | "term_move_up" | "term_move_down" | "term_move_left" | "term_move_right" | "term_next_line" | "term_prev_line" | "term_scroll_up" | "term_scroll_down" | "term_set_size" | "term_poll" => return self::terminal::compile_term_with_args(self, func_name, args),
            "term_set_title" | "term_print" => return self::terminal::compile_term_str(self, func_name, args),
            "term_get_size" => return self::terminal::compile_term_get_size(self),
            "term_get_cursor_pos" => return self::terminal::compile_term_get_cursor_pos(self),
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
            "arr_chunk" | "arr_windows" => return self::collections::compile_arr_chunk_windows(self, func_name, args),
            "arr_swap" => return self::collections::compile_arr_swap(self, args),
            "arr_cycle_take" => return self::collections::compile_arr_cycle_take(self, args),
            "arr_partition" | "arr_max_by" | "arr_min_by" => return self::collections::compile_arr_partition_closure(self, func_name, args),
            "arr_zip_longest" => return self::collections::compile_arr_zip_longest(self, args),
            "set_union" | "set_intersection" | "set_difference" | "set_symmetric_difference" => return self::collections::compile_set_algebra(self, func_name, args),
            "set_is_subset" | "set_is_superset" => return self::collections::compile_set_subset(self, func_name, args),
            "map_get_or" | "map_get_or_insert" => return self::collections::compile_map_get_or(self, func_name, args),
            "heap_push" => return self::collections::compile_heap_push(self, args),
            "heap_pop" | "heap_peek" => return self::collections::compile_heap_pop_peek(self, func_name, args),
            "deque_push_front" => return self::collections::compile_deque_push(self, args),
            "deque_pop_front" => return self::collections::compile_deque_pop(self, args),
            "bisect_left" | "bisect_right" => return self::collections::compile_bisect(self, func_name, args),
            "sorted_insert" => return self::collections::compile_sorted_insert(self, args),
            "result_map" | "result_map_err" => return self::closure::compile_result_closure(self, func_name, args),
            "bench" if args.len() >= 2 => return self::closure::compile_bench(self, args),
            "to_string" | "to_bin" | "to_hex" | "to_oct" => return self::types::compile_to_string_bin_hex_oct(self, func_name, args),
            "to_int" | "to_float" | "to_bool" | "to_byte" | "to_char" => return self::types::compile_to_primitive(self, func_name, args),
            "error_unwrap" => return self::types::compile_error_unwrap(self, args),
            "is_bool" | "is_int" | "is_float" | "is_string" | "is_null" | "is_char" | "is_byte" | "is_error" | "is_array" | "is_map" | "is_set" | "is_tuple" | "is_function" | "is_uint" | "is_sbyte" | "is_bsbyte" | "is_bbyte" | "is_sint" | "is_suint" | "is_sfloat" | "is_c_handle" | "is_net_handle" | "is_http_handle" | "is_audio_handle" | "is_gui_handle" | "is_file_handle" => return self::types::compile_type_check(self, func_name, args),
            "type_of" => return self::types::compile_type_of(self, args),
            "dbg" => return self::types::compile_dbg(self, args),
            "assert" => return self::assert::compile_assert(self, args),
            "assert_eq" | "assert_ne" | "assert_lt" | "assert_le" | "assert_gt" | "assert_ge" => return self::assert::compile_assert_cmp(self, func_name, args),
            "assert_approx_eq" => return self::assert::compile_assert_approx_eq(self, args),
            "panic" => return self::assert::compile_panic(self, args),
            "unreachable" => return self::assert::compile_unreachable(self),
            "todo" => return self::assert::compile_todo(self),
            "warn" => return self::assert::compile_warn(self, args),
            "stack_trace" => return self::assert::compile_stack_trace(self),
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
            "play_file" => return self::audio::compile_play_file(self, args),
            "play_file_async" => return self::audio::compile_play_file_async(self, args),
            "beep" => return self::audio::compile_beep(self, args),
            "sound_pause" => return self::audio::compile_sound_pause(self, args),
            "sound_resume" => return self::audio::compile_sound_resume(self, args),
            "sound_stop" => return self::audio::compile_sound_stop(self, args),
            "sound_is_paused" => return self::audio::compile_sound_is_paused(self, args),
            "sound_set_volume" => return self::audio::compile_sound_set_volume(self, args),
            "sound_get_volume" => return self::audio::compile_sound_get_volume(self, args),
            "sound_set_speed" => return self::audio::compile_sound_set_speed(self, args),
            "sound_seek" => return self::audio::compile_sound_seek(self, args),
            "sound_is_finished" => return self::audio::compile_sound_is_finished(self, args),
            "sound_wait" => return self::audio::compile_sound_wait(self, args),
            "list_output_devices" => return self::audio::compile_list_output_devices(self),
            "set_output_device" => return self::audio::compile_set_output_device(self, args),
            "set_master_volume" => return self::audio::compile_set_master_volume(self, args),
            "audio_duration" => return self::audio::compile_audio_duration(self, args),
            "audio_file_info" => return self::audio::compile_audio_file_info(self, args),
            "compile" if self.std_c_imports.contains("compile") => return self::c_ffi::compile_c_compile(self, args),
            "load" if self.std_c_imports.contains("load") => return self::c_ffi::compile_c_load(self, args),
            "has_symbol" if self.std_c_imports.contains("has_symbol") => return self::c_ffi::compile_c_has_symbol(self, args),
            "close" => {
                // Qualified dispatch wins over imports.
                if path.len() >= 3 && path[0] == "std" && path[1] == "fs" {
                    return self::process::compile_fs_close(self, args);
                }
                if path.len() >= 3 && path[0] == "std" && path[1] == "c" {
                    return self::c_ffi::compile_c_close(self, args);
                }
                // Bare call: later import wins like the VM.
                if let Some((ns, _)) = self.resolve_std_name("close") {
                    if ns == "std::fs" {
                        return self::process::compile_fs_close(self, args);
                    }
                    if ns == "std::c" {
                        return self::c_ffi::compile_c_close(self, args);
                    }
                }
                let want_c = self.std_c_imports.contains("close")
                    || self.std_c_imports.contains("*");
                let want_fs = self.std_fs_imports.contains("close")
                    || self.std_fs_imports.contains("*");
                if want_fs && !want_c {
                    return self::process::compile_fs_close(self, args);
                }
                if want_c && !want_fs {
                    return self::c_ffi::compile_c_close(self, args);
                }
                if want_fs && want_c {
                    return self::process::compile_fs_close(self, args);
                }
            }
            "clear_cache" if self.std_c_imports.contains("clear_cache") => return self::c_ffi::compile_c_clear_cache(self),
            "call" if self.std_c_imports.contains("call") => return self::c_ffi::compile_c_call(self, args),
            _ => {}
        }

        self.compile_user_call(&path.join("_"), args)
    }

    /// Plain C call to a user-defined RL function (no stdlib dispatch).
    fn compile_user_call(&mut self, name: &str, args: &[ExprId]) -> Result<(), Error> {
        let c_name = mangle(name);
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

    fn is_string_valued(cc: &CCodegen, id: ExprId) -> bool {
        matches!(
            cc.inferred_expr_type(id).as_ref(),
            Some(TypeAnnotation::String) | Some(TypeAnnotation::CString)
        )
    }

    pub fn compile_method_call(
        &mut self,
        caller: ExprId,
        method: &[String],
        args: &[ExprId],
    ) -> Result<(), Error> {
        // Qualified path (`x.std::ns::f(args)`): canonical stdlib call
        // with the receiver prepended, mirroring the VM.
        if method.len() > 1 {
            let mut full_args = Vec::with_capacity(args.len() + 1);
            full_args.push(caller);
            full_args.extend_from_slice(args);
            return self.compile_func_call(method, &full_args);
        }
        let method_name = method.first().map(|s| s.as_str()).unwrap_or("");

        match method_name {
            "len" => {
                // Same shape as the `len()` function: result[int].
                return self::collections::compile_len(self, &[caller]);
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

        // A record's own impl method wins, exactly like the VM. When the
        // receiver is a record the call must resolve to an impl method;
        // anything else is a compile error here instead of a broken
        // C function call downstream.
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

        // Imported stdlib function with the receiver as first argument
        // (`"ab".repeat(3)` calls `repeat("ab", 3)` when imported).
        if let Some((namespace, original)) = self.resolve_std_name(method_name) {
            let mut path: Vec<String> =
                namespace.split("::").map(|s| s.to_string()).collect();
            path.push(original);
            let mut full_args = Vec::with_capacity(args.len() + 1);
            full_args.push(caller);
            full_args.extend_from_slice(args);
            return self.compile_func_call(&path, &full_args);
        }

        // Named user function with the receiver as first argument.
        if self.user_fns.contains(method_name) {
            let c_fn = mangle(method_name);
            self.writer.write(&format!("{}(", c_fn));
            self.compile_expr(caller)?;
            for arg in args.iter() {
                self.writer.write(", ");
                self.compile_expr(*arg)?;
            }
            self.writer.write(")");
            return Ok(());
        }

        Err(Error::at(
            Reason::Compile,
            format!("cannot call method `{}` on this value", method_name),
            Span::dummy(),
        ))
    }

    fn compile_lambda(
        &mut self,
        params: &[rl_ast::statements::Param],
        _return_type: &Option<rl_ast::statements::TypeAnnotation>,
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

        let mut func_code = String::new();
        func_code.push_str(&format!("static rl_result {}(rl_closure *_self, rl_result *_args, uint64_t _argc) {{\n", fn_name));

        for (i, p) in params.iter().enumerate() {
            let c_type = type_to_c(&p.param_type);
            let c_name = mangle(&p.param_name);
            func_code.push_str(&format!("    {} {} = ", c_type, c_name));
            match Self::closure_payload_read(&p.param_type, &format!("_args[{}]", i)) {
                Ok(read) => func_code.push_str(&format!("{};\n", read)),
                Err(msg) => {
                    return Err(Error::at(
                        Reason::Compile,
                        format!("lambda parameter `{}`: {}", p.param_name, msg),
                        Span::dummy(),
                    ));
                }
            }
        }

        for (i, name) in captured_names.iter().enumerate() {
            let c_name = mangle(name);
            let ta = self
                .var_types
                .get(name)
                .cloned()
                .unwrap_or(TypeAnnotation::Int);
            let c_type = type_to_c(&ta);
            func_code.push_str(&format!("    {} {} = ", c_type, c_name));
            match Self::closure_payload_read(&ta, &format!("_self->captures[{}]", i)) {
                Ok(read) => func_code.push_str(&format!("{};\n", read)),
                Err(msg) => {
                    return Err(Error::at(
                        Reason::Compile,
                        format!("lambda capture `{}`: {}", name, msg),
                        Span::dummy(),
                    ));
                }
            }
        }

        // Lambda bodies run as rl_result functions: trailing expressions
        // are the return value (mirroring the VM), explicit `return`
        // otherwise, null when neither is present.
        let captured_typed: Vec<(String, TypeAnnotation)> = captured_names
            .iter()
            .map(|name| {
                (
                    name.clone(),
                    self.var_types
                        .get(name)
                        .cloned()
                        .unwrap_or(TypeAnnotation::Int),
                )
            })
            .collect();
        let saved_lambda = self.in_lambda_body;
        let saved_init = self.in_global_init;
        self.in_lambda_body = true;
        self.in_global_init = false;
        let body_result =
            self.compile_lambda_body(params, &captured_typed, body, &mut func_code);
        self.in_lambda_body = saved_lambda;
        self.in_global_init = saved_init;
        body_result?;

        func_code.push_str("}\n\n");

        self.static_funcs.push(func_code);

        let mut captures_code = String::new();
        for (i, name) in captured_names.iter().enumerate() {
            if i > 0 { captures_code.push_str(", "); }
            let c_name = self.lookup(name);
            let c_type = self.var_types.get(name).cloned().unwrap_or(TypeAnnotation::Int);
            captures_code.push_str(&Self::closure_capture_wrap(&c_name, &c_type)?);
        }

        let capture_count = captured_names.len();
        self.writer.write(&format!(
            "rl_closure_new_heap({}, (rl_result[]){{ {} }}, {})",
            fn_name, captures_code, capture_count
        ));

        Ok(())
    }

    /// C expression reading an `rl_result` payload as type `ta`.
    /// Struct types (records, tuples) cannot cross the boundary.
    fn closure_payload_read(ta: &TypeAnnotation, src: &str) -> Result<String, String> {
        use TypeAnnotation as T;
        let read = match ta {
            T::Int | T::CInt | T::UInt | T::CUInt | T::SInt | T::CSInt | T::SUInt
            | T::CSUInt | T::Byte | T::CByte | T::SByte | T::CSByte | T::BByte
            | T::CBByte | T::BSByte | T::CBSByte | T::Handle(_) | T::HandleInfer => {
                format!("({}).data.i64", src)
            }
            T::Float | T::CFloat | T::SFloat | T::CSFloat => {
                format!("({}).data.f64", src)
            }
            T::Bool | T::CBool => format!("({}).data.boolean", src),
            T::String | T::CString => format!("({}).data.str", src),
            T::Char | T::CChar => format!("(char)({}).data.i64", src),
            T::Array(_) | T::CArray(_) => format!("({}).data.arr", src),
            T::Map(_, _) | T::CMap(_, _) => format!("({}).data.map", src),
            T::Set(_) | T::CSet(_) => format!("({}).data.set", src),
            T::Result(_) | T::CResult(_) | T::Error | T::CError => src.to_string(),
            T::Fn | T::Callback(_, _) => format!("(*({}).data.closure)", src),
            _ => {
                return Err("cannot pass records, tuples or generic values through a closure boundary".to_string());
            }
        };
        Ok(read)
    }

    /// C expression wrapping a C value of type `ta` into an `rl_result`
    /// for the captures array.
    fn closure_capture_wrap(c_name: &str, ta: &TypeAnnotation) -> Result<String, Error> {
        use TypeAnnotation as T;
        let wrapped = match ta {
            T::Int | T::CInt => format!("rl_ok_i64({})", c_name),
            T::Float | T::CFloat => format!("rl_ok_f64({})", c_name),
            T::Bool | T::CBool => format!("rl_ok_bool({})", c_name),
            T::String | T::CString => format!("rl_ok_str({})", c_name),
            T::Array(_) | T::CArray(_) => format!("rl_ok_arr({})", c_name),
            T::Map(_, _) | T::CMap(_, _) => format!("rl_ok_map({})", c_name),
            T::Set(_) | T::CSet(_) => format!("rl_ok_set({})", c_name),
            T::Result(_) | T::CResult(_) | T::Error | T::CError => c_name.to_string(),
            // The generic macro covers narrow ints, floats, chars and
            // closures; anything else (records, tuples) fails loudly.
            _ => format!("rl_ok({})", c_name),
        };
        // Struct types reach the generic fallback and fail in _Generic;
        // catch records and tuples here with a clear message instead.
        // (Enums are plain int64 values and wrap fine.)
        match ta {
            T::Record(_) | T::CRecord(_) | T::Tuple(_) | T::CTuple(_) => Err(Error::at(
                Reason::Compile,
                "cannot capture records or tuples in a lambda",
                Span::dummy(),
            )),
            _ => Ok(wrapped),
        }
    }

    /// Compiles a lambda body with trailing-expression value semantics.
    /// Params and captures are declared in a dedicated scope so body
    /// statements resolve names and types through the normal paths.
    fn compile_lambda_body(
        &mut self,
        params: &[rl_ast::statements::Param],
        captured: &[(String, TypeAnnotation)],
        body: &[rl_ast::statements::Statement],
        func_code: &mut String,
    ) -> Result<(), Error> {
        use rl_ast::statements::StatementKind;
        self.push_scope();
        let saved_types = self.var_types.clone();
        let saved_nullable = self.nullable_vars.clone();
        let saved_closures = self.closure_return_types.clone();
        for p in params {
            self.declare(&p.param_name, &mangle(&p.param_name));
            self.var_types
                .insert(p.param_name.clone(), p.param_type.clone());
        }
        for (name, ta) in captured {
            self.declare(name, &mangle(name));
            self.var_types.insert(name.clone(), ta.clone());
        }
        let trailing_expr = match body.last().map(|s| &s.kind) {
            Some(StatementKind::Expression(expr_id)) => Some(*expr_id),
            _ => None,
        };
        let main = if trailing_expr.is_some() {
            &body[..body.len() - 1]
        } else {
            body
        };
        let mut result: Result<(), Error> = Ok(());
        for s in main {
            result = self.compile_lambda_statement(s, func_code);
            if result.is_err() {
                break;
            }
        }
        if result.is_ok() {
            if let Some(expr_id) = trailing_expr {
                func_code.push_str("    return rl_ok(");
                result = self.compile_expr_to_string(expr_id, func_code);
                if result.is_ok() {
                    func_code.push_str(");\n");
                }
            } else {
                func_code.push_str("    return rl_ok_null();\n");
            }
        }
        self.var_types = saved_types;
        self.nullable_vars = saved_nullable;
        self.closure_return_types = saved_closures;
        self.pop_scope();
        result
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
                Ok(())
            }
            StatementKind::Return(None) => {
                func_code.push_str("    return rl_ok_null();\n");
                Ok(())
            }
            _ => self.compile_lambda_delegate(stmt, func_code),
        }
    }

    /// Compiles a lambda body statement with the normal statement
    /// compilers into the static function text. This gives lambda bodies
    /// the full statement support (loops, match, nested conditionals)
    /// for free. Locals stay scoped to the lambda via save/restore.
    fn compile_lambda_delegate(
        &mut self,
        stmt: &rl_ast::statements::Statement,
        func_code: &mut String,
    ) -> Result<(), Error> {
        let saved_writer = std::mem::take(&mut self.writer);
        self.writer = CWriter::new();
        self.writer.indent();
        self.push_scope();
        let saved_types = self.var_types.clone();
        let saved_nullable = self.nullable_vars.clone();
        let saved_closures = self.closure_return_types.clone();
        let result = self.compile_statement(stmt);
        self.var_types = saved_types;
        self.nullable_vars = saved_nullable;
        self.closure_return_types = saved_closures;
        self.pop_scope();
        let chunk = std::mem::replace(&mut self.writer, saved_writer).into_source();
        result?;
        func_code.push_str(&chunk);
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
                if !param_names.contains(name) && !captured.contains(name) {
                    // Globals live at C file scope: reference them directly
                    // instead of snapshotting. A same-named local in an
                    // enclosing function scope still captures by value.
                    let is_local = self.scopes.iter().skip(1).any(|s| s.contains_key(name));
                    let is_known = self.var_types.contains_key(name)
                        || self.scopes.iter().rev().any(|s| s.contains_key(name));
                    if is_known && (is_local || !self.global_names.contains(name)) {
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
                self.writer.write(&format!("rl_ok_str(rl_str_literal(\"{}\", {}))", escaped, v.len()));
            }
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                if self.nullable_vars.contains(name) {
                    // Stored as rl_result: compile_expr unwraps to the C
                    // value, then re-wrap.
                    self.writer.write("rl_ok(");
                    self.compile_expr(id)?;
                    self.writer.write(")");
                } else if let Some(ta) = self.var_types.get(name) {
                    let c_name = self.lookup(name);
                    match ta {
                        TypeAnnotation::Result(_) | TypeAnnotation::CResult(_) => {
                            self.writer.write(&c_name);
                        }
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
                            self.writer.write("rl_ok(");
                            self.compile_expr(id)?;
                            self.writer.write(")");
                        }
                    }
                } else {
                    self.writer.write("rl_ok(");
                    self.compile_expr(id)?;
                    self.writer.write(")");
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

    /// True when a format/concat argument is already a plain value rather
    /// than a wrapped result. Results render `ok(...)`, bare values render
    /// the payload, mirroring the VM's Display.
    pub fn fmt_arg_is_bare(&self, id: ExprId) -> bool {
        match self.inferred_expr_type(id).as_ref() {
            // Results and dynamically-typed values render wrapped. The
            // fallback storage for unknown calls is rl_result, so treat
            // unknown the same way for calls.
            Some(TypeAnnotation::Result(_))
            | Some(TypeAnnotation::CResult(_))
            | Some(TypeAnnotation::Infer)
            | Some(TypeAnnotation::Generic(_)) => false,
            Some(_) => true,
            None => {
                let kind = &self.ast.exprs.get(id).kind;
                !matches!(
                    kind,
                    ExpressionKind::Call { .. }
                        | ExpressionKind::MethodCall { .. }
                        | ExpressionKind::CallExpr { .. }
                )
            }
        }
    }

    /// Emits one `(rl_fmt_arg)` element: the value as `rl_result` plus
    /// whether it was already bare.
    pub fn write_arg_as_fmt(&mut self, id: ExprId) -> Result<(), Error> {
        let bare = self.fmt_arg_is_bare(id);
        self.writer.write("{ ");
        self.write_arg_as_result(id)?;
        if bare {
            self.writer.write(", true }");
        } else {
            self.writer.write(", false }");
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

    // Builds a map literal into a fresh temp, returning the temp name.
    // Statement-shaped by nature: valid wherever statements are, including
    // hoisted array elements and declaration initializers, but NOT nested
    // inside other expressions.
    pub(crate) fn emit_map_lit(
        &mut self,
        entries: &[(ExprId, ExprId)],
    ) -> Result<String, Error> {
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
        Ok(temp)
    }

    // Same statement-shaped contract as `emit_map_lit`, for set literals.
    pub(crate) fn emit_set_lit(&mut self, items: &[ExprId]) -> Result<String, Error> {
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
        Ok(temp)
    }

    // Pre-statement hoist for statement-shaped literals. The root value
    // itself (a bare map/set literal) and direct elements of array
    // literals are built into temps BEFORE the statement writes anything
    // (e.g. the `name = ` prefix), so later emission just names temps.
    // Anything nested deeper (call args, map values, ...) is left alone:
    // still unsupported, exactly as before.
    pub(crate) fn hoist_stmt_literals(&mut self, id: ExprId) -> Result<(), Error> {
        let kind = self.ast.exprs.get(id).kind.clone();
        match kind {
            ExpressionKind::MapLiteral(entries) => {
                if !self.hoisted_tmps.contains_key(&id) {
                    let temp = self.emit_map_lit(&entries)?;
                    self.hoisted_tmps.insert(id, temp);
                }
            }
            ExpressionKind::SetLiteral(items) => {
                if !self.hoisted_tmps.contains_key(&id) {
                    let temp = self.emit_set_lit(&items)?;
                    self.hoisted_tmps.insert(id, temp);
                }
            }
            ExpressionKind::ArrayLiteral(elems) => {
                for elem in elems {
                    self.hoist_array_elem(elem)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn hoist_array_elem(&mut self, id: ExprId) -> Result<(), Error> {
        if self.hoisted_tmps.contains_key(&id) {
            return Ok(());
        }
        let kind = self.ast.exprs.get(id).kind.clone();
        match kind {
            ExpressionKind::MapLiteral(entries) => {
                let temp = self.emit_map_lit(&entries)?;
                self.hoisted_tmps.insert(id, temp);
            }
            ExpressionKind::SetLiteral(items) => {
                let temp = self.emit_set_lit(&items)?;
                self.hoisted_tmps.insert(id, temp);
            }
            ExpressionKind::ArrayLiteral(elems) => {
                for elem in elems {
                    self.hoist_array_elem(elem)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
