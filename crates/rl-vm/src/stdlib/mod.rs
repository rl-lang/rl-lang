//! The VM's standard library - built-in modules registered under `std::*`.
//!
//! Only `io` exists so far (`print`/`println`). Everything else in
//! `rl-interpreter`'s stdlib (math, string, array, fs, ...) hasn't been
//! ported yet - most of it needs `VmValue` to grow array/tuple/error
//! variants first.

pub mod io;

use crate::native::Module;

/// Builds the compiler-facing native module tree: an unnamed root holding
/// a `std` submodule, mirroring `rl-interpreter`'s `root_module` shape so
/// `std::io::println` resolves the same way in both.
pub fn root() -> Module {
    Module::new("root").with_module(Module::new("std").with_module(io::module()))
}
