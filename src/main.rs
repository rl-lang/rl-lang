use crate::{interpreter::evaluator::Evaluator, parser::parser::Parser};

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod utils;

fn main() {
    // arguments
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() != 3 || arguments[1] != "run" {
        eprintln!("Usage: rlp run <source-file.rl>");
        return;
    }

    let source_file = match std::fs::read_to_string(&arguments[2]) {
        Ok(file) => file,
        Err(_) => {
            utils::errors::Error::init(
                "Failed To Read File".to_string(),
                None,
                Some(utils::errors::ErrorReason::init(
                    utils::errors::Reason::Compile,
                    None,
                )),
            )
            .print_error();
            return;
        }
    };
    println!("[source file: {}]", arguments[2]);

    // debug prints the selected file
    // println!("{}", source_file);

    let lexer = lexer::tokenizer::Tokenizer::lex(&source_file);

    // let mut current_debug_line: usize = 0;
    // for token in lexer.iter() {
    //    if token.line != current_debug_line {
    //        print!("\n{} ", current_debug_line);
    //        current_debug_line += 1;
    //    }
    //    if matches!(token.token, lexer::tokentypes::TokenType::Newline)
    //        || matches!(token.token, lexer::tokentypes::TokenType::Eof)
    //    {
    //        continue;
    //    }
    //    print!("[{}] ", token.lexeme);
    // }

    let parser = Parser::parse(lexer);
}
