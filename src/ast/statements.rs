use crate::{ast::nodes::Expression, lexer::tokentypes::TokenType};

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        type_annotation: TokenType,
        value: Expression,
    },
    Array {
        name: String,
        type_annotation: TypeAnnotation,
        value: Vec<Expression>,
    },
    Expression(Expression),
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    ConditionalBranch {
        condition: Option<Expression>,
        body: Vec<Statement>,
    },
    Conditional {
        if_branch: Box<Statement>,
        elseif_branch: Option<Vec<Statement>>,
        else_branch: Option<Box<Statement>>,
    },
}

#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Int,
    Float,
    Bool,
    String,
    Char,
    Array(Box<TypeAnnotation>),
}
