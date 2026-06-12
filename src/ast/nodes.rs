use crate::ast::statements::{Param, Statement, TypeAnnotation};
use crate::lexer::tokentypes;
use crate::utils::span::Span;

/// An expression paired with its source span.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Null,
    Integer(i64),
    Binary {
        left: Box<Expression>,
        operator: tokentypes::TokenType,
        right: Box<Expression>,
    },
    Unary {
        operator: tokentypes::TokenType,
        operand: Box<Expression>,
    },
    Grouping(Box<Expression>),
    String(String),
    Bool(bool),
    Float(f64),
    Character(char),
    Identifier(String),
    ArrayLiteral(Vec<Expression>),
    Assign {
        name: String,
        value: Box<Expression>,
    },
    Call {
        path: Vec<String>,
        args: Vec<Expression>,
    },

    MethodCall {
        caller: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },
    IndexAssign {
        target: Box<Expression>,
        index: Box<Expression>,
        value: Box<Expression>,
    },

    Lambda {
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Statement>,
    },
    CallExpr {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
}
