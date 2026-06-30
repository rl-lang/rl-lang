//! Core evaluator - expression evaluation, function calls, and the runtime state.

use std::sync::Arc;

use crate::interpreter::stdlib::random::xoshiro::Xoshiro256;
use crate::lexer::tokentypes::TokenType;
use crate::resolver::Resolver;
use crate::{
    ast::{
        nodes::{Expression, ExpressionKind},
        statements::TypeAnnotation,
    },
    interpreter::{
        native::{IntoNativeFn, Module},
        stdlib,
        values::Value,
    },
    utils::{
        errors::{Error, Reason},
        source::SourceFile,
        span::Span,
        suggest::closest_match,
    },
};

/// A slot in the environment - holds a value, its declared type, and mutability.
#[derive(Debug, Clone, PartialEq)]
pub struct PItem {
    /// The runtime value stored in this slot.
    pub value: Value,
    /// The declared type of this slot, used for assignment type checking.
    pub type_annotation: TypeAnnotation,
    /// Whether this slot is immutable (`CONST`).
    pub is_const: bool,
}

/// A single environment entry. Currently only [`PItem`] exists;
/// the enum wrapper leaves room for future variants (e.g. closures, records).
#[derive(Debug, Clone, PartialEq)]
pub enum EnvironmentItem {
    PItem(PItem),
}

/// The tree-walking interpreter, carrying all runtime state.
pub struct Evaluator {
    /// The environment stack - each frame is a scope; innermost is last.
    pub environment: Vec<Vec<EnvironmentItem>>,
    /// Source file for Ariadne error rendering; `None` in embedded/test contexts.
    pub source_file: Option<SourceFile>,
    /// The stdlib module tree, used for resolving `std::*` calls.
    pub root_module: Module,
    /// Set by a `return` statement; cleared when the enclosing function call collects it.
    pub return_value: Option<Value>,
    /// Set by `break`; cleared by the enclosing loop after it exits.
    pub is_breaking: bool,
    /// Set by `continue`; cleared by the enclosing loop at the top of the next iteration.
    pub is_continuing: bool,
    /// When `Some`, `print`/`println` write here instead of stdout (used by the LSP and REPL).
    pub output_buffer: Option<String>,
    /// The Xoshiro256** PRNG instance shared across all `std::random` calls.
    pub rng: Xoshiro256,
    /// The resolver, kept alive so import statements can resolve newly loaded files inline.
    pub resolver: Resolver,
    /// Maps top-level function names to their environment slot for `call_path` shortcut.
    pub fn_names: std::collections::HashMap<String, usize>,
    // for diffrent calls
    pub user_args_offset: usize,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            environment: vec![vec![]],
            source_file: None,
            root_module: Module::new(""),
            return_value: None,
            is_breaking: false,
            is_continuing: false,
            output_buffer: None,
            rng: Xoshiro256::default(),
            resolver: Resolver::new(),
            fn_names: std::collections::HashMap::new(),
            user_args_offset: 1,
        }
    }

    /// Attaches a source file for error rendering.
    pub fn with_source_file(mut self, file: SourceFile) -> Self {
        self.source_file = Some(file);
        self
    }

    /// Attaches a source file for error rendering (mutable reference variant).
    pub fn set_source_file(&mut self, file: SourceFile) {
        self.source_file = Some(file);
    }

    /// Registers a [`Module`] as a submodule of the root module.
    pub fn with_module(mut self, m: Module) -> Self {
        self.root_module.submodules.insert(m.name.clone(), m);
        self
    }

    /// Registers a typed Rust function directly on the root module.
    pub fn with_function<F, A>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: IntoNativeFn<A>,
    {
        self.root_module
            .functions
            .insert(name.into(), f.into_native());
        self
    }

    /// Loads the full stdlib into the root module under `std::*`.
    pub fn with_stdlib(self) -> Self {
        self.with_module(
            Module::new("std")
                .with_module(stdlib::math::module())
                .with_module(stdlib::io::module())
                .with_module(stdlib::bitwise::module())
                .with_module(stdlib::string::module())
                .with_module(stdlib::types::module())
                .with_module(stdlib::array::module())
                .with_module(stdlib::path::module())
                .with_module(stdlib::fs::module())
                .with_module(stdlib::random::module())
                .with_module(stdlib::time::module())
                .with_module(stdlib::process::module())
                .with_module(stdlib::result::module())
                .with_module(stdlib::terminal::module())
                .with_module(stdlib::rl::module()),
        )
    }

    pub fn with_user_args_offset(mut self, offset: usize) -> Self {
        self.user_args_offset = offset;
        self
    }

    /// Build a [`Reason::Runtime`] error anchored at `span`, with source attached when known.
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        let err = Error::at(Reason::Runtime, message, span);
        match &self.source_file {
            Some(file) => err.with_source_file(file),
            None => err,
        }
    }

    /// Infers the [`TypeAnnotation`] of a runtime [`Value`].
    ///
    /// For arrays, uses the type of the first element; empty arrays infer `Null`.
    /// `is_const` controls whether the result is the mutable or const variant.
    pub fn infer_type(value: &Value, is_const: bool) -> TypeAnnotation {
        match value {
            Value::Integer(_) => {
                if is_const {
                    TypeAnnotation::CInt
                } else {
                    TypeAnnotation::Int
                }
            }
            Value::Float(_) => {
                if is_const {
                    TypeAnnotation::CFloat
                } else {
                    TypeAnnotation::Float
                }
            }
            Value::String(_) => {
                if is_const {
                    TypeAnnotation::CString
                } else {
                    TypeAnnotation::String
                }
            }
            Value::Bool(_) => {
                if is_const {
                    TypeAnnotation::CBool
                } else {
                    TypeAnnotation::Bool
                }
            }
            Value::Byte(_) => {
                if is_const {
                    TypeAnnotation::CByte
                } else {
                    TypeAnnotation::Byte
                }
            }
            Value::Char(_) => {
                if is_const {
                    TypeAnnotation::CChar
                } else {
                    TypeAnnotation::Char
                }
            }
            Value::Values { items, .. } => {
                let inner = items
                    .first()
                    .map(|v| Self::infer_type(v, false))
                    .unwrap_or(TypeAnnotation::Null);
                if is_const {
                    TypeAnnotation::CArray(Box::new(inner))
                } else {
                    TypeAnnotation::Array(Box::new(inner))
                }
            }
            Value::Null => TypeAnnotation::Null,
            Value::Function { .. } => TypeAnnotation::Fn,
            Value::Tuple(items) => {
                let inner: Vec<TypeAnnotation> =
                    items.iter().map(|v| Self::infer_type(v, false)).collect();
                if is_const {
                    TypeAnnotation::CTuple(inner)
                } else {
                    TypeAnnotation::Tuple(inner)
                }
            }
            Value::Error(_) => {
                if is_const {
                    TypeAnnotation::CError
                } else {
                    TypeAnnotation::Error
                }
            }
            Value::Ok(inner) | Value::Err(inner) => {
                let inner_ty = Self::infer_type(inner, false);
                if is_const {
                    TypeAnnotation::CResult(Box::new(inner_ty))
                } else {
                    TypeAnnotation::Result(Box::new(inner_ty))
                }
            }
        }
    }

    /// Returns `true` if `value`'s inferred type is compatible with `expected`.
    pub fn value_compatible(value: &Value, expected: &TypeAnnotation) -> bool {
        let actual = Self::infer_type(value, false);
        Self::types_compatible(&actual, expected)
    }

    /// Returns `true` if `actual` and `expected` types are assignment-compatible.
    ///
    /// Compatibility rules (beyond equality):
    /// - `Null` is compatible with anything
    /// - `Byte` widens to `Int` / `CInt`
    /// - Arrays are compatible if their element types are compatible (recursive)
    pub fn types_compatible(actual: &TypeAnnotation, expected: &TypeAnnotation) -> bool {
        if actual == expected {
            return true;
        }
        if *actual == TypeAnnotation::Null {
            return true;
        }
        if *expected == TypeAnnotation::Null {
            return true;
        }
        match (actual, expected) {
            (
                TypeAnnotation::Array(a) | TypeAnnotation::CArray(a),
                TypeAnnotation::Array(b) | TypeAnnotation::CArray(b),
            ) => Self::types_compatible(a, b),
            (
                TypeAnnotation::Tuple(a) | TypeAnnotation::CTuple(a),
                TypeAnnotation::Tuple(b) | TypeAnnotation::CTuple(b),
            ) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(x, y)| Self::types_compatible(x, y))
            }
            (
                TypeAnnotation::Error | TypeAnnotation::CError,
                TypeAnnotation::Error | TypeAnnotation::CError,
            ) => true,
            (
                TypeAnnotation::Result(_) | TypeAnnotation::CResult(_),
                TypeAnnotation::Result(_) | TypeAnnotation::CResult(_),
            ) => true,
            _ => false,
        }
    }

    /// Return an error if `value` is [`Value::Null`], pointing at `span`.
    pub fn check_not_null(&self, value: &Value, span: Span) -> Result<(), Error> {
        if matches!(value, Value::Null) {
            Err(self.err("value is null", span))
        } else {
            Ok(())
        }
    }

    pub fn evaluate(&mut self, expression: &Expression) -> Result<Value, Error> {
        let value = match &expression.kind {
            ExpressionKind::Null => Value::Null,
            ExpressionKind::Integer(i) => Value::Integer(*i),
            ExpressionKind::Byte(b) => Value::Byte(*b),
            ExpressionKind::String(s) => Value::String(s.clone()),
            ExpressionKind::Bool(b) => Value::Bool(*b),
            ExpressionKind::Float(f) => Value::Float(*f),
            ExpressionKind::Character(c) => Value::Char(*c),
            ExpressionKind::Index { target, index } => {
                let arr = self.evaluate(target)?;
                self.check_not_null(&arr, target.span)?;
                let idx = self.evaluate(index)?;
                self.check_not_null(&idx, index.span)?;
                match (&arr, &idx) {
                    (Value::Values { items, .. }, Value::Integer(i)) => {
                        let i_usize = *i as usize;
                        if i_usize >= items.len() {
                            return Err(self
                                .err(
                                    format!("index {} out of bounds (len {})", i, items.len()),
                                    expression.span,
                                )
                                .with_label(
                                    target.span,
                                    format!("this array has length {}", items.len()),
                                ));
                        }
                        items[i_usize].clone()
                    }
                    (Value::Values { items, .. }, Value::Byte(b)) => {
                        let b_usize = *b as usize;
                        if b_usize >= items.len() {
                            return Err(self
                                .err(
                                    format!("index {} out of bounds (len {})", b, items.len()),
                                    expression.span,
                                )
                                .with_label(
                                    target.span,
                                    format!("this array has length {}", items.len()),
                                ));
                        }
                        items[b_usize].clone()
                    }

                    (Value::Tuple(items), Value::Integer(i)) => {
                        let i_usize = *i as usize;
                        if i_usize >= items.len() {
                            return Err(self
                                .err(
                                    format!(
                                        "tuple index {} out of bounds (len {})",
                                        i,
                                        items.len()
                                    ),
                                    expression.span,
                                )
                                .with_label(
                                    target.span,
                                    format!("this tuple has {} elements", items.len()),
                                ));
                        }
                        items[i_usize].clone()
                    }
                    (Value::Tuple(items), Value::Byte(b)) => {
                        let b_usize = *b as usize;
                        if b_usize >= items.len() {
                            return Err(self
                                .err(
                                    format!(
                                        "tuple index {} out of bounds (len {})",
                                        b,
                                        items.len()
                                    ),
                                    expression.span,
                                )
                                .with_label(
                                    target.span,
                                    format!("this tuple has {} elements", items.len()),
                                ));
                        }
                        items[b_usize].clone()
                    }
                    _ => {
                        return Err(self
                            .err("invalid index operation", expression.span)
                            .with_label(target.span, format!("this is {}", arr.type_name()))
                            .with_label(index.span, format!("this is {}", idx.type_name())));
                    }
                }
            }
            ExpressionKind::ArrayLiteral(items) => {
                let mut values = Vec::with_capacity(items.len());
                for e in items {
                    values.push(self.evaluate(e)?);
                }
                let items_type = values
                    .first()
                    .map(|v| Self::infer_type(v, false))
                    .unwrap_or(TypeAnnotation::Null);

                // Validate every element after the first against items_type.
                if items_type != TypeAnnotation::Null {
                    for (i, v) in values.into_iter().enumerate() {
                        let actual = Self::infer_type(&v, false);
                        if !Self::types_compatible(&actual, &items_type) {
                            return Err(self.err(
                                format!(
                                    "array element type mismatch: element {} is {:?}, expected {:?}",
                                    i,
                                    actual,
                                    items_type,
                                ),
                                expression.span,
                            ));
                        }
                    }
                    Value::Values {
                        items_type,
                        items: values,
                    }
                } else {
                    Value::Values {
                        items_type,
                        items: values,
                    }
                }
            }

            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => self.index_assign(target, index, value, expression.span)?,
            ExpressionKind::Grouping(inner) => self.evaluate(inner)?,
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                if matches!(operator, TokenType::And) {
                    let l = self.evaluate(left)?;
                    if let Value::Bool(false) = l {
                        return Ok(Value::Bool(false));
                    }
                    let r = self.evaluate(right)?;
                    return Ok(Value::Bool(matches!(r, Value::Bool(true))));
                }

                if matches!(operator, TokenType::Or) {
                    let l = self.evaluate(left)?;
                    if let Value::Bool(true) = l {
                        return Ok(Value::Bool(true));
                    }
                    let r = self.evaluate(right)?;
                    return Ok(Value::Bool(matches!(r, Value::Bool(true))));
                }

                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                if matches!(operator, TokenType::Compare | TokenType::BangEqual)
                    && (matches!(left_val, Value::Null) || matches!(right_val, Value::Null))
                {
                    let result = matches!((&left_val, &right_val), (Value::Null, Value::Null));
                    return Ok(if matches!(operator, TokenType::Compare) {
                        Value::Bool(result)
                    } else {
                        Value::Bool(!result)
                    });
                }
                self.check_not_null(&left_val, left.span)?;
                self.check_not_null(&right_val, right.span)?;
                self.match_binary_operator(
                    left_val,
                    left.span,
                    right_val,
                    right.span,
                    operator,
                    expression.span,
                )?
            }
            ExpressionKind::Unary { operator, operand } => {
                let operand_val = self.evaluate(operand)?;
                self.check_not_null(&operand_val, operand.span)?;
                self.match_unary_operator(operand_val, operand.span, operator, expression.span)?
            }
            ExpressionKind::ResolvedIdentifier { depth, slot, .. } => {
                self.get_value(*depth, *slot, expression.span)?
            }
            ExpressionKind::ResolvedAssign {
                depth, slot, value, ..
            } => {
                let val = self.evaluate(value)?;

                let inferred_type = Self::infer_type(&val, false);
                self.assign_value(*depth, *slot, val.clone(), inferred_type, expression.span)?;
                val
            }
            ExpressionKind::Call { path, args } => {
                let mut evaluated_args = Vec::with_capacity(args.len());
                for arg in args {
                    let val = self.evaluate(arg)?;
                    evaluated_args.push(val);
                }
                self.call_path(path, evaluated_args, expression.span)?
            }

            ExpressionKind::CallExpr { callee, args } => {
                let func_val = self.evaluate(callee)?;
                let mut evaluated_args = Vec::with_capacity(args.len());
                for arg in args {
                    let val = self.evaluate(arg)?;
                    evaluated_args.push(val);
                }
                self.call_value(func_val, evaluated_args, expression.span)?
            }

            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => {
                let first_arg = self.evaluate(caller)?;
                let mut evaluated_args = vec![first_arg];
                // elevate and push the args
                for arg in args {
                    evaluated_args.push(self.evaluate(arg)?);
                }
                self.call_path(method, evaluated_args, expression.span)?
            }
            ExpressionKind::ResolvedLambda {
                params,
                return_type,
                body,
                capture_depth,
            } => {
                let total = self.environment.len();
                let start = if total > *capture_depth {
                    total - capture_depth
                } else {
                    0
                };
                let captured_env: Vec<Vec<EnvironmentItem>> = self.environment[start..].to_vec();

                Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    return_type: return_type.clone(),
                    captured_env,
                }
            }

            ExpressionKind::Identifier(name) => {
                return Err(self.err(format!("undefined variable '{}'", name), expression.span));
            }
            ExpressionKind::Assign { name, .. } => {
                return Err(self.err(format!("undefined variable '{}'", name), expression.span));
            }

            ExpressionKind::Cast { value, target_type } => {
                let val = self.evaluate(value)?;
                self.check_not_null(&val, value.span)?;
                match (&val, target_type) {
                    (Value::Integer(n), TypeAnnotation::Float) => Value::Float(*n as f64),
                    (Value::Integer(n), TypeAnnotation::Byte) => Value::Byte(*n as u8),
                    (Value::Integer(_), TypeAnnotation::Int) => val,
                    (Value::Float(f), TypeAnnotation::Int) => Value::Integer(*f as i64),
                    (Value::Float(f), TypeAnnotation::Byte) => Value::Byte(*f as u8),
                    (Value::Float(_), TypeAnnotation::Float) => val,
                    (Value::Byte(b), TypeAnnotation::Float) => Value::Float(*b as f64),
                    (Value::Byte(b), TypeAnnotation::Int) => Value::Integer(*b as i64),
                    (Value::Byte(_), TypeAnnotation::Byte) => val,

                    _ => {
                        return Err(self.err(
                            format!(
                                "invalid cast: cannot cast {} to {:?}",
                                val.type_name(),
                                target_type
                            ),
                            expression.span,
                        ));
                    }
                }
            }

            ExpressionKind::TupleLiteral(items) => {
                let mut values = Vec::with_capacity(items.len());
                for e in items {
                    values.push(self.evaluate(e)?);
                }
                Value::Tuple(values)
            }
            ExpressionKind::ErrorLiteral(inner) => {
                let val = self.evaluate(inner)?;
                if matches!(val, Value::Error(_)) {
                    return Err(self.err("error cannot wrap another error", expression.span));
                }
                Value::Error(Box::new(val))
            }
            ExpressionKind::OkLiteral(inner) => {
                let val = self.evaluate(inner)?;
                Value::Ok(Box::new(val))
            }
            ExpressionKind::ErrLiteral(inner) => {
                let val = self.evaluate(inner)?;
                Value::Err(Box::new(val))
            }

            ExpressionKind::Propagate(inner) => {
                let val = self.evaluate(inner)?;
                match val {
                    Value::Ok(v) => *v,
                    Value::Err(_) => {
                        self.return_value = Some(val);
                        return Ok(Value::Null);
                    }
                    other => other,
                }
            }

            _ => Value::Null,
        };
        Ok(value)
    }

    /// Calls a [`Value::Function`], setting up the captured environment, inserting
    /// arguments into a new scope, running the body, and validating the return type.
    ///
    /// Global scope mutations made inside the call are propagated back to the caller.
    pub fn call_value(
        &mut self,
        func: Value,
        args: Vec<Value>,
        span: Span,
    ) -> Result<Value, Error> {
        if let Value::Function {
            params,
            body,
            return_type,
            captured_env,
        } = func
        {
            if params.len() != args.len() {
                return Err(self.err(
                    format!(
                        "function '' expects {} argument(s), got {}",
                        params.len(),
                        args.len()
                    ),
                    span,
                ));
            }

            let saved_env = std::mem::take(&mut self.environment);
            let saved_return = self.return_value.take();

            if captured_env.is_empty() {
                self.environment = vec![saved_env[0].clone()]
            } else {
                self.environment = captured_env;
            }
            self.push_scope();

            for (slot, (_, arg)) in params.iter().zip(args).enumerate() {
                let arg_type = Self::infer_type(&arg, false);
                self.insert_value(slot, arg, arg_type, span)?;
            }

            for statement in &body {
                self.evaluate_statement(statement)?;
                if self.return_value.is_some() {
                    break;
                }
            }

            let result = self.return_value.take().unwrap_or(Value::Null);

            let updated_global = self.environment[0].clone();
            self.environment = saved_env;
            self.environment[0] = updated_global;
            self.return_value = saved_return;

            if let Some(expected) = &return_type
                && *expected != TypeAnnotation::Null
            {
                let actual = Self::infer_type(&result, false);
                if !Self::types_compatible(&actual, expected) {
                    return Err(self.err(
                        format!(
                            "function '' declared to return {:?} but returned {:?}",
                            expected, actual
                        ),
                        span,
                    ));
                }
            }

            return Ok(result);
        }

        Err(self.err("value is not callable", span))
    }

    /// Resolves a call path and dispatches to either a stdlib [`NativeFn`] or a
    /// user-defined function looked up via [`fn_names`].
    ///
    /// Resolution order:
    /// 1. Full stdlib path via [`root_module.resolve`]
    /// 2. Single-name shorthand via [`fn_names`]
    /// 3. Error with "did you mean?" suggestion from stdlib keywords
    pub fn call_path(
        &mut self,
        path: &[String],
        args: Vec<Value>,
        span: Span,
    ) -> Result<Value, Error> {
        if let Some(f) = self.root_module.resolve(path) {
            let f = Arc::clone(f);
            return match f(self, args, span) {
                Ok(v) => Ok(v),
                Err(e) if e.span().is_some() => Err(match &self.source_file {
                    Some(file) => e.with_source_file(file),
                    None => e,
                }),
                Err(e) => Err(self.err(e.message(), span)),
            };
        }
        if path.len() == 1
            && let Some(&slot) = self.fn_names.get(&path[0])
        {
            let func = self.get_value(0, slot, span)?;
            return self.call_value(func, args, span);
        }
        let mut err = self.err(format!("undefined function {}", path.join("::")), span);
        // suggest a stdlib leaf name if the last segment is a close typo
        if let Some(last) = path.last() {
            let candidates = stdlib::math::KEYWORDS
                .iter()
                .chain(stdlib::math::constants::KEYWORDS)
                .chain(stdlib::bitwise::KEYWORDS)
                .chain(stdlib::io::KEYWORDS)
                .chain(stdlib::string::KEYWORDS)
                .chain(stdlib::types::KEYWORDS)
                .chain(stdlib::array::KEYWORDS)
                .chain(stdlib::path::KEYWORDS)
                .chain(stdlib::fs::KEYWORDS)
                .chain(stdlib::random::KEYWORDS)
                .chain(stdlib::time::KEYWORDS)
                .chain(stdlib::process::KEYWORDS)
                .chain(stdlib::result::KEYWORDS)
                .chain(stdlib::terminal::KEYWORDS)
                .chain(stdlib::rl::KEYWORDS)
                .copied();
            if let Some(suggestion) = closest_match(last, candidates) {
                err = err.with_help(format!("did you mean `{}`?", suggestion));
            }
        }
        Err(err)
    }
}
