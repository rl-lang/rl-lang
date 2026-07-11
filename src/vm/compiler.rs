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
}
