use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn json_round_trip_types() {
    let result = compile_and_run(
        r#"
get json_parse, json_stringify from std::serialize
get result_unwrap from std::res
dec data = result_unwrap(json_parse("{\"s\": \"x\", \"i\": 12, \"f\": 12.5, \"b\": true, \"n\": null, \"a\": [1, 2]}"))
json_stringify(data)
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str("{\"a\":[1,2],\"b\":true,\"f\":12.5,\"i\":12,\"n\":null,\"s\":\"x\"}".into())
    );
}

#[test]
fn json_int_float_split() {
    let result = compile_and_run(
        r#"
get json_parse from std::serialize
get result_unwrap, is_err from std::res
get map_get from std::collections
dec data = result_unwrap(json_parse("{\"i\": 12, \"f\": 1e3}"))
dec int i = map_get(data, "i").result_unwrap()
dec float f = map_get(data, "f").result_unwrap()
i
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(12));
}

#[test]
fn json_parse_rejects_garbage() {
    let result = compile_and_run(
        r#"
get json_parse from std::serialize
get is_err from std::res
dec bool x = is_err(json_parse("{oops"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn json_get_paths() {
    let result = compile_and_run(
        r#"
get json_parse, json_get from std::serialize
get result_unwrap from std::res
dec data = result_unwrap(json_parse("{\"server\": {\"hosts\": [\"a\", \"b\"]}}"))
dec string h = result_unwrap(json_get(data, "server.hosts.1"))
h
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str("b".into()));
}

#[test]
fn json_get_missing_is_err() {
    let result = compile_and_run(
        r#"
get json_parse, json_get from std::serialize
get result_unwrap, is_err from std::res
dec data = result_unwrap(json_parse("{\"a\": 1}"))
dec bool x = is_err(json_get(data, "a.b.c"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn json_is_valid_both_ways() {
    let result = compile_and_run(
        r#"
get json_is_valid from std::serialize
dec bool a = json_is_valid("[1, 2]")
dec bool b = json_is_valid("[1,")
a
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    let result = compile_and_run(
        r#"
get json_is_valid from std::serialize
json_is_valid("[1,")
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn csv_round_trip() {
    let result = compile_and_run(
        r#"
get csv_parse, csv_stringify from std::serialize
get result_unwrap from std::res
get len from std
dec rows = result_unwrap(csv_parse("a,b\n1,2\n\"x,y\",3"))
result_unwrap(len(rows))
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn csv_headers_key_rows() {
    let result = compile_and_run(
        r#"
get csv_parse_headers from std::serialize
get result_unwrap from std::res
get map_get from std::collections
dec rows = result_unwrap(csv_parse_headers("name,age\nbob,30"))
dec row = rows[0]
dec string n = map_get(row, "name").result_unwrap()
n
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str("bob".into()));
}

#[test]
fn csv_delimiter_and_stringify() {
    let result = compile_and_run(
        r#"
get csv_parse_with_delimiter, csv_stringify from std::serialize
get result_unwrap from std::res
dec rows = result_unwrap(csv_parse_with_delimiter("a;b\n1;2", ";"))
csv_stringify(rows)
"#,
    )
    .unwrap();
    // stringify always emits comma-separated CSV.
    assert_eq!(result, VmValue::Str("a,b\n1,2\n".into()));
}

#[test]
fn toml_round_trip() {
    let result = compile_and_run(
        r#"
get toml_parse, toml_stringify from std::serialize
get result_unwrap from std::res
get map_get from std::collections
dec cfg = result_unwrap(toml_parse("[server]\nhost = \"x\"\nport = 80"))
dec server = map_get(cfg, "server").result_unwrap()
dec string host = map_get(server, "host").result_unwrap()
host
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str("x".into()));
}

#[test]
fn toml_stringify_needs_map() {
    let result = compile_and_run(
        r#"
get toml_stringify from std::serialize
get is_err from std::res
dec bool x = is_err(toml_stringify([1, 2]))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn ini_round_trip() {
    let result = compile_and_run(
        r#"
get ini_parse, ini_stringify from std::serialize
get result_unwrap, is_err from std::res
get map_get from std::collections
dec cfg = result_unwrap(ini_parse("[server]\nhost = x"))
dec server = map_get(cfg, "server").result_unwrap()
dec string host = map_get(server, "host").result_unwrap()
dec string back = result_unwrap(ini_stringify(cfg))
host
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str("x".into()));
}

#[test]
fn yaml_round_trip() {
    let result = compile_and_run(
        r#"
get yaml_parse, yaml_stringify from std::serialize
get result_unwrap, is_err from std::res
get map_get from std::collections
dec cfg = result_unwrap(yaml_parse("host: x\nport: 80\nflag: true"))
dec int port = map_get(cfg, "port").result_unwrap()
dec string back = result_unwrap(yaml_stringify(cfg))
port
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(80));
}

#[test]
fn yaml_rejects_garbage() {
    let result = compile_and_run(
        r#"
get yaml_parse from std::serialize
get is_err from std::res
dec bool x = is_err(yaml_parse("key: [unclosed"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}
