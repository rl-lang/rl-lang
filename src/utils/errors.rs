/// represents an Interpreter error with optional line number and error category
pub struct Error {
    /// readable message
    message: String,
    /// line number of error in source file
    line: Option<usize>,
    /// the category and optional context of the error
    reason: Option<ErrorReason>,
}

/// provides an error category with optional error context
pub struct ErrorReason {
    /// error category
    error_type: Reason,
    /// optional lines of error output
    data: Option<Vec<String>>,
}

/// the error category
pub enum Reason {
    /// error occured during parsing
    Parse,
    /// error occured when building the ast
    AST,
    /// error occured during lexing
    Lexer,
    /// error occured during evaluation
    Interpreter,
    /// error orginated from utils
    Utils,
    /// error occured during compilation
    Compile,
    /// error occured during runtime
    Runtime,
}

impl Error {
    /// creates a new [`Error`] with given message , optional line number and optional reason
    ///
    /// # example
    /// ```rust
    /// use rl_lang::utils::errors::Error;
    /// Error::init("unexpected error".to_string(), Some(10), None);
    /// ```
    pub fn init(message: String, line: Option<usize>, reason: Option<ErrorReason>) -> Self {
        log::debug!("Error: {}", message);
        Self {
            message,
            line,
            reason,
        }
    }

    /// prints the error stdout and exits the process with code `1`
    ///
    /// # output format
    /// ```text
    /// [10)unexpected error]
    /// [Lexer Error]
    /// unknown token `$`
    /// ```
    pub fn print_error(&self) {
        match &self.line {
            Some(l) => println!("[{}) Error: {}]", l, self.message),
            None => println!("[Error: {}]", self.message),
        }

        if let Some(r) = &self.reason {
            match &r.data {
                Some(d) => {
                    println!("[{}]", r.get_type_string());
                    for l in d {
                        println!("{}", l);
                    }
                }
                _ => println!("[{}]", r.get_type_string()),
            }
        }

        std::process::exit(1);
    }
}

impl ErrorReason {
    /// creates a new [`ErrorReason`] with category type and optional data
    ///
    /// # example
    /// ```rust
    /// use rl_lang::utils::errors::{ErrorReason, Reason};
    /// ErrorReason::init(Reason::Lexer, Some(vec!["unknown token `$`".to_string()]));
    /// ```
    pub fn init(error_type: Reason, data: Option<Vec<String>>) -> Self {
        Self { error_type, data }
    }

    /// returns the display of category type
    fn get_type_string(&self) -> String {
        match &self.error_type {
            Reason::Parse => "Parse Error",
            Reason::AST => "AST Error",
            Reason::Lexer => "Lexer Error",
            Reason::Interpreter => "Interpreter Error",
            Reason::Utils => "Utils Error",
            Reason::Compile => "Compile Error",
            Reason::Runtime => "Runtime Error",
        }
        .to_string()
    }
}
