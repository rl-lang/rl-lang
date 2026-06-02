mod stdlib_functions;
mod variable_declaration;
use crate::{
    interpreter::evaluator::Evaluator, lexer::tokentypes::TokenType, parser::parser::Parser,
};

impl Parser {
    pub fn parse_statement(&mut self, evaluator: &mut Evaluator) {
        match self.peek() {
            TokenType::Newline => self.advance(),
            TokenType::Dec => {
                self.advance();
                let stmt = self.parse_variable_declartion();
                evaluator.evaluate_statement(&stmt);
                // println!("{:?}", stmt);
            }
            _ => {
                let expr = self.parse_expression();
                // println!("{:?}", expr);
                // println!("{:?}", evaluator.evaluate(&expr));
                evaluator.evaluate(&expr);
            }
        }
    }
}
