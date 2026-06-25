//! Abstract Syntax Tree (AST) node definitions.
//!
//! # Pipeline position
//! ```text
//! source -> Lexer -> [Token] -> Parser -> [Statement] -> Checker -> Evaluator
//! ```
//!
//! # Module layout
//! - [`nodes`] - [`Expression`] and [`ExpressionKind`]: the expression AST
//! - [`statements`] - [`Statement`], [`StatementKind`], [`TypeAnnotation`], [`Param`]
//!
//! [`Expression`]: nodes::Expression
//! [`Statement`]: statements::Statement
pub mod nodes;
pub mod statements;
// pub mod variables;
pub type ScopeMap = Vec<Vec<String>>;
