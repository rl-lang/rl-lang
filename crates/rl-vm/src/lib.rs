pub mod bytecode;
pub mod chunk;
pub mod compiler;
pub mod native;
pub mod runtime;
pub mod stdlib;
pub mod values;
pub mod vm_logic;

pub use bytecode::{deserialize_chunk, serialize_chunk};
pub use chunk::{Chunk, OpCode};
pub use compiler::{CompileError, Compiler, TestPlan, TestTarget};
pub use native::{Module, NativeFn};
pub use runtime::VmRuntime;
pub use values::{VmNative, VmNativeFn, VmValue};
pub use vm_logic::{Vm, VmError};
