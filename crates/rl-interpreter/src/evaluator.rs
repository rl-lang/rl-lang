//! Core evaluator - expression evaluation, function calls, and the runtime state.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::{
    native::{IntoNativeFn, Module},
    stdlib,
    stdlib::{http::HttpHandle, net::NetHandle, random::xoshiro::Xoshiro256},
    values::{FunctionData, MapKey, Value},
};
use rl_ast::{ExprId, nodes::ExpressionKind, statements::TypeAnnotation};
use rl_lexer::tokentypes::TokenType;
use rl_resolver::Resolver;
use rl_utils::{
    errors::{Error, Reason},
    source::SourceFile,
    span::Span,
    suggest::closest_match,
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
    /// Global scope, addressed at the outermost depth
    pub globals: Vec<EnvironmentItem>,
    /// The environment stack - each frame is a scope; innermost is last.
    /// Holds only local call frames
    pub environment: Vec<Vec<EnvironmentItem>>,
    /// Pool of previously-used, now-empty scope frames. `push_scope`/
    /// `pop_scope` recycle through here instead of allocating a fresh
    /// `Vec` on every function call and block entry - a hot recursive
    /// function otherwise pays one malloc+free per call just for its frame.
    pub scope_pool: Vec<Vec<EnvironmentItem>>,
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
    /// Also owns `ast_arena: Ast` - the single expression arena for this session,
    /// shared by the resolver and evaluator alike. Never construct a second `Ast`
    /// anywhere else; everything reads/writes through `self.resolver.ast_arena`.
    pub resolver: Resolver,
    /// Maps top-level function names to their environment slot for `call_path` shortcut.
    pub fn_names: HashMap<String, usize>,
    // for diffrent calls
    pub user_args_offset: usize,
    /// Side-table of native networking resources (`std::net`), keyed by handle id.
    pub net_handles: HashMap<i64, NetHandle>,
    /// Next handle id to hand out for `std::net` resources; only ever increments.
    pub net_next_handle: i64,
    /// Side-table of native HTTP resources (`std::http`), keyed by handle id.
    pub http_handles: HashMap<i64, HttpHandle>,
    /// Next handle id to hand out for `std::http` resources; only ever increments.
    pub http_next_handle: i64,
    /// Maps `record` type names to their declared `(field name, field type)` list,
    /// in declaration order. Populated when a `RecordDeclaration` statement runs.
    pub records: HashMap<String, Vec<(String, TypeAnnotation)>>,
    /// Maps `tag` (enum) type names to their declared variant name list,
    /// in declaration order. Populated when a `TagDeclaration` statement runs.
    pub tags: HashMap<String, Vec<String>>,
    /// Maps `"Record::method"` names to their function body, populated when
    /// an `impl` block statement runs. Instance methods (declared with a
    /// leading `self` param) are dispatched here from `MethodCall`; associated
    /// functions (no `self`, e.g. `Point::new`) are dispatched here from
    /// `call_path` when given a two-segment path.
    pub impl_methods: HashMap<String, Rc<FunctionData>>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            globals: vec![],
            environment: vec![],
            scope_pool: vec![],
            source_file: None,
            root_module: Module::new(""),
            return_value: None,
            is_breaking: false,
            is_continuing: false,
            output_buffer: None,
            rng: Xoshiro256::default(),
            resolver: Resolver::new(),
            fn_names: HashMap::new(),
            user_args_offset: 1,
            net_handles: HashMap::new(),
            net_next_handle: 1,
            http_handles: HashMap::new(),
            http_next_handle: 1,
            records: HashMap::new(),
            tags: HashMap::new(),
            impl_methods: HashMap::new(),
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
                .with_module(stdlib::rl::module())
                .with_module(stdlib::debug::module())
                .with_module(stdlib::net::module())
                .with_module(stdlib::http::module())
                .with_module(stdlib::collections::module()),
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
            Value::Set { items, .. } => {
                let inner = items
                    .borrow()
                    .iter()
                    .next()
                    .map(|k| Self::infer_type(&k.clone().into_value(), false))
                    .unwrap_or(TypeAnnotation::Null);
                if is_const {
                    TypeAnnotation::CSet(Box::new(inner))
                } else {
                    TypeAnnotation::Set(Box::new(inner))
                }
            }
            Value::Map {
                key_type,
                value_type,
                ..
            } => {
                if is_const {
                    TypeAnnotation::CMap(Box::new(key_type.clone()), Box::new(value_type.clone()))
                } else {
                    TypeAnnotation::Map(Box::new(key_type.clone()), Box::new(value_type.clone()))
                }
            }
            Value::Struct { name, .. } => {
                if is_const {
                    TypeAnnotation::CRecord(name.clone())
                } else {
                    TypeAnnotation::Record(name.clone())
                }
            }
            Value::Enum { name, .. } => {
                if is_const {
                    TypeAnnotation::CEnum(name.clone())
                } else {
                    TypeAnnotation::Enum(name.clone())
                }
            }
            Value::Null => TypeAnnotation::Null,
            Value::Function { .. } => TypeAnnotation::Fn,
            Value::Tuple(items) => {
                let inner: Vec<TypeAnnotation> =
                    items.iter().map(|v| Self::infer_type(v, false)).collect();
                if is_const {
                    TypeAnnotation::CTuple(Rc::new(inner))
                } else {
                    TypeAnnotation::Tuple(Rc::new(inner))
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
                TypeAnnotation::Record(a) | TypeAnnotation::CRecord(a),
                TypeAnnotation::Record(b) | TypeAnnotation::CRecord(b),
            ) => a == b,
            (
                TypeAnnotation::Result(_) | TypeAnnotation::CResult(_),
                TypeAnnotation::Result(_) | TypeAnnotation::CResult(_),
            ) => true,
            (
                TypeAnnotation::Enum(a) | TypeAnnotation::CEnum(a),
                TypeAnnotation::Enum(b) | TypeAnnotation::CEnum(b),
            ) => a == b,
            (
                TypeAnnotation::Map(ak, av) | TypeAnnotation::CMap(ak, av),
                TypeAnnotation::Map(bk, bv) | TypeAnnotation::CMap(bk, bv),
            ) => Self::types_compatible(ak, bk) && Self::types_compatible(av, bv),
            (
                TypeAnnotation::Set(a) | TypeAnnotation::CSet(a),
                TypeAnnotation::Set(b) | TypeAnnotation::CSet(b),
            ) => Self::types_compatible(a, b),
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

    pub fn evaluate(&mut self, id: ExprId) -> Result<Value, Error> {
        let span = self.resolver.ast_arena.exprs.get(id).span;

        #[cfg(feature = "debug")]
        log::trace!("evaluate {:?} @ {:?}", id, span);

        match &self.resolver.ast_arena.exprs.get(id).kind {
            ExpressionKind::Null => Ok(Value::Null),
            ExpressionKind::Integer(i) => Ok(Value::Integer(*i)),
            ExpressionKind::Byte(b) => Ok(Value::Byte(*b)),
            ExpressionKind::Bool(b) => Ok(Value::Bool(*b)),
            ExpressionKind::Float(f) => Ok(Value::Float(*f)),
            ExpressionKind::Character(c) => Ok(Value::Char(*c)),
            ExpressionKind::String(s) => {
                let s = s.clone();
                Ok(Value::String(s))
            }

            ExpressionKind::Index { target, index } => {
                let (target, index) = (*target, *index);
                let target_span = self.resolver.ast_arena.exprs.get(target).span;
                let index_span = self.resolver.ast_arena.exprs.get(index).span;

                if let ExpressionKind::ResolvedIdentifier { depth, slot, .. } =
                    &self.resolver.ast_arena.exprs.get(target).kind
                {
                    let (depth, slot) = (*depth, *slot);
                    let is_map = matches!(
                        self.slot_ref(depth, slot),
                        Some(EnvironmentItem::PItem(p)) if matches!(p.value, Value::Map { .. })
                    );
                    let idx = self.evaluate(index)?;
                    self.check_not_null(&idx, index_span)?;
                    if !is_map {
                        match idx {
                            Value::Integer(i) => {
                                if i < 0 {
                                    return Err(
                                        self.err(format!("index cannot be negative: {}", i), span)
                                    );
                                }
                                return self.index_read(depth, slot, &[i as usize], span);
                            }
                            Value::Byte(b) => {
                                return self.index_read(depth, slot, &[b as usize], span);
                            }
                            _ => {}
                        }
                    }
                    let arr = self.evaluate(target)?;
                    self.check_not_null(&arr, target_span)?;
                    return self.index_read_value(&arr, &idx, target_span, index_span, span);
                }

                let arr = self.evaluate(target)?;
                self.check_not_null(&arr, target_span)?;
                let idx = self.evaluate(index)?;
                self.check_not_null(&idx, index_span)?;
                self.index_read_value(&arr, &idx, target_span, index_span, span)
            }

            ExpressionKind::ArrayLiteral(items) => {
                let len = items.len();
                let mut values = Vec::with_capacity(len);
                for i in 0..len {
                    // Re-fetch per element instead of cloning the whole Vec<ExprId>
                    // up front - each fetch is a cheap bounds-checked index, no alloc.
                    let item_id = match &self.resolver.ast_arena.exprs.get(id).kind {
                        ExpressionKind::ArrayLiteral(items) => items[i],
                        _ => unreachable!(),
                    };
                    values.push(self.evaluate(item_id)?);
                }
                let items_type = values
                    .first()
                    .map(|v| Self::infer_type(v, false))
                    .unwrap_or(TypeAnnotation::Null);

                if items_type != TypeAnnotation::Null {
                    for (i, v) in values.iter().enumerate() {
                        let actual = Self::infer_type(v, false);
                        if !Self::types_compatible(&actual, &items_type) {
                            return Err(self.err(
                                format!(
                                    "array element type mismatch: element {} is {:?}, expected {:?}",
                                    i, actual, items_type,
                                ),
                                span,
                            ));
                        }
                    }
                }
                Ok(Value::Values {
                    items_type,
                    items: values,
                })
            }

            ExpressionKind::SetLiteral(items) => {
                let len = items.len();
                let mut values = Vec::with_capacity(len);
                for i in 0..len {
                    let item_id = match &self.resolver.ast_arena.exprs.get(id).kind {
                        ExpressionKind::SetLiteral(items) => items[i],
                        _ => unreachable!(),
                    };
                    values.push(self.evaluate(item_id)?);
                }
                let items_type = values
                    .first()
                    .map(|v| Self::infer_type(v, false))
                    .unwrap_or(TypeAnnotation::Null);

                let mut set = std::collections::HashSet::with_capacity(len);
                for (i, v) in values.iter().enumerate() {
                    if items_type != TypeAnnotation::Null {
                        let actual = Self::infer_type(v, false);
                        if !Self::types_compatible(&actual, &items_type) {
                            return Err(self.err(
                                format!(
                                    "set element type mismatch: element {} is {:?}, expected {:?}",
                                    i, actual, items_type,
                                ),
                                span,
                            ));
                        }
                    }
                    let key = MapKey::from_value(v).ok_or_else(|| {
                        self.err(
                            format!("type {} cannot be a set element", v.type_name()),
                            span,
                        )
                    })?;
                    set.insert(key);
                }
                Ok(Value::Set {
                    items_type,
                    items: Rc::new(RefCell::new(set)),
                })
            }
            ExpressionKind::MapLiteral(entries) => {
                let len = entries.len();
                let mut map = HashMap::with_capacity(len);
                let mut key_type = TypeAnnotation::Null;
                let mut value_type = TypeAnnotation::Null;

                for i in 0..len {
                    let (key_id, value_id) = match &self.resolver.ast_arena.exprs.get(id).kind {
                        ExpressionKind::MapLiteral(entries) => entries[i],
                        _ => unreachable!(),
                    };
                    let key_val = self.evaluate(key_id)?;
                    let value_val = self.evaluate(value_id)?;

                    if i == 0 {
                        key_type = Self::infer_type(&key_val, false);
                        value_type = Self::infer_type(&value_val, false);
                    } else {
                        let actual_key = Self::infer_type(&key_val, false);
                        if !Self::types_compatible(&actual_key, &key_type) {
                            return Err(self.err(
                                format!(
                                    "map key type mismatch: entry {} key is {:?}, expected {:?}",
                                    i, actual_key, key_type
                                ),
                                span,
                            ));
                        }
                        let actual_value = Self::infer_type(&value_val, false);
                        if !Self::types_compatible(&actual_value, &value_type) {
                            return Err(self.err(
                                            format!("map value type mismatch: entry {} value is {:?}, expected {:?}", i, actual_value, value_type),
                                            span,
                                        ));
                        }
                    }

                    let map_key = MapKey::from_value(&key_val).ok_or_else(|| {
                        self.err(
                            format!("type {} cannot be used as a map key", key_val.type_name()),
                            span,
                        )
                    })?;
                    map.insert(map_key, value_val);
                }

                Ok(Value::Map {
                    key_type,
                    value_type,
                    entries: Rc::new(RefCell::new(map)),
                })
            }

            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => {
                let (target, index, value) = (*target, *index, *value);
                self.index_assign(target, index, value, span)
            }

            ExpressionKind::Grouping(inner) => {
                let inner = *inner;
                self.evaluate(inner)
            }

            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                let (left, operator, right) = (*left, operator.clone(), *right);

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

                let left_span = self.resolver.ast_arena.exprs.get(left).span;
                let right_span = self.resolver.ast_arena.exprs.get(right).span;

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
                self.check_not_null(&left_val, left_span)?;
                self.check_not_null(&right_val, right_span)?;
                self.match_binary_operator(
                    left_val, left_span, right_val, right_span, &operator, span,
                )
            }

            ExpressionKind::Unary { operator, operand } => {
                let (operator, operand) = (operator.clone(), *operand);
                let operand_span = self.resolver.ast_arena.exprs.get(operand).span;
                let operand_val = self.evaluate(operand)?;
                self.check_not_null(&operand_val, operand_span)?;
                self.match_unary_operator(operand_val, operand_span, &operator, span)
            }

            ExpressionKind::ResolvedIdentifier { depth, slot, .. } => {
                let (depth, slot) = (*depth, *slot);
                self.get_value(depth, slot, span)
            }

            ExpressionKind::ResolvedAssign {
                depth, slot, value, ..
            } => {
                let (depth, slot, value) = (*depth, *slot, *value);
                let val = self.evaluate(value)?;
                let inferred_type = Self::infer_type(&val, false);
                self.assign_value(depth, slot, val.clone(), inferred_type, span)?;
                Ok(val)
            }

            ExpressionKind::Call { .. } => {
                let (path, len) = match &self.resolver.ast_arena.exprs.get(id).kind {
                    ExpressionKind::Call { path, args } => (path.clone(), args.len()),
                    _ => unreachable!(),
                };
                let mut evaluated_args = Vec::with_capacity(len);
                for i in 0..len {
                    let arg_id = match &self.resolver.ast_arena.exprs.get(id).kind {
                        ExpressionKind::Call { args, .. } => args[i],
                        _ => unreachable!(),
                    };
                    evaluated_args.push(self.evaluate(arg_id)?);
                }
                self.call_path(&path, evaluated_args, span)
            }

            ExpressionKind::CallExpr { .. } => {
                let (callee, len) = match &self.resolver.ast_arena.exprs.get(id).kind {
                    ExpressionKind::CallExpr { callee, args } => (*callee, args.len()),
                    _ => unreachable!(),
                };
                let func_val = self.evaluate(callee)?;
                let mut evaluated_args = Vec::with_capacity(len);
                for i in 0..len {
                    let arg_id = match &self.resolver.ast_arena.exprs.get(id).kind {
                        ExpressionKind::CallExpr { args, .. } => args[i],
                        _ => unreachable!(),
                    };
                    evaluated_args.push(self.evaluate(arg_id)?);
                }
                self.call_value(func_val, evaluated_args, span)
            }

            ExpressionKind::MethodCall { .. } => {
                let (caller, method, len) = match &self.resolver.ast_arena.exprs.get(id).kind {
                    ExpressionKind::MethodCall {
                        caller,
                        method,
                        args,
                    } => (*caller, method.clone(), args.len()),
                    _ => unreachable!(),
                };
                let first_arg = self.evaluate(caller)?;
                let mut evaluated_args = vec![first_arg];
                for i in 0..len {
                    let arg_id = match &self.resolver.ast_arena.exprs.get(id).kind {
                        ExpressionKind::MethodCall { args, .. } => args[i],
                        _ => unreachable!(),
                    };
                    evaluated_args.push(self.evaluate(arg_id)?);
                }
                self.call_path(&method, evaluated_args, span)
            }

            ExpressionKind::ResolvedLambda { .. } => {
                let (params, body, return_type, capture_depth) =
                    match &self.resolver.ast_arena.exprs.get(id).kind {
                        ExpressionKind::ResolvedLambda {
                            params,
                            body,
                            return_type,
                            capture_depth,
                        } => (
                            params.clone(),
                            body.clone(),
                            return_type.clone(),
                            *capture_depth,
                        ),
                        _ => unreachable!(),
                    };
                let total = self.environment.len();
                let start = total.saturating_sub(capture_depth);
                let captured_env: Vec<Vec<EnvironmentItem>> = self.environment[start..].to_vec();

                Ok(Value::Function(Rc::new(FunctionData {
                    params: Rc::new(params),
                    body: Rc::new(body),
                    return_type,
                    captured_env,
                })))
            }

            ExpressionKind::Identifier(name) => {
                Err(self.err(format!("undefined variable '{}'", name), span))
            }
            ExpressionKind::Assign { name, .. } => {
                Err(self.err(format!("undefined variable '{}'", name), span))
            }

            ExpressionKind::Cast { value, target_type } => {
                let (value, target_type) = (*value, target_type.clone());
                let value_span = self.resolver.ast_arena.exprs.get(value).span;
                let val = self.evaluate(value)?;
                self.check_not_null(&val, value_span)?;
                match (&val, &target_type) {
                    (Value::Integer(n), TypeAnnotation::Float) => Ok(Value::Float(*n as f64)),
                    (Value::Integer(n), TypeAnnotation::Byte) => Ok(Value::Byte(*n as u8)),
                    (Value::Integer(_), TypeAnnotation::Int) => Ok(val),
                    (Value::Float(f), TypeAnnotation::Int) => Ok(Value::Integer(*f as i64)),
                    (Value::Float(f), TypeAnnotation::Byte) => Ok(Value::Byte(*f as u8)),
                    (Value::Float(_), TypeAnnotation::Float) => Ok(val),
                    (Value::Byte(b), TypeAnnotation::Float) => Ok(Value::Float(*b as f64)),
                    (Value::Byte(b), TypeAnnotation::Int) => Ok(Value::Integer(*b as i64)),
                    (Value::Byte(_), TypeAnnotation::Byte) => Ok(val),
                    _ => Err(self.err(
                        format!(
                            "invalid cast: cannot cast {} to {:?}",
                            val.type_name(),
                            target_type
                        ),
                        span,
                    )),
                }
            }

            ExpressionKind::TupleLiteral(items) => {
                let len = items.len();
                let mut values = Vec::with_capacity(len);
                for i in 0..len {
                    let item_id = match &self.resolver.ast_arena.exprs.get(id).kind {
                        ExpressionKind::TupleLiteral(items) => items[i],
                        _ => unreachable!(),
                    };
                    values.push(self.evaluate(item_id)?);
                }
                Ok(Value::Tuple(values))
            }

            ExpressionKind::ErrorLiteral(inner) => {
                let inner = *inner;
                let val = self.evaluate(inner)?;
                if matches!(val, Value::Error(_)) {
                    return Err(self.err("error cannot wrap another error", span));
                }
                Ok(Value::Error(Box::new(val)))
            }
            ExpressionKind::OkLiteral(inner) => {
                let inner = *inner;
                let val = self.evaluate(inner)?;
                Ok(Value::Ok(Box::new(val)))
            }
            ExpressionKind::ErrLiteral(inner) => {
                let inner = *inner;
                let val = self.evaluate(inner)?;
                Ok(Value::Err(Box::new(val)))
            }

            ExpressionKind::Propagate(inner) => {
                let inner = *inner;
                let val = self.evaluate(inner)?;
                match val {
                    Value::Ok(v) => Ok(*v),
                    Value::Err(_) => {
                        self.return_value = Some(val);
                        Ok(Value::Null)
                    }
                    other => Ok(other),
                }
            }

            ExpressionKind::StructLiteral { .. } => {
                let (name, fields) = match &self.resolver.ast_arena.exprs.get(id).kind {
                    ExpressionKind::StructLiteral { name, fields } => {
                        (name.clone(), fields.clone())
                    }
                    _ => unreachable!(),
                };

                let mut evaluated: Vec<(String, Value)> = Vec::with_capacity(fields.len());
                for (field_name, value_id) in &fields {
                    let value = self.evaluate(*value_id)?;
                    evaluated.push((field_name.clone(), value));
                }

                if let Some(declared_fields) = self.records.get(&name).cloned() {
                    if declared_fields.len() != evaluated.len() {
                        return Err(self.err(
                            format!(
                                "record `{}` expects {} field(s), got {}",
                                name,
                                declared_fields.len(),
                                evaluated.len()
                            ),
                            span,
                        ));
                    }

                    let mut ordered: Vec<(String, Value)> =
                        Vec::with_capacity(declared_fields.len());
                    for (field_name, field_type) in &declared_fields {
                        let Some((_, value)) = evaluated.iter().find(|(n, _)| n == field_name)
                        else {
                            return Err(self.err(
                                format!("record `{}` is missing field `{}`", name, field_name),
                                span,
                            ));
                        };
                        let value_type = Self::infer_type(value, false);
                        if !Self::types_compatible(&value_type, field_type)
                            && value_type != TypeAnnotation::Null
                        {
                            return Err(self.err(
                                format!(
                                    "field `{}` of record `{}` expects {:?}, got {:?}",
                                    field_name, name, field_type, value_type
                                ),
                                span,
                            ));
                        }
                        ordered.push((field_name.clone(), value.clone()));
                    }
                    for (field_name, _) in &evaluated {
                        if !declared_fields.iter().any(|(n, _)| n == field_name) {
                            return Err(self.err(
                                format!("record `{}` has no field `{}`", name, field_name),
                                span,
                            ));
                        }
                    }

                    Ok(Value::Struct {
                        name,
                        fields: Rc::new(std::cell::RefCell::new(ordered)),
                    })
                } else {
                    Ok(Value::Struct {
                        name,
                        fields: Rc::new(std::cell::RefCell::new(evaluated)),
                    })
                }
            }

            ExpressionKind::FieldAccess { .. } => {
                let (target, field) = match &self.resolver.ast_arena.exprs.get(id).kind {
                    ExpressionKind::FieldAccess { target, field } => (*target, field.clone()),
                    _ => unreachable!(),
                };
                let target_span = self.resolver.ast_arena.exprs.get(target).span;
                let target_val = self.evaluate(target)?;
                match target_val {
                    Value::Struct { name, fields } => {
                        let fields = fields.borrow();
                        match fields.iter().find(|(n, _)| *n == field) {
                            Some((_, value)) => Ok(value.clone()),
                            None => Err(self
                                .err(format!("record `{}` has no field `{}`", name, field), span)),
                        }
                    }
                    other => Err(self
                        .err(
                            format!("cannot access field `{}` on {}", field, other.type_name()),
                            span,
                        )
                        .with_label(target_span, format!("this is {}", other.type_name()))),
                }
            }

            ExpressionKind::FieldAssign { .. } => {
                let (target, field, value) = match &self.resolver.ast_arena.exprs.get(id).kind {
                    ExpressionKind::FieldAssign {
                        target,
                        field,
                        value,
                    } => (*target, field.clone(), *value),
                    _ => unreachable!(),
                };
                let target_span = self.resolver.ast_arena.exprs.get(target).span;
                let target_val = self.evaluate(target)?;
                let new_val = self.evaluate(value)?;
                match target_val {
                    Value::Struct { name, fields } => {
                        let mut fields = fields.borrow_mut();
                        match fields.iter_mut().find(|(n, _)| *n == field) {
                            Some((_, slot)) => {
                                *slot = new_val.clone();
                                Ok(new_val)
                            }
                            None => Err(self
                                .err(format!("record `{}` has no field `{}`", name, field), span)),
                        }
                    }
                    other => Err(self
                        .err(
                            format!("cannot assign field `{}` on {}", field, other.type_name()),
                            span,
                        )
                        .with_label(target_span, format!("this is {}", other.type_name()))),
                }
            }

            ExpressionKind::EnumVariant { .. } => {
                let (enum_name, variant) = match &self.resolver.ast_arena.exprs.get(id).kind {
                    ExpressionKind::EnumVariant { enum_name, variant } => {
                        (enum_name.clone(), variant.clone())
                    }
                    _ => unreachable!(),
                };

                if let Some(declared_variants) = self.tags.get(&enum_name)
                    && !declared_variants.contains(&variant)
                {
                    return Err(self.err(
                        format!("tag `{}` has no variant `{}`", enum_name, variant),
                        span,
                    ));
                }

                Ok(Value::Enum {
                    name: enum_name,
                    variant,
                })
            }

            _ => Ok(Value::Null),
        }
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
        if let Value::Function(data) = func {
            if data.params.len() != args.len() {
                return Err(self.err(
                    format!(
                        "function expects {} argument(s), got {}",
                        data.params.len(),
                        args.len()
                    ),
                    span,
                ));
            }

            let saved_env = std::mem::replace(&mut self.environment, data.captured_env.clone());
            let saved_return = self.return_value.take();

            self.push_scope();

            for (slot, (_, arg)) in data.params.iter().zip(args).enumerate() {
                let arg_type = Self::infer_type(&arg, false);
                self.insert_value(slot, arg, arg_type, span)?;
            }

            for statement in &*data.body {
                self.evaluate_statement(statement)?;
                if self.return_value.is_some() {
                    break;
                }
            }

            let result = self.return_value.take().unwrap_or(Value::Null);

            self.environment = saved_env;
            self.return_value = saved_return;

            if let Some(expected) = &data.return_type
                && *expected != TypeAnnotation::Null
            {
                let actual = Self::infer_type(&result, false);
                if !Self::types_compatible(&actual, expected) {
                    return Err(self.err(
                        format!(
                            "function declared to return {:?} but returned {:?}",
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
                .chain(stdlib::debug::KEYWORDS)
                .chain(stdlib::net::KEYWORDS)
                .chain(stdlib::http::KEYWORDS)
                .chain(stdlib::collections::KEYWORDS)
                .copied();
            if let Some(suggestion) = closest_match(last, candidates) {
                err = err.with_help(format!("did you mean `{}`?", suggestion));
            }
        }
        Err(err)
    }
}
