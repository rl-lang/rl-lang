use std::collections::HashMap;
use std::sync::Arc;

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

pub struct PItem {
    pub value: Value,
    pub type_annotation: TypeAnnotation,
    pub is_const: bool,
}

pub enum EnvironmentItem {
    PItem(PItem),
}

pub struct Evaluator {
    pub environment: Vec<HashMap<String, EnvironmentItem>>,
    pub source_file: Option<SourceFile>,
    pub root_module: Module,
    pub return_value: Option<Value>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            environment: vec![HashMap::new()],
            source_file: None,
            root_module: Module::new(""),
            return_value: None,
        }
    }

    pub fn with_source_file(mut self, file: SourceFile) -> Self {
        self.source_file = Some(file);
        self
    }

    pub fn set_source_file(&mut self, file: SourceFile) {
        self.source_file = Some(file);
    }

    pub fn with_module(mut self, m: Module) -> Self {
        self.root_module.submodules.insert(m.name.clone(), m);
        self
    }

    pub fn with_function<F, A>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: IntoNativeFn<A>,
    {
        self.root_module
            .functions
            .insert(name.into(), f.into_native());
        self
    }

    pub fn with_stdlib(self) -> Self {
        self.with_module(
            Module::new("std")
                .with_module(stdlib::math::module())
                .with_module(stdlib::display::module())
                .with_module(stdlib::io::module()),
        )
    }

    /// Build a [`Reason::Runtime`] error anchored at `span`, with source attached when known.
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        let err = Error::at(Reason::Runtime, message, span);
        match &self.source_file {
            Some(file) => err.with_source_file(file),
            None => err,
        }
    }

    pub fn evaluate(&mut self, expression: &Expression) -> Result<Value, Error> {
        let value = match &expression.kind {
            ExpressionKind::Integer(i) => Value::Integer(*i),
            ExpressionKind::String(s) => Value::String(s.clone()),
            ExpressionKind::Bool(b) => Value::Bool(*b),
            ExpressionKind::Float(f) => Value::Float(*f),
            ExpressionKind::Character(c) => Value::Char(*c),
            ExpressionKind::Index { target, index } => {
                let arr = self.evaluate(target)?;
                let idx = self.evaluate(index)?;
                match (&arr, &idx) {
                    (Value::Values(items), Value::Integer(i)) => {
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
                Value::Values(values)
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
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
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
                self.match_unary_operator(operand_val, operand.span, operator, expression.span)?
            }
            ExpressionKind::Identifier(name) => self.get_value(name, expression.span)?,
            ExpressionKind::Assign { name, value } => {
                let val = self.evaluate(value)?;
                let inferred_type = match &val {
                    Value::Integer(_) => TypeAnnotation::Int,
                    Value::Float(_) => TypeAnnotation::Float,
                    Value::String(_) => TypeAnnotation::String,
                    Value::Bool(_) => TypeAnnotation::Bool,
                    Value::Char(_) => TypeAnnotation::Char,
                    Value::Values(items) => {
                        let inner = items
                            .first()
                            .map(|v| match v {
                                Value::Integer(_) => TypeAnnotation::Int,
                                Value::Float(_) => TypeAnnotation::Float,
                                Value::String(_) => TypeAnnotation::String,
                                Value::Bool(_) => TypeAnnotation::Bool,
                                Value::Char(_) => TypeAnnotation::Char,
                                _ => TypeAnnotation::Null,
                            })
                            .unwrap_or(TypeAnnotation::Null);
                        TypeAnnotation::Array(Box::new(inner))
                    }
                    Value::Null => TypeAnnotation::Null,
                    Value::Function { .. } => TypeAnnotation::Fn,
                };
                self.assign_value(name.clone(), val.clone(), inferred_type, expression.span)?;
                val
            }
            ExpressionKind::Call { path, args } => {
                let mut evaluated_args = Vec::with_capacity(args.len());
                for arg in args {
                    evaluated_args.push(self.evaluate(arg)?);
                }
                self.call_path(path, evaluated_args, expression.span)?
            }
        };
        Ok(value)
    }

    pub fn call_path(
        &mut self,
        path: &[String],
        args: Vec<Value>,
        span: Span,
    ) -> Result<Value, Error> {
        if let Some(f) = self.root_module.resolve(path) {
            let f = Arc::clone(f);
            return Ok(f(self, args));
        }

        // detect user defined functions
        if path.len() == 1 {
            let func = self.get_value(&path[0], span)?;

            if let Value::Function {
                params,
                body,
                return_type,
            } = func
            {
                if params.len() != args.len() {
                    return Err(self.err(
                        format!(
                            "function '{}' expects {} argument(s), got {}",
                            path[0],
                            params.len(),
                            args.len()
                        ),
                        span,
                    ));
                }

                let saved_env = std::mem::take(&mut self.environment);
                self.environment = vec![HashMap::new()];

                for (param, arg) in params.iter().zip(args) {
                    let arg_type = match &arg {
                        Value::Integer(_) => TypeAnnotation::Int,
                        Value::Float(_) => TypeAnnotation::Float,
                        Value::String(_) => TypeAnnotation::String,
                        Value::Bool(_) => TypeAnnotation::Bool,
                        Value::Char(_) => TypeAnnotation::Char,
                        Value::Values(items) => {
                            let inner = items
                                .first()
                                .map(|v| match v {
                                    Value::Integer(_) => TypeAnnotation::Int,
                                    Value::Float(_) => TypeAnnotation::Float,
                                    Value::String(_) => TypeAnnotation::String,
                                    Value::Bool(_) => TypeAnnotation::Bool,
                                    Value::Char(_) => TypeAnnotation::Char,
                                    _ => TypeAnnotation::Null,
                                })
                                .unwrap_or(TypeAnnotation::Null);
                            TypeAnnotation::Array(Box::new(inner))
                        }
                        Value::Null => TypeAnnotation::Null,
                        Value::Function { .. } => TypeAnnotation::Fn,
                    };
                    self.insert_value(param.param_name.clone(), arg, arg_type, span)?;
                }

                for statement in &body {
                    self.evaluate_statement(statement)?;
                    if self.return_value.is_some() {
                        break;
                    }
                }

                let result = self.return_value.take().unwrap_or(Value::Null);

                self.environment = saved_env;

                if let Some(expected) = &return_type {
                    let actual = match &result {
                        Value::Integer(_) => TypeAnnotation::Int,
                        Value::Float(_) => TypeAnnotation::Float,
                        Value::String(_) => TypeAnnotation::String,
                        Value::Bool(_) => TypeAnnotation::Bool,
                        Value::Char(_) => TypeAnnotation::Char,
                        Value::Values(items) => {
                            let inner = items
                                .first()
                                .map(|v| match v {
                                    Value::Integer(_) => TypeAnnotation::Int,
                                    Value::Float(_) => TypeAnnotation::Float,
                                    Value::String(_) => TypeAnnotation::String,
                                    Value::Bool(_) => TypeAnnotation::Bool,
                                    Value::Char(_) => TypeAnnotation::Char,
                                    _ => TypeAnnotation::Null,
                                })
                                .unwrap_or(TypeAnnotation::Null);
                            TypeAnnotation::Array(Box::new(inner))
                        }
                        Value::Null => TypeAnnotation::Null,
                        Value::Function { .. } => TypeAnnotation::Fn,
                    };
                    if *expected != actual {
                        return Err(self.err(
                            format!(
                                "function '{}' declared to return {:?} but returned {:?}",
                                path[0], expected, actual
                            ),
                            span,
                        ));
                    }
                }

                return Ok(result);
            }
        }

        let mut err = self.err(format!("undefined function {}", path.join("::")), span);
        // suggest a stdlib leaf name if the last segment is a close typo
        if let Some(last) = path.last() {
            let candidates = stdlib::display::KEYWORDS
                .iter()
                .chain(stdlib::math::KEYWORDS)
                .chain(stdlib::math::constants::KEYWORDS)
                .chain(stdlib::io::KEYWORDS)
                .copied();
            if let Some(suggestion) = closest_match(last, candidates) {
                err = err.with_help(format!("did you mean `{}`?", suggestion));
            }
        }
        Err(err)
    }
}
