use rl_ast::statements::TypeAnnotation;
use rl_commons::{stdlib_signatures::params, *};

#[test]
fn resolves_nested_path() {
    let tree = stdlib_names();
    let path = vec!["math".to_string(), "sin".to_string()];
    assert!(tree.resolve(&path).is_some());
}

#[test]
fn resolves_double_nested_path() {
    let tree = stdlib_names();
    let path = vec!["math".to_string(), "consts".to_string(), "PI".to_string()];
    assert!(tree.resolve(&path).is_some());
}

#[test]
fn rejects_unknown_path() {
    let tree = stdlib_names();
    let path = vec!["math".to_string(), "not_a_real_fn".to_string()];
    assert!(tree.resolve(&path).is_none());
}

#[test]
fn pow_has_four_overloads() {
    let tree = stdlib_names();
    let path = vec!["math".to_string(), "pow".to_string()];
    let f = tree.resolve(&path).expect("pow should resolve");
    assert_eq!(f.signatures.len(), 4);
}

#[test]
fn bitwise_bit_and_resolves_with_signatures() {
    let tree = stdlib_names();
    let path = vec!["bitwise".to_string(), "bit_and".to_string()];
    let f = tree.resolve(&path).expect("bit_and should resolve");
    assert_eq!(f.signatures.len(), 4);
}

#[test]
fn io_read_int_resolves_with_signatures() {
    let tree = stdlib_names();
    let path = vec!["io".to_string(), "read_int".to_string()];
    let f = tree.resolve(&path).expect("read_int should resolve");
    assert!(!f.signatures.is_empty());
}

#[test]
fn terminal_set_fg_has_eight_overloads() {
    let tree = stdlib_names();
    let path = vec!["term".to_string(), "term_set_fg".to_string()];
    let f = tree.resolve(&path).expect("term_set_fg should resolve");
    assert_eq!(f.signatures.len(), 8);
}

#[test]
fn terminal_read_key_returns_result_array_string() {
    let tree = stdlib_names();
    let path = vec!["term".to_string(), "term_read_key".to_string()];
    let f = tree.resolve(&path).expect("term_read_key should resolve");
    assert_eq!(f.signatures.len(), 1);
    assert_eq!(
        f.signatures[0].1,
        TypeAnnotation::Result(Box::new(TypeAnnotation::Array(Box::new(
            TypeAnnotation::String
        ))))
    );
}

#[test]
fn terminal_get_size_returns_result_array_int() {
    let tree = stdlib_names();
    let path = vec!["term".to_string(), "term_get_size".to_string()];
    let f = tree.resolve(&path).expect("term_get_size should resolve");
    assert_eq!(f.signatures.len(), 1);
    assert_eq!(
        f.signatures[0].1,
        TypeAnnotation::Result(Box::new(TypeAnnotation::Array(Box::new(
            TypeAnnotation::Int
        ))))
    );
}

#[test]
fn math_sin_returns_plain_float_not_result() {
    let tree = stdlib_names();
    let path = vec!["math".to_string(), "sin".to_string()];
    let f = tree.resolve(&path).expect("sin should resolve");
    assert_eq!(f.signatures.len(), 1);
    assert_eq!(
        f.signatures[0],
        (params(vec![TypeAnnotation::Float]), TypeAnnotation::Float)
    );
}

#[test]
fn math_max_has_no_mixed_overload() {
    let tree = stdlib_names();
    let path = vec!["math".to_string(), "max".to_string()];
    let f = tree.resolve(&path).expect("max should resolve");
    assert_eq!(f.signatures.len(), 2);
}

#[test]
fn math_log_always_returns_float() {
    let tree = stdlib_names();
    let path = vec!["math".to_string(), "log".to_string()];
    let f = tree.resolve(&path).expect("log should resolve");
    assert_eq!(f.signatures.len(), 4);
    assert!(
        f.signatures
            .iter()
            .all(|(_, ret)| *ret == TypeAnnotation::Result(Box::new(TypeAnnotation::Float)))
    );
}

#[test]
fn math_constants_pi_is_zero_arg_float() {
    let tree = stdlib_names();
    let path = vec!["math".to_string(), "consts".to_string(), "PI".to_string()];
    let f = tree.resolve(&path).expect("PI should resolve");
    assert_eq!(f.signatures, vec![(params(vec![]), TypeAnnotation::Float)]);
}

#[test]
fn math_constants_is_nan_takes_float_returns_bool() {
    let tree = stdlib_names();
    let path = vec![
        "math".to_string(),
        "consts".to_string(),
        "is_nan".to_string(),
    ];
    let f = tree.resolve(&path).expect("is_nan should resolve");
    assert_eq!(
        f.signatures,
        vec![(params(vec![TypeAnnotation::Float]), TypeAnnotation::Bool)]
    );
}

#[test]
fn types_to_int_has_six_overloads_no_null() {
    let tree = stdlib_names();
    let path = vec!["types".to_string(), "to_int".to_string()];
    let f = tree.resolve(&path).expect("to_int should resolve");
    assert_eq!(f.signatures.len(), 6);
    assert!(
        f.signatures
            .iter()
            .all(|(p, _)| *p != params(vec![TypeAnnotation::Null]))
    );
}

#[test]
fn types_error_unwrap_stays_untyped() {
    let tree = stdlib_names();
    let path = vec!["types".to_string(), "error_unwrap".to_string()];
    let f = tree.resolve(&path).expect("error_unwrap should resolve");
    assert!(f.signatures.is_empty());
}

#[test]
fn types_to_char_rejects_bool() {
    let tree = stdlib_names();
    let path = vec!["types".to_string(), "to_char".to_string()];
    let f = tree.resolve(&path).expect("to_char should resolve");
    assert!(
        f.signatures
            .iter()
            .all(|(p, _)| *p != params(vec![TypeAnnotation::Bool]))
    );
}

#[test]
fn array_arr_push_takes_array_t_and_t_returns_result_array_t() {
    let tree = stdlib_names();
    let path = vec!["array".to_string(), "arr_push".to_string()];
    let f = tree.resolve(&path).expect("arr_push should resolve");
    let t = TypeAnnotation::Generic("T".to_string());
    let arr_t = TypeAnnotation::Array(Box::new(t.clone()));
    assert_eq!(
        f.signatures,
        vec![(
            params(vec![arr_t.clone(), t]),
            TypeAnnotation::Result(Box::new(arr_t))
        )]
    );
}

#[test]
fn array_arr_max_has_int_and_float_overloads_only() {
    let tree = stdlib_names();
    let path = vec!["array".to_string(), "arr_max".to_string()];
    let f = tree.resolve(&path).expect("arr_max should resolve");
    assert_eq!(f.signatures.len(), 2);
}

#[test]
fn collections_set_len_takes_set_t_returns_result_int() {
    let tree = stdlib_names();
    let path = vec!["collections".to_string(), "set_len".to_string()];
    let f = tree.resolve(&path).expect("set_len should resolve");
    let set_t = TypeAnnotation::Set(Box::new(TypeAnnotation::Generic("T".to_string())));
    assert_eq!(
        f.signatures,
        vec![(
            params(vec![set_t]),
            TypeAnnotation::Result(Box::new(TypeAnnotation::Int))
        )]
    );
}

#[test]
fn random_rand_choice_takes_array_t_returns_result_t() {
    let tree = stdlib_names();
    let path = vec!["random".to_string(), "rand_choice".to_string()];
    let f = tree.resolve(&path).expect("rand_choice should resolve");
    let t = TypeAnnotation::Generic("T".to_string());
    let arr_t = TypeAnnotation::Array(Box::new(t.clone()));
    assert_eq!(
        f.signatures,
        vec![(params(vec![arr_t]), TypeAnnotation::Result(Box::new(t)))]
    );
}

#[test]
fn res_result_unwrap_takes_result_t_returns_t() {
    let tree = stdlib_names();
    let path = vec!["res".to_string(), "result_unwrap".to_string()];
    let f = tree.resolve(&path).expect("result_unwrap should resolve");
    let t = TypeAnnotation::Generic("T".to_string());
    assert_eq!(
        f.signatures,
        vec![(params(vec![TypeAnnotation::Result(Box::new(t.clone()))]), t)]
    );
}

#[test]
fn http_server_recv_has_two_handle_overloads() {
    let tree = stdlib_names();
    let path = vec!["http".to_string(), "http_server_recv".to_string()];
    let f = tree
        .resolve(&path)
        .expect("http_server_recv should resolve");
    assert_eq!(f.signatures.len(), 2);
}

#[test]
fn http_respond_has_four_overloads_across_both_arities() {
    let tree = stdlib_names();
    let path = vec!["http".to_string(), "http_respond".to_string()];
    let f = tree.resolve(&path).expect("http_respond should resolve");
    assert_eq!(f.signatures.len(), 4);
}

#[test]
fn http_request_method_returns_result_string_via_handle_to_string() {
    let tree = stdlib_names();
    let path = vec!["http".to_string(), "http_request_method".to_string()];
    let f = tree
        .resolve(&path)
        .expect("http_request_method should resolve");
    assert_eq!(f.signatures.len(), 2);
    assert!(
        f.signatures
            .iter()
            .all(|(_, ret)| *ret == TypeAnnotation::Result(Box::new(TypeAnnotation::String)))
    );
}

#[test]
fn net_tcp_read_has_four_overloads_from_two_handle_slots() {
    let tree = stdlib_names();
    let path = vec!["net".to_string(), "tcp_read".to_string()];
    let f = tree.resolve(&path).expect("tcp_read should resolve");
    assert_eq!(f.signatures.len(), 4);
}

#[test]
fn net_tcp_peer_addr_and_local_addr_share_shape_with_http() {
    let tree = stdlib_names();
    for name in ["tcp_peer_addr", "tcp_local_addr"] {
        let path = vec!["net".to_string(), name.to_string()];
        let f = tree
            .resolve(&path)
            .unwrap_or_else(|| panic!("{name} should resolve"));
        assert_eq!(f.signatures.len(), 2);
        assert!(
            f.signatures
                .iter()
                .all(|(_, ret)| *ret == TypeAnnotation::Result(Box::new(TypeAnnotation::String)))
        );
    }
}

#[test]
fn path_extension_and_str_to_lower_share_the_string_to_string_shape() {
    let tree = stdlib_names();
    let expected = vec![(params(vec![TypeAnnotation::String]), TypeAnnotation::String)];

    let path_fn = tree
        .resolve(&["path".to_string(), "path_extension".to_string()])
        .expect("path_extension should resolve");
    assert_eq!(path_fn.signatures, expected);

    let str_fn = tree
        .resolve(&["str".to_string(), "to_lower".to_string()])
        .expect("to_lower should resolve");
    assert_eq!(str_fn.signatures, expected);
}
