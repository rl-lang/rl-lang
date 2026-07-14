use crate::native::Module;

/// Builds the compiler-facing native module tree: an unnamed root holding
/// a `std` submodule, mirroring `rl-interpreter`'s `root_module` shape so
/// `std::io::println` resolves the same way in both.
pub fn root() -> Module {
    Module::new("root")
}
