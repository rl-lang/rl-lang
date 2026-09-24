//! `std::io` - input/output: reading from stdin, printing.
//!
//! `print` and `println` write to [`Runtime::output_buffer`] when set (the REPL
//! captures per-input output there), otherwise directly to stdout. They are
//! variadic and untyped: they stringify each argument via `R::display` (the old
//! `Value::to_string`).
//!
//! `read`/`read_int`/`read_float` read a line from stdin. They are variadic and
//! untyped, accepting 0 or 1 optional prompt argument (any scalar, stringified
//! via `R::display`); with more than one argument they return an `err(..)`.
//! `read_int`/`read_float` then parse the line and return a language
//! `result[int]` / `result[float]`.
//!
//! `eprint` raises a propagating runtime error rather than writing to stderr, so
//! errors surface through rl's normal error reporting pipeline.

#[cfg(feature = "impls")]
use std::io::{Read, Write};
use rl_ast::statements::HandleKind;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- handle store ---------------------------------------------------------

/// A single native I/O resource stored behind an integer handle.
pub enum IoFileHandle {
    Read(std::io::BufReader<std::fs::File>),
    Write(std::fs::File),
    ReadWrite(std::io::BufReader<std::fs::File>, std::fs::File),
}

/// Per-runtime access to the `io` handle table. Implemented by `VmRuntime`.
pub trait IoStore: Runtime {
    fn io_insert(cx: &mut Self::Cx, h: IoFileHandle) -> u64;
    fn io_get(cx: &Self::Cx, id: u64) -> Option<&IoFileHandle>;
    fn io_get_mut(cx: &mut Self::Cx, id: u64) -> Option<&mut IoFileHandle>;
    fn io_remove(cx: &mut Self::Cx, id: u64) -> Option<IoFileHandle>;
}

/// Inserts a handle and returns its rl handle value.
#[cfg(feature = "impls")]
pub fn insert_handle<R: IoStore>(cx: &mut R::Cx, h: IoFileHandle) -> R::Value {
    let id = R::io_insert(cx, h);
    R::make_handle(HandleKind::File, id)
}

/// Extracts a `File` handle id from a value, or returns a type error.
#[cfg(feature = "impls")]
pub fn extract_handle<R: IoStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    match R::as_handle(v, HandleKind::File) {
        Some(id) => Ok(id),
        None => match R::as_handle(v, HandleKind::C)
            .map(|_| HandleKind::C)
            .or_else(|| R::as_handle(v, HandleKind::Http).map(|_| HandleKind::Http))
            .or_else(|| R::as_handle(v, HandleKind::Audio).map(|_| HandleKind::Audio))
            .or_else(|| R::as_handle(v, HandleKind::Gui).map(|_| HandleKind::Gui))
            .or_else(|| R::as_handle(v, HandleKind::Net).map(|_| HandleKind::Net))
        {
            Some(kind) => Err(format!(
                "{}: expected a {:?} handle, got a {:?} handle",
                name,
                HandleKind::File,
                kind
            )),
            None => Err(format!(
                "{}: expected a handle, got {}",
                name,
                R::type_name(v)
            )),
        },
    }
}

// ---- printing (variadic, untyped) -----------------------------------------

#[native_fn(module = "io", untyped)]
pub fn print<R: Runtime>(cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(&text);
    } else {
        print!("{}", text);
    }
    R::null()
}

#[native_fn(module = "io", untyped)]
pub fn println<R: Runtime>(cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(&text);
        buffer.push('\n');
    } else {
        println!("{}", text);
    }
    R::null()
}

// ---- stdin reading --------------------------------------------------------

#[cfg(feature = "impls")]
fn read_line<R: Runtime>() -> R::Value {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => R::ok(R::from_string(input.trim().to_string())),
        Err(e) => R::err(R::from_string(format!("read: failed to read line: {}", e))),
    }
}

#[cfg(feature = "impls")]
fn input<R: Runtime>(prompt: Option<&R::Value>) -> R::Value {
    match prompt {
        None => read_line::<R>(),
        Some(p) => {
            print!("{}", R::display(p));
            std::io::stdout().flush().ok();
            read_line::<R>()
        }
    }
}

#[native_fn(module = "io",
    sig(-> result[string]),
    sig(int -> result[string]),
    sig(float -> result[string]),
    sig(string -> result[string]),
    sig(bool -> result[string]),
    sig(char -> result[string]))]
pub fn read<R: Runtime>(args: Vec<R::Value>) -> R::Value {
    match args.len() {
        0 => input::<R>(None),
        1 => input::<R>(args.first()),
        n => R::err(R::from_string(format!(
            "read: expects 0 or 1 argument(s), got {}",
            n
        ))),
    }
}

#[native_fn(module = "io",
    sig(-> result[int]),
    sig(int -> result[int]),
    sig(float -> result[int]),
    sig(string -> result[int]),
    sig(bool -> result[int]),
    sig(char -> result[int]))]
pub fn read_int<R: Runtime>(args: Vec<R::Value>) -> R::Value {
    let value = match args.len() {
        0 => input::<R>(None),
        1 => input::<R>(args.first()),
        n => {
            return R::err(R::from_string(format!(
                "read_int: expects 0 or 1 argument(s), got {}",
                n
            )));
        }
    };

    if let Some(inner) = R::as_ok_inner(&value) {
        match R::as_str(&inner) {
            Some(s) => match s.parse::<i64>() {
                Ok(i) => R::ok(R::from_i64(i)),
                Err(_) => R::err(R::from_string(format!(
                    "read_int: \"{}\" is not a valid integer",
                    s
                ))),
            },
            None => R::err(R::from_string(format!(
                "read_int: found unsupported type from input, got {}",
                R::type_name(&inner)
            ))),
        }
    } else if R::as_err_inner(&value).is_some() {
        value
    } else {
        R::err(R::from_string(format!(
            "read_int: found unsupported type from input, got {}",
            R::type_name(&value)
        )))
    }
}

#[native_fn(module = "io",
    sig(-> result[float]),
    sig(int -> result[float]),
    sig(float -> result[float]),
    sig(string -> result[float]),
    sig(bool -> result[float]),
    sig(char -> result[float]))]
pub fn read_float<R: Runtime>(args: Vec<R::Value>) -> R::Value {
    let value = match args.len() {
        0 => input::<R>(None),
        1 => input::<R>(args.first()),
        n => {
            return R::err(R::from_string(format!(
                "read_float: expects 0 or 1 argument(s), got {}",
                n
            )));
        }
    };

    if let Some(inner) = R::as_ok_inner(&value) {
        match R::as_str(&inner) {
            Some(s) => match s.parse::<f64>() {
                Ok(f) => R::ok(R::from_f64(f)),
                Err(_) => R::err(R::from_string(format!(
                    "read_float: \"{}\" is not a valid float",
                    s
                ))),
            },
            None => R::err(R::from_string(format!(
                "read_float: found unsupported type from input, got {}",
                R::type_name(&inner)
            ))),
        }
    } else if R::as_err_inner(&value).is_some() {
        value
    } else {
        R::err(R::from_string(format!(
            "read_float: found unsupported type from input, got {}",
            R::type_name(&value)
        )))
    }
}

// ---- stdin advanced -------------------------------------------------------

#[native_fn(module = "io")]
pub fn read_all_stdin() -> Result<String, String> {
    let mut input = String::new();
    match std::io::stdin().read_to_string(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => Err(format!("read_all_stdin: {}", e)),
    }
}

// ---- encoding / decoding --------------------------------------------------

#[native_fn(module = "io")]
pub fn decode_utf8(bytes: Vec<u8>) -> Result<String, String> {
    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!(
            "decode_utf8: invalid UTF-8 at byte {}",
            e.utf8_error()
        )),
    }
}

#[native_fn(module = "io")]
pub fn encode_utf8(string: String) -> Vec<u8> {
    string.into_bytes()
}

// ---- terminal check -------------------------------------------------------

#[native_fn(module = "io")]
pub fn isatty() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
    }
    #[cfg(not(unix))]
    {
        false
    }
}

// ---- stderr ---------------------------------------------------------------

#[native_fn(module = "io", untyped)]
pub fn eprint<R: Runtime>(_cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    eprint!("{}", text);
    R::null()
}

#[native_fn(module = "io", untyped)]
pub fn eprintln<R: Runtime>(_cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    eprintln!("{}", text);
    R::null()
}

rl_std_core::native_module!("io";
    bound: IoStore;
    funcs: [
        print, println,
        read, read_int, read_float,
        read_all_stdin,
        decode_utf8, encode_utf8,
        isatty,
        eprint, eprintln,
    ],
);
