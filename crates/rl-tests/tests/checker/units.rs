use {
    crate::common,
    rl_ast::statements::{TypeAnnotation, UnitAnnotation},
    rl_checker::{
        structs::CheckType,
        units::Unit,
    },
};

#[test]
fn creates_symbol_unit() {
    let meters = Unit::symbol("m");

    assert_eq!(meters.exponent("m"), 1);
    assert_eq!(meters.exponent("s"), 0);
}

#[test]
fn multiplies_units_and_cancels_symbols() {
    let meters = Unit::symbol("m");
    let seconds = Unit::symbol("s");

    let speed = meters.divide(&seconds);
    let distance = speed.multiply(&seconds);

    assert_eq!(distance, Unit::symbol("m"));
}

#[test]
fn normalizes_compound_unit_annotation() {
    let annotation = UnitAnnotation::Divide(
        Box::new(UnitAnnotation::Multiply(
            Box::new(UnitAnnotation::Multiply(
                Box::new(UnitAnnotation::Symbol("x".to_string())),
                Box::new(UnitAnnotation::Symbol("y".to_string())),
            )),
            Box::new(UnitAnnotation::Symbol("r".to_string())),
        )),
        Box::new(UnitAnnotation::Symbol("i".to_string())),
    );

    let unit = Unit::from_annotation(&annotation);

    assert_eq!(unit.exponent("x"), 1);
    assert_eq!(unit.exponent("y"), 1);
    assert_eq!(unit.exponent("r"), 1);
    assert_eq!(unit.exponent("i"), -1);
}

#[test]
fn stores_unit_for_variable_declaration() {
    let checker = common::check(
        "dec float speed: m/s = 12.5",
    );

    assert!(
        checker.errors.is_empty(),
        "unexpected checker errors: {:?}",
        checker.errors
    );

    let item = checker.scopes[0]
        .get("speed")
        .expect("speed should be declared");

    assert_eq!(
        item.type_annotation,
        CheckType::Known(TypeAnnotation::Float)
    );

    assert!(!item.is_const);

    let unit = item
        .unit
        .as_ref()
        .expect("speed should have a unit");

    assert_eq!(unit.exponent("m"), 1);
    assert_eq!(unit.exponent("s"), -1);
}

#[test]
fn stores_unit_for_constant_declaration() {
    let checker = common::check(
        "CONST float speed: m/s = 12.5",
    );

    assert!(
        checker.errors.is_empty(),
        "unexpected checker errors: {:?}",
        checker.errors
    );

    let item = checker.scopes[0]
        .get("speed")
        .expect("speed should be declared");

    assert_eq!(
        item.type_annotation,
        CheckType::Known(TypeAnnotation::CFloat)
    );

    assert!(item.is_const);

    let unit = item
        .unit
        .as_ref()
        .expect("speed should have a unit");

    assert_eq!(unit.exponent("m"), 1);
    assert_eq!(unit.exponent("s"), -1);
}

#[test]
fn declaration_without_unit_has_none() {
    let checker = common::check(
        "dec float value = 12.5",
    );

    assert!(
        checker.errors.is_empty(),
        "unexpected checker errors: {:?}",
        checker.errors
    );

    let item = checker.scopes[0]
        .get("value")
        .expect("value should be declared");

    assert!(item.unit.is_none());
}

#[test]
fn stores_normalized_compound_unit() {
    let checker = common::check(
        "dec float value: x*y*r/i = 1.0",
    );

    assert!(
        checker.errors.is_empty(),
        "unexpected checker errors: {:?}",
        checker.errors
    );

    let item = checker.scopes[0]
        .get("value")
        .expect("value should be declared");

    let unit = item
        .unit
        .as_ref()
        .expect("value should have a unit");

    assert_eq!(unit.exponent("x"), 1);
    assert_eq!(unit.exponent("y"), 1);
    assert_eq!(unit.exponent("r"), 1);
    assert_eq!(unit.exponent("i"), -1);
}