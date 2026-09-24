//! `std::str` - string manipulation.
//!
//! The rl module name is `str`; the Rust module is `string` to avoid clashing
//! with the `str` primitive. Ported once from the former per-runtime
//! `stdlib/string/*.rs` copies.

use rl_ast::statements::TypeAnnotation;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use rl_utils::errors::Error;

// ---- simple `string -> string` transforms ---------------------------------

#[native_fn(module = "str")]
pub fn to_upper(string: String) -> String {
    string.to_uppercase()
}

#[native_fn(module = "str")]
pub fn to_lower(string: String) -> String {
    string.to_lowercase()
}

#[native_fn(module = "str")]
pub fn trim(string: String) -> String {
    string.trim().to_string()
}

#[native_fn(module = "str")]
pub fn trim_end(string: String) -> String {
    string.trim_end().to_string()
}

#[native_fn(module = "str")]
pub fn trim_start(string: String) -> String {
    string.trim_start().to_string()
}

#[native_fn(module = "str")]
pub fn reverse(string: String) -> String {
    string.chars().rev().collect()
}

// ---- scalar queries / transforms ------------------------------------------

#[native_fn(module = "str")]
pub fn repeat(string: String, count: i64) -> String {
    string.repeat(count as usize)
}

#[native_fn(module = "str")]
pub fn is_empty(string: String) -> bool {
    string.is_empty()
}

#[native_fn(module = "str")]
pub fn contains(string: String, sub: String) -> bool {
    string.contains(&sub)
}

#[native_fn(module = "str")]
pub fn starts_with(string: String, sub: String) -> bool {
    string.starts_with(&sub)
}

#[native_fn(module = "str")]
pub fn ends_with(string: String, sub: String) -> bool {
    string.ends_with(&sub)
}

#[native_fn(module = "str")]
pub fn replace(string: String, from: String, to: String) -> String {
    string.replace(&from, &to)
}

#[native_fn(module = "str")]
pub fn pad_left(string: String, width: i64, character: char) -> String {
    let pad = (width as usize).saturating_sub(string.chars().count());
    format!("{}{}", character.to_string().repeat(pad), string)
}

#[native_fn(module = "str")]
pub fn pad_right(string: String, width: i64, character: char) -> String {
    let pad = (width as usize).saturating_sub(string.chars().count());
    format!("{}{}", string, character.to_string().repeat(pad))
}

#[native_fn(module = "str")]
pub fn count(string: String, to_count: String) -> i64 {
    string.matches(&to_count).count() as i64
}

#[native_fn(module = "str")]
pub fn index_of(string: String, sub: String) -> i64 {
    match string.find(&sub) {
        Some(i) => string[..i].chars().count() as i64,
        None => -1_i64,
    }
}

// ---- array-producing ------------------------------------------------------

#[native_fn(module = "str")]
pub fn bytes(string: String) -> Vec<u8> {
    string.bytes().collect()
}

#[native_fn(module = "str")]
pub fn chars(string: String) -> Vec<char> {
    string.chars().collect()
}

#[native_fn(module = "str")]
pub fn split(string: String, delim: String) -> Vec<String> {
    string.split(&delim).map(|s| s.to_string()).collect()
}

// ---- fallible (language `result[T]`) --------------------------------------

#[native_fn(module = "str")]
pub fn char_at(string: String, index: i64) -> Result<char, String> {
    if index < 0 {
        return Err(format!("index cannot be negative: {index}"));
    }
    let mut chars = string.chars();
    let chars_count = chars.clone().count();
    if index as usize >= chars_count {
        Err(format!(
            "index out of bounds string length is {chars_count} , used {index}"
        ))
    } else {
        Ok(chars.nth(index as usize).unwrap())
    }
}

#[native_fn(module = "str")]
pub fn slice(string: String, start: i64, end: i64) -> Result<String, String> {
    let chars = string.chars();
    let chars_count = chars.clone().count();
    if start as usize >= chars_count || end as usize > chars_count {
        return Err(format!(
            "index out of bounds string legth: {chars_count}, found start: {start} and end: {end}"
        ));
    }
    Ok(chars
        .skip(start as usize)
        .take(end as usize - start as usize)
        .collect::<String>())
}

// `array[_]` first arg: any element type, stringified. Explicit signature
// because the argument is a raw runtime value.
#[native_fn(module = "str", sig(array[_], string -> result[string]))]
pub fn join<R: Runtime>(_cx: &mut R::Cx, array: R::Value, delim: String) -> Result<String, String> {
    let Some((items, _)) = R::as_array(&array) else {
        return Err("join() expects an array as first argument".to_string());
    };
    let mut parts: Vec<String> = Vec::with_capacity(items.len());
    for v in items {
        if R::is_callable(v) {
            return Err("functions/lambdas/enclosures are not supported via join()".to_string());
        }
        parts.push(R::display(v));
    }
    Ok(parts.join(&delim))
}

// ---- variadic -------------------------------------------------------------

#[native_fn(module = "str", untyped)]
pub fn concat<R: Runtime>(args: Vec<R::Value>) -> String {
    args.iter().map(|v| R::display(v)).collect()
}

#[native_fn(module = "str", untyped)]
pub fn format<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<String, Error> {
    if args.is_empty() {
        return Err(R::error(cx, "expected arguments", span));
    }

    let args: Vec<String> = args.iter().map(|v| R::display(v)).collect();
    let text = &args[0];
    let mut rest_args = args[1..].iter();
    let mut chars = text.chars().peekable();
    let mut result = String::new();
    let mut used = 0;
    let mut missing = 0;

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'}') {
            chars.next();
            match rest_args.next() {
                Some(v) => {
                    result.push_str(v);
                    used += 1;
                }
                None => {
                    result.push_str("{}");
                    missing += 1;
                }
            }
        } else {
            result.push(c);
        }
    }

    if missing > 0 {
        return Err(R::error(
            cx,
            format!("format() has {missing} placeholder(s) with no matching argument"),
            span,
        ));
    }
    if used < args.len() - 1 {
        return Err(R::error(
            cx,
            format!(
                "format() received {} argument(s) but only {} placeholder(s) were used",
                args.len() - 1,
                used
            ),
            span,
        ));
    }

    Ok(result)
}

// ---- prefix / suffix stripping --------------------------------------------

#[native_fn(module = "str")]
pub fn strip_prefix(string: String, prefix: String) -> Result<String, String> {
    match string.strip_prefix(&prefix) {
        Some(rest) => Ok(rest.to_string()),
        None => Err(format!(
            "strip_prefix: string does not start with \"{}\"",
            prefix
        )),
    }
}

#[native_fn(module = "str")]
pub fn strip_suffix(string: String, suffix: String) -> Result<String, String> {
    match string.strip_suffix(&suffix) {
        Some(rest) => Ok(rest.to_string()),
        None => Err(format!(
            "strip_suffix: string does not end with \"{}\"",
            suffix
        )),
    }
}

// ---- search from end ------------------------------------------------------

#[native_fn(module = "str")]
pub fn last_index_of(string: String, needle: String) -> i64 {
    match string.rfind(&needle) {
        Some(i) => string[..i].chars().count() as i64,
        None => -1_i64,
    }
}

// ---- split variants -------------------------------------------------------

#[native_fn(module = "str")]
pub fn split_once(string: String, sep: String) -> Result<Vec<String>, String> {
    match string.split_once(&sep) {
        Some((before, after)) => Ok(vec![before.to_string(), after.to_string()]),
        None => Err(format!(
            "split_once: separator \"{}\" not found in string",
            sep
        )),
    }
}

#[native_fn(module = "str")]
pub fn lines(string: String) -> Vec<String> {
    string.lines().map(String::from).collect()
}

// ---- wrap / indent / dedent -----------------------------------------------

#[native_fn(module = "str")]
pub fn wrap(string: String, width: i64) -> String {
    if width <= 0 {
        return string;
    }
    let width = width as usize;
    let mut result = String::new();
    let mut line_len = 0;

    for word in string.split_whitespace() {
        if line_len == 0 {
            result.push_str(word);
            line_len = word.len();
        } else if line_len + 1 + word.len() <= width {
            result.push(' ');
            result.push_str(word);
            line_len += 1 + word.len();
        } else {
            result.push('\n');
            result.push_str(word);
            line_len = word.len();
        }
    }
    result
}

#[native_fn(module = "str")]
pub fn indent(string: String, prefix: String) -> String {
    let lines: Vec<String> = string.lines().map(|l| format!("{}{}", prefix, l)).collect();
    lines.join("\n")
}

#[native_fn(module = "str")]
pub fn dedent(string: String) -> String {
    let prefix_len = string
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    string
        .lines()
        .map(|l| {
            if l.is_empty() {
                String::new()
            } else {
                l[prefix_len..].to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---- line-level diff ------------------------------------------------------

#[native_fn(module = "str", sig(string, string -> result[array[tuple[string, int]]]))]
pub fn diff_lines<R: Runtime>(_cx: &mut R::Cx, a: String, b: String) -> R::Value {
    let lines_a: Vec<&str> = a.lines().collect();
    let lines_b: Vec<&str> = b.lines().collect();

    let mut result = Vec::new();
    let max_len = lines_a.len().max(lines_b.len());
    let mut i = 0;

    while i < max_len {
        let in_a = lines_a.get(i).copied();
        let in_b = lines_b.get(i).copied();

        match (in_a, in_b) {
            (Some(a_line), Some(b_line)) if a_line == b_line => {
                result.push(R::tuple(vec![
                    R::from_string(a_line.to_string()),
                    R::from_i64(0),
                ]));
            }
            (Some(a_line), Some(b_line)) => {
                result.push(R::tuple(vec![
                    R::from_string(format!("-{}", a_line)),
                    R::from_i64(-1),
                ]));
                result.push(R::tuple(vec![
                    R::from_string(format!("+{}", b_line)),
                    R::from_i64(1),
                ]));
            }
            (Some(a_line), None) => {
                result.push(R::tuple(vec![
                    R::from_string(format!("-{}", a_line)),
                    R::from_i64(-1),
                ]));
            }
            (None, Some(b_line)) => {
                result.push(R::tuple(vec![
                    R::from_string(format!("+{}", b_line)),
                    R::from_i64(1),
                ]));
            }
            (None, None) => {}
        }
        i += 1;
    }

    let inner = TypeAnnotation::Tuple(std::rc::Rc::new(vec![
        TypeAnnotation::String,
        TypeAnnotation::Int,
    ]));
    R::ok(R::array(result, inner))
}

// ---- character class predicates -------------------------------------------

#[native_fn(module = "str")]
pub fn is_alpha(string: String) -> bool {
    !string.is_empty() && string.chars().all(|c| c.is_alphabetic())
}

#[native_fn(module = "str")]
pub fn is_numeric(string: String) -> bool {
    !string.is_empty() && string.chars().all(|c| c.is_ascii_digit())
}

#[native_fn(module = "str")]
pub fn is_whitespace(string: String) -> bool {
    !string.is_empty() && string.chars().all(|c| c.is_whitespace())
}

// ---- unicode category -----------------------------------------------------

#[native_fn(module = "str")]
pub fn unicode_category(ch: String) -> Result<String, String> {
    let c = match ch.chars().next() {
        Some(c) => c,
        None => return Err("unicode_category: empty string".to_string()),
    };
    let cat = if c.is_alphabetic() {
        if c.is_uppercase() { "Lu" }
        else if c.is_lowercase() { "Ll" }
        else { "Lt" }
    } else if c.is_numeric() {
        "Nd"
    } else if c.is_whitespace() {
        "Zs"
    } else if c == '\n' || c == '\r' || c == '\t' {
        "Zl"
    } else if !c.is_control() {
        "Po"
    } else {
        "Cc"
    };
    Ok(cat.to_string())
}

rl_std_core::native_module!("str";
    funcs: [
        to_upper, to_lower, trim, trim_end, trim_start, reverse,
        repeat, is_empty, contains, starts_with, ends_with, replace,
        pad_left, pad_right, count, index_of,
        bytes, chars, split,
        char_at, slice, join,
        concat, format,
        strip_prefix, strip_suffix, last_index_of,
        split_once, lines,
        wrap, indent, dedent, diff_lines,
        is_alpha, is_numeric, is_whitespace, unicode_category,
    ],
);
