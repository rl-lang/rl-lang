//! Best-effort static type inference for unannotated (`Infer`/`Generic`)
//! initializer expressions.
//!
//! The resolver leaves `dec x = <expr>` without an annotation as
//! `TypeAnnotation::Infer`, which maps to the broken `rl_value` C type.
//! These helpers recover the real type from the initializer shape and from
//! the checker's stdlib signatures so locals, globals and unwraps emit
//! correctly typed C.

use crate::codegen::CCodegen;
use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::TypeAnnotation;
use rl_ast::ExprId;

impl<'a> CCodegen<'a> {
    /// Returns true for annotations that carry no usable C type and need
    /// inference from the initializer.
    pub fn needs_inference(ta: &TypeAnnotation) -> bool {
        matches!(
            ta,
            TypeAnnotation::Infer | TypeAnnotation::Generic(_)
        )
    }

    /// Infer the value type of an expression. Returns None when nothing
    /// reliable can be said; callers fall back to `rl_result`.
    pub fn inferred_expr_type(&self, id: ExprId) -> Option<TypeAnnotation> {
        let expr = self.ast.exprs.get(id);
        match &expr.kind {
            ExpressionKind::Integer(_) => Some(TypeAnnotation::Int),
            ExpressionKind::UInt(_) => Some(TypeAnnotation::UInt),
            ExpressionKind::SInt(_) => Some(TypeAnnotation::SInt),
            ExpressionKind::SUInt(_) => Some(TypeAnnotation::SUInt),
            ExpressionKind::Float(_) => Some(TypeAnnotation::Float),
            ExpressionKind::SFloat(_) => Some(TypeAnnotation::SFloat),
            ExpressionKind::Bool(_) => Some(TypeAnnotation::Bool),
            ExpressionKind::Character(_) => Some(TypeAnnotation::Char),
            ExpressionKind::String(_) => Some(TypeAnnotation::String),
            ExpressionKind::Byte(_) => Some(TypeAnnotation::Byte),
            ExpressionKind::BByte(_) => Some(TypeAnnotation::BByte),
            ExpressionKind::SByte(_) => Some(TypeAnnotation::SByte),
            ExpressionKind::BSByte(_) => Some(TypeAnnotation::BSByte),
            ExpressionKind::Grouping(inner) => self.inferred_expr_type(*inner),
            ExpressionKind::ResolvedIdentifier { name, .. } => self.var_types.get(name).cloned(),
            ExpressionKind::ArrayLiteral(elems) => {
                let elem = elems
                    .first()
                    .and_then(|e| self.inferred_expr_type(*e))
                    .unwrap_or(TypeAnnotation::Infer);
                Some(TypeAnnotation::Array(Box::new(elem)))
            }
            ExpressionKind::MapLiteral(_) => Some(TypeAnnotation::Map(
                Box::new(TypeAnnotation::String),
                Box::new(TypeAnnotation::Infer),
            )),
            ExpressionKind::SetLiteral(_) => {
                Some(TypeAnnotation::Set(Box::new(TypeAnnotation::Infer)))
            }
            ExpressionKind::TupleLiteral(elems) => {
                let mut parts = Vec::with_capacity(elems.len());
                for e in elems {
                    parts.push(self.inferred_expr_type(*e).unwrap_or(TypeAnnotation::Infer));
                }
                Some(TypeAnnotation::Tuple(parts.into()))
            }
            ExpressionKind::Cast { target_type, .. } => Some(target_type.clone()),
            ExpressionKind::ResolvedLambda { .. } => Some(TypeAnnotation::Fn),
            ExpressionKind::Binary {
                left,
                right,
                operator,
            } => self.inferred_binary_type(*left, *right, operator),
            ExpressionKind::Unary { operator, operand } => {
                use rl_lexer::tokentypes::TokenType;
                match operator {
                    TokenType::Bang => Some(TypeAnnotation::Bool),
                    TokenType::Minus => match self.inferred_expr_type(*operand)? {
                        t if Self::is_float_type(&t) => Some(TypeAnnotation::Float),
                        t if Self::is_int_type(&t) => Some(TypeAnnotation::Int),
                        _ => None,
                    },
                    _ => None,
                }
            }
            ExpressionKind::OkLiteral(inner) => {
                let payload = self
                    .inferred_expr_type(*inner)
                    .unwrap_or(TypeAnnotation::Infer);
                Some(TypeAnnotation::Result(Box::new(payload)))
            }
            ExpressionKind::ErrLiteral(inner) | ExpressionKind::ErrorLiteral(inner) => {
                let payload = self
                    .inferred_expr_type(*inner)
                    .unwrap_or(TypeAnnotation::Infer);
                Some(TypeAnnotation::Result(Box::new(payload)))
            }
            ExpressionKind::Propagate(inner) => {
                // `x?` unwraps one Result layer when the shape is known.
                match self.inferred_expr_type(*inner)? {
                    TypeAnnotation::Result(payload) => Some(*payload),
                    other => Some(other),
                }
            }
            ExpressionKind::Index { target, .. } => match self.inferred_expr_type(*target)? {
                TypeAnnotation::Array(elem) | TypeAnnotation::CArray(elem) => Some(*elem),
                _ => None,
            },
            ExpressionKind::Call { path, args } => {
                let name = path.last().map(|s| s.as_str()).unwrap_or("");
                // `dbg(x)` returns its argument unchanged.
                if name == "dbg" && !args.is_empty() {
                    return self.inferred_expr_type(args[0]);
                }
                // Pairwise zip builds an array of 2-tuples when both
                // sides are statically known.
                if name == "arr_zip" && args.len() >= 2 {
                    let ea = self.array_arg_elem(args[0]);
                    let eb = self.array_arg_elem(args[1]);
                    if let (Some(ea), Some(eb)) = (ea, eb) {
                        return Some(TypeAnnotation::Result(Box::new(
                            TypeAnnotation::Array(Box::new(TypeAnnotation::Tuple(
                                std::rc::Rc::new(vec![ea, eb]),
                            ))),
                        )));
                    }
                }
                // Padded zip builds the same tuple array as plain zip.
                if name == "arr_zip_longest" && args.len() >= 2 {
                    let ea = self.array_arg_elem(args[0]);
                    let eb = self.array_arg_elem(args[1]);
                    if let (Some(ea), Some(eb)) = (ea, eb) {
                        return Some(TypeAnnotation::Result(Box::new(
                            TypeAnnotation::Array(Box::new(TypeAnnotation::Tuple(
                                std::rc::Rc::new(vec![ea, eb]),
                            ))),
                        )));
                    }
                }
                // Generic array returns refine from the array argument:
                // `first`/`last`/`find` give the element, `filter` the
                // array, `contains`/`all`/`any` a bool, `find_index` an int.
                if !args.is_empty() {
                    let elem = self.array_arg_elem(args[0]);
                    match name {
                        "arr_first" | "arr_last" | "arr_find" | "arr_max_by" | "arr_min_by" | "heap_peek" => {
                            if let Some(e) = elem {
                                return Some(TypeAnnotation::Result(Box::new(e)));
                            }
                        }
                        "arr_filter" => {
                            if let Some(e) = elem {
                                return Some(TypeAnnotation::Result(Box::new(
                                    TypeAnnotation::Array(Box::new(e)),
                                )));
                            }
                        }
                        _ => {}
                    }
                }
                // Map default lookup returns the default's type.
                if (name == "map_get_or" || name == "map_get_or_insert") && args.len() >= 3 {
                    if let Some(ta) = self.inferred_expr_type(args[2]) {
                        if !Self::needs_inference(&ta) {
                            return Some(TypeAnnotation::Result(Box::new(ta)));
                        }
                    }
                }
                // `result_unwrap(x)` returns the ok payload.
                if name == "result_unwrap" && !args.is_empty() {
                    if let Some(TypeAnnotation::Result(payload)) =
                        self.inferred_expr_type(args[0])
                    {
                        return Some(*payload);
                    }
                    return None;
                }
                // `result_unwrap_or(x, d)` returns the payload or the default.
                if name == "result_unwrap_or" && args.len() >= 2 {
                    if let Some(TypeAnnotation::Result(payload)) =
                        self.inferred_expr_type(args[0])
                    {
                        return Some(*payload);
                    }
                    return self.inferred_expr_type(args[1]);
                }
                // `result_unwrap_or_else(x, f)` returns the payload like
                // `result_unwrap` does (f only runs on the error path).
                if name == "result_unwrap_or_else" && !args.is_empty() {
                    if let Some(TypeAnnotation::Result(payload)) =
                        self.inferred_expr_type(args[0])
                    {
                        return Some(*payload);
                    }
                    return None;
                }
                // `result_and_then(x, f)` threads results: error passes
                // through, ok applies f. The output payload is dynamic.
                if name == "result_and_then" && !args.is_empty() {
                    return Some(TypeAnnotation::Result(Box::new(
                        TypeAnnotation::Infer,
                    )));
                }
                self.stdlib_return_type(path, name).or_else(|| {
                    // Unknown call: most stdlib calls return results.
                    if path.first().map(|s| s.as_str()) == Some("std") {
                        Some(TypeAnnotation::Result(Box::new(TypeAnnotation::Infer)))
                    } else {
                        self.user_fn_returns.get(name).cloned()
                    }
                })
            }
            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => {
                let resolved = if method.len() > 1 {
                    let original = method.last().cloned().unwrap_or_default();
                    let namespace = method[..method.len() - 1].join("::");
                    Some((namespace, original))
                } else {
                    let name = method.first().map(|s| s.as_str()).unwrap_or("");
                    self.resolve_std_name(name)
                };
                match resolved {
                    Some((namespace, original)) => {
                        // Pairwise zip: receiver is the left side here.
                        if original == "arr_zip" && !args.is_empty() {
                            let ea = self.array_arg_elem(*caller);
                            let eb = self.array_arg_elem(args[0]);
                            if let (Some(ea), Some(eb)) = (ea, eb) {
                                return Some(TypeAnnotation::Result(Box::new(
                                    TypeAnnotation::Array(Box::new(
                                        TypeAnnotation::Tuple(std::rc::Rc::new(vec![
                                            ea, eb,
                                        ])),
                                    )),
                                )));
                            }
                        }
                        // Padded zip mirrors plain zip for method calls.
                        if original == "arr_zip_longest" && !args.is_empty() {
                            let ea = self.array_arg_elem(*caller);
                            let eb = self.array_arg_elem(args[0]);
                            if let (Some(ea), Some(eb)) = (ea, eb) {
                                return Some(TypeAnnotation::Result(Box::new(
                                    TypeAnnotation::Array(Box::new(
                                        TypeAnnotation::Tuple(std::rc::Rc::new(vec![
                                            ea, eb,
                                        ])),
                                    )),
                                )));
                            }
                        }
                        // Unwrap family on the receiver: payload type out,
                        // mirroring the plain-call arms above.
                        match original.as_str() {
                            "result_unwrap" | "result_unwrap_err" => {
                                if let Some(TypeAnnotation::Result(payload))
                                | Some(TypeAnnotation::CResult(payload)) =
                                    self.inferred_expr_type(*caller)
                                {
                                    return Some(*payload);
                                }
                            }
                            "result_unwrap_or" | "result_unwrap_or_else" => {
                                if let Some(TypeAnnotation::Result(payload))
                                | Some(TypeAnnotation::CResult(payload)) =
                                    self.inferred_expr_type(*caller)
                                {
                                    return Some(*payload);
                                }
                                if !args.is_empty() {
                                    return self.inferred_expr_type(args[0]);
                                }
                            }
                            "result_map" | "result_map_err" | "result_and_then" => {
                                return Some(TypeAnnotation::Result(Box::new(
                                    TypeAnnotation::Infer,
                                )));
                            }
                            _ => {}
                        }
                        // Same generic-array refinement as plain calls,
                        // but the array is the receiver here.
                        match original.as_str() {
                            "arr_first" | "arr_last" | "arr_find" | "arr_max_by" | "arr_min_by" | "heap_peek" => {
                                if let Some(e) = self.array_arg_elem(*caller) {
                                    return Some(TypeAnnotation::Result(Box::new(e)));
                                }
                            }
                            "arr_filter" => {
                                if let Some(e) = self.array_arg_elem(*caller) {
                                    return Some(TypeAnnotation::Result(Box::new(
                                        TypeAnnotation::Array(Box::new(e)),
                                    )));
                                }
                            }
                            _ => {}
                        }
                        let path: Vec<String> = namespace
                            .split("::")
                            .map(|s| s.to_string())
                            .chain(std::iter::once(original.clone()))
                            .collect();
                        self.stdlib_return_type(&path, &original)
                    }
                    None => {
                        // User-function method: receiver becomes arg one,
                        // return type is the function's declared return.
                        let name = method.first().map(|s| s.as_str()).unwrap_or("");
                        let _ = (caller, args);
                        self.user_fn_returns.get(name).cloned()
                    }
                }
            }
            ExpressionKind::CallExpr { callee, .. } => {
                let callee_expr = self.ast.exprs.get(*callee);
                if let ExpressionKind::ResolvedIdentifier { name, .. } = &callee_expr.kind {
                    if let Some(rt) = self.closure_return_types.get(name) {
                        return Some(rt.clone());
                    }
                    // Factory call yields a closure value.
                    if self.closure_factories.contains_key(name) {
                        return Some(TypeAnnotation::Fn);
                    }
                    if let Some((_, original)) = self.resolve_std_name(name) {
                        let path = vec!["std".to_string(), original.clone()];
                        if let Some(rt) = self.stdlib_return_type(&path, &original) {
                            return Some(rt);
                        }
                    }
                    return self.user_fn_returns.get(name).cloned();
                }
                None
            }
            _ => None,
        }
    }

    /// Return type of a canonical stdlib call: first the checker's imported
    /// signatures, then a small table for untyped-but-known functions
    /// (`env`, `type_of`, `concat`, ...). None means unknown.
    fn stdlib_return_type(&self, _path: &[String], name: &str) -> Option<TypeAnnotation> {
        if let Some(std_fn) = self
            .checker
            .imported_std_fns
            .get(name)
            .or_else(|| self.checker.stdlib_fn_names.get(name))
        {
            if let Some((_, ret)) = std_fn.signatures.first() {
                let ret = ret.clone();
                if !Self::needs_inference(&ret) {
                    return Some(ret);
                }
            }
        }
        // Untyped in the checker but behaviorally fixed (verified against
        // the VM).
        match name {
            "env" => Some(TypeAnnotation::String),
            "type_of" => Some(TypeAnnotation::String),
            "concat" => Some(TypeAnnotation::String),
            "format" => Some(TypeAnnotation::String),
            "stack_trace" => Some(TypeAnnotation::String),
            "len" => Some(TypeAnnotation::Result(Box::new(TypeAnnotation::Int))),
            _ => None,
        }
    }

    fn is_float_type(ta: &TypeAnnotation) -> bool {
        matches!(
            ta,
            TypeAnnotation::Float
                | TypeAnnotation::CFloat
                | TypeAnnotation::SFloat
                | TypeAnnotation::CSFloat
        )
    }

    fn is_int_type(ta: &TypeAnnotation) -> bool {
        matches!(
            ta,
            TypeAnnotation::Int
                | TypeAnnotation::CInt
                | TypeAnnotation::UInt
                | TypeAnnotation::CUInt
                | TypeAnnotation::SInt
                | TypeAnnotation::CSInt
                | TypeAnnotation::SUInt
                | TypeAnnotation::CSUInt
                | TypeAnnotation::Byte
                | TypeAnnotation::CByte
                | TypeAnnotation::SByte
                | TypeAnnotation::CSByte
                | TypeAnnotation::BByte
                | TypeAnnotation::CBByte
                | TypeAnnotation::BSByte
                | TypeAnnotation::CBSByte
        )
    }

    /// Result type of a binary operator: comparisons and logic give bool,
    /// `+` on strings gives string, other arithmetic unifies operands
    /// (float wins, else int when both sides are int-like).
    fn inferred_binary_type(
        &self,
        left: ExprId,
        right: ExprId,
        operator: &rl_lexer::tokentypes::TokenType,
    ) -> Option<TypeAnnotation> {
        use rl_lexer::tokentypes::TokenType;
        match operator {
            TokenType::Compare
            | TokenType::BangEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::And
            | TokenType::Or => Some(TypeAnnotation::Bool),
            TokenType::Plus => {
                let l = self.inferred_expr_type(left);
                let r = self.inferred_expr_type(right);
                match (l.as_ref(), r.as_ref()) {
                    (
                        Some(TypeAnnotation::String)
                        | Some(TypeAnnotation::CString),
                        _,
                    )
                    | (
                        _,
                        Some(TypeAnnotation::String)
                        | Some(TypeAnnotation::CString),
                    ) => Some(TypeAnnotation::String),
                    _ => Self::unify_arith(l, r),
                }
            }
            TokenType::Minus | TokenType::Star | TokenType::Slash => {
                Self::unify_arith(
                    self.inferred_expr_type(left),
                    self.inferred_expr_type(right),
                )
            }
            _ => None,
        }
    }

    fn unify_arith(
        l: Option<TypeAnnotation>,
        r: Option<TypeAnnotation>,
    ) -> Option<TypeAnnotation> {
        match (l.as_ref(), r.as_ref()) {
            (Some(a), Some(b))
                if Self::is_float_type(a) || Self::is_float_type(b) =>
            {
                Some(TypeAnnotation::Float)
            }
            (Some(a), Some(b)) if Self::is_int_type(a) && Self::is_int_type(b) => {
                Some(TypeAnnotation::Int)
            }
            _ => None,
        }
    }

    /// Result-payload tag constant for an array's element type, for the
    /// closure-consuming runtime fns. Unknown elements use the legacy
    /// int64 tag.
    pub fn array_elem_tag(&self, id: ExprId) -> &'static str {
        match self.array_arg_elem(id).as_ref() {
            Some(ta) => Self::array_elem_tag_from_type(ta),
            None => "RL_TAG_I64",
        }
    }

    /// Tag constant for a known element annotation.
    pub fn array_elem_tag_from_type(ta: &TypeAnnotation) -> &'static str {
        match ta {
            TypeAnnotation::Float | TypeAnnotation::CFloat => "RL_TAG_F64",
            TypeAnnotation::Bool | TypeAnnotation::CBool => "RL_TAG_BOOL",
            TypeAnnotation::String | TypeAnnotation::CString => "RL_TAG_STR",
            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => "RL_TAG_ARR",
            TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _) => "RL_TAG_MAP",
            TypeAnnotation::Set(_) | TypeAnnotation::CSet(_) => "RL_TAG_SET",
            TypeAnnotation::Char | TypeAnnotation::CChar => "RL_TAG_CHAR",
            _ => "RL_TAG_I64",
        }
    }

    /// Concrete element type of an array--typed expression, for generic
    /// array ops. None when unknown or still generic.
    pub fn array_arg_elem(&self, id: ExprId) -> Option<TypeAnnotation> {
        match self.inferred_expr_type(id)? {
            TypeAnnotation::Array(elem) | TypeAnnotation::CArray(elem) => {
                let e = *elem;
                if Self::needs_inference(&e) {
                    None
                } else {
                    Some(e)
                }
            }
            _ => None,
        }
    }

    /// True for a call through a closure of unknown return type (untyped
    /// factory results, immediately-invoked lambdas): the call yields an
    /// opaque `rl_result` that concrete storage must unwrap.
    pub fn is_dynamic_closure_call(&self, id: ExprId) -> bool {
        let expr = self.ast.exprs.get(id);
        if let ExpressionKind::CallExpr { callee, .. } = &expr.kind {
            let callee_expr = self.ast.exprs.get(*callee);
            match &callee_expr.kind {
                ExpressionKind::ResolvedLambda { .. } => true,
                ExpressionKind::ResolvedIdentifier { name, .. } => {
                    let known = matches!(
                        self.var_types.get(name),
                        Some(TypeAnnotation::Fn)
                            | Some(TypeAnnotation::Callback(_, _))
                    );
                    known && !self.closure_return_types.contains_key(name)
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// True for `result_unwrap(x)` (function or method form) where x's
    /// payload type is unknown. Lets declarations unwrap by their storage
    /// type instead of the i64 default (e.g. `dec bool b` over a map
    /// lookup, whose value type is a free generic).
    /// A fully unknown payload (no checker signature, e.g. bare-path
    /// stdlib calls) also counts: storage type is the best available
    /// guess, strictly better than the blind i64 default.
    pub fn is_unwrap_of_dynamic(&self, id: ExprId) -> bool {
        let expr = self.ast.exprs.get(id);
        let inner = match &expr.kind {
            ExpressionKind::Call { path, args } => {
                if path.last().map(|s| s.as_str()) != Some("result_unwrap") {
                    return false;
                }
                args.first().copied()
            }
            ExpressionKind::MethodCall { caller, method, .. } => {
                if method.last().map(|s| s.as_str()) != Some("result_unwrap") {
                    return false;
                }
                Some(*caller)
            }
            _ => None,
        };
        let Some(arg) = inner else {
            return false;
        };
        match self.inferred_expr_type(arg) {
            Some(TypeAnnotation::Result(payload)) | Some(TypeAnnotation::CResult(payload)) => {
                Self::needs_inference(&payload)
            }
            // No static payload at all: dynamic by definition.
            Some(_) | None => true,
        }
    }
    /// A bare `handle` (HandleInfer) refines to the initializer handle
    /// kind like the checker does. Falls back to `Result(Infer)` - most
    /// dynamic values in generated code are results.
    /// The `x` inside `result_unwrap(x)` (function or method form),
    /// so declarations that unwrap by storage type can compile the inner
    /// call directly instead of double-unwrapping.
    pub fn unwrap_inner_arg(&self, id: ExprId) -> Option<ExprId> {
        let expr = self.ast.exprs.get(id);
        match &expr.kind {
            ExpressionKind::Call { path, args } => {
                if path.last().map(|s| s.as_str()) != Some("result_unwrap") {
                    return None;
                }
                args.first().copied()
            }
            ExpressionKind::MethodCall { caller, method, .. } => {
                if method.last().map(|s| s.as_str()) != Some("result_unwrap") {
                    return None;
                }
                Some(*caller)
            }
            _ => None,
        }
    }
    /// True for `__arr_get` / `__map_get` (function or method form)
    /// whose container layout is statically unknown, so the emitter
    /// takes the boxed form. Declarations over these get storage-driven
    /// unboxing; anything else would mistype the payload.
    pub fn core_get_needs_boxing(&self, id: ExprId) -> bool {
        let expr = self.ast.exprs.get(id);
        let (name, container) = match &expr.kind {
            ExpressionKind::Call { path, args } => {
                let name = path.last().map(|s| s.as_str()).unwrap_or("");
                if name != "__arr_get" && name != "__map_get" {
                    return false;
                }
                (name, args.first().copied())
            }
            ExpressionKind::MethodCall { caller, method, .. } => {
                let name = method.last().map(|s| s.as_str()).unwrap_or("");
                if name != "__arr_get" && name != "__map_get" {
                    return false;
                }
                (name, Some(*caller))
            }
            _ => return false,
        };
        let Some(cid) = container else {
            return true;
        };
        if name == "__arr_get" {
            // Known element type takes the typed getter.
            self.array_arg_elem(cid).is_none()
        } else {
            match self.inferred_expr_type(cid) {
                Some(TypeAnnotation::Map(_, vt)) | Some(TypeAnnotation::CMap(_, vt)) => {
                    Self::needs_inference(&vt)
                }
                // Unknown container entirely: box it.
                _ => true,
            }
        }
    }
    /// The message argument of an `__abort(...)` call, if `id` is one.
    /// Lets declarations panic first and feed storage an unreachable
    /// dummy, so abort's `-> T` contract holds in generated code too.
    pub fn abort_message_arg(&self, id: ExprId) -> Option<ExprId> {
        let expr = self.ast.exprs.get(id);
        match &expr.kind {
            ExpressionKind::Call { path, args } => {
                if path.last().map(|s| s.as_str()) != Some("__abort") {
                    return None;
                }
                args.first().copied()
            }
            ExpressionKind::MethodCall { method, args, .. } => {
                if method.last().map(|s| s.as_str()) != Some("__abort") {
                    return None;
                }
                args.first().copied()
            }
            _ => None,
        }
    }
    /// Effective storage annotation for a declaration: the annotation
    /// itself, unless it is `Infer`/`Generic`, in which case the
    /// initializer shape (and the checker's stdlib signatures) decide.
    pub fn effective_decl_type(
        &self,
        annotation: &TypeAnnotation,
        value: ExprId,
    ) -> TypeAnnotation {
        if Self::needs_inference(annotation) {
            match self.inferred_expr_type(value) {
                Some(ta) if !Self::needs_inference(&ta) => ta,
                _ => TypeAnnotation::Result(Box::new(TypeAnnotation::Infer)),
            }
        } else if matches!(annotation, TypeAnnotation::HandleInfer) {
            match self.inferred_expr_type(value) {
                Some(TypeAnnotation::Handle(kind)) => TypeAnnotation::Handle(kind),
                _ => annotation.clone(),
            }
        } else {
            annotation.clone()
        }
    }

    /// Static `is_*` answer for an expression, matching the VM exactly.
    /// Results (including ok/err wrappers) are never their payload, so
    /// they fold to false; concrete bare types compare directly. None
    /// means dynamic and needs the runtime tag check.
    pub fn static_is_truth(&self, func_name: &str, id: ExprId) -> Option<bool> {
        use rl_ast::statements::TypeAnnotation as T;
        if let Some(ta) = self.inferred_expr_type(id) {
            match &ta {
                T::Result(_) | T::CResult(_) => return Some(false),
                T::Infer | T::Generic(_) | T::HandleInfer => {}
                other => return Some(Self::is_truth_for_type(func_name, other)),
            }
        }
        let kind = &self.ast.exprs.get(id).kind;
        match kind {
            ExpressionKind::Null => Some(func_name == "is_null"),
            ExpressionKind::OkLiteral(_) | ExpressionKind::ErrLiteral(_) => {
                Some(false)
            }
            ExpressionKind::ErrorLiteral(_) => Some(func_name == "is_error"),
            ExpressionKind::StructLiteral { .. } => Some(false),
            ExpressionKind::TupleLiteral(_) => Some(func_name == "is_tuple"),
            ExpressionKind::ArrayLiteral(_) => Some(func_name == "is_array"),
            ExpressionKind::MapLiteral(_) => Some(func_name == "is_map"),
            ExpressionKind::SetLiteral(_) => Some(func_name == "is_set"),
            ExpressionKind::ResolvedLambda { .. } => {
                Some(func_name == "is_function")
            }
            ExpressionKind::EnumVariant { .. } => Some(false),
            // Bare user function names are C function pointers with no
            // _Generic arm; they are always functions in RL too.
            ExpressionKind::ResolvedIdentifier { name, .. }
            | ExpressionKind::Identifier(name) => {
                if self.user_fns.contains(name) {
                    Some(func_name == "is_function")
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// True when `func_name` predicate holds for concrete type `ta`.
    fn is_truth_for_type(func_name: &str, ta: &TypeAnnotation) -> bool {
        use rl_ast::statements::{HandleKind, TypeAnnotation as T};
        match func_name {
            "is_bool" => matches!(ta, T::Bool | T::CBool),
            "is_int" => matches!(ta, T::Int | T::CInt),
            "is_float" => matches!(ta, T::Float | T::CFloat),
            "is_string" => matches!(ta, T::String | T::CString),
            "is_null" => matches!(ta, T::Null),
            "is_char" => matches!(ta, T::Char | T::CChar),
            "is_byte" => matches!(ta, T::Byte | T::CByte),
            "is_error" => matches!(ta, T::Error | T::CError),
            "is_array" => matches!(ta, T::Array(_) | T::CArray(_)),
            "is_map" => matches!(ta, T::Map(_, _) | T::CMap(_, _)),
            "is_set" => matches!(ta, T::Set(_) | T::CSet(_)),
            "is_tuple" => matches!(ta, T::Tuple(_) | T::CTuple(_)),
            "is_function" => matches!(ta, T::Fn | T::Callback(_, _)),
            "is_uint" => matches!(ta, T::UInt | T::CUInt),
            "is_sbyte" => matches!(ta, T::SByte | T::CSByte),
            "is_bsbyte" => matches!(ta, T::BSByte | T::CBSByte),
            "is_bbyte" => matches!(ta, T::BByte | T::CBByte),
            "is_sint" => matches!(ta, T::SInt | T::CSInt),
            "is_suint" => matches!(ta, T::SUInt | T::CSUInt),
            "is_sfloat" => matches!(ta, T::SFloat | T::CSFloat),
            "is_c_handle" => matches!(ta, T::Handle(HandleKind::C)),
            "is_net_handle" => matches!(ta, T::Handle(HandleKind::Net)),
            "is_http_handle" => matches!(ta, T::Handle(HandleKind::Http)),
            "is_audio_handle" => matches!(ta, T::Handle(HandleKind::Audio)),
            "is_gui_handle" => matches!(ta, T::Handle(HandleKind::Gui)),
            "is_file_handle" => matches!(ta, T::Handle(HandleKind::File)),
            _ => false,
        }
    }

    /// Static `type_of` name for an expression, matching the VM's
    /// `type_name` strings exactly. None when only known at runtime.
    pub fn static_type_name(&self, id: ExprId) -> Option<&'static str> {
        use rl_ast::statements::{HandleKind, TypeAnnotation as T};
        let expr = self.ast.exprs.get(id);
        if matches!(&expr.kind, ExpressionKind::ResolvedLambda { .. }) {
            return Some("closure");
        }
        if let ExpressionKind::ResolvedIdentifier { name, .. } = &expr.kind
            && matches!(
                self.var_types.get(name),
                Some(T::Fn) | Some(T::Callback(_, _))
            )
        {
            if self.closure_return_types.contains_key(name) {
                return Some("closure");
            }
            return Some("function");
        }
        if matches!(
            &expr.kind,
            ExpressionKind::Null
                | ExpressionKind::OkLiteral(_)
                | ExpressionKind::ErrLiteral(_)
                | ExpressionKind::ErrorLiteral(_)
        ) {
            return match &expr.kind {
                ExpressionKind::Null => Some("null"),
                ExpressionKind::OkLiteral(_) => Some("ok"),
                ExpressionKind::ErrLiteral(_) => Some("err"),
                _ => Some("error"),
            };
        }
        match self.inferred_expr_type(id).as_ref()? {
            T::Int | T::CInt => Some("int"),
            T::UInt | T::CUInt => Some("uint"),
            T::SInt | T::CSInt => Some("small int"),
            T::SUInt | T::CSUInt => Some("small uint"),
            T::Byte | T::CByte => Some("byte"),
            T::SByte | T::CSByte => Some("sbyte"),
            T::BByte | T::CBByte => Some("big byte"),
            T::BSByte | T::CBSByte => Some("big sbyte"),
            T::Float | T::CFloat => Some("float"),
            T::SFloat | T::CSFloat => Some("small float"),
            T::Bool | T::CBool => Some("bool"),
            T::Char | T::CChar => Some("char"),
            T::String | T::CString => Some("string"),
            T::Array(_) | T::CArray(_) => Some("arr"),
            T::Map(_, _) | T::CMap(_, _) => Some("map"),
            T::Set(_) | T::CSet(_) => Some("set"),
            T::Tuple(_) | T::CTuple(_) => Some("tuple"),
            T::Record(_) | T::CRecord(_) => Some("record"),
            T::Enum(_) | T::CEnum(_) => Some("tag"),
            T::Error | T::CError => Some("error"),
            T::Fn | T::Callback(_, _) => Some("function"),
            T::Handle(HandleKind::C) => Some("c handle"),
            T::Handle(HandleKind::Net) => Some("net handle"),
            T::Handle(HandleKind::Http) => Some("http handle"),
            T::Handle(HandleKind::Audio) => Some("audio handle"),
            T::Handle(HandleKind::Gui) => Some("gui handle"),
            T::Handle(HandleKind::File) => Some("file handle"),
            _ => None,
        }
    }

    /// Tuple fields of a `result_unwrap(x)` payload, when it is a tuple.
    /// Single tuple results (HTTP responses, UDP sender pairs) travel as
    /// one element arrays; callers deref element zero into the program
    /// tuple struct.
    pub fn tuple_payload_fields(&self, arg_id: ExprId) -> Option<Vec<TypeAnnotation>> {
        use TypeAnnotation as T;
        let payload = match self.inferred_expr_type(arg_id) {
            Some(T::Result(inner)) | Some(T::CResult(inner)) => Some(*inner),
            Some(other) => Some(other),
            None => None,
        };
        match payload.as_ref() {
            Some(T::Tuple(fields)) | Some(T::CTuple(fields)) => {
                Some(fields.as_ref().clone())
            }
            _ => None,
        }
    }

    /// Checked-unwrap function for a `result_unwrap(x)` argument, chosen
    /// from the payload type. Defaults to i64 for fully dynamic values.
    /// Tuples need struct deref instead; callers check
    /// `tuple_payload_fields` first.
    pub fn unwrap_fn_for_result(&self, arg_id: ExprId) -> &'static str {
        use TypeAnnotation as T;
        let payload = match self.inferred_expr_type(arg_id) {
            Some(T::Result(inner)) => Some(*inner),
            // Already-unwrapped statics cannot be Err here in practice;
            // still route through the matching unwrapper for safety.
            Some(other) => Some(other),
            None => None,
        };
        match payload.as_ref() {
            Some(T::Float) | Some(T::SFloat) => "rl_result_unwrap_f64",
            Some(T::Bool) => "rl_result_unwrap_bool",
            Some(T::String) | Some(T::CString) => "rl_result_unwrap_str",
            Some(T::Array(_)) | Some(T::CArray(_)) => "rl_result_unwrap_arr",
            Some(T::Map(_, _)) | Some(T::CMap(_, _)) => "rl_result_unwrap_map",
            Some(T::Set(_)) | Some(T::CSet(_)) => "rl_result_unwrap_set",
            Some(T::Fn) | Some(T::Callback(_, _)) => "rl_result_unwrap_closure",
            _ => "rl_result_unwrap_i64",
        }
    }
}
