//! Function declaration parser (`fn`).
//!
//! Handles named function declarations in the form:
//!
//! ```text
//! fn name(T param, T param) -> ReturnType {
//!     body
//! }
//! ```
//!
//! Return type annotations are optional; omitting `-> T` defaults to
//! [`TypeAnnotation::Null`]. The `is_entry` flag is set when the function is
//! preceded by a `!#[entry]` attribute, marking it as the program entry point.

use crate::parser_logic::Parser;
use rl_ast::statements::{ContractClause, FunctionAttribute, ItemAttribute, Param, ParamRefinement, RefineOp, RefineOperand, Statement, StatementKind, TypeAnnotation};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Parser {
    /// Parses a named function declaration.
    ///
    /// Called after `fn` has been consumed (either by [`parse_statement_to_ast`]
    /// or [`parse_entry_attribute`]). Reads:
    ///
    /// 1. The function name (identifier).
    /// 2. A `(`-delimited, comma-separated parameter list of `T name` pairs.
    /// 3. An optional `-> T` return type annotation (defaults to
    ///    [`TypeAnnotation::Null`] when absent).
    /// 4. The function body via [`parse_block`].
    ///
    /// Produces [`StatementKind::FunctionDeclaration`].
    ///
    /// # Parameters
    /// - `start` - the span of the `fn` token, used as the start of the overall span.
    /// - `is_entry` - `true` when the function was marked with `!#[entry]`.
    ///
    /// # Errors
    /// Returns an error if the function name, a parameter name, or the body
    /// block are missing or malformed.
    ///
    /// [`parse_statement_to_ast`]: Parser::parse_statement_to_ast
    /// [`parse_entry_attribute`]: Parser::parse_entry_attribute
    /// [`parse_block`]: Parser::parse_block
    pub fn parse_function(
        &mut self,
        start: Span,
        attribute: Option<FunctionAttribute>,
        item_attributes: Vec<ItemAttribute>,
    ) -> Result<Statement, Error> {
        while self.match_type(&[TokenType::Newline]) {}
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected function name", self.peek_span())),
        };

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::LeftParen]) {
            return Err(self.err("expected `(` after function name", self.peek_span()));
        }

        let mut params: Vec<Param> = Vec::new();
        while self.match_type(&[TokenType::Newline]) {}
        while !self.match_type(&[TokenType::RightParen]) {
            let param_type = self.parse_param_type()?;
            while self.match_type(&[TokenType::Newline]) {}
            match self.peek() {
                TokenType::Identifier(p) => {
                    self.advance();
                    // Optional refinement predicate (`int amt: >0`).
                    let refinement = if self.match_type(&[TokenType::Colon]) {
                        Some(self.parse_param_refinement()?)
                    } else {
                        None
                    };
                    params.push(Param {
                        param_name: p,
                        param_type,
                        refinement,
                    });
                }
                _ => return Err(self.err("expected parameter name", self.peek_span())),
            }
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Comma]) {
                while self.match_type(&[TokenType::Newline]) {}
                if !self.match_type(&[TokenType::RightParen]) {
                    return Err(self.err("expected `)` after function parameters", self.peek_span()));
                }
                break;
            }
            while self.match_type(&[TokenType::Newline]) {}
        }

        // optional return type annotation; defaults to Null when omitted
        while self.match_type(&[TokenType::Newline]) {}
        let return_type = if self.match_type(&[TokenType::Arrow]) {
            while self.match_type(&[TokenType::Newline]) {}
            match self.parse_param_type() {
                Ok(a) => a,
                Err(_) => TypeAnnotation::Null,
            }
        } else {
            TypeAnnotation::Null
        };

        // optional contract clauses (`requires`/`ensures`) before the body
        while self.match_type(&[TokenType::Newline]) {}
        let mut requires = Vec::new();
        let mut ensures = Vec::new();
        loop {
            match self.peek() {
                TokenType::Identifier(name) if name == "requires" => {
                    self.advance();
                    requires.extend(self.parse_contract_items("requires")?);
                }
                TokenType::Identifier(name) if name == "ensures" => {
                    self.advance();
                    ensures.extend(self.parse_contract_items("ensures")?);
                }
                _ => break,
            }
            while self.match_type(&[TokenType::Newline]) {}
        }

        while self.match_type(&[TokenType::Newline]) {}
        let body = self.parse_block()?;

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
                attribute,
                item_attributes,
                requires,
                ensures,
            },
            span,
        ))
    }

    /// Parses one parameter refinement predicate (`>0`, `>=amt`, `=="x"`).
    /// Called after the `:` following a parameter name.
    fn parse_param_refinement(&mut self) -> Result<ParamRefinement, Error> {
        while self.match_type(&[TokenType::Newline]) {}
        let op = match self.peek() {
            TokenType::Greater => RefineOp::Gt,
            TokenType::GreaterEqual => RefineOp::Ge,
            TokenType::Less => RefineOp::Lt,
            TokenType::LessEqual => RefineOp::Le,
            TokenType::Compare => RefineOp::Eq,
            TokenType::BangEqual => RefineOp::Ne,
            _ => return Err(self.err("expected a comparison operator", self.peek_span())),
        };
        self.advance();
        while self.match_type(&[TokenType::Newline]) {}
        let operand = match self.peek() {
            TokenType::NumberLiteral(n) => {
                let v = i64::try_from(n).map_err(|_| {
                    self.err(
                        format!(
                            "value {} is out of range for int ({}..={})",
                            n,
                            i64::MIN,
                            i64::MAX
                        ),
                        self.peek_span(),
                    )
                })?;
                self.advance();
                RefineOperand::Integer(v)
            }
            TokenType::StringLiteral(s) => {
                self.advance();
                RefineOperand::Str(s)
            }
            TokenType::BoolLiteral(b) => {
                self.advance();
                RefineOperand::Bool(b)
            }
            TokenType::Identifier(p) => {
                self.advance();
                RefineOperand::Param(p)
            }
            _ => {
                return Err(self.err(
                    "expected a literal or parameter name",
                    self.peek_span(),
                ))
            }
        };
        Ok(ParamRefinement { op, operand })
    }

    /// Parses one `requires`/`ensures` item list: comma-separated
    /// `condition [, "message"]` pairs. A string literal right after a
    /// comma belongs to the preceding condition as its message; otherwise
    /// the comma starts the next condition.
    fn parse_contract_items(&mut self, kind: &str) -> Result<Vec<ContractClause>, Error> {
        let mut items = Vec::new();
        loop {
            while self.match_type(&[TokenType::Newline]) {}
            let condition = self.parse_expression()?;
            let mut message = None;
            while self.match_type(&[TokenType::Newline]) {}
            if self.match_type(&[TokenType::Comma]) {
                while self.match_type(&[TokenType::Newline]) {}
                if let TokenType::StringLiteral(s) = self.peek() {
                    self.advance();
                    let span = self.previous_span();
                    message = Some(self.ast_arena.alloc_expr(
                        rl_ast::nodes::ExpressionKind::String(s),
                        span,
                    ));
                    while self.match_type(&[TokenType::Newline]) {}
                    if self.match_type(&[TokenType::Comma]) {
                        items.push(ContractClause { condition, message });
                        while self.match_type(&[TokenType::Newline]) {}
                        continue;
                    }
                } else {
                    items.push(ContractClause { condition, message });
                    while self.match_type(&[TokenType::Newline]) {}
                    continue;
                }
            }
            items.push(ContractClause { condition, message });
            match self.peek() {
                TokenType::LeftBrace => break,
                TokenType::Identifier(name) if name == "requires" || name == "ensures" => break,
                _ => {
                    return Err(self.err(
                        format!("expected `{{` or another contract clause after `{kind}` item"),
                        self.peek_span(),
                    ))
                }
            }
        }
        if items.is_empty() {
            return Err(self.err(format!("`{kind}` needs at least one condition"), self.peek_span()));
        }
        Ok(items)
    }
}
