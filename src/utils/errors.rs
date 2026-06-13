use std::sync::Arc;

use ariadne::{Color, Label, Report, ReportKind, Source};

use crate::utils::source::SourceFile;
use crate::utils::span::Span;

/// heavy optional fields, heap-allocated so `Error` stays small on the stack
#[derive(Debug)]
struct ErrorDetail {
    /// primary span (anchor of the ariadne report) and its label text
    primary: (Span, String),
    /// secondary spans with labels
    labels: Vec<(Span, String)>,
    /// source string for rendering; supplied by the subsystem that built the error
    source: Option<Arc<String>>,
    /// source file name shown in the report header
    source_name: Option<String>,
    /// optional help/hint line shown after the snippet (e.g. "did you mean foo?")
    help: Option<String>,
}

/// represents an Interpreter error with optional line number and error category
#[derive(Debug)]
pub struct Error {
    /// readable message
    message: String,
    /// line number of error in source file (legacy; superseded by `detail.primary.0` when present)
    line: Option<usize>,
    /// the category and optional context of the error
    reason: Option<ErrorReason>,
    /// boxed span-aware detail; `None` for legacy errors that have no span
    detail: Option<Box<ErrorDetail>>,
}

/// provides an error category with optional error context
#[derive(Debug)]
pub struct ErrorReason {
    /// error category
    error_type: Reason,
    /// optional lines of error output
    data: Option<Vec<String>>,
}

/// the error category
#[derive(Clone, Copy, Debug)]
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
    /// legacy constructor: builds an error with just a message, optional line, and reason.
    /// Used by call sites that haven't been migrated to span-based errors yet.
    pub fn init(message: String, line: Option<usize>, reason: Option<ErrorReason>) -> Self {
        #[cfg(feature = "debug")]
        log::debug!("Error: {}", message);
        Self {
            message,
            line,
            reason,
            detail: None,
        }
    }

    /// builder-style constructor for span-aware errors.
    /// the `span` becomes the primary anchor of the report.
    pub fn at(kind: Reason, message: impl Into<String>, span: Span) -> Self {
        let message = message.into();
        #[cfg(feature = "debug")]
        log::debug!("Error: {}", message);
        Self {
            message: message.clone(),
            line: None,
            reason: Some(ErrorReason::init(kind, None)),
            detail: Some(Box::new(ErrorDetail {
                primary: (span, message),
                labels: Vec::new(),
                source: None,
                source_name: None,
                help: None,
            })),
        }
    }

    /// override the primary label text (defaults to the error message).
    pub fn with_primary_label(mut self, label: impl Into<String>) -> Self {
        if let Some(d) = &mut self.detail {
            d.primary.1 = label.into();
        }
        self
    }

    /// add a secondary label to the report.
    pub fn with_label(mut self, span: Span, label: impl Into<String>) -> Self {
        if let Some(d) = &mut self.detail {
            d.labels.push((span, label.into()));
        }
        self
    }

    /// attach the source string so ariadne can render snippets.
    pub fn with_source(mut self, source: Arc<String>) -> Self {
        if let Some(d) = &mut self.detail {
            d.source = Some(source);
        }
        self
    }

    /// attach a human-readable source name (e.g. file path).
    pub fn with_source_name(mut self, name: impl Into<String>) -> Self {
        if let Some(d) = &mut self.detail {
            d.source_name = Some(name.into());
        }
        self
    }

    /// attach a help/hint line shown beneath the snippet (e.g. "did you mean foo?").
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        if let Some(d) = &mut self.detail {
            d.help = Some(help.into());
        }
        self
    }

    /// attach both the source text and name from a [`SourceFile`].
    pub fn with_source_file(mut self, file: &SourceFile) -> Self {
        if let Some(d) = &mut self.detail {
            d.source = Some(Arc::clone(&file.text));
            d.source_name = Some(file.name.to_string());
        }
        self
    }

    /// prints the error and exits via panic so existing call sites and the REPL keep working.
    ///
    /// uses ariadne when `source` and a primary span are available; falls back to the legacy
    /// text format otherwise.
    pub fn print_error(&self) {
        self.report_to_stderr();
        panic!("rl error");
    }

    /// renders the error to stderr without terminating. used by call sites that already
    /// own their control flow (e.g. anything returning `Result`).
    pub fn report_to_stderr(&self) {
        if let Some(d) = &self.detail
            && let Some(src) = &d.source
        {
            let name: &str = d.source_name.as_deref().unwrap_or("<source>");
            let (sp, primary_label) = &d.primary;
            let mut builder = Report::build(ReportKind::Error, (name, sp.start..sp.end))
                .with_message(&self.message)
                .with_label(
                    Label::new((name, sp.start..sp.end))
                        .with_message(primary_label)
                        .with_color(Color::Red),
                );
            for (lsp, label) in &d.labels {
                builder = builder.with_label(
                    Label::new((name, lsp.start..lsp.end))
                        .with_message(label)
                        .with_color(Color::Yellow),
                );
            }
            if let Some(help) = &d.help {
                builder = builder.with_help(help);
            }
            let _ = builder.finish().eprint((name, Source::from(src.as_str())));
            return;
        }

        self.fallback_text();
    }

    /// legacy text rendering kept verbatim from the original implementation.
    fn fallback_text(&self) {
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
    }

    // adds a way to extract bytes from span in Error
    pub fn span(&self) -> Option<crate::utils::span::Span> {
        self.detail.as_ref().map(|d| d.primary.0)
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

impl Error {
    pub fn message(&self) -> &str {
        &self.message
    }
}
