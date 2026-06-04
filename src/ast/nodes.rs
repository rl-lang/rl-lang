use crate::lexer::tokentypes;

#[derive(Debug)]
pub enum Expression {
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
    Assign {
        name: String,
        value: Box<Expression>,
    },
    Call {
        name: String,
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
}
