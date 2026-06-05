use rl_lang::{
    ast::{
        nodes::Expression,
        statements::{Statement, TypeAnnotation},
    },
    lexer::tokentypes::TokenType,
};

use crate::common;

#[test]
fn for_c() {
    let statements = common::parse("for [int i = 1, i < 10, i += 1] {0}");
    assert_eq!(
        statements[0],
        Statement::For {
            initializer: Box::new(Statement::VariableDeclaration {
                name: "i".to_string(),
                type_annotation: TypeAnnotation::Int,
                value: Expression::Integer(1)
            }),
            condition: Expression::Binary {
                left: Box::new(Expression::Identifier("i".to_string())),
                operator: TokenType::Less,
                right: Box::new(Expression::Integer(10)),
            },
            increment: Expression::Assign {
                name: "i".to_string(),
                value: Box::new(Expression::Binary {
                    left: Box::new(Expression::Identifier("i".to_string())),
                    operator: TokenType::Plus,
                    right: Box::new(Expression::Integer(1))
                })
            },

            body: vec![Statement::Expression(Expression::Integer(0))]
        }
    );
}

#[test]
fn for_range() {
    let statements = common::parse("for i in 1..10 {0}");
    assert_eq!(
        statements[0],
        Statement::ForRange {
            variable: "i".to_string(),
            range: Box::new(Statement::Range(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])),
            body: vec![Statement::Expression(Expression::Integer(0))]
        }
    );
}

#[test]
fn for_iterable() {
    let statements = common::parse("for i in [1,2,3,4,5,6,7,8,9] {0}");
    assert_eq!(
        statements[0],
        Statement::ForRange {
            variable: "i".to_string(),
            range: Box::new(Statement::Range(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])),
            body: vec![Statement::Expression(Expression::Integer(0))]
        }
    );
}
