use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_ast::statements::TypeAnnotation;
use rl_utils::errors::{Error, Reason};
use rl_utils::span::Span;

/// Compiles `value is Type` to a C bool.
///
/// Most operands fold statically: CC values carry exact static types,
/// so a concrete operand answers at transpile time (const modulo const,
/// nominal by name, `handle` annotations match any concrete handle).
/// Only dynamically-typed operands (`any` storage, untyped values)
/// need a runtime tag check against the boxed `rl_value`:
///
/// - the target must occur in the operand's known members (when known),
///   else the answer is statically false;
/// - boxable targets compare `rl_value.tag`;
/// - `handle` members share the int tag, so `is handle` is approximate
///   on the C backend (true for any int payload) while exact on the VM.
///
/// Anything else (unknown shapes, unboxable targets over dynamic
/// storage) fails loudly instead of generating a blind guess.
pub(super) fn compile_is(
    cc: &mut CCodegen,
    value: &ExprId,
    target: &TypeAnnotation,
) -> Result<(), Error> {
    // C tag for a boxable target, or None when the target can never
    // sit in dynamic storage.
    fn tag_of(target: &TypeAnnotation) -> Option<&'static str> {
        use TypeAnnotation as T;
        match target {
            T::Int | T::CInt | T::UInt | T::CUInt | T::SInt | T::CSInt | T::SUInt | T::CSUInt => {
                Some("RL_VTAG_I64")
            }
            T::Float | T::CFloat | T::SFloat | T::CSFloat => Some("RL_VTAG_F64"),
            T::Bool | T::CBool => Some("RL_VTAG_BOOL"),
            T::String | T::CString => Some("RL_VTAG_STR"),
            T::Array(_) | T::CArray(_) => Some("RL_VTAG_ARR"),
            T::Map(_, _) | T::CMap(_, _) => Some("RL_VTAG_MAP"),
            T::Set(_) | T::CSet(_) => Some("RL_VTAG_SET"),
            T::Null => Some("RL_VTAG_NULL"),
            _ => None,
        }
    }
    // exact static equality, const-agnostic, nominal by name.
    // containers compare shape-only (never element types), exactly
    // like the VM's kind test behind `__type_of`.
    fn static_eq(a: &TypeAnnotation, b: &TypeAnnotation) -> bool {
        use TypeAnnotation as T;
        match (a, b) {
            (T::Record(x) | T::CRecord(x), T::Record(y) | T::CRecord(y)) => x == y,
            (T::Enum(x) | T::CEnum(x), T::Enum(y) | T::CEnum(y)) => x == y,
            (T::Handle(_) | T::HandleInfer, T::Handle(_) | T::HandleInfer) => true,
            (T::Result(x) | T::CResult(x), T::Result(y) | T::CResult(y)) => static_eq(x, y),
            _ => shape_of(a) == shape_of(b) && shape_of(a).is_some(),
        }
    }

    /// Collapses a type to a shape code for `static_eq` (const-agnostic;
    /// containers by kind). Mirrors the VM `IsKind` codes 1:1.
    fn shape_of(t: &TypeAnnotation) -> Option<u8> {
        use TypeAnnotation as T;
        Some(match t {
            T::Int | T::CInt => 1,
            T::UInt | T::CUInt => 2,
            T::SInt | T::CSInt => 3,
            T::SUInt | T::CSUInt => 4,
            T::Float | T::CFloat => 5,
            T::SFloat | T::CSFloat => 6,
            T::Bool | T::CBool => 7,
            T::String | T::CString => 8,
            T::Char | T::CChar => 9,
            T::Byte | T::CByte => 10,
            T::SByte | T::CSByte => 11,
            T::BByte | T::CBByte => 12,
            T::BSByte | T::CBSByte => 13,
            T::Array(_) | T::CArray(_) => 14,
            T::Map(_, _) | T::CMap(_, _) => 15,
            T::Set(_) | T::CSet(_) => 16,
            T::Tuple(_) | T::CTuple(_) => 17,
            T::Fn => 20,
            T::Null => 0,
            T::Error | T::CError => 23,
            _ => return None,
        })
    }

    let operand = cc.inferred_expr_type(*value);
    // 1. concrete operand: fold statically.
    if let Some(t) = &operand {
        match t {
            TypeAnnotation::Any(_) | TypeAnnotation::CAny(_) | TypeAnnotation::Infer | TypeAnnotation::Generic(_) => {}
            concrete => {
                cc.writer
                    .write(if static_eq(concrete, target) { "true" } else { "false" });
                return Ok(());
            }
        }
    }
    // 2. union storage: the target must occur in the known members.
    if let Some(TypeAnnotation::Any(members) | TypeAnnotation::CAny(members)) = &operand {
        if !members.iter().any(|m| static_eq(m, target)) {
            cc.writer.write("false");
            return Ok(());
        }
    }
    // 3. runtime tag check over the boxed value (single evaluation
    // inside the call arguments - no temporary needed).
    match tag_of(target) {
        Some(tag) => {
            cc.writer.write(&format!("rl_value_has_tag("));
            cc.compile_expr(*value)?;
            cc.writer.write(&format!(", {tag})"));
            Ok(())
        }
        None => Err(Error::at(
            Reason::Compile,
            format!("`is` cannot test {target:?} over a dynamic value on the C backend yet"),
            Span::dummy(),
        )),
    }
}
