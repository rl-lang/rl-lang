use crate::ast::nodes::Expression;
use crate::utils::span::Span;

/// A statement paired with its source span.
#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone)]
pub enum StatementKind {
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
        variable: String,
        range: Box<Statement>,
        body: Vec<Statement>,
    },
    ForEach {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Range(Vec<i64>),
    ConditionalBranch {
        condition: Option<Expression>,
        body: Vec<Statement>,
    },
    Conditional {
        if_branch: Box<Statement>,
        elseif_branch: Option<Vec<Statement>>,
        else_branch: Option<Box<Statement>>,
    },

    FunctionDeclaration {
        name: String,
        params: Vec<Param>,
        return_type: TypeAnnotation,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),

    Break,
    Continue,

    /// the start of awesomeness
    Import {
        /// list of the functions name
        names: Vec<String>,
        /// list of paths to functions
        path: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
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
    Fn,
    Null,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub param_name: String,
    pub param_type: TypeAnnotation,
}
