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

pub struct Compiler<'a> {
    ast: &'a Ast,
    chunk: Chunk,
    next_slot: u16,
    scope_bases: Vec<u16>,
    stdlib: Module,
}

impl<'a> Compiler<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self {
            ast,
            chunk: Chunk::new(),
            next_slot: 0,
            scope_bases: Vec::new(),
            stdlib: stdlib::root(),
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
        let last_expr_idx = statements
            .iter()
            .rposition(|s| matches!(s.kind, StatementKind::Expression(_)));

        for (i, stmt) in statements.iter().enumerate() {
            if let StatementKind::Expression(id) = &stmt.kind {
                if Some(i) == last_expr_idx {
                    self.compile_expr(*id)?;
                } else {
                    self.compile_expr_statement(*id)?;
                }
            } else {
                self.compile_statement(stmt)?;
            }
        }
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
            | StatementKind::ResolvedConstantArray { value, .. } => {
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
            | StatementKind::ConstantArray { .. } => Err(CompileError(
                "unresolved declaration reached the compiler - run the resolver pass first".into(),
            )),

            StatementKind::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                self.compile_expr(*condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                // resolver unconditionally pushes a scope for `while` bodies
                self.compile_block(body, true, line)?;
                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
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
            self.scope_bases.pop();
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
        for stmt in body {
            sub.compile_statement(stmt)?;
        }
        sub.chunk.write_op(OpCode::Return, 0); // implicit `return null` on fallthrough
        Ok(sub.chunk)
    }
}
