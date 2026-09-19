//! The single, runtime-agnostic implementation of the rl-lang standard library.
//!
//! Each function is written once, generic over `rl_std_core::Runtime`, and
//! annotated with `#[native_fn]` so it is available to both the VM and the
//! interpreter as a thin function pointer, and to the checker as a signature.
//!
//! ## Feature split
//! - `signatures` (default): only the pure `signature()`/`KEYWORDS` data is
//!   compiled - no runtime, no OS-facing dependencies. This is what the type
//!   checker and language server use.
//! - `impls`: the actual function bodies + `handles::<R>()` builders, pulling
//!   in the OS-facing crates. The VM and interpreter enable this.
//!
//! `#[native_fn]` gates the body/wrapper/handle behind `impls` per function, and
//! the OS-facing modules (`audio`, `c`, `gui`, `http`, `process`, `terminal`)
//! gate their hand-written handle types/helpers/imports too, so *every* module's
//! `signature()` data is available in a signatures-only build - the checker/LSP
//! read the whole tree via [`signatures`] without compiling eframe/rodio/libffi.

// In a signatures-only build the per-function bodies (and the imports/helpers
// they use) are gated out, leaving some imports unused. That is expected; keep
// the (light) checker/LSP build warning-free without hiding real unused warnings
// in the `impls` build.
#![cfg_attr(
    not(feature = "impls"),
    allow(unused_imports, dead_code, unused_macros)
)]

// Per-module declarations gated behind feature flags.
// `impls` enables all modules for backward compatibility.
#[cfg(any(feature = "std-array", feature = "impls"))]
pub mod array;
#[cfg(any(feature = "std-audio", feature = "impls"))]
pub mod audio;
#[cfg(any(feature = "std-bitwise", feature = "impls"))]
pub mod bitwise;
#[cfg(any(feature = "std-c", feature = "impls"))]
pub mod c;
#[cfg(any(feature = "std-collections", feature = "impls"))]
pub mod collections;
#[cfg(any(feature = "std-debug", feature = "impls"))]
pub mod debug;
#[cfg(any(feature = "std-fs", feature = "impls"))]
pub mod fs;
#[cfg(any(feature = "std-gui", feature = "impls"))]
pub mod gui;
#[cfg(any(feature = "std-http", feature = "impls"))]
pub mod http;
#[cfg(any(feature = "std-io", feature = "impls"))]
pub mod io;
#[cfg(any(feature = "std-math", feature = "impls"))]
pub mod math;
#[cfg(any(feature = "std-net", feature = "impls"))]
pub mod net;
#[cfg(any(feature = "std-path", feature = "impls"))]
pub mod path;
#[cfg(any(feature = "std-process", feature = "impls"))]
pub mod process;
#[cfg(any(feature = "std-random", feature = "impls"))]
pub mod random;
#[cfg(any(feature = "std-result", feature = "impls"))]
pub mod result;
#[cfg(any(feature = "std-string", feature = "impls"))]
pub mod string;
#[cfg(any(feature = "std-terminal", feature = "impls"))]
pub mod terminal;
#[cfg(any(feature = "std-time", feature = "impls"))]
pub mod time;
pub mod types;

/// The full `std::*` checker signature tree. Built from the same `#[native_fn]`
/// annotations that generate the runtime handles, so signatures can never drift
/// from implementations. Available without the `impls` feature (no OS-facing
/// deps), so the checker and LSP stay light.
pub fn signatures() -> rl_std_core::ModuleNames {
    #[allow(unused_mut)]
    let mut m = rl_std_core::ModuleNames::new("std")
        .with_functions(&["len"]);
    #[cfg(any(feature = "std-array", feature = "impls"))]
    { m = m.with_module(array::signatures().with_functions(&["len"])); }
    #[cfg(any(feature = "std-audio", feature = "impls"))]
    { m = m.with_module(audio::signatures()); }
    #[cfg(any(feature = "std-bitwise", feature = "impls"))]
    { m = m.with_module(bitwise::signatures()); }
    #[cfg(any(feature = "std-c", feature = "impls"))]
    { m = m.with_module(c::signatures()); }
    #[cfg(any(feature = "std-collections", feature = "impls"))]
    { m = m.with_module(collections::signatures()); }
    #[cfg(any(feature = "std-debug", feature = "impls"))]
    { m = m.with_module(debug::signatures()); }
    #[cfg(any(feature = "std-fs", feature = "impls"))]
    { m = m.with_module(fs::signatures()); }
    #[cfg(any(feature = "std-gui", feature = "impls"))]
    { m = m.with_module(gui::signatures()); }
    #[cfg(any(feature = "std-http", feature = "impls"))]
    { m = m.with_module(http::signatures()); }
    #[cfg(any(feature = "std-io", feature = "impls"))]
    { m = m.with_module(io::signatures()); }
    #[cfg(any(feature = "std-math", feature = "impls"))]
    { m = m.with_module(math::signatures()); }
    #[cfg(any(feature = "std-net", feature = "impls"))]
    { m = m.with_module(net::signatures()); }
    #[cfg(any(feature = "std-path", feature = "impls"))]
    { m = m.with_module(path::signatures()); }
    #[cfg(any(feature = "std-process", feature = "impls"))]
    { m = m.with_module(process::signatures()); }
    #[cfg(any(feature = "std-random", feature = "impls"))]
    { m = m.with_module(random::signatures()); }
    #[cfg(any(feature = "std-result", feature = "impls"))]
    { m = m.with_module(result::signatures()); }
    #[cfg(any(feature = "std-string", feature = "impls"))]
    { m = m.with_module(string::signatures()); }
    #[cfg(any(feature = "std-terminal", feature = "impls"))]
    { m = m.with_module(terminal::signatures()); }
    #[cfg(any(feature = "std-time", feature = "impls"))]
    { m = m.with_module(time::signatures()); }
    #[cfg(any(feature = "std-types", feature = "impls"))]
    { m = m.with_module(types::signatures()); }
    m
}
