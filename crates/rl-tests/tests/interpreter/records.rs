use rl_interpreter::values::Value;

use crate::common::eval_program;

fn field(v: &Value, name: &str) -> Value {
    match v {
        Value::Struct { fields, .. } => fields
            .borrow()
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, val)| val.clone())
            .unwrap_or_else(|| panic!("record has no field `{name}`")),
        other => panic!("expected Struct, got {other:?}"),
    }
}

#[test]
fn instance_method_reads_self_field() {
    let ev = eval_program(
        r#"
        record Point {
            int x,
            int y,
        }

        impl Point {
            fn sum(self) -> int {
                return self.x + self.y
            }
        }

        dec Point p = Point { x: 3, y: 4 }
        dec int total = p.sum()
        "#,
    )
    .unwrap();

    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(7)));
}

#[test]
fn associated_function_constructs_record() {
    let ev = eval_program(
        r#"
        record Point {
            int x,
            int y,
        }

        impl Point {
            fn new(int x, int y) -> Point {
                return Point { x: x, y: y }
            }
        }

        dec Point p = Point::new(1, 2)
        "#,
    )
    .unwrap();

    let p = ev.get_value_raw("p").expect("p should exist");
    assert_eq!(field(&p, "x"), Value::Integer(1));
    assert_eq!(field(&p, "y"), Value::Integer(2));
}

#[test]
fn method_calling_associated_function_internally() {
    let ev = eval_program(
        r#"
        record Point {
            int x,
            int y,
        }

        impl Point {
            fn new(int x, int y) -> Point {
                return Point { x: x, y: y }
            }

            fn scaled(self, int factor) -> Point {
                return Point::new(self.x * factor, self.y * factor)
            }
        }

        dec Point p = Point::new(1, 2)
        dec Point doubled = p.scaled(2)
        dec int dx = doubled.x
        "#,
    )
    .unwrap();

    assert_eq!(ev.get_value_raw("dx"), Some(Value::Integer(2)));
}

#[test]
fn self_field_mutation_is_visible_through_shared_reference() {
    let ev = eval_program(
        r#"
        record Counter {
            int count,
        }

        impl Counter {
            fn increment(self) {
                self.count = self.count + 1
            }
        }

        dec Counter c = Counter { count: 0 }
        c.increment()
        c.increment()
        c.increment()
        dec int final_count = c.count
        "#,
    )
    .unwrap();

    assert_eq!(ev.get_value_raw("final_count"), Some(Value::Integer(3)));
}

#[test]
fn multiple_impl_blocks_for_the_same_record_merge() {
    let ev = eval_program(
        r#"
        record Point {
            int x,
            int y,
        }

        impl Point {
            fn getx(self) -> int {
                return self.x
            }
        }

        impl Point {
            fn gety(self) -> int {
                return self.y
            }
        }

        dec Point p = Point { x: 5, y: 9 }
        dec int total = p.getx() + p.gety()
        "#,
    )
    .unwrap();

    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(14)));
}

#[test]
fn calling_undefined_method_on_a_record_falls_back_to_stdlib_lookup_and_errors() {
    let err = eval_program(
        r#"
        record Point {
            int x,
        }

        impl Point {
            fn known(self) -> int {
                return self.x
            }
        }

        dec Point p = Point { x: 0 }
        p.unknown()
        "#,
    )
    .is_err();

    assert!(err);
}

#[test]
fn calling_a_method_on_a_non_record_value_is_an_error() {
    let err = eval_program(
        r#"
        dec int x = 5
        x.magnitude()
        "#,
    )
    .is_err();

    assert!(err);
}

#[test]
fn calling_undefined_associated_function_is_an_error() {
    let err = eval_program(
        r#"
        record Point {
            int x,
        }

        Point::missing()
        "#,
    )
    .is_err();

    assert!(err);
}

#[test]
fn calling_a_self_less_associated_function_through_instance_syntax_mismatches_arity() {
    let err = eval_program(
        r#"
        record Point {
            int x,
        }

        impl Point {
            fn new(int x) -> Point {
                return Point { x: x }
            }
        }

        dec Point p = Point { x: 0 }
        p.new(1)
        "#,
    )
    .is_err();

    assert!(err);
}
