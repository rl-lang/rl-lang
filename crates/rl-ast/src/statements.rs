//! Statement AST nodes, type annotations, and parameter definitions.
//!
//! A [`Statement`] is any construct that does not directly produce a value
//! (declarations, control flow, imports). Every node carries the source [`Span`]
//! it was parsed from.
//!
//! # Resolved variants
//! The [`Resolver`] pass rewrites name-based declaration and loop variants into
//! their `Resolved*` counterparts, adding a `slot: usize` field that gives the
//! variable's index in its environment frame. This eliminates all runtime
//! HashMap lookups in the evaluator.
//!
//! # Type annotations
//! [`TypeAnnotation`] distinguishes mutable (`dec`) and constant (`const`)
//! bindings at the type level via separate variants (`Int` vs `CInt`, etc.).
//!
//! [`Resolver`]: crate::resolver
use std::rc::Rc;

use crate::ExprId;
use rl_utils::span::Span;

/// A statement paired with its source span.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HandleKind {
    C = 0,
    Net = 1,
    Http = 2,
    Audio = 3,
    Gui = 4,
    File = 5,
    Buffer = 6,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// A unit-of-measure annotation attached to an `int`/`float` declaration,
/// e.g. the `m/s` in `dec float speed: m/s = 12.5`.
///
/// Units are a compile-time-only concept: they are parsed into this syntax
/// tree, normalized by the checker's unit module and then discarded - they
/// never reach the resolver, VM, or interpreter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitAnnotation {
    /// A single unit symbol, e.g. `m`, `s`, or `kg`.
    Symbol(String),
    /// Multiplication of two units, e.g. `kg*m`.
    Multiply(Box<UnitAnnotation>, Box<UnitAnnotation>),
    /// Division of two units, e.g. `m/s`.
    Divide(Box<UnitAnnotation>, Box<UnitAnnotation>),
}

/// A program-level attribute declared with `#![name(...)]`, e.g.
/// `#![convert(kg=1000(g))]`. Program attributes are compile-time-only: the
/// checker consumes them and they never reach the resolver, VM, or interpreter.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramAttribute {
    /// Declares a unit conversion factor between two symbols:
    /// `#![convert(kg=1000(g))]` reads "1 `symbol` = `factor` × `base_symbol`",
    /// so a value of `symbol` multiplied by `factor` yields `base_symbol`.
    Convert {
        symbol: String,
        factor: f64,
        base_symbol: String,
    },
    /// Declares a custom item attribute: `#![define(x)]` allows `!#[x]`
    /// and `!#[x("arg", ...)]` on items. Markers only - the compiler
    /// attaches them, user tooling and future phases interpret them.
    Define {
        name: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    /// A mutable variable declaration: `dec T name = value`.
    VariableDeclaration {
        name: String,
        type_annotation: TypeAnnotation,
        value: ExprId,
        /// Compile-time-only unit annotation (`dec float speed: m/s = ...`).
        /// Discarded by the resolver before execution.
        unit_annotation: Option<UnitAnnotation>,
        /// Item-level attributes (e.g. `!#[allow(unused)]`).
        item_attributes: Vec<ItemAttribute>,
    },
    /// Resolver-annotated mutable variable declaration. `slot` is the index
    /// in the current environment frame.
    ResolvedVariableDeclaration {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// An immutable constant declaration: `const T NAME = value`.
    ConstantDeclaration {
        name: String,
        type_annotation: TypeAnnotation,
        value: ExprId,
        /// Compile-time-only unit annotation (`const float SPEED: m/s = 12.5`).
        /// Discarded by the resolver before execution.
        unit_annotation: Option<UnitAnnotation>,
        /// Item-level attributes (e.g. `!#[allow(unused)]`).
        item_attributes: Vec<ItemAttribute>,
    },
    /// Resolver-annotated constant declaration.
    ResolvedConstantDeclaration {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// A mutable array declaration with an inline literal: `dec array[T] name = [items]`.
    Array {
        name: String,
        type_annotation: TypeAnnotation,
        value: Vec<ExprId>,
    },
    /// An immutable array declaration with an inline literal: `const array[T] NAME = [items]`.
    ConstantArray {
        name: String,
        type_annotation: TypeAnnotation,
        value: Vec<ExprId>,
    },
    /// Resolver-annotated mutable array declaration. `value` is the
    /// initialiser expression (may be an [`ExpressionKind::ArrayLiteral`]).
    ///
    /// [`ExpressionKind::ArrayLiteral`]: crate::ast::nodes::ExpressionKind::ArrayLiteral
    ResolvedArray {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// Resolver-annotated constant array declaration.
    ResolvedConstantArray {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },

    Map {
        name: String,
        type_annotation: TypeAnnotation,
        entries: Vec<(ExprId, ExprId)>,
    },
    ConstantMap {
        name: String,
        type_annotation: TypeAnnotation,
        entries: Vec<(ExprId, ExprId)>,
    },
    ResolvedMap {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    ResolvedConstantMap {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },

    Set {
        name: String,
        type_annotation: TypeAnnotation,
        items: Vec<ExprId>,
    },
    ConstantSet {
        name: String,
        type_annotation: TypeAnnotation,
        items: Vec<ExprId>,
    },
    ResolvedSet {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    ResolvedConstantSet {
        name: String,
        slot: usize,
        type_annotation: TypeAnnotation,
        value: ExprId,
    },
    /// A bare expression used as a statement (e.g. a function call whose
    /// return value is discarded, or a newline placeholder).
    Expression(ExprId),
    /// A `while condition { body }` loop.
    While {
        condition: ExprId,
        body: Vec<Statement>,
    },
    Loop(Vec<Statement>),
    /// A C-style `for [init, cond, incr] { body }` loop.
    For {
        initializer: Box<Statement>,
        condition: ExprId,
        increment: ExprId,
        body: Vec<Statement>,
    },
    /// Resolver-annotated C-style for loop.
    ResolvedFor {
        initializer: Box<Statement>,
        condition: ExprId,
        increment: ExprId,
        body: Vec<Statement>,
    },
    /// A range-based `for x in N..M { body }` loop. The range is pre-evaluated
    /// at parse time into a [`Range`] statement.
    ForRange {
        variable: String,
        range: Box<Statement>,
        body: Vec<Statement>,
    },
    /// Resolver-annotated range-based for loop. `slot` is the loop variable's
    /// environment index.
    ResolvedForRange {
        slot: usize,
        variable: String,
        range: Box<Statement>,
        body: Vec<Statement>,
    },
    /// A foreach `for item in iterable { body }` loop over an array expression.
    ForEach {
        variable: String,
        iterable: ExprId,
        body: Vec<Statement>,
    },
    /// Resolver-annotated foreach loop.
    ResolvedForEach {
        slot: usize,
        variable: String,
        iterable: ExprId,
        body: Vec<Statement>,
    },
    /// A pre-evaluated integer range produced by the parser for `for x in N..M`.
    Range(Vec<i64>),
    /// A single branch of a conditional: either `if condition { body }` (`condition`
    /// is `Some`) or `else { body }` (`condition` is `None`).
    ConditionalBranch {
        condition: Option<ExprId>,
        body: Vec<Statement>,
        /// Precomputed by the resolver
        /// true if `body` declares any local variable/constant
        /// and therefore needs its own scope frame
        needs_scope: bool,
    },
    /// A full if / else-if / else chain.
    Conditional {
        if_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    /// A named function declaration.
    FunctionDeclaration {
        name: String,
        params: Vec<Param>,
        return_type: TypeAnnotation,
        body: Vec<Statement>,
        /// `true` when the function is marked with `!#[entry]`.
        attribute: Option<FunctionAttribute>,
        /// Item-level attributes (e.g. `!#[allow(unused)]`).
        item_attributes: Vec<ItemAttribute>,
        /// `requires` contract clauses (checked at entry by desugared guards).
        requires: Vec<ContractClause>,
        /// `ensures` contract clauses (checked at every return by desugared guards).
        ensures: Vec<ContractClause>,
    },
    /// Resolver-annotated function declaration. `slot` is the function's
    /// index in the current environment frame.
    ResolvedFunctionDeclaration {
        name: String,
        slot: usize,
        params: Vec<Param>,
        return_type: TypeAnnotation,
        body: Vec<Statement>,
        attribute: Option<FunctionAttribute>,
        requires: Vec<ContractClause>,
        ensures: Vec<ContractClause>,
    },
    /// A `return expr` or bare `return` statement.
    Return(Option<ExprId>),
    /// Breaks out of the nearest enclosing loop.
    Break,
    /// Skips to the next iteration of the nearest enclosing loop.
    Continue,
    /// A stdlib import: `get std::ns::fn` or `get fn from std::ns`.
    Import {
        /// Imported names as `(original_name, alias)`. Empty when `wildcard` is true.
        names: Vec<(String, Option<String>)>,
        /// When true, imports all functions from the module (`get * from std::ns`).
        wildcard: bool,
        /// Module path (e.g. `["std", "math"]`).
        path: Vec<String>,
    },
    /// A file module import: `get mymodule` or `get mymodule::sub`.
    ImportFile {
        path: Vec<String>,
    },
    /// A resolved file import - the imported file's statements are inlined here.
    ResolvedImportFile {
        path: Vec<String>,
        body: Vec<Statement>,
    },
    /// A named file import: `get fn, fn from mymodule::sub`.
    ImportFileNamed {
        path: Vec<String>,
        names: Vec<String>,
    },
    /// A type alias: `type Name Target`. Compile-time only; the target
    /// is stored resolved in the `Ast` alias table, and uses vanish
    /// before codegen. Carries item attributes (`!#[deprecated]`).
    TypeAlias {
        name: String,
        target: TypeAnnotation,
        item_attributes: Vec<ItemAttribute>,
    },

    DestructureDeclaration {
        bindings: Vec<(TypeAnnotation, String)>,
        value: ExprId,
    },
    ResolvedDestructureDeclaration {
        bindings: Vec<(TypeAnnotation, String)>,
        slots: Vec<usize>,
        value: ExprId,
    },

    Match {
        value: ExprId,
        arms: Vec<(MatchPattern, Vec<Statement>)>,
    },

    /// A record (struct) type declaration: `record Name { int a, string b }`.
    RecordDeclaration {
        name: String,
        fields: Vec<(String, TypeAnnotation)>,
    },

    /// An `impl` block attaching methods to a record:
    /// `impl Name { fn method(self) { ... } }`.
    ///
    /// A method with `self` as its first parameter is an instance method,
    /// called via `value.method(args)`. A method without `self` is an
    /// associated function, called via `Name::method(args)`.
    ///
    /// `methods` holds unresolved `FunctionDeclaration` statements.
    ImplBlock {
        record: String,
        methods: Vec<Statement>,
    },
    /// Resolver-annotated impl block. `methods` holds
    /// `ResolvedFunctionDeclaration` statements whose `slot` is unused
    ResolvedImplBlock {
        record: String,
        methods: Vec<Statement>,
    },

    /// A tag (enum) type declaration: `tag Name { VariantA, VariantB }`.
    TagDeclaration {
        name: String,
        variants: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum MatchPattern {
    Literal(ExprId),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionAttribute {
    Entry,
    /// `!#[init]` (no priority) or `!#[init=n]` (numbered priorities run
    /// first, ascending; unnumbered runs last in declaration order).
    Init(Option<u32>),
    /// `!#[final]` (no priority) or `!#[final=n]` (same ordering as `init`).
    Final(Option<u32>),
    /// `!#[test]` or `!#[test(group("g"), register("r"), cases(n))]`.
    Test(TestParams),
    /// `!#[setup]` - runs before each `!#[test]` case in the file.
    Setup,
    /// `!#[teardown]` - runs after each `!#[test]` case in the file.
    Teardown,
}

/// Parameters for `!#[test(...)]`. All optional and freely composable:
/// `group` attaches the case to a named group (filtering, reporting),
/// `register` puts it in a named registry (dynamic lookup, benchmarks),
/// `cases(n)` turns it into a property test over `n` generated inputs.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestParams {
    pub group: Option<String>,
    pub register: Option<String>,
    pub cases: Option<u64>,
}

/// A lint name recognized by `!#[allow(...)]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lint {
    Unused,
    Deprecated,
}

/// A general item-level attribute - as opposed to [`FunctionAttribute`], which
/// only covers the four function-lifecycle markers (`entry`/`init`/`final`/`test`).
/// Attached to whichever `fn`/`dec`/`const` immediately follows it.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemAttribute {
    /// `!#[allow(unused)]`, `!#[allow(deprecated)]`, or both:
    /// `!#[allow(unused, deprecated)]`
    Allow(Vec<Lint>),
    /// `!#[deprecated]` or `!#[deprecated("use foo() instead")]`
    Deprecated(Option<String>),
    /// A user-defined marker from `#![define(name)]`, applied as `!#[name]`
    /// or `!#[name("arg", ...)]`. Attaches only - interpretation belongs
    /// to tooling and future compiler phases.
    Custom {
        name: String,
        args: Vec<String>,
    },
}

/// The type of a variable, constant, or parameter binding.
///
/// Mutable variants (`Int`, `Float`, etc.) are produced by `dec` declarations.
/// Constant variants (`CInt`, `CFloat`, etc.) are produced by `const` declarations.
/// `Array(T)` / `CArray(T)` are the mutable / constant array forms.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    /// Mutable 64-bit signed integer.
    Int,
    /// Mutable 64-bit unsigned integer.
    UInt,
    SInt,
    SUInt,
    /// Mutable 64-bit float.
    Float,
    SFloat,
    /// Mutable boolean.
    Bool,
    /// Mutable string.
    String,
    /// Mutable byte (`u8`).
    Byte,
    SByte,
    BByte,
    BSByte,
    /// Mutable character.
    Char,
    /// Mutable array with a typed element.
    Array(Box<TypeAnnotation>),
    Map(Box<TypeAnnotation>, Box<TypeAnnotation>),
    Set(Box<TypeAnnotation>),
    CSInt,
    CSUInt,
    /// Constant 64-bit signed integer.
    CInt,
    /// Constant 64-bit Unsigned integer.
    CUInt,
    /// Constant 64-bit float.
    CFloat,
    CSFloat,
    /// Constant boolean.
    CBool,
    /// Constant string.
    CString,
    /// Constant byte.
    CByte,
    CSByte,
    CBByte,
    CBSByte,
    /// Constant character.
    CChar,
    /// Constant array with a typed element.
    CArray(Box<TypeAnnotation>),
    CMap(Box<TypeAnnotation>, Box<TypeAnnotation>),
    CSet(Box<TypeAnnotation>),
    /// A function value (used for `fn`-typed parameters and variables).
    Fn,
    /// Absence of a type - used as the default return type when none is annotated.
    Null,
    /// Placeholder used by `dec name = value` bindings.
    Infer,

    Tuple(Rc<Vec<TypeAnnotation>>),
    CTuple(Rc<Vec<TypeAnnotation>>),

    Error,
    CError,

    Result(Box<TypeAnnotation>),
    CResult(Box<TypeAnnotation>),

    /// A named record (struct) type, mutable binding.
    Record(String),
    /// A named record (struct) type, constant binding.
    CRecord(String),

    /// A named tag (enum) type, mutable binding.
    Enum(String),
    /// A named tag (enum) type, constant binding.
    CEnum(String),

    Generic(String),
    Callback(Vec<TypeAnnotation>, Box<TypeAnnotation>),

    // ---std-specific---
    Handle(HandleKind),
    /// Placeholder used by `dec handle name = v` bindings.
    HandleInfer,
    /// Union of member types (`any[int, string]`): a value of any one
    /// member type. Members are normalized at parse (flattened,
    /// deduplicated, never empty).
    Any(Rc<Vec<TypeAnnotation>>),
    /// Constant union (`const any[int, string] ...`).
    CAny(Rc<Vec<TypeAnnotation>>),
}

/// A single function or lambda parameter: a name, its type annotation,
/// and an optional refinement predicate (`int amt: >0`).
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub param_name: String,
    pub param_type: TypeAnnotation,
    pub refinement: Option<ParamRefinement>,
}

/// Comparison operator in a parameter refinement (`: >0`, `: >=amt`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefineOp {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
}

/// Right-hand side of a parameter refinement: a literal, or another
/// parameter's name (e.g. `int balance: >=amt`).
#[derive(Debug, Clone, PartialEq)]
pub enum RefineOperand {
    Integer(i64),
    Str(String),
    Bool(bool),
    Param(String),
}

/// A parameter refinement predicate (`amt: >0` on `int amt`).
#[derive(Debug, Clone, PartialEq)]
pub struct ParamRefinement {
    pub op: RefineOp,
    pub operand: RefineOperand,
}

/// One `requires`/`ensures` contract clause: a boolean condition over the
/// parameters (plus `ret` in `ensures`) with an optional failure message.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractClause {
    pub condition: ExprId,
    pub message: Option<ExprId>,
}

impl TypeAnnotation {
    pub fn contains_handle_infer(&self) -> bool {
        match self {
            TypeAnnotation::HandleInfer => true,
            TypeAnnotation::Array(inner)
            | TypeAnnotation::CArray(inner)
            | TypeAnnotation::Set(inner)
            | TypeAnnotation::CSet(inner)
            | TypeAnnotation::Result(inner)
            | TypeAnnotation::CResult(inner) => inner.contains_handle_infer(),

            TypeAnnotation::Map(k, v) | TypeAnnotation::CMap(k, v) => {
                k.contains_handle_infer() || v.contains_handle_infer()
            }

            TypeAnnotation::Tuple(items) | TypeAnnotation::CTuple(items) => {
                items.iter().any(TypeAnnotation::contains_handle_infer)
            }

            _ => false,
        }
    }

    pub fn resolve_handle_infer(&self, actual: &TypeAnnotation) -> Option<TypeAnnotation> {
        use TypeAnnotation::*;

        if !self.contains_handle_infer() {
            return if self == actual {
                Some(self.clone())
            } else {
                None
            };
        }

        Some(match (self, actual) {
            (HandleInfer, Handle(kind)) => Handle(*kind),
            (Array(d), Array(a)) | (Array(d), CArray(a)) => {
                Array(Box::new(d.resolve_handle_infer(a)?))
            }
            (CArray(d), Array(a)) | (CArray(d), CArray(a)) => {
                CArray(Box::new(d.resolve_handle_infer(a)?))
            }
            (Set(d), Set(a)) | (Set(d), CSet(a)) => Set(Box::new(d.resolve_handle_infer(a)?)),
            (CSet(d), Set(a)) | (CSet(d), CSet(a)) => CSet(Box::new(d.resolve_handle_infer(a)?)),
            (Result(d), Result(a)) | (Result(d), CResult(a)) => {
                Result(Box::new(d.resolve_handle_infer(a)?))
            }
            (CResult(d), Result(a)) | (CResult(d), CResult(a)) => {
                CResult(Box::new(d.resolve_handle_infer(a)?))
            }
            (Map(dk, dv), Map(ak, av)) | (Map(dk, dv), CMap(ak, av)) => Map(
                Box::new(dk.resolve_handle_infer(ak)?),
                Box::new(dv.resolve_handle_infer(av)?),
            ),
            (Tuple(d), Tuple(a)) | (Tuple(d), CTuple(a)) if d.len() == a.len() => Tuple(Rc::new(
                d.iter()
                    .zip(a.iter())
                    .map(|(d, a)| d.resolve_handle_infer(a))
                    .collect::<Option<Vec<_>>>()?,
            )),
            _ => return None,
        })
    }
}

impl StatementKind {
    /// Item-level attributes (`!#[allow(...)]`, `!#[deprecated(...)]`) on
    /// `fn` / `dec` / `const` declarations. The single canonical query -
    /// compiler passes should call this instead of matching variants.
    pub fn item_attributes(&self) -> &[ItemAttribute] {
        match self {
            StatementKind::FunctionDeclaration { item_attributes, .. }
            | StatementKind::VariableDeclaration { item_attributes, .. }
            | StatementKind::ConstantDeclaration { item_attributes, .. } => item_attributes,
            _ => &[],
        }
    }

    /// Function-lifecycle marker (`!#[entry]`, `!#[test]`, ...), if any.
    pub fn function_attribute(&self) -> Option<&FunctionAttribute> {
        match self {
            StatementKind::FunctionDeclaration { attribute, .. }
            | StatementKind::ResolvedFunctionDeclaration { attribute, .. } => attribute.as_ref(),
            _ => None,
        }
    }

    /// True when `!#[allow(lint)]` covers this item.
    pub fn has_lint(&self, lint: Lint) -> bool {
        self.item_attributes().iter().any(|attr| match attr {
            ItemAttribute::Allow(lints) => lints.contains(&lint),
            _ => false,
        })
    }

    /// The deprecation message, if `!#[deprecated]` marks this item.
    /// `None` means not deprecated; `Some(None)` is a bare marker.
    pub fn deprecation(&self) -> Option<Option<&str>> {
        self.item_attributes().iter().find_map(|attr| match attr {
            ItemAttribute::Deprecated(msg) => Some(msg.as_deref()),
            _ => None,
        })
    }

    /// Custom marker args, if `!#[name]` (from `#![define(name)]`) marks
    /// this item. Answers "does something have x" with its arguments.
    pub fn has_custom_attr(&self, name: &str) -> Option<&[String]> {
        self.item_attributes().iter().find_map(|attr| match attr {
            ItemAttribute::Custom { name: n, args } if n == name => Some(args.as_slice()),
            _ => None,
        })
    }
}
