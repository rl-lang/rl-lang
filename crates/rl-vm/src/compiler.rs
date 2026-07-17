use std::rc::Rc;

use crate::chunk::{Chunk, OpCode};
use crate::native::Module;
use crate::stdlib;
use crate::values::{VmFunction, VmValue};
use rl_ast::{
    Ast, ExprId, nodes::ExpressionKind, statements::Statement, statements::StatementKind,
};
use rl_lexer::tokentypes::TokenType;

#[derive(Debug)]
pub struct CompileError(pub String);

enum ContinueTarget {
    Backward(usize),
    #[allow(unused)]
    Forward,
}

struct LoopCtx {
    continue_target: ContinueTarget,
    continue_jumps: Vec<usize>,
    break_jumps: Vec<usize>,
    scope_depth: u16,
}

pub struct Compiler<'a> {
    ast: &'a Ast,
    chunk: Chunk,
    next_slot: u16,
    scope_bases: Vec<u16>,
    stdlib: Module,
    loop_stack: Vec<LoopCtx>,
}

impl<'a> Compiler<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self {
            ast,
            chunk: Chunk::new(),
            next_slot: 0,
            scope_bases: Vec::new(),
            stdlib: stdlib::root(),
            loop_stack: Vec::new(),
        }
    }

    fn resolve(&self, depth: usize, slot: usize) -> Option<u16> {
        if depth == self.scope_bases.len() {
            return None;
        }
        let base = self.scope_bases[self.scope_bases.len() - 1 - depth];
        Some(base + slot as u16)
    }

    /// Entry function
    /// returns compiled Chunk
    /// stops on first error
    /// will consume and discard the Compiler
    pub fn compile(mut self, statements: &[Statement]) -> Result<Chunk, CompileError> {
        self.compile_body(statements)?;
        self.chunk.write_op(OpCode::Return, 0);
        Ok(self.chunk)
    }

    /// todo function
    /// should convert source span into line number for lines
    fn line(&self, _stmt_or_expr_span: u32) -> u32 {
        0
    }

    /// Statement entry function
    fn compile_statement(&mut self, stmt: &Statement) -> Result<(), CompileError> {
        let line = self.line(0); // todo
        match &stmt.kind {
            StatementKind::ResolvedVariableDeclaration { value, .. }
            | StatementKind::ResolvedConstantDeclaration { value, .. }
            | StatementKind::ResolvedArray { value, .. }
            | StatementKind::ResolvedConstantArray { value, .. }
            | StatementKind::ResolvedMap { value, .. }
            | StatementKind::ResolvedConstantMap { value, .. }
            | StatementKind::ResolvedSet { value, .. }
            | StatementKind::ResolvedConstantSet { value, .. } => {
                self.compile_expr(*value)?;
                let slot = self.next_slot;
                self.next_slot += 1;
                self.chunk.write_op(OpCode::DefineLocal, line);
                self.chunk.write_u16(slot, line);
                Ok(())
            }

            StatementKind::VariableDeclaration { .. }
            | StatementKind::ConstantDeclaration { .. }
            | StatementKind::Array { .. }
            | StatementKind::ConstantArray { .. }
            | StatementKind::Map { .. }
            | StatementKind::ConstantMap { .. }
            | StatementKind::Set { .. }
            | StatementKind::ConstantSet { .. } => Err(CompileError(
                "unresolved declaration reached the compiler - run the resolver pass first".into(),
            )),

            StatementKind::RecordDeclaration { .. } | StatementKind::TagDeclaration { .. } => {
                Ok(())
            }

            StatementKind::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                self.compile_expr(*condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);

                self.loop_stack.push(LoopCtx {
                    continue_target: ContinueTarget::Backward(loop_start),
                    continue_jumps: Vec::new(),
                    break_jumps: Vec::new(),
                    scope_depth: self.scope_bases.len() as u16,
                });
                // resolver unconditionally pushes a scope for `while` bodies
                self.compile_block(body, true, line)?;
                let ctx = self.loop_stack.pop().unwrap();

                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                for pos in ctx.break_jumps {
                    self.patch_jump(pos);
                }
                Ok(())
            }

            StatementKind::ResolvedFor {
                initializer,
                condition,
                increment,
                body,
            } => {
                // The resolver never pushes a scope around a C-style for loop
                // (see rl-resolver/src/statements.rs), so the initializer's
                // variable - and anything the body declares - lives directly
                // in the enclosing frame, with no PushScope/PopScope pair.
                self.compile_statement(initializer)?;

                let loop_start = self.chunk.code.len();
                self.compile_expr(*condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);

                // `continue` must still run the increment before re-checking
                // the condition, so it can't jump straight back to
                // `loop_start` like `while` does - it jumps forward to just
                // before the increment instead.
                self.loop_stack.push(LoopCtx {
                    continue_target: ContinueTarget::Forward,
                    continue_jumps: Vec::new(),
                    break_jumps: Vec::new(),
                    scope_depth: self.scope_bases.len() as u16,
                });
                // no extra scope for the body either, matching the interpreter
                self.compile_block(body, false, line)?;
                let ctx = self.loop_stack.pop().unwrap();

                // `continue` jumps land here, right before the increment.
                for pos in ctx.continue_jumps {
                    self.patch_jump(pos);
                }
                self.compile_expr_statement(*increment)?;
                self.emit_loop(loop_start, line);

                self.patch_jump(exit_jump);
                for pos in ctx.break_jumps {
                    self.patch_jump(pos);
                }
                Ok(())
            }

            StatementKind::ResolvedForRange {
                slot, range, body, ..
            } => {
                let items = match &range.kind {
                    StatementKind::Range(items) => items.clone(),
                    _ => {
                        return Err(CompileError("for-range: expected a range statement".into()));
                    }
                };

                let mut break_jumps = Vec::new();
                for item in items {
                    let pre_depth = self.scope_bases.len() as u16;

                    self.chunk.write_op(OpCode::PushScope, line);
                    self.scope_bases.push(self.next_slot);

                    self.emit_const(VmValue::Int(item), line);
                    let loop_var_slot = self.next_slot + *slot as u16;
                    self.next_slot = loop_var_slot + 1;
                    self.emit_define_slot(loop_var_slot, line);

                    self.loop_stack.push(LoopCtx {
                        continue_target: ContinueTarget::Forward,
                        continue_jumps: Vec::new(),
                        break_jumps: Vec::new(),
                        scope_depth: pre_depth,
                    });
                    self.compile_block(body, false, line)?;
                    let ctx = self.loop_stack.pop().unwrap();

                    self.next_slot = self.scope_bases.pop().unwrap();
                    self.chunk.write_op(OpCode::PopScope, line);

                    for pos in ctx.continue_jumps {
                        self.patch_jump(pos);
                    }
                    break_jumps.extend(ctx.break_jumps);
                }
                for pos in break_jumps {
                    self.patch_jump(pos);
                }
                Ok(())
            }

            StatementKind::ResolvedForEach {
                slot,
                iterable,
                body,
                ..
            } => {
                let hidden_base = self.next_slot;
                self.next_slot += 2;
                let arr_slot = hidden_base;
                let idx_slot = hidden_base + 1;
                let hidden_global = self.scope_bases.is_empty();

                self.compile_expr(*iterable)?;
                self.emit_define_slot(arr_slot, line);

                self.emit_const(VmValue::Int(0), line);
                self.emit_define_slot(idx_slot, line);

                let loop_start = self.chunk.code.len();
                self.emit_get_slot(idx_slot, hidden_global, line);
                self.emit_get_slot(arr_slot, hidden_global, line);
                self.chunk.write_op(OpCode::ArrLen, line);
                self.chunk.write_op(OpCode::Less, line);
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);

                let pre_depth = self.scope_bases.len() as u16;
                self.chunk.write_op(OpCode::PushScope, line);
                self.scope_bases.push(self.next_slot);

                self.emit_get_slot(arr_slot, hidden_global, line);
                self.emit_get_slot(idx_slot, hidden_global, line);
                self.chunk.write_op(OpCode::Index, line);
                let loop_var_slot = self.next_slot + *slot as u16;
                self.next_slot = loop_var_slot + 1;
                self.emit_define_slot(loop_var_slot, line);

                self.loop_stack.push(LoopCtx {
                    continue_target: ContinueTarget::Forward,
                    continue_jumps: Vec::new(),
                    break_jumps: Vec::new(),
                    scope_depth: pre_depth,
                });
                self.compile_block(body, false, line)?;
                let ctx = self.loop_stack.pop().unwrap();

                self.next_slot = self.scope_bases.pop().unwrap();
                self.chunk.write_op(OpCode::PopScope, line);

                for pos in ctx.continue_jumps {
                    self.patch_jump(pos);
                }
                self.emit_get_slot(idx_slot, hidden_global, line);
                self.emit_const(VmValue::Int(1), line);
                self.chunk.write_op(OpCode::Add, line);
                self.emit_set_slot(idx_slot, hidden_global, line);

                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                for pos in ctx.break_jumps {
                    self.patch_jump(pos);
                }

                self.next_slot = hidden_base;
                Ok(())
            }

            StatementKind::Break => {
                if self.loop_stack.is_empty() {
                    return Err(CompileError("`break` outside of a loop".into()));
                }
                let target_depth = self.loop_stack.last().unwrap().scope_depth;
                let current_depth = self.scope_bases.len() as u16;
                for _ in target_depth..current_depth {
                    self.chunk.write_op(OpCode::PopScope, line);
                }
                let pos = self.emit_jump(OpCode::Jump, line);
                self.loop_stack.last_mut().unwrap().break_jumps.push(pos);
                Ok(())
            }

            StatementKind::Continue => {
                if self.loop_stack.is_empty() {
                    return Err(CompileError("`continue` outside of a loop".into()));
                }
                let target_depth = self.loop_stack.last().unwrap().scope_depth;
                let current_depth = self.scope_bases.len() as u16;
                for _ in target_depth..current_depth {
                    self.chunk.write_op(OpCode::PopScope, line);
                }
                match self.loop_stack.last().unwrap().continue_target {
                    ContinueTarget::Backward(target) => self.emit_loop(target, line),
                    ContinueTarget::Forward => {
                        let pos = self.emit_jump(OpCode::Jump, line);
                        self.loop_stack.last_mut().unwrap().continue_jumps.push(pos);
                    }
                }
                Ok(())
            }

            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => self.compile_conditional(if_branch, else_branch.as_deref(), line),

            StatementKind::Expression(id) => self.compile_expr_statement(*id),

            StatementKind::ResolvedFunctionDeclaration {
                name,
                slot,
                params,
                body,
                ..
            } => {
                let func_chunk = Self::compile_function_chunk(self.ast, body, params.len())?;
                let func = VmValue::Function(Rc::new(VmFunction {
                    name: name.clone(),
                    arity: params.len(),
                    chunk: func_chunk,
                }));
                self.emit_const(func, line);
                self.chunk.write_op(OpCode::DefineLocal, line);
                self.chunk.write_u16(*slot as u16, line);
                Ok(())
            }

            StatementKind::Return(expr_opt) => {
                match expr_opt {
                    Some(e) => self.compile_expr(*e)?,
                    None => self.emit_const(VmValue::Null, line),
                }
                self.chunk.write_op(OpCode::Return, line);
                Ok(())
            }

            other => Err(CompileError(format!(
                "statement kind not yet supported by the vm compiler: {other:?}"
            ))),
        }
    }

    fn compile_conditional(
        &mut self,
        if_branch: &Statement,
        else_branch: Option<&Statement>,
        line: u32,
    ) -> Result<(), CompileError> {
        let StatementKind::ConditionalBranch {
            condition,
            body,
            needs_scope,
        } = &if_branch.kind
        else {
            return Err(CompileError(
                "malformed if-branch reached the vm compiler".into(),
            ));
        };
        let condition =
            condition.ok_or_else(|| CompileError("if-branch is missing its condition".into()))?;

        self.compile_expr(condition)?;
        let else_jump = self.emit_jump(OpCode::JumpIfFalse, line);
        self.compile_block(body, *needs_scope, line)?;

        let end_jump = if else_branch.is_some() {
            Some(self.emit_jump(OpCode::Jump, line))
        } else {
            None
        };
        self.patch_jump(else_jump);

        if let Some(else_stmt) = else_branch {
            match &else_stmt.kind {
                StatementKind::Conditional {
                    if_branch,
                    else_branch,
                } => self.compile_conditional(if_branch, else_branch.as_deref(), line)?,
                StatementKind::ConditionalBranch {
                    body, needs_scope, ..
                } => {
                    self.compile_block(body, *needs_scope, line)?;
                }
                other => {
                    return Err(CompileError(format!(
                        "unexpected else-branch kind: {other:?}"
                    )));
                }
            }
        }

        if let Some(end_jump) = end_jump {
            self.patch_jump(end_jump);
        }
        Ok(())
    }

    fn compile_body(&mut self, statements: &[Statement]) -> Result<(), CompileError> {
        let trailing_expr = matches!(
            statements.last().map(|s| &s.kind),
            Some(StatementKind::Expression(_))
        );
        for (i, stmt) in statements.iter().enumerate() {
            if let StatementKind::Expression(id) = &stmt.kind {
                if trailing_expr && i == statements.len() - 1 {
                    self.compile_expr(*id)?;
                } else {
                    self.compile_expr_statement(*id)?;
                }
            } else {
                self.compile_statement(stmt)?;
            }
        }
        Ok(())
    }

    /// Compiles a `{ }` body. Every expression statement inside a block is
    /// always discarded (Pop) - only the top-level program's trailing
    /// expression statement keeps its value.
    fn compile_block(
        &mut self,
        body: &[Statement],
        needs_scope: bool,
        line: u32,
    ) -> Result<(), CompileError> {
        if needs_scope {
            self.chunk.write_op(OpCode::PushScope, line);
            self.scope_bases.push(self.next_slot);
        }
        for stmt in body {
            if let StatementKind::Expression(id) = &stmt.kind {
                self.compile_expr_statement(*id)?;
            } else {
                self.compile_statement(stmt)?;
            }
        }
        if needs_scope {
            self.chunk.write_op(OpCode::PopScope, line);
            self.next_slot = self.scope_bases.pop().unwrap();
        }
        Ok(())
    }

    /// Expression statement entry function
    fn compile_expr_statement(&mut self, id: ExprId) -> Result<(), CompileError> {
        self.compile_expr(id)?;
        self.chunk.write_op(OpCode::Pop, 0);
        Ok(())
    }

    /// Expression entry function
    fn compile_expr(&mut self, id: ExprId) -> Result<(), CompileError> {
        let expr = self.ast.exprs.get(id);
        let line = self.line(0);
        match &expr.kind {
            ExpressionKind::Null => self.emit_const(VmValue::Null, line),
            ExpressionKind::Integer(v) => self.emit_const(VmValue::Int(*v), line),
            ExpressionKind::Float(v) => self.emit_const(VmValue::Float(*v), line),
            ExpressionKind::Bool(v) => self.emit_const(VmValue::Bool(*v), line),
            ExpressionKind::Byte(v) => self.emit_const(VmValue::Byte(*v), line),
            ExpressionKind::Character(v) => self.emit_const(VmValue::Char(*v), line),
            ExpressionKind::String(v) => self.emit_const(VmValue::Str(Rc::from(v.as_str())), line),

            ExpressionKind::Grouping(inner) => self.compile_expr(*inner)?,

            ExpressionKind::Unary { operator, operand } => {
                self.compile_expr(*operand)?;
                match operator {
                    TokenType::Minus => self.chunk.write_op(OpCode::Negate, line),
                    TokenType::Bang => self.chunk.write_op(OpCode::Not, line),
                    other => {
                        return Err(CompileError(format!(
                            "unsupported unary operator in vm compiler: {other:?}"
                        )));
                    }
                }
            }

            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                let op = match operator {
                    TokenType::Plus => OpCode::Add,
                    TokenType::Minus => OpCode::Sub,
                    TokenType::Star => OpCode::Mul,
                    TokenType::Slash => OpCode::Div,
                    TokenType::Compare => OpCode::Eq,
                    TokenType::BangEqual => OpCode::NotEq,
                    TokenType::Less => OpCode::Less,
                    TokenType::LessEqual => OpCode::LessEq,
                    TokenType::Greater => OpCode::Greater,
                    TokenType::GreaterEqual => OpCode::GreaterEq,
                    other => {
                        return Err(CompileError(format!(
                            "unsupported binary operator in vm compiler: {other:?}"
                        )));
                    }
                };
                self.chunk.write_op(op, line);
            }

            ExpressionKind::ResolvedIdentifier { depth, slot, .. } => {
                match self.resolve(*depth, *slot) {
                    Some(s) => {
                        self.chunk.write_op(OpCode::GetLocal, line);
                        self.chunk.write_u16(s, line);
                    }

                    None => {
                        self.chunk.write_op(OpCode::GetGlobal, line);
                        self.chunk.write_u16(*slot as u16, line);
                    }
                }
            }

            ExpressionKind::ResolvedAssign {
                depth, slot, value, ..
            } => {
                self.compile_expr(*value)?;
                match self.resolve(*depth, *slot) {
                    Some(s) => {
                        self.chunk.write_op(OpCode::SetLocal, line);
                        self.chunk.write_u16(s, line);
                    }

                    None => {
                        self.chunk.write_op(OpCode::SetGlobal, line);
                        self.chunk.write_u16(*slot as u16, line);
                    }
                }
            }

            ExpressionKind::Call { path, args } => {
                let native = self.stdlib.resolve(path).ok_or_else(|| {
                    CompileError(format!("undefined function {}", path.join("::")))
                })?;
                self.emit_const(VmValue::Native(native), line);
                for arg in args {
                    self.compile_expr(*arg)?;
                }
                self.chunk.write_op(OpCode::Call, line);
                self.chunk.write_u16(args.len() as u16, line);
            }

            ExpressionKind::CallExpr { callee, args } => {
                self.compile_expr(*callee)?;
                for arg in args {
                    self.compile_expr(*arg)?;
                }
                self.chunk.write_op(OpCode::Call, line);
                self.chunk.write_u16(args.len() as u16, line);
            }

            ExpressionKind::OkLiteral(inner) => {
                self.compile_expr(*inner)?;
                self.chunk.write_op(OpCode::Ok, line);
            }

            ExpressionKind::ErrLiteral(inner) => {
                self.compile_expr(*inner)?;
                self.chunk.write_op(OpCode::Err, line);
            }

            ExpressionKind::Propagate(inner) => {
                self.compile_expr(*inner)?;
                self.chunk.write_op(OpCode::Propagate, line);
            }

            ExpressionKind::ErrorLiteral(inner) => {
                self.compile_expr(*inner)?;
                self.chunk.write_op(OpCode::Error, line);
            }

            ExpressionKind::ArrayLiteral(items) => {
                for item in items {
                    self.compile_expr(*item)?;
                }
                self.chunk.write_op(OpCode::BuildArr, line);
                self.chunk.write_u16(items.len() as u16, line);
            }

            ExpressionKind::TupleLiteral(items) => {
                for item in items {
                    self.compile_expr(*item)?;
                }
                self.chunk.write_op(OpCode::BuildTuple, line);
                self.chunk.write_u16(items.len() as u16, line);
            }

            ExpressionKind::SetLiteral(items) => {
                for item in items {
                    self.compile_expr(*item)?;
                }
                self.chunk.write_op(OpCode::BuildSet, line);
                self.chunk.write_u16(items.len() as u16, line);
            }

            ExpressionKind::MapLiteral(items) => {
                for (key, value) in items {
                    self.compile_expr(*key)?;
                    self.compile_expr(*value)?;
                }
                self.chunk.write_op(OpCode::BuildMap, line);
                self.chunk.write_u16(items.len() as u16, line);
            }

            ExpressionKind::Index { target, index } => {
                self.compile_expr(*target)?;
                self.compile_expr(*index)?;
                self.chunk.write_op(OpCode::Index, line);
            }

            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => {
                let ExpressionKind::ResolvedIdentifier { depth, slot, .. } =
                    &self.ast.exprs.get(*target).kind
                else {
                    return Err(CompileError(
                        "vm compiler only supports index-assignment on a plain variable \
                         (e.g. `arr[i] = x`), not a chained or computed target"
                            .into(),
                    ));
                };
                let (depth, slot) = (*depth, *slot);
                let resolved = self.resolve(depth, slot);

                // push current array
                match resolved {
                    Some(s) => {
                        self.chunk.write_op(OpCode::GetLocal, line);
                        self.chunk.write_u16(s, line);
                    }
                    None => {
                        self.chunk.write_op(OpCode::GetGlobal, line);
                        self.chunk.write_u16(slot as u16, line);
                    }
                }
                self.compile_expr(*index)?;
                self.compile_expr(*value)?;
                self.chunk.write_op(OpCode::ArrSet, line);
                // write the updated array back; result stays on the stack as the expr's value
                match resolved {
                    Some(s) => {
                        self.chunk.write_op(OpCode::SetLocal, line);
                        self.chunk.write_u16(s, line);
                    }
                    None => {
                        self.chunk.write_op(OpCode::SetGlobal, line);
                        self.chunk.write_u16(slot as u16, line);
                    }
                }
            }

            ExpressionKind::StructLiteral { name, fields } => {
                let field_names: Vec<VmValue> = fields
                    .iter()
                    .map(|(fname, _)| VmValue::Str(Rc::from(fname.as_str())))
                    .collect();
                for (_, value_id) in fields {
                    self.compile_expr(*value_id)?;
                }
                let name_idx = self
                    .chunk
                    .add_constant(VmValue::Str(Rc::from(name.as_str())));
                let fields_idx = self.chunk.add_constant(VmValue::Arr(Rc::new(field_names)));
                self.chunk.write_op(OpCode::BuildRecord, line);
                self.chunk.write_u16(name_idx, line);
                self.chunk.write_u16(fields_idx, line);
                self.chunk.write_u16(fields.len() as u16, line);
            }

            ExpressionKind::FieldAccess { target, field } => {
                self.compile_expr(*target)?;
                let field_idx = self
                    .chunk
                    .add_constant(VmValue::Str(Rc::from(field.as_str())));
                self.chunk.write_op(OpCode::FieldGet, line);
                self.chunk.write_u16(field_idx, line);
            }

            ExpressionKind::FieldAssign {
                target,
                field,
                value,
            } => {
                self.compile_expr(*target)?;
                self.compile_expr(*value)?;
                let field_idx = self
                    .chunk
                    .add_constant(VmValue::Str(Rc::from(field.as_str())));
                self.chunk.write_op(OpCode::FieldSet, line);
                self.chunk.write_u16(field_idx, line);
            }

            ExpressionKind::EnumVariant { enum_name, variant } => {
                self.emit_const(
                    VmValue::Tag {
                        name: Rc::from(enum_name.as_str()),
                        variant: Rc::from(variant.as_str()),
                    },
                    line,
                );
            }

            ExpressionKind::ResolvedLambda {
                params,
                body,
                capture_depth,
                ..
            } => {
                let param_count = params.len();
                let (capture_start, captured_scope_bases, outer_next_slot): (u16, &[u16], u16) =
                    if self.scope_bases.is_empty() {
                        (0, &[], 0)
                    } else {
                        let cap_depth = (*capture_depth).min(self.scope_bases.len());
                        let capture_start = if cap_depth == 0 {
                            self.next_slot
                        } else {
                            self.scope_bases[self.scope_bases.len() - cap_depth]
                        };
                        let captured_scope_bases =
                            &self.scope_bases[self.scope_bases.len() - cap_depth..];
                        (capture_start, captured_scope_bases, self.next_slot)
                    };
                let chunk = Self::compile_closure_chunk(
                    self.ast,
                    body,
                    param_count,
                    captured_scope_bases,
                    outer_next_slot,
                )?;

                let func = VmValue::Function(Rc::new(VmFunction {
                    name: "<lambda>".to_string(),
                    arity: param_count,
                    chunk,
                }));
                let const_idx = self.chunk.add_constant(func);
                self.chunk.write_op(OpCode::BuildClosure, line);
                self.chunk.write_u16(const_idx, line);
                self.chunk.write_u16(capture_start, line);
            }

            other => {
                return Err(CompileError(format!(
                    "expression kind not yet supported by the vm compiler: {other:?}"
                )));
            }
        }
        Ok(())
    }

    /// Helper function that adds the value into Chunk constants
    fn emit_const(&mut self, value: VmValue, line: u32) {
        let idx = self.chunk.add_constant(value);
        self.chunk.write_op(OpCode::Const, line);
        self.chunk.write_u16(idx, line);
    }

    /// Emits `op` with a placeholder u16 operand; returns the byte offset
    /// of that operand so it can be filled in later via `patch_jump`.
    fn emit_jump(&mut self, op: OpCode, line: u32) -> usize {
        self.chunk.write_op(op, line);
        self.chunk.write_u16(0xFFFF, line);
        self.chunk.code.len() - 2
    }

    /// Backpatches a previously-emitted forward jump to land at the
    /// current end of the chunk.
    fn patch_jump(&mut self, operand_pos: usize) {
        let jump = self.chunk.code.len() - (operand_pos + 2);
        let bytes = (jump as u16).to_le_bytes();
        self.chunk.code[operand_pos] = bytes[0];
        self.chunk.code[operand_pos + 1] = bytes[1];
    }

    /// Emits a backward `Loop` jump targeting `loop_start`.
    fn emit_loop(&mut self, loop_start: usize, line: u32) {
        self.chunk.write_op(OpCode::Loop, line);
        let pos_after_operand = self.chunk.code.len() + 2;
        let offset = (pos_after_operand - loop_start) as u16;
        self.chunk.write_u16(offset, line);
    }

    fn compile_function_chunk(
        ast: &Ast,
        body: &[Statement],
        param_count: usize,
    ) -> Result<Chunk, CompileError> {
        let mut sub = Compiler::new(ast);
        sub.scope_bases.push(0);
        sub.next_slot = param_count as u16;
        sub.compile_body(body)?;
        sub.chunk.write_op(OpCode::Return, 0); // implicit `return null` on fallthrough
        Ok(sub.chunk)
    }

    fn compile_closure_chunk(
        ast: &Ast,
        body: &[Statement],
        param_count: usize,
        captured_scope_bases: &[u16],
        outer_next_slot: u16,
    ) -> Result<Chunk, CompileError> {
        let mut sub = Compiler::new(ast);
        sub.scope_bases = captured_scope_bases.to_vec();
        sub.scope_bases.push(outer_next_slot);
        sub.next_slot = outer_next_slot + param_count as u16;
        sub.compile_body(body)?;
        sub.chunk.write_op(OpCode::Return, 0);
        Ok(sub.chunk)
    }

    /// Defines a raw, compiler-managed slot.
    fn emit_define_slot(&mut self, slot: u16, line: u32) {
        self.chunk.write_op(OpCode::DefineLocal, line);
        self.chunk.write_u16(slot, line);
    }

    /// Reads a raw, compiler-managed slot.
    fn emit_get_slot(&mut self, slot: u16, is_global: bool, line: u32) {
        if is_global {
            self.chunk.write_op(OpCode::GetGlobal, line);
        } else {
            self.chunk.write_op(OpCode::GetLocal, line);
        }
        self.chunk.write_u16(slot, line);
    }

    /// Writes a raw, compiler-managed slot and discards the leftover
    /// value that SetLocal/SetGlobal leave on the stack.
    fn emit_set_slot(&mut self, slot: u16, is_global: bool, line: u32) {
        if is_global {
            self.chunk.write_op(OpCode::SetGlobal, line);
        } else {
            self.chunk.write_op(OpCode::SetLocal, line);
        }
        self.chunk.write_u16(slot, line);
        self.chunk.write_op(OpCode::Pop, line);
    }
}
