use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::TypeAnnotation;
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// `x = [items]` — binds the evaluated array. `is_const` selects a
/// `const rl_array` binding and records a `CArray` element type.
pub(super) fn compile_array_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
    is_const: bool,
) -> Result<(), Error> {
    let c_name = mangle(name);
    cc.declare(name, &c_name);
    cc.var_types.insert(
        name.to_string(),
        if is_const {
            TypeAnnotation::CArray(Box::new(type_annotation.clone()))
        } else {
            TypeAnnotation::Array(Box::new(type_annotation.clone()))
        },
    );
    cc.writer.write_indent();
    if is_const {
        cc.writer.write(&format!("const rl_array {} = ", c_name));
    } else {
        cc.writer.write(&format!("rl_array {} = ", c_name));
    }
    cc.compile_expr(value)?;
    cc.writer.write(";\n");
    Ok(())
}

/// `x = {k: v, ...}` — creates an empty map, then inserts each
/// string-keyed literal entry with value wrapping for the value type.
pub(super) fn compile_map_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
    is_const: bool,
) -> Result<(), Error> {
    let c_name = mangle(name);
    cc.declare(name, &c_name);
    cc.var_types
        .insert(name.to_string(), type_annotation.clone());
    cc.writer.write_indent();
    if is_const {
        cc.writer
            .write(&format!("const rl_map {} = rl_map_new();\n", c_name));
    } else {
        cc.writer
            .write(&format!("rl_map {} = rl_map_new();\n", c_name));
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
                if is_const {
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
/// item with element-type wrapping.
pub(super) fn compile_set_decl(
    cc: &mut CCodegen,
    name: &str,
    type_annotation: &TypeAnnotation,
    value: ExprId,
    is_const: bool,
) -> Result<(), Error> {
    let c_name = mangle(name);
    cc.declare(name, &c_name);
    cc.var_types
        .insert(name.to_string(), type_annotation.clone());
    cc.writer.write_indent();
    if is_const {
        cc.writer
            .write(&format!("const rl_set {} = rl_set_new();\n", c_name));
    } else {
        cc.writer
            .write(&format!("rl_set {} = rl_set_new();\n", c_name));
    }
    let set_expr = cc.ast.exprs.get(value);
    if let ExpressionKind::SetLiteral(items) = &set_expr.kind {
        let elem_type = match type_annotation {
            TypeAnnotation::Set(et) | TypeAnnotation::CSet(et) => (**et).clone(),
            _ => TypeAnnotation::Int,
        };
        for item_id in items {
            cc.writer.write_indent();
            if is_const {
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
