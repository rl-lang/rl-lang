//! `std::cli` - command-line interface helpers: arg parsing, prompts, progress.
//!
//! `parse_args` is hand-rolled over `process::args` (no parser dependency) so
//! the VM and the C transpiler share the exact same semantics. The spec is an
//! array of maps, one per option:
//!
//! ```rl
//! get parse_args from std::cli
//! dec result[map] r = parse_args([
//!     {"name": "verbose", "flag": "true"},
//!     {"name": "output", "short": "o", "default": "out.txt"},
//! ])
//! ```
//!
//! Supported spec keys: `name` (required), `flag` (`"true"`/`"false"` string -
//! maps are homogeneous at runtime so a real bool cannot appear here),
//! `short` (single-char alias), `default` (string), `help` (shown in usage).
//! Both `--name value` and `--name=value` forms work; `--` ends flag parsing
//! and everything after it (plus any positional) lands in the `"_"` array.
//! A flag without `flag: true` requires a value. Missing options without a
//! default are an error unless they are flags (default `false`).
//!
//! Prompts read from stdin; `prompt_password` hides input via `rpassword`.
//! `read_line_editable` / `read_line_with_history` use `rustyline` for
//! arrow-key editing. `progress_bar` / `spinner_tick` are hand-rolled ANSI
//! writes routed through the runtime output buffer when one is present, so
//! test harnesses can capture them.

#[cfg(feature = "impls")]
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use rl_ast::statements::TypeAnnotation;
#[cfg(feature = "impls")]
use std::io::Write;

// ---- spec -----------------------------------------------------------------

#[cfg(feature = "impls")]
struct Opt {
    name: String,
    short: Option<char>,
    is_flag: bool,
    default: Option<String>,
    help: Option<String>,
}

#[cfg(feature = "impls")]
fn read_spec<R: Runtime>(spec: &R::Value) -> Result<Vec<Opt>, String> {
    let Some((items, _)) = R::as_array(spec) else {
        return Err(format!(
            "parse_args: spec must be an array of maps, got {}",
            R::type_name(spec)
        ));
    };
    let mut opts = Vec::with_capacity(items.len());
    for item in items {
        let Some((entries, _, _)) = R::as_map(item) else {
            return Err(format!(
                "parse_args: spec entries must be maps, got {}",
                R::type_name(item)
            ));
        };
        let get = |key: &str| -> Option<R::Value> {
            let k = R::from_string(key.to_string());
            entries
                .iter()
                .find(|(ek, _)| R::keys_equal(ek, &k))
                .map(|(_, v)| v.clone())
        };
        let Some(name_v) = get("name") else {
            return Err("parse_args: spec entry is missing \"name\"".to_string());
        };
        let Some(name) = R::as_str(&name_v).map(str::to_owned) else {
            return Err(format!(
                "parse_args: option name must be a string, got {}",
                R::type_name(&name_v)
            ));
        };
        if name.is_empty() || name.starts_with('-') {
            return Err(format!("parse_args: bad option name \"{name}\""));
        }
        let is_flag = match get("flag") {
            None => false,
            Some(v) => {
                let w = R::as_str(&v).ok_or_else(|| {
                    format!(
                        "parse_args: \"flag\" for --{name} must be a \"true\"/\"false\" string, got {}",
                        R::type_name(&v)
                    )
                })?;
                // NOTE: maps are homogeneous at runtime, so the spec cannot
                // carry a real bool here; the flag arrives string-encoded.
                parse_bool_word(w).ok_or_else(|| {
                    format!(
                        "parse_args: \"flag\" for --{name} must be \"true\" or \"false\", got \"{w}\""
                    )
                })?
            }
        };
        let short = match get("short") {
            None => None,
            Some(v) => {
                let s = R::as_str(&v).ok_or_else(|| {
                    format!(
                        "parse_args: \"short\" for --{name} must be a string, got {}",
                        R::type_name(&v)
                    )
                })?;
                let mut chars = s.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => Some(c),
                    _ => {
                        return Err(format!(
                            "parse_args: \"short\" for --{name} must be one character"
                        ));
                    }
                }
            }
        };
        let default = match get("default") {
            None => None,
            Some(v) => Some(R::as_str(&v).ok_or_else(|| {
                format!(
                    "parse_args: \"default\" for --{name} must be a string, got {}",
                    R::type_name(&v)
                )
            })?.to_owned()),
        };
        if is_flag && default.is_some() {
            return Err(format!("parse_args: flag --{name} cannot have a default"));
        }
        let help = get("help").and_then(|v| R::as_str(&v).map(str::to_owned));
        opts.push(Opt {
            name,
            short,
            is_flag,
            default,
            help,
        });
    }
    Ok(opts)
}

#[cfg(feature = "impls")]
fn usage_of(opts: &[Opt], err: Option<&str>) -> String {
    let mut out = String::new();
    if let Some(e) = err {
        out.push_str(e);
        out.push_str("\n\n");
    }
    out.push_str("usage: program [options] [--] [args...]\n\noptions:\n");
    for o in opts {
        out.push_str("  --");
        out.push_str(&o.name);
        if let Some(s) = o.short {
            out.push_str(", -");
            out.push(s);
        }
        if o.is_flag {
            out.push_str("  (flag)");
        }
        if let Some(d) = &o.default {
            out.push_str("  (default: ");
            out.push_str(d);
            out.push(')');
        }
        if let Some(h) = &o.help {
            out.push_str("  ");
            out.push_str(h);
        }
        out.push('\n');
    }
    out
}

#[cfg(feature = "impls")]
fn parse_bool_word(word: &str) -> Option<bool> {
    match word.to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Some(true),
        "false" | "0" | "no" | "n" => Some(false),
        _ => None,
    }
}

// ---- parse_args -----------------------------------------------------------

#[cfg(feature = "impls")]
fn do_parse_args<R: Runtime>(
    cx: &mut R::Cx,
    spec: &R::Value,
) -> Result<R::Value, String> {
    let opts = read_spec::<R>(spec)?;
    let mut argv: Vec<String> = std::env::args().skip(R::user_args_offset(cx)).collect();
    // `rl run prog.rl -- -v` leaves the runner preamble (run, prog.rl, --)
    // in argv. Drop everything through the first `--` so scripts only ever
    // see their own arguments. Without `--` the preamble stays visible as
    // positionals (documented; prefer `--`).
    if let Some(pos) = argv.iter().position(|a| a == "--") {
        argv.drain(..=pos);
    }
    // name -> value (string or bool), positionals collected separately.
    let mut values: Vec<(String, R::Value)> = Vec::new();
    let mut positionals: Vec<R::Value> = Vec::new();
    let mut i = 0;
    let mut only_positional = false;
    while i < argv.len() {
        let arg = &argv[i];
        if only_positional || arg == "-" || !arg.starts_with('-') || arg == "--" {
            if arg == "--" && !only_positional {
                only_positional = true;
                i += 1;
                continue;
            }
            positionals.push(R::from_string(arg.clone()));
            i += 1;
            continue;
        }
        // Split --name=value; short flags take `-s value` only.
        let (key, inline_val) = match arg.strip_prefix("--") {
            Some(rest) => match rest.split_once('=') {
                Some((k, v)) => (k.to_string(), Some(v.to_string())),
                None => (rest.to_string(), None),
            },
            None => (arg.trim_start_matches('-').to_string(), None),
        };
        let is_long = arg.starts_with("--");
        let opt = opts.iter().find(|o| {
            o.name == key || (!is_long && o.short == key.chars().next())
        });
        let Some(opt) = opt else {
            let msg = format!("unknown argument: {arg}");
            return Err(format!("{}\n\n{}", msg, usage_of(&opts, None)));
        };
        if opt.is_flag {
            let val = match inline_val {
                None => true,
                Some(w) => parse_bool_word(&w).ok_or_else(|| {
                    format!("flag --{} expects true/false, got \"{w}\"", opt.name)
                })?,
            };
            set_value(&mut values, &opt.name, R::from_bool(val));
            i += 1;
            continue;
        }
        let val = match inline_val {
            Some(v) => v,
            None => {
                i += 1;
                argv.get(i).cloned().ok_or_else(|| {
                    format!("option --{} expects a value", opt.name)
                })?
            }
        };
        set_value(&mut values, &opt.name, R::from_string(val));
        i += 1;
    }
    // Fill defaults, complain about missing required options.
    for o in &opts {
        if values.iter().any(|(n, _)| n == &o.name) {
            continue;
        }
        if o.is_flag {
            values.push((o.name.clone(), R::from_bool(false)));
        } else if let Some(d) = &o.default {
            values.push((o.name.clone(), R::from_string(d.clone())));
        } else {
            return Err(format!("missing required option: --{}", o.name));
        }
    }
    values.push((
        "_".to_string(),
        R::array(positionals, TypeAnnotation::String),
    ));
    let entries: Vec<(R::Value, R::Value)> = values
        .into_iter()
        .map(|(k, v)| (R::from_string(k), v))
        .collect();
    Ok(R::map(
        entries,
        TypeAnnotation::String,
        TypeAnnotation::Infer,
    ))
}

#[cfg(feature = "impls")]
fn set_value<V>(values: &mut Vec<(String, V)>, name: &str, val: V) {
    if let Some(slot) = values.iter_mut().find(|(n, _)| n == name) {
        slot.1 = val;
    } else {
        values.push((name.to_string(), val));
    }
}

#[native_fn(module = "cli", sig(array[map[string, string]] -> result[map[string, U]]))]
pub fn parse_args<R: Runtime>(cx: &mut R::Cx, spec: R::Value) -> R::Value {
    match do_parse_args::<R>(cx, &spec) {
        Ok(m) => R::ok(m),
        Err(e) => R::err(R::from_string(e)),
    }
}

#[native_fn(module = "cli", sig(array[map[string, string]] -> map[string, U]))]
pub fn parse_args_or_exit<R: Runtime>(cx: &mut R::Cx, spec: R::Value) -> R::Value {
    match do_parse_args::<R>(cx, &spec) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
}

#[native_fn(module = "cli", sig(array[map[string, string]] -> result[string]))]
pub fn usage_string<R: Runtime>(spec: R::Value) -> R::Value {
    match read_spec::<R>(&spec) {
        Ok(opts) => R::ok(R::from_string(usage_of(&opts, None))),
        Err(e) => R::err(R::from_string(e)),
    }
}

// ---- prompts --------------------------------------------------------------

#[cfg(feature = "impls")]
fn read_line_trimmed(prompt: &str) -> String {
    print!("{prompt}");
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => line.trim_end_matches(['\r', '\n']).to_string(),
        Err(_) => String::new(),
    }
}

#[native_fn(module = "cli", sig(string -> string))]
pub fn prompt<R: Runtime>(msg: String) -> String {
    read_line_trimmed(&msg)
}

#[native_fn(module = "cli", sig(string -> string))]
pub fn prompt_password<R: Runtime>(msg: String) -> String {
    rpassword::prompt_password(msg).unwrap_or_default()
}

#[native_fn(module = "cli", sig(string -> bool))]
pub fn prompt_confirm<R: Runtime>(msg: String) -> bool {
    let answer = read_line_trimmed(&format!("{msg} [y/n] "));
    matches!(answer.to_ascii_lowercase().as_str(), "y" | "yes")
}

#[native_fn(module = "cli", sig(string, array[string] -> string))]
pub fn prompt_choice<R: Runtime>(msg: String, options: Vec<String>) -> String {
    if options.is_empty() {
        return String::new();
    }
    loop {
        println!("{msg}");
        for (n, o) in options.iter().enumerate() {
            println!("  {}. {o}", n + 1);
        }
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() {
            return String::new();
        }
        let answer = line.trim();
        if let Ok(n) = answer.parse::<usize>()
            && n >= 1
            && n <= options.len()
        {
            return options[n - 1].clone();
        }
        if options.iter().any(|o| o == answer) {
            return answer.to_string();
        }
        println!("pick 1-{} or one of the listed values", options.len());
    }
}

// ---- shell words ----------------------------------------------------------

#[native_fn(module = "cli", sig(string -> result[array[string]]))]
pub fn shell_split<R: Runtime>(s: String) -> R::Value {
    match shell_words::split(&s) {
        Ok(parts) => R::ok(R::array(
            parts.into_iter().map(R::from_string).collect(),
            TypeAnnotation::String,
        )),
        Err(e) => R::err(R::from_string(format!("shell_split: {e}"))),
    }
}

#[native_fn(module = "cli", sig(array[string] -> string))]
pub fn shell_join<R: Runtime>(parts: Vec<String>) -> String {
    shell_words::join(parts)
}

// ---- editable input -------------------------------------------------------

#[cfg(feature = "impls")]
use rustyline::history::History;

#[cfg(feature = "impls")]
type MemHistory = rustyline::history::MemHistory;

#[cfg(feature = "impls")]
type MemEd = rustyline::Editor<(), MemHistory>;

#[cfg(feature = "impls")]
fn edit_line(msg: &str, history: &[String]) -> String {
    let mut h = MemHistory::new();
    for line in history {
        let _ = h.add(line);
    }
    let Ok(mut ed) = MemEd::with_history(rustyline::Config::default(), h) else {
        return String::new();
    };
    ed.readline(msg).unwrap_or_default()
}

#[native_fn(module = "cli", sig(string -> string))]
pub fn read_line_editable<R: Runtime>(msg: String) -> String {
    edit_line(&msg, &[])
}

#[native_fn(module = "cli", sig(string, array[string] -> tuple[string, array[string]]))]
pub fn read_line_with_history<R: Runtime>(
    msg: String,
    history: Vec<String>,
) -> R::Value {
    let line = edit_line(&msg, &history);
    let mut updated = history;
    if !line.is_empty() {
        updated.push(line.clone());
    }
    R::tuple(vec![
        R::from_string(line),
        R::array(
            updated.into_iter().map(R::from_string).collect(),
            TypeAnnotation::String,
        ),
    ])
}

// ---- progress -------------------------------------------------------------

#[cfg(feature = "impls")]
fn emit<R: Runtime>(cx: &mut R::Cx, text: &str) {
    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(text);
    } else {
        eprint!("{text}");
        let _ = std::io::stderr().flush();
    }
}

#[native_fn(module = "cli", sig(int, int, string -> null))]
pub fn progress_bar<R: Runtime>(cx: &mut R::Cx, current: i64, total: i64, label: String) -> R::Value {
    const WIDTH: usize = 24;
    let frac = if total <= 0 {
        0.0
    } else {
        (current.max(0) as f64 / total as f64).clamp(0.0, 1.0)
    };
    let filled = (frac * WIDTH as f64).round() as usize;
    let mut bar = String::with_capacity(WIDTH + 32);
    bar.push('\r');
    bar.push_str(&label);
    bar.push_str(" [");
    for _ in 0..filled {
        bar.push('#');
    }
    for _ in filled..WIDTH {
        bar.push('-');
    }
    bar.push_str(&format!("] {:3}%", (frac * 100.0).round() as i64));
    if frac >= 1.0 {
        bar.push('\n');
    }
    emit::<R>(cx, &bar);
    R::null()
}

#[native_fn(module = "cli", sig(int -> null))]
pub fn spinner_tick<R: Runtime>(cx: &mut R::Cx, frame: i64) -> R::Value {
    const FRAMES: [char; 4] = ['|', '/', '-', '\\'];
    let c = FRAMES[(frame.rem_euclid(4)) as usize];
    emit::<R>(cx, &format!("\r{c}"));
    R::null()
}

// ---- module registration --------------------------------------------------

rl_std_core::native_module!("cli";
    funcs: [
        parse_args, parse_args_or_exit, usage_string,
        prompt, prompt_password, prompt_confirm, prompt_choice,
        shell_split, shell_join,
        read_line_editable, read_line_with_history,
        progress_bar, spinner_tick,
    ],
);
