use crate::ast::nodes::Expression;

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        type_annotation: TypeAnnotation,
        value: Expression,
    },
    ConstantDeclaration {
        name: String,
        type_annotation: TypeAnnotation,
        value: Expression,
    },
    Array {
        name: String,
        type_annotation: TypeAnnotation,
        value: Vec<Expression>,
    },
    ConstantArray {
        name: String,
        type_annotation: TypeAnnotation,
        value: Vec<Expression>,
    },
    Expression(Expression),
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        initializer: Box<Statement>,
        condition: Expression,
        increment: Expression,
        body: Vec<Statement>,
    },
    ForRange {
        variable: Expression,
        range: Box<Statement>,
        body: Vec<Statement>,
    },
    Range(Vec<Expression>),
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
    CInt,
    CFloat,
    CBool,
    CString,
    CChar,
    CArray(Box<TypeAnnotation>),
}
