use std::{
    io::{self, Write},
    panic,
};

use crate::{
    interpreter::evaluator::Evaluator, lexer::tokenizer::Tokenizer, parser::parser_logic::Parser,
    utils::source::SourceFile,
};

pub fn repl() {
    panic::set_hook(Box::new(|_| {}));

    let mut evaluator = Evaluator::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        let source = SourceFile::new("<repl>", input.to_string());
        let tokens = match Tokenizer::lex(source) {
            Ok(t) => t,
            Err(e) => {
                e.report_to_stderr();
                continue;
            }
        };

        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let statements = Parser::parse(tokens);

            for statement in statements {
                evaluator.evaluate_statement(&statement);
            }
        }));
        if result.is_err() {
            eprintln!("error: aborted")
        }
    }

    println!("Exited <<");
}
