use crate::entry::{FnEntry, StdEntry};

mod csv_parse;
mod csv_parse_headers;
mod csv_parse_with_delimiter;
mod csv_stringify;
mod ini_parse;
mod ini_stringify;
mod json_get;
mod json_is_valid;
mod json_parse;
mod json_stringify;
mod json_stringify_pretty;
mod toml_parse;
mod toml_stringify;
mod yaml_parse;
mod yaml_stringify;

pub static SERIALIZE: StdEntry = StdEntry {
    name: "serialize",
    description: "functions for JSON, CSV, TOML, INI and YAML text interop",
    functions: FUNCTIONS,
    since: Some("v2.2.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &json_parse::JSON_PARSE,
    &json_stringify::JSON_STRINGIFY,
    &json_stringify_pretty::JSON_STRINGIFY_PRETTY,
    &json_is_valid::JSON_IS_VALID,
    &json_get::JSON_GET,
    &csv_parse::CSV_PARSE,
    &csv_parse_with_delimiter::CSV_PARSE_WITH_DELIMITER,
    &csv_stringify::CSV_STRINGIFY,
    &csv_parse_headers::CSV_PARSE_HEADERS,
    &toml_parse::TOML_PARSE,
    &toml_stringify::TOML_STRINGIFY,
    &ini_parse::INI_PARSE,
    &ini_stringify::INI_STRINGIFY,
    &yaml_parse::YAML_PARSE,
    &yaml_stringify::YAML_STRINGIFY,
];
