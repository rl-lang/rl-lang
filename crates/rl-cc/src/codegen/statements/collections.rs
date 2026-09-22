use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::TypeAnnotation;
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// File-scope storage for a collection global plus scope registration.
/// `const` is dropped: C file-scope initializers must be constant, so
/// globals initialize inside `main` instead.
fn declare_global_collection(
    cc: &mut CCodegen,
    name: &str,
    stored: TypeAnnotation,
) {
    if cc.global_names.contains(name) {
        return;
    }
    let c_name = mangle(name);
    let c_type = match &stored {
        TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => "rl_array".to_string(),
        TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _) => "rl_map".to_string(),
        TypeAnnotation::Set(_) | TypeAnnotation::CSet(_) => "rl_set".to_string(),
        other => crate::types::type_to_c(other),
    };
    cc.globals_code.push_str(&format!("{} {};\n", c_type, c_name));
    cc.declare(name, &c_name);
    cc.var_types.insert(name.to_string(), stored);
    cc.global_names.insert(name.to_string());
}

/// Declare a top-level array at file scope (pre-pass).
pub(crate) fn declare_global_array(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    is_const: bool,
) {
    let stored = if is_const {
        TypeAnnotation::CArray(Box::new(type_annotation.clone()))
    } else {
        TypeAnnotation::Array(Box::new(type_annotation.clone()))
    };
    declare_global_collection(cc, name, stored);
}

/// Declare a top-level map or set at file scope (pre-pass).
pub(crate) fn declare_global_map_set(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
) {
    // Resolved sets carry only the element type while maps carry the
    // full Map(K, V); wrap a bare element into Set so storage is rl_set.
    match type_annotation {
        TypeAnnotation::Map(_, _)
        | TypeAnnotation::CMap(_, _)
        | TypeAnnotation::Set(_)
        | TypeAnnotation::CSet(_) => {
            declare_global_collection(cc, name, type_annotation.clone());
        }
        other => {
            declare_global_collection(
                cc,
                name,
                TypeAnnotation::Set(Box::new(other.clone())),
            );
        }
    }
}

/// `x = [items]` — binds the evaluated array. `is_const` selects a
/// `const rl_array` binding and records a `CArray` element type.
/// During global init the storage already exists: only assign.
pub(super) fn compile_array_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
    is_const: bool,
) -> Result<(), Error> {
    let c_name = if cc.in_global_init {
        mangle(name)
    } else {
        cc.declare_unique(name)
    };
    if !cc.in_global_init {
        cc.var_types.insert(
            name.to_string(),
            if is_const {
                TypeAnnotation::CArray(Box::new(type_annotation.clone()))
            } else {
                TypeAnnotation::Array(Box::new(type_annotation.clone()))
            },
        );
    }
    cc.writer.write_indent();
    if cc.in_global_init {
        cc.writer.write(&format!("{} = ", c_name));
    } else if is_const {
        cc.writer.write(&format!("const rl_array {} = ", c_name));
    } else {
        cc.writer.write(&format!("rl_array {} = ", c_name));
    }
    let saved_hint = cc.array_elem_hint.clone();
    cc.array_elem_hint = Some(type_annotation.clone());
    cc.compile_expr(value)?;
    cc.array_elem_hint = saved_hint;
    cc.writer.write(";\n");
    Ok(())
}

/// `x = {k: v, ...}` — creates an empty map, then inserts each
/// string-keyed literal entry with value wrapping for the value type.
/// During global init the storage already exists: only assign.
pub(super) fn compile_map_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
    is_const: bool,
) -> Result<(), Error> {
    let c_name = if cc.in_global_init {
        mangle(name)
    } else {
        cc.declare_unique(name)
    };
    if !cc.in_global_init {
        cc.var_types
            .insert(name.to_string(), type_annotation.clone());
    }
    cc.writer.write_indent();
    if cc.in_global_init || !is_const {
        cc.writer
            .write(&format!("{} = rl_map_new();\n", c_name));
    } else {
        cc.writer
            .write(&format!("const rl_map {} = rl_map_new();\n", c_name));
    }
    let map_expr = cc.ast.exprs.get(value);
    if let ExpressionKind::MapLiteral(entries) = &map_expr.kind {
        let val_type = match type_annotation {
            TypeAnnotation::Map(_, vt) | TypeAnnotation::CMap(_, vt) => (**vt).clone(),
            _ => TypeAnnotation::Int,
        };
        for (key_id, val_id) in entries {
            let key_expr = cc.ast.exprs.get(*key_id);
            if let ExpressionKind::String(key_str) = &key_expr.kind {
                cc.writer.write_indent();
                if is_const && !cc.in_global_init {
                    cc.writer.write(&format!(
                        "rl_map_set((rl_map*)&{}, \"{}\", ",
                        c_name, key_str
                    ));
                } else {
                    cc.writer.write(&format!(
                        "rl_map_set(&{}, \"{}\", ",
                        c_name, key_str
                    ));
                }
                cc.emit_value_wrapping(&val_type, *val_id)?;
                cc.writer.write(");\n");
            }
        }
    }
    Ok(())
}

/// `x = set{items}` — creates an empty set, then adds each literal
/// item with element-type wrapping. During global init only assign.
pub(super) fn compile_set_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
    is_const: bool,
) -> Result<(), Error> {
    let c_name = if cc.in_global_init {
        mangle(name)
    } else {
        cc.declare_unique(name)
    };
    if !cc.in_global_init {
        cc.var_types
            .insert(name.to_string(), type_annotation.clone());
    }
    cc.writer.write_indent();
    if cc.in_global_init || !is_const {
        cc.writer
            .write(&format!("{} = rl_set_new();\n", c_name));
    } else {
        cc.writer
            .write(&format!("const rl_set {} = rl_set_new();\n", c_name));
    }
    let set_expr = cc.ast.exprs.get(value);
    if let ExpressionKind::SetLiteral(items) = &set_expr.kind {
        let elem_type = match type_annotation {
            TypeAnnotation::Set(et) | TypeAnnotation::CSet(et) => (**et).clone(),
            // Resolved sets carry only the element type.
            other => other.clone(),
        };
        for item_id in items {
            cc.writer.write_indent();
            if is_const && !cc.in_global_init {
                cc.writer
                    .write(&format!("rl_set_add((rl_set*)&{}, ", c_name));
            } else {
                cc.writer.write(&format!("rl_set_add(&{}, ", c_name));
            }
            cc.emit_value_wrapping(&elem_type, *item_id)?;
            cc.writer.write(");\n");
        }
    }
    Ok(())
}
