mod variable_declaration;

use crate::{
    ast::{nodes::Expression, statements::Statement},
    lexer::tokentypes::TokenType,
    parser::parser::Parser,
};

impl Parser {
    pub fn parse_statement_to_ast(&mut self) -> Statement {
        match self.peek() {
            TokenType::Newline => {
                self.advance();
                Statement::Expression(Expression::Integer(0))
            }
            TokenType::Dec => {
                self.advance();
                self.parse_variable_declartion()
                // println!("{:?}", stmt);
            }
            _ => {
                let expr = self.parse_expression();
                Statement::Expression(expr)
                // println!("{:?}", expr);
                // println!("{:?}", evaluator.evaluate(&expr));
            }
        }
    }
}
