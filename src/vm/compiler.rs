use crate::ast::{
    Ast, ExprId, nodes::ExpressionKind, statements::Statement, statements::StatementKind,
};
use crate::lexer::tokentypes::TokenType;
use crate::vm::chunk::{Chunk, OpCode, VmValue};

#[derive(Debug)]
pub struct CompileError(pub String);

pub struct Compiler<'a> {
    ast: &'a Ast,
    chunk: Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self {
            ast,
            chunk: Chunk::new(),
        }
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
            StatementKind::ResolvedVariableDeclaration { slot, value, .. }
            | StatementKind::ResolvedConstantDeclaration { slot, value, .. } => {
                self.compile_expr(*value)?;
                self.chunk.write_op(OpCode::DefineLocal, line);
                self.chunk.write_u16(*slot as u16, line);
                Ok(())
            }

            StatementKind::VariableDeclaration { .. }
            | StatementKind::ConstantDeclaration { .. } => Err(CompileError(
                "unresolved declaration reached the compiler - run the resolver pass first".into(),
            )),

            other => Err(CompileError(format!(
                "statement kind not yet supported by the vm compiler: {other:?}"
            ))),
        }
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
            ExpressionKind::String(v) => self.emit_const(VmValue::Str(v.clone()), line),

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
                self.chunk.write_op(OpCode::GetLocal, line);
                self.chunk.write_u16(*depth as u16, line);
                self.chunk.write_u16(*slot as u16, line);
            }

            ExpressionKind::ResolvedAssign {
                depth, slot, value, ..
            } => {
                self.compile_expr(*value)?;
                self.chunk.write_op(OpCode::SetLocal, line);
                self.chunk.write_u16(*depth as u16, line);
                self.chunk.write_u16(*slot as u16, line);
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
}
