//! Runtime-agnostic core for the rl-lang standard library.
//!
//! This crate holds everything the shared stdlib (`rl-std`) and the `rl-vm`
//! runtime need in common, without referencing the runtime's value type - so
//! it can sit below it in the dependency graph:
//!
//! - [`Runtime`] / [`HandleStore`] - the abstraction each runtime implements,
//! - [`NativeHandle`] / [`Arity`] - the thin-`fn`-pointer native descriptor,
//! - [`ValueType`] / [`FromValueR`] / [`IntoValueR`] - value <-> Rust type
//!   conversions,
//! - [`StdFn`] / [`ModuleNames`] - the checker signature types (moved here from
//!   `rl-commons`),
//! - [`Xoshiro256`] - the shared PRNG.
//! - [`TestState`] / [`TestCase`] - the `std::test` registry types.

pub mod convert;
pub mod handle;
#[macro_use]
pub mod module;
pub mod rng;
pub mod runtime;
pub mod signatures;
pub mod test_state;

pub use convert::{Bytes, FromValueR, IntoValueR, ValueType};
pub use handle::{Arity, NativeHandle, NativeThunk};
pub use rng::Xoshiro256;
pub use runtime::{HandleStore, Runtime};
pub use signatures::{ModuleNames, StdFn};
pub use test_state::{TestCase, TestState};
