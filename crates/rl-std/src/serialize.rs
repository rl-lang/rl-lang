//! `std::serialize` - JSON/CSV/TOML/INI/YAML text interop.
//!
//! Parse functions return `result[T]` with line/column errors; values map
//! onto RL scalars, arrays and string-keyed maps. JSON numbers without a
//! fraction or exponent become `int`, the rest `float`; JSON/YAML `null`
//! becomes RL `null`. Stringifiers accept the same shapes back; maps with
//! non-string keys and non-scalar leaves (closures, handles) are errors.
//! TOML datetimes stringify as their RFC text. `null` has no TOML form
//! and is an error there.

#[cfg(feature = "impls")]
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use rl_ast::statements::TypeAnnotation;

// ---- shared value conversions ---------------------------------------------

#[cfg(feature = "impls")]
fn json_to_rl<R: Runtime>(v: &serde_json::Value) -> R::Value {
    match v {
        serde_json::Value::Null => R::null(),
        serde_json::Value::Bool(b) => R::from_bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                R::from_i64(i)
            } else if let Some(u) = n.as_u64() {
                // Beyond i64 range: closest representable is float.
                R::from_f64(u as f64)
            } else {
                R::from_f64(n.as_f64().unwrap_or(f64::NAN))
            }
        }
        serde_json::Value::String(s) => R::from_string(s.clone()),
        serde_json::Value::Array(items) => R::array(
            items.iter().map(json_to_rl::<R>).collect(),
            TypeAnnotation::Infer,
        ),
        serde_json::Value::Object(map) => R::map(
            map.iter()
                .map(|(k, v)| (R::from_string(k.clone()), json_to_rl::<R>(v)))
                .collect(),
            TypeAnnotation::String,
            TypeAnnotation::Infer,
        ),
    }
}

#[cfg(feature = "impls")]
fn rl_to_json<R: Runtime>(v: &R::Value) -> Result<serde_json::Value, String> {
    if R::type_name(v) == "null" {
        return Ok(serde_json::Value::Null);
    }
    if let Some(b) = R::as_bool(v) {
        return Ok(serde_json::Value::Bool(b));
    }
    if let Some(s) = R::as_str(v) {
        return Ok(serde_json::Value::String(s.to_owned()));
    }
    if let Some(i) = R::as_i64(v) {
        return Ok(serde_json::Value::Number(i.into()));
    }
    if let Some(b) = R::as_u8(v) {
        return Ok(serde_json::Value::Number(b.into()));
    }
    if let Some(f) = R::as_f64(v) {
        return Ok(serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null));
    }
    if let Some((items, _)) = R::as_array(v) {
        return items
            .iter()
            .map(rl_to_json::<R>)
            .collect::<Result<Vec<_>, _>>()
            .map(serde_json::Value::Array);
    }
    if let Some((entries, _, _)) = R::as_map(v) {
        let mut map = serde_json::Map::with_capacity(entries.len());
        for (k, val) in &entries {
            let Some(key) = R::as_str(k) else {
                return Err("json only supports string keys".to_string());
            };
            map.insert(key.to_owned(), rl_to_json::<R>(val)?);
        }
        return Ok(serde_json::Value::Object(map));
    }
    Err(format!(
        "value of type {} is not JSON-representable",
        R::type_name(v)
    ))
}

#[cfg(feature = "impls")]
fn yaml_to_rl<R: Runtime>(v: &serde_yaml::Value) -> Result<R::Value, String> {
    match v {
        serde_yaml::Value::Null => Ok(R::null()),
        serde_yaml::Value::Bool(b) => Ok(R::from_bool(*b)),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(R::from_i64(i))
            } else if let Some(u) = n.as_u64() {
                Ok(R::from_f64(u as f64))
            } else {
                Ok(R::from_f64(n.as_f64().unwrap_or(f64::NAN)))
            }
        }
        serde_yaml::Value::String(s) => Ok(R::from_string(s.clone())),
        serde_yaml::Value::Sequence(items) => Ok(R::array(
            items
                .iter()
                .map(yaml_to_rl::<R>)
                .collect::<Result<Vec<_>, _>>()?,
            TypeAnnotation::Infer,
        )),
        serde_yaml::Value::Mapping(map) => {
            let mut entries = Vec::with_capacity(map.len());
            for (k, val) in map {
                let serde_yaml::Value::String(key) = k else {
                    return Err("yaml mappings need string keys".to_string());
                };
                entries.push((R::from_string(key.clone()), yaml_to_rl::<R>(val)?));
            }
            Ok(R::map(
                entries,
                TypeAnnotation::String,
                TypeAnnotation::Infer,
            ))
        }
        serde_yaml::Value::Tagged(tagged) => yaml_to_rl::<R>(&tagged.value),
    }
}

#[cfg(feature = "impls")]
fn toml_to_rl<R: Runtime>(v: &toml::Value) -> R::Value {
    match v {
        toml::Value::String(s) => R::from_string(s.clone()),
        toml::Value::Integer(i) => R::from_i64(*i),
        toml::Value::Float(f) => R::from_f64(*f),
        toml::Value::Boolean(b) => R::from_bool(*b),
        toml::Value::Datetime(dt) => R::from_string(dt.to_string()),
        toml::Value::Array(items) => R::array(
            items.iter().map(toml_to_rl::<R>).collect(),
            TypeAnnotation::Infer,
        ),
        toml::Value::Table(map) => R::map(
            map.iter()
                .map(|(k, val)| (R::from_string(k.clone()), toml_to_rl::<R>(val)))
                .collect(),
            TypeAnnotation::String,
            TypeAnnotation::Infer,
        ),
    }
}

#[cfg(feature = "impls")]
fn rl_to_toml<R: Runtime>(v: &R::Value) -> Result<toml::Value, String> {
    if R::type_name(v) == "null" {
        return Err("toml has no null value".to_string());
    }
    if let Some(b) = R::as_bool(v) {
        return Ok(toml::Value::Boolean(b));
    }
    if let Some(s) = R::as_str(v) {
        return Ok(toml::Value::String(s.to_owned()));
    }
    if let Some(i) = R::as_i64(v) {
        return Ok(toml::Value::Integer(i));
    }
    if let Some(b) = R::as_u8(v) {
        return Ok(toml::Value::Integer(b as i64));
    }
    if let Some(f) = R::as_f64(v) {
        return Ok(toml::Value::Float(f));
    }
    if let Some((items, _)) = R::as_array(v) {
        return items
            .iter()
            .map(rl_to_toml::<R>)
            .collect::<Result<Vec<_>, _>>()
            .map(toml::Value::Array);
    }
    if let Some((entries, _, _)) = R::as_map(v) {
        let mut map = toml::map::Map::with_capacity(entries.len());
        for (k, val) in &entries {
            let Some(key) = R::as_str(k) else {
                return Err("toml only supports string keys".to_string());
            };
            map.insert(key.to_owned(), rl_to_toml::<R>(val)?);
        }
        return Ok(toml::Value::Table(map));
    }
    Err(format!(
        "value of type {} is not TOML-representable",
        R::type_name(v)
    ))
}

// ---- json -------------------------------------------------------------------

#[native_fn(module = "serialize", sig(string -> result[T]))]
pub fn json_parse<R: Runtime>(s: String) -> R::Value {
    match serde_json::from_str::<serde_json::Value>(&s) {
        Ok(v) => R::ok(json_to_rl::<R>(&v)),
        Err(e) => R::err(R::from_string(format!("json_parse: {e}"))),
    }
}

#[native_fn(module = "serialize", sig(T -> string))]
pub fn json_stringify<R: Runtime>(v: R::Value) -> String {
    // The checker guarantees representable shapes; fall back to null
    // rather than failing the whole call on exotic leaves.
    rl_to_json::<R>(&v)
        .map(|j| serde_json::to_string(&j).unwrap_or_else(|_| "null".to_string()))
        .unwrap_or_else(|_| "null".to_string())
}

#[native_fn(module = "serialize", sig(T -> string))]
pub fn json_stringify_pretty<R: Runtime>(v: R::Value) -> String {
    rl_to_json::<R>(&v)
        .map(|j| serde_json::to_string_pretty(&j).unwrap_or_else(|_| "null".to_string()))
        .unwrap_or_else(|_| "null".to_string())
}

#[native_fn(module = "serialize")]
pub fn json_is_valid(s: String) -> bool {
    serde_json::from_str::<serde_json::Value>(&s).is_ok()
}

#[native_fn(module = "serialize", sig(T, string -> result[T]))]
pub fn json_get<R: Runtime>(v: R::Value, path: String) -> R::Value {
    let mut cur = v;
    for seg in path.split('.') {
        if seg.is_empty() {
            return R::err(R::from_string("json_get: empty path segment".to_string()));
        }
        if let Some((items, _)) = R::as_array(&cur) {
            match seg.parse::<usize>() {
                Ok(i) => match items.get(i) {
                    Some(next) => cur = next.clone(),
                    None => {
                        return R::err(R::from_string(format!(
                            "json_get: index {i} out of bounds"
                        )));
                    }
                },
                Err(_) => {
                    return R::err(R::from_string(format!(
                        "json_get: cannot index array with \"{seg}\""
                    )));
                }
            }
            continue;
        }
        if let Some((entries, _, _)) = R::as_map(&cur) {
            let key = R::from_string(seg.to_string());
            match entries
                .iter()
                .find(|(k, _)| R::keys_equal(k, &key))
                .map(|(_, val)| val.clone())
            {
                Some(next) => cur = next,
                None => {
                    return R::err(R::from_string(format!(
                        "json_get: no such key \"{seg}\""
                    )));
                }
            }
            continue;
        }
        return R::err(R::from_string(format!(
            "json_get: cannot index into {}",
            R::type_name(&cur)
        )));
    }
    R::ok(cur)
}

// ---- csv ----------------------------------------------------------------------

#[cfg(feature = "impls")]
fn csv_rows(s: &str, delim: u8) -> Result<Vec<Vec<String>>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(delim)
        .from_reader(s.as_bytes());
    let mut rows = Vec::new();
    for record in reader.records() {
        match record {
            Ok(rec) => rows.push(rec.iter().map(str::to_owned).collect()),
            Err(e) => return Err(format!("csv_parse: {e}")),
        }
    }
    Ok(rows)
}

#[native_fn(module = "serialize", sig(string -> result[array[array[string]]]))]
pub fn csv_parse<R: Runtime>(s: String) -> R::Value {
    match csv_rows(&s, b',') {
        Ok(rows) => R::ok(R::array(
            rows.into_iter()
                .map(|r| {
                    R::array(
                        r.into_iter().map(R::from_string).collect(),
                        TypeAnnotation::String,
                    )
                })
                .collect(),
            TypeAnnotation::Array(Box::new(TypeAnnotation::String)),
        )),
        Err(e) => R::err(R::from_string(e)),
    }
}

#[native_fn(module = "serialize", sig(string, string -> result[array[array[string]]]))]
pub fn csv_parse_with_delimiter<R: Runtime>(s: String, delim: String) -> R::Value {
    let Some(d) = delim.as_bytes().first() else {
        return R::err(R::from_string(
            "csv_parse_with_delimiter: delimiter must not be empty".to_string(),
        ));
    };
    match csv_rows(&s, *d) {
        Ok(rows) => R::ok(R::array(
            rows.into_iter()
                .map(|r| {
                    R::array(
                        r.into_iter().map(R::from_string).collect(),
                        TypeAnnotation::String,
                    )
                })
                .collect(),
            TypeAnnotation::Array(Box::new(TypeAnnotation::String)),
        )),
        Err(e) => R::err(R::from_string(e)),
    }
}

#[native_fn(module = "serialize", sig(array[array[string]] -> string))]
pub fn csv_stringify<R: Runtime>(rows: Vec<Vec<String>>) -> String {
    let mut writer = csv::WriterBuilder::new().from_writer(Vec::new());
    for row in &rows {
        if writer.write_record(row).is_err() {
            return String::new();
        }
    }
    String::from_utf8(writer.into_inner().unwrap_or_default()).unwrap_or_default()
}

#[native_fn(module = "serialize", sig(string -> result[array[map[string, string]]]))]
pub fn csv_parse_headers<R: Runtime>(s: String) -> R::Value {
    match csv_rows(&s, b',') {
        Ok(rows) => {
            let mut iter = rows.into_iter();
            let Some(headers) = iter.next() else {
                return R::ok(R::array(
                    Vec::new(),
                    TypeAnnotation::Map(
                        Box::new(TypeAnnotation::String),
                        Box::new(TypeAnnotation::String),
                    ),
                ));
            };
            let maps: Vec<R::Value> = iter
                .map(|row| {
                    R::map(
                        headers
                            .iter()
                            .zip(row.iter())
                            .map(|(k, val)| {
                                (R::from_string(k.clone()), R::from_string(val.clone()))
                            })
                            .collect(),
                        TypeAnnotation::String,
                        TypeAnnotation::String,
                    )
                })
                .collect();
            R::ok(R::array(
                maps,
                TypeAnnotation::Map(
                    Box::new(TypeAnnotation::String),
                    Box::new(TypeAnnotation::String),
                ),
            ))
        }
        Err(e) => R::err(R::from_string(e)),
    }
}

// ---- toml ---------------------------------------------------------------------

#[native_fn(module = "serialize", sig(string -> result[T]))]
pub fn toml_parse<R: Runtime>(s: String) -> R::Value {
    match s.parse::<toml::Value>() {
        Ok(v) => R::ok(toml_to_rl::<R>(&v)),
        Err(e) => R::err(R::from_string(format!("toml_parse: {e}"))),
    }
}

#[native_fn(module = "serialize", sig(T -> result[string]))]
pub fn toml_stringify<R: Runtime>(v: R::Value) -> R::Value {
    match rl_to_toml::<R>(&v) {
        Ok(toml::Value::Table(_)) => {}
        Ok(_) => {
            return R::err(R::from_string(
                "toml_stringify: top level must be a map".to_string(),
            ));
        }
        Err(e) => return R::err(R::from_string(format!("toml_stringify: {e}"))),
    }
    let value = rl_to_toml::<R>(&v).expect("already a table");
    match toml::to_string(&value) {
        Ok(s) => R::ok(R::from_string(s)),
        Err(e) => R::err(R::from_string(format!("toml_stringify: {e}"))),
    }
}

// ---- ini ------------------------------------------------------------------------

#[native_fn(module = "serialize", sig(string -> result[map[string, map[string, string]]]))]
pub fn ini_parse<R: Runtime>(s: String) -> R::Value {
    match ini::Ini::load_from_str(&s) {
        Ok(ini) => {
            // Every section (None = the general unsectioned keys, stored
            // under "") becomes a nested string map.
            let mut sections = Vec::new();
            for section in ini.sections() {
                let Some(props) = ini.section(section) else {
                    continue;
                };
                let mut entries = Vec::new();
                for (k, val) in props.iter() {
                    entries.push((R::from_string(k.to_owned()), R::from_string(val.to_owned())));
                }
                sections.push((
                    R::from_string(section.unwrap_or_default().to_owned()),
                    R::map(entries, TypeAnnotation::String, TypeAnnotation::String),
                ));
            }
            R::ok(R::map(
                sections,
                TypeAnnotation::String,
                TypeAnnotation::Map(
                    Box::new(TypeAnnotation::String),
                    Box::new(TypeAnnotation::String),
                ),
            ))
        }
        Err(e) => R::err(R::from_string(format!("ini_parse: {e}"))),
    }
}

#[native_fn(module = "serialize", sig(map[string, map[string, string]] -> result[string]))]
pub fn ini_stringify<R: Runtime>(v: R::Value) -> R::Value {
    let Some((sections, _, _)) = R::as_map(&v) else {
        return R::err(R::from_string(
            "ini_stringify: expects a map of sections".to_string(),
        ));
    };
    let mut ini = ini::Ini::new();
    for (section, props) in &sections {
        let Some(name) = R::as_str(section) else {
            return R::err(R::from_string(
                "ini_stringify: section names must be strings".to_string(),
            ));
        };
        let Some((entries, _, _)) = R::as_map(props) else {
            return R::err(R::from_string(
                "ini_stringify: sections must be maps".to_string(),
            ));
        };
        let section_key: Option<String> =
            if name.is_empty() { None } else { Some(name.to_owned()) };
        for (k, val) in &entries {
            let (Some(key), Some(value)) = (R::as_str(k), R::as_str(val)) else {
                return R::err(R::from_string(
                    "ini_stringify: keys and values must be strings".to_string(),
                ));
            };
            ini.with_section(section_key.clone())
                .set(key.to_owned(), value.to_owned());
        }
    }
    let mut out = Vec::new();
    if ini.write_to(&mut out).is_err() {
        return R::err(R::from_string("ini_stringify: write failed".to_string()));
    }
    match String::from_utf8(out) {
        Ok(s) => R::ok(R::from_string(s)),
        Err(_) => R::err(R::from_string(
            "ini_stringify: non-utf8 output".to_string(),
        )),
    }
}

// ---- yaml -----------------------------------------------------------------------

#[native_fn(module = "serialize", sig(string -> result[T]))]
pub fn yaml_parse<R: Runtime>(s: String) -> R::Value {
    match serde_yaml::from_str::<serde_yaml::Value>(&s) {
        Ok(v) => match yaml_to_rl::<R>(&v) {
            Ok(rl) => R::ok(rl),
            Err(e) => R::err(R::from_string(format!("yaml_parse: {e}"))),
        },
        Err(e) => R::err(R::from_string(format!("yaml_parse: {e}"))),
    }
}

#[native_fn(module = "serialize", sig(T -> result[string]))]
pub fn yaml_stringify<R: Runtime>(v: R::Value) -> R::Value {
    // Reuse the JSON shape (superset-compatible scalars); YAML emits it
    // in block style. Tagged values never arise from RL data.
    match rl_to_json::<R>(&v) {
        Ok(json) => {
            let yaml: serde_yaml::Value = match json {
                serde_json::Value::Null => serde_yaml::Value::Null,
                serde_json::Value::Bool(b) => serde_yaml::Value::Bool(b),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        serde_yaml::Value::Number(i.into())
                    } else if let Some(f) = n.as_f64() {
                        serde_yaml::Value::Number(f.into())
                    } else {
                        serde_yaml::Value::Null
                    }
                }
                serde_json::Value::String(s) => serde_yaml::Value::String(s),
                serde_json::Value::Array(items) => serde_yaml::Value::Sequence(
                    items
                        .into_iter()
                        .map(yaml_from_json)
                        .collect(),
                ),
                serde_json::Value::Object(map) => serde_yaml::Value::Mapping(
                    map.into_iter()
                        .map(|(k, val)| {
                            (
                                serde_yaml::Value::String(k),
                                yaml_from_json(val),
                            )
                        })
                        .collect(),
                ),
            };
            match serde_yaml::to_string(&yaml) {
                Ok(s) => R::ok(R::from_string(s)),
                Err(e) => R::err(R::from_string(format!("yaml_stringify: {e}"))),
            }
        }
        Err(e) => R::err(R::from_string(format!("yaml_stringify: {e}"))),
    }
}

#[cfg(feature = "impls")]
fn yaml_from_json(v: serde_json::Value) -> serde_yaml::Value {
    match v {
        serde_json::Value::Null => serde_yaml::Value::Null,
        serde_json::Value::Bool(b) => serde_yaml::Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_yaml::Value::Number(i.into())
            } else if let Some(f) = n.as_f64() {
                serde_yaml::Value::Number(f.into())
            } else {
                serde_yaml::Value::Null
            }
        }
        serde_json::Value::String(s) => serde_yaml::Value::String(s),
        serde_json::Value::Array(items) => {
            serde_yaml::Value::Sequence(items.into_iter().map(yaml_from_json).collect())
        }
        serde_json::Value::Object(map) => serde_yaml::Value::Mapping(
            map.into_iter()
                .map(|(k, val)| (serde_yaml::Value::String(k), yaml_from_json(val)))
                .collect(),
        ),
    }
}

// ---- module registration --------------------------------------------------

rl_std_core::native_module!("serialize";
    funcs: [
        json_parse, json_stringify, json_stringify_pretty, json_is_valid, json_get,
        csv_parse, csv_parse_with_delimiter, csv_stringify, csv_parse_headers,
        toml_parse, toml_stringify,
        ini_parse, ini_stringify,
        yaml_parse, yaml_stringify,
    ],
);
