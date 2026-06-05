use crate::{
    ast::statements::Statement, lexer::tokentypes::TokenType, parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    pub fn parse_import(&mut self) -> Statement {
        // target would be
        // use println, print from std::display
        // which means use will be consumed when found by parser
        // println, print will be identifiers and be thrown in the Vec<String>
        // std::display will be separated to std display and thrown in another Vec<String>
        // checks for comma and coloncolon ummm might make loop

        let mut names = Vec::new();

        loop {
            match self.peek() {
                TokenType::Identifier(name) => {
                    // consuming the token
                    self.advance();
                    // adding the target to names list
                    names.push(name);
                }
                _ => {
                    Error::init("expected identifier after 'get'".to_string(), None, None)
                        .print_error();
                    unreachable!()
                }
            }

            if !self.match_type(&[TokenType::Comma]) {
                break;
            }
        }

        if !self.match_type(&[TokenType::From]) {
            Error::init("expected 'from' after names".to_string(), None, None).print_error();
        }

        let mut path = Vec::new();
        loop {
            match self.peek() {
                TokenType::Identifier(segment) => {
                    // consuming the token
                    self.advance();
                    // adding the target to names list
                    path.push(segment);
                }
                _ => {
                    Error::init("expected path after 'from'".to_string(), None, None).print_error();
                    unreachable!()
                }
            }

            if !self.match_type(&[TokenType::ColonColon]) {
                break;
            }
        }

        Statement::Import { names, path }
    }
}
