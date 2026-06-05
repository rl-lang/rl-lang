use rl_lang::ast::statements::Statement;

use crate::common;

#[test]
fn import_single_tiem_single_path() {
    let statements = common::parse("get x from y");
    assert_eq!(
        statements[0],
        Statement::Import {
            names: vec!["x".to_string()],
            path: vec!["y".to_string()]
        }
    );
}

#[test]
fn import_single_item_multi_path() {
    let statements = common::parse("get x from y::z");
    assert_eq!(
        statements[0],
        Statement::Import {
            names: vec!["x".to_string()],
            path: vec!["y".to_string(), "z".to_string()]
        }
    );
}

#[test]
fn import_multi_item_single_path() {
    let statements = common::parse("get x , z from y");
    assert_eq!(
        statements[0],
        Statement::Import {
            names: vec!["x".to_string(), "z".to_string()],
            path: vec!["y".to_string()]
        }
    );
}

#[test]
fn import_multi_item_multi_path() {
    let statements = common::parse("get x, z from y::w");
    assert_eq!(
        statements[0],
        Statement::Import {
            names: vec!["x".to_string(), "z".to_string()],
            path: vec!["y".to_string(), "w".to_string()]
        }
    );
}
