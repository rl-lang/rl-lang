use rl_ast::statements::ProgramAttribute;

use crate::common::parse;

#[test]
fn program_convert_attribute() {
    let (ast, statements) = parse("#![convert(kg=1000(g))]");
    assert!(statements.is_empty());

    assert_eq!(
        ast.program_attributes,
        vec![ProgramAttribute::Convert {
            symbol: "kg".to_string(),
            factor: 1000.0,
            base_symbol: "g".to_string(),
        }],
    );
}

#[test]
fn multiple_program_attributes() {
    let (ast, statements) = parse("#![convert(kg=1000(g))]\n#![convert(km=1000(m))]");
    assert!(statements.is_empty());

    assert_eq!(
        ast.program_attributes,
        vec![
            ProgramAttribute::Convert {
                symbol: "kg".to_string(),
                factor: 1000.0,
                base_symbol: "g".to_string(),
            },
            ProgramAttribute::Convert {
                symbol: "km".to_string(),
                factor: 1000.0,
                base_symbol: "m".to_string(),
            },
        ],
    );
}

#[test]
fn convert_attribute_can_use_float_factor() {
    let (ast, _) = parse("#![convert(cm=0.01(m))]");

    assert_eq!(
        ast.program_attributes,
        vec![ProgramAttribute::Convert {
            symbol: "cm".to_string(),
            factor: 0.01,
            base_symbol: "m".to_string(),
        }],
    );
}

#[test]
fn convert_attribute_before_declarations() {
    let (ast, statements) = parse("#![convert(kg=1000(g))]\ndec float weight: kg = 2.5");
    assert_eq!(statements.len(), 1);

    assert_eq!(
        ast.program_attributes,
        vec![ProgramAttribute::Convert {
            symbol: "kg".to_string(),
            factor: 1000.0,
            base_symbol: "g".to_string(),
        }],
    );
}

#[test]
fn define_custom_attribute() {
    let (ast, statements) = parse("#![define(route)]");
    assert!(statements.is_empty());

    assert_eq!(
        ast.program_attributes,
        vec![ProgramAttribute::Define {
            name: "route".to_string(),
        }],
    );
}

#[test]
fn custom_attribute_on_function() {
    use rl_ast::statements::{ItemAttribute, StatementKind};

    let (_, statements) =
        parse("#![define(route)]\n!#[route(\"/hi\")]\nfn hello() {\n}");
    assert_eq!(statements.len(), 1);

    let attrs = statements[0].kind.item_attributes();
    assert_eq!(
        attrs,
        &[ItemAttribute::Custom {
            name: "route".to_string(),
            args: vec!["/hi".to_string()],
        }],
    );
    assert_eq!(
        statements[0].kind.has_custom_attr("route"),
        Some(vec!["/hi".to_string()].as_slice())
    );
    assert_eq!(statements[0].kind.has_custom_attr("other"), None);
}

#[test]
fn undeclared_custom_attribute_errors() {
    let msg = crate::common::parse_assert_err("!#[bogus]\nfn f() {\n}");
    assert!(msg.contains("expected valid attribute"), "{msg}");
}

#[test]
fn duplicate_define_errors() {
    let msg = crate::common::parse_assert_err("#![define(x)]\n#![define(x)]");
    assert!(msg.contains("already defined"), "{msg}");
}

#[test]
fn builtin_name_cannot_be_redefined() {
    let msg = crate::common::parse_assert_err("#![define(test)]");
    assert!(msg.contains("built-in"), "{msg}");
}
