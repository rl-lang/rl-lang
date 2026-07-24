use crate::common;
use rl_vm::VmValue;

fn field(v: &VmValue, name: &str) -> VmValue {
    match v {
        VmValue::Record { fields, .. } => fields
            .get(name)
            .unwrap_or_else(|| panic!("record has no field `{name}`")),
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn instance_method_reads_self_field() {
    let result = common::compile_and_run(
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
        p.sum()
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(7));
}

#[test]
fn associated_function_constructs_record() {
    let result = common::compile_and_run(
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

        Point::new(1, 2)
        "#,
    )
    .expect("vm run failed");

    assert_eq!(field(&result, "x"), VmValue::Int(1));
    assert_eq!(field(&result, "y"), VmValue::Int(2));
}

#[test]
fn method_calling_associated_function_internally() {
    let result = common::compile_and_run(
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
        doubled.x
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(2));
}

#[test]
fn chained_method_calls() {
    let result = common::compile_and_run(
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

        Point::new(1, 1).scaled(2).scaled(3).x
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(6));
}

#[test]
fn self_field_mutation_is_visible_through_shared_reference() {
    let result = common::compile_and_run(
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
        c.count
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn multiple_impl_blocks_for_the_same_record_merge() {
    let result = common::compile_and_run(
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
        p.getx() + p.gety()
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(14));
}

#[test]
fn redeclared_method_name_uses_the_later_declaration() {
    let result = common::compile_and_run(
        r#"
        record Point {
            int x,
        }

        impl Point {
            fn label(self) -> int {
                return 1
            }
        }

        impl Point {
            fn label(self) -> int {
                return 2
            }
        }

        dec Point p = Point { x: 0 }
        p.label()
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(2));
}

#[test]
fn instance_method_can_take_extra_params_after_self() {
    let result = common::compile_and_run(
        r#"
        record Point {
            int x,
            int y,
        }

        impl Point {
            fn translated(self, int dx, int dy) -> Point {
                return Point { x: self.x + dx, y: self.y + dy }
            }
        }

        dec Point p = Point { x: 1, y: 1 }
        dec Point moved = p.translated(2, 3)
        moved.x + moved.y
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(7));
}

#[test]
fn calling_undefined_method_on_a_record_is_a_runtime_error() {
    let err = common::compile_and_run(
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
    .expect_err("expected a runtime error for an undefined method");

    assert!(
        err.message().contains("unknown") && err.message().contains("Point"),
        "unexpected error message: {}",
        err.message()
    );
}

#[test]
fn calling_a_method_on_a_non_record_value_is_a_runtime_error() {
    let err = common::compile_and_run(
        r#"
        dec int x = 5
        x.magnitude()
        "#,
    )
    .expect_err("expected a runtime error for a non-record method call");

    assert!(
        err.message().contains("magnitude"),
        "unexpected error message: {}",
        err.message()
    );
}

#[test]
fn calling_undefined_associated_function_is_a_runtime_error() {
    let err = common::compile_and_run(
        r#"
        record Point {
            int x,
        }

        Point::missing()
        "#,
    )
    .expect_err("expected a runtime error for an undefined associated function");

    assert!(
        err.message().contains("Point::missing"),
        "unexpected error message: {}",
        err.message()
    );
}

#[test]
fn calling_an_associated_function_through_instance_syntax_mismatches_arity() {
    let err = common::compile_and_run(
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
    .expect_err("expected an arity-mismatch runtime error");

    assert!(
        err.message().contains("new") && err.message().contains("expects"),
        "unexpected error message: {}",
        err.message()
    );
}
