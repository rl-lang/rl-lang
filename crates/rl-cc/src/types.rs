use rl_ast::statements::TypeAnnotation as T;

pub fn type_to_c(ta: &T) -> String {
    match ta {
        T::Int | T::CInt => "int64_t".into(),
        T::UInt | T::CUInt => "uint64_t".into(),
        T::SInt | T::CSInt => "int32_t".into(),
        T::SUInt | T::CSUInt => "uint32_t".into(),
        T::Float | T::CFloat => "double".into(),
        T::SFloat | T::CSFloat => "float".into(),
        T::Bool | T::CBool => "bool".into(),
        T::Char | T::CChar => "char".into(),
        T::String | T::CString => "rl_string".into(),
        T::Byte | T::CByte => "uint8_t".into(),
        T::SByte | T::CSByte => "int8_t".into(),
        T::BByte | T::CBByte => "uint16_t".into(),
        T::BSByte | T::CBSByte => "int16_t".into(),
        T::Null => "void".into(),
        // Compound types
        T::Array(_) | T::CArray(_) => "rl_array".into(),
        T::Map(_, _) | T::CMap(_, _) => "rl_map".into(),
        T::Set(_) | T::CSet(_) => "rl_set".into(),
        T::Result(_) | T::CResult(_) => "rl_result".into(),
        T::Tuple(elems) | T::CTuple(elems) => format!("rl_tuple_{}", elems.len()),
        T::Record(name) | T::CRecord(name) => format!("rl_Record_{}", name),
        T::Enum(name) | T::CEnum(name) => format!("int64_t /* {} */", name),
        T::Fn => "rl_closure".into(),
        T::Handle(_) => "int64_t".into(),
        T::HandleInfer => "int64_t".into(),
        T::Infer => "rl_value".into(),
        T::Generic(_) => "rl_value".into(),
        T::Any(_) | T::CAny(_) => "rl_value".into(),
        T::Callback(_, _) => "rl_closure".into(),
        T::Error | T::CError => "rl_result".into(),
    }
}
