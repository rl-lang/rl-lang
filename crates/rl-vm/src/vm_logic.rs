use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::VmNative;
use crate::chunk::{Chunk, OpCode};
use crate::values::{RecordFields, VmFunction, VmMapKey, VmValue};
use rl_std::gui::GuiHandle;
use rl_utils::errors::{Error, Reason};
use rl_utils::line_index::LineIndex;
use rl_utils::source::SourceFile;
use rl_utils::span::Span;

/// Errors raised while executing a compiled [`Chunk`].
///
/// A plain alias over the shared [`Error`] type (see `rl-vm::compiler::CompileError`),
/// so runtime errors get the same ariadne-rendered source snippets as
/// everywhere else in the pipeline, anchored at the currently executing
/// instruction's [`Span`].
pub type VmError = Error;

/// Shared arithmetic-op expansion for `+`, `-`, `*`e.
macro_rules! binary_arith {
    ($self:expr, $checked:ident, $float_op:tt, $op_str:literal) => {{
        let (a, b) = $self.pop_two_unchecked();
        let out = match (a, b) {
            (VmValue::Int(a), VmValue::Int(b)) => VmValue::Int(a.$checked(b).ok_or_else(|| {
                $self.err(format!("integer overflow: {a} {} {b}", $op_str))
            })?),
            (VmValue::UInt(a), VmValue::UInt(b)) => VmValue::UInt(a.$checked(b).ok_or_else(|| {
                $self.err(format!("integer overflow: {a} {} {b}", $op_str))
            })?),
            (VmValue::SInt(a), VmValue::SInt(b)) => VmValue::SInt(a.$checked(b).ok_or_else(|| {
                $self.err(format!("integer overflow: {a} {} {b}", $op_str))
            })?),
            (VmValue::SUInt(a), VmValue::SUInt(b)) => VmValue::SUInt(a.$checked(b).ok_or_else(|| {
                $self.err(format!("integer overflow: {a} {} {b}", $op_str))
            })?),
            (VmValue::BByte(a), VmValue::BByte(b)) => VmValue::BByte(a.$checked(b).ok_or_else(|| {
                $self.err(format!("integer overflow: {a} {} {b}", $op_str))
            })?),
            (VmValue::BSByte(a), VmValue::BSByte(b)) => {
                VmValue::BSByte(a.$checked(b).ok_or_else(|| {
                    $self.err(format!("integer overflow: {a} {} {b}", $op_str))
                })?)
            }
            (VmValue::Byte(a), VmValue::Byte(b)) => VmValue::Byte(a.$checked(b).ok_or_else(|| {
                $self.err(format!("integer overflow: {a} {} {b}", $op_str))
            })?),
            (VmValue::SByte(a), VmValue::SByte(b)) => VmValue::SByte(a.$checked(b).ok_or_else(|| {
                $self.err(format!("integer overflow: {a} {} {b}", $op_str))
            })?),
            (VmValue::Float(a), VmValue::Float(b)) => VmValue::Float(a $float_op b),
            (VmValue::SFloat(a), VmValue::SFloat(b)) => VmValue::SFloat(a $float_op b),
            (a, b) => {
                return Err($self.err(format!("cannot apply arithmetic op to {a:?} and {b:?}")));
            }
        };
        $self.stack.push(out);
    }};
}

enum FrameSource<'a> {
    Top(&'a Chunk),
    Func(Rc<VmFunction>),
}

impl<'a> FrameSource<'a> {
    #[inline]
    fn chunk(&self) -> &Chunk {
        match self {
            FrameSource::Top(c) => c,
            FrameSource::Func(f) => &f.chunk,
        }
    }
}

struct CallFrame<'a> {
    source: FrameSource<'a>,
    ip: usize,
    scope_base: usize,
    /// `scope_starts[scope_base]`, the locals base of this frame. Cached here
    /// (instead of re-indexing the scope table at every frame switch) so
    /// `Return`/`Propagate` can restore the caller's base from a plain struct
    /// field read.
    frame_base: usize,
}

pub struct Vm {
    stack: Vec<VmValue>,
    globals: Vec<VmValue>,
    locals: Vec<VmValue>,
    scope_starts: Vec<usize>,
    /// Byte offset of the instruction currently executing, written once per
    /// dispatch loop iteration. The [`Span`] is *not* materialized eagerly -
    /// it's recovered from the chunk on demand via [`Vm::cur_span`] only when
    /// an error is built or a native function asks for it, which avoids a
    /// per-instruction copy in the hot loop.
    current_ip: usize,
    /// The chunk the currently-executing instruction belongs to, kept in sync
    /// with the dispatch loop's `cur_chunk` local so [`Vm::cur_span`] can
    /// resolve `current_ip`. `null` when no chunk is running.
    current_chunk: *const Chunk,
    /// Original source text, so runtime errors can render ariadne snippets.
    source: Option<SourceFile>,
    /// Byte-offset -> line/col table, used when `source` is `None` (e.g.
    /// running compiled `.rlc` bytecode, which embeds this instead of the
    /// full source) so runtime errors can still report a precise
    /// `file:line:col` location instead of a bare message.
    line_index: Option<LineIndex>,
    /// `impl` methods registered via `OpCode::RegisterMethod`, keyed by
    /// `"Record::method"`. Populated as the enclosing `impl` block's
    /// statement runs, then consulted by `OpCode::LookupAssoc` (associated
    /// functions, `Record::method(...)`) and `OpCode::LookupMethod`
    /// (instance methods, `value.method(...)`).
    impl_methods: HashMap<String, Rc<VmFunction>>,
    /// stdlib functions imported via `get x from std::module`, keyed by
    /// name. Populated by `OpCode::RegisterStdlibMethod` (emitted per
    /// import) and consulted by `OpCode::LookupMethod` as the
    /// free-function fallback for `value.method(...)` on non-record
    /// receivers - mirroring the interpreter's `call_path` stdlib step.
    stdlib_methods: HashMap<String, VmNative>,
    /// named user functions, keyed by name. Populated by
    /// `OpCode::RegisterUserMethod` (emitted per function declaration) and
    /// consulted by `OpCode::LookupMethod` after the stdlib fallback -
    /// mirroring the interpreter's `fn_names`.
    user_methods: HashMap<String, Rc<VmFunction>>,
    /// Side-table of native C-interop resources (`std::c`), keyed by handle
    /// id. `pub(crate)` (unlike every field above) because, unlike every
    /// other native function so far, `std::c`'s functions need persistent
    /// state across calls, not just their own arguments - see `stdlib::c`.
    pub(crate) c_handles: HashMap<u64, rl_std::c::CHandle>,
    /// Next handle id to hand out for `std::c` resources; only ever increments.
    pub(crate) c_next_handle: u64,
    /// Side-table of native audio-playback resources (`std::audio`), keyed by handle id.
    pub(crate) audio_handles: HashMap<u64, rl_std::audio::AudioHandle>,
    /// Next handle id to hand out for `std::audio` resources; only ever increments.
    pub(crate) audio_next_handle: u64,
    /// Output device selected via `std::audio::set_output_device`, if any;
    /// `None` means the system default device.
    pub(crate) audio_output_device: Option<String>,
    /// Global volume scalar set via `std::audio::set_master_volume`, applied
    /// on top of each sound's own `sound_set_volume` value. Defaults to `1.0`.
    pub(crate) audio_master_volume: f32,
    /// Side-table of native GUI resources (`std::gui`), keyed by handle id.
    pub(crate) gui_handles: HashMap<u64, GuiHandle<VmValue>>,
    /// Next handle id to hand out for `std::gui` resources; only ever increments.
    pub(crate) gui_next_handle: u64,
    /// Set by `gui_quit`; checked by `gui_run`'s frame loop after that frame's
    /// click callbacks have run, so the window closes on the next frame instead
    /// of being torn down mid-callback.
    pub(crate) gui_quit_requested: bool,
    /// Side-table of native TCP/UDP resources (`std::net`), keyed by handle id.
    pub(crate) net_handles: HashMap<u64, rl_std::net::NetHandle>,
    /// Next handle id to hand out for `std::net` resources; only ever increments.
    pub(crate) net_next_handle: u64,
    /// Side-table of native HTTP resources (`std::http`), keyed by handle id.
    pub(crate) http_handles: HashMap<u64, rl_std::http::HttpHandle>,
    /// Next handle id to hand out for `std::http` resources; only ever increments.
    pub(crate) http_next_handle: u64,
    /// Side-table of native I/O file resources (`std::io`), keyed by handle id.
    pub(crate) io_handles: HashMap<u64, rl_std::io::IoFileHandle>,
    /// Next handle id to hand out for `std::io` resources; only ever increments.
    pub(crate) io_next_handle: u64,
    /// Side-table of byte buffers (`core::__buf_*`), keyed by handle id.
    pub(crate) buf_handles: HashMap<u64, Vec<u8>>,
    /// Next handle id to hand out for buffers; only ever increments.
    pub(crate) buf_next_handle: u64,
    /// PRNG state for `std::random`, seeded from the system clock at startup.
    pub(crate) rng: rl_std_core::Xoshiro256,
    /// Registry for `std::test` (cases, grouping, results), isolated per Vm.
    pub(crate) test_state: rl_std_core::TestState<crate::VmValue>,
    /// Number of leading `std::env::args()` entries to skip when reporting
    /// `std::process::args()` (defaults to 1 - the program name itself).
    pub user_args_offset: usize,
    /// When set, `std::io::print`/`std::io::println` append into this buffer
    /// instead of writing to stdout. The REPL sets this per-input so `print`
    /// output lands in the output area instead of cluttering the terminal.
    pub output_buffer: Option<String>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            globals: Vec::new(),
            locals: Vec::new(),
            scope_starts: Vec::new(),
            current_ip: 0,
            current_chunk: std::ptr::null(),
            source: None,
            line_index: None,
            impl_methods: HashMap::new(),
            stdlib_methods: HashMap::new(),
            user_methods: HashMap::new(),
            c_handles: HashMap::new(),
            c_next_handle: 1,
            audio_handles: HashMap::new(),
            audio_next_handle: 1,
            audio_output_device: None,
            audio_master_volume: 1.0,
            gui_handles: HashMap::new(),
            gui_next_handle: 1,
            gui_quit_requested: false,
            net_handles: HashMap::new(),
            net_next_handle: 1,
            http_handles: HashMap::new(),
            http_next_handle: 1,
            io_handles: HashMap::new(),
            io_next_handle: 1,
            buf_handles: HashMap::new(),
            buf_next_handle: 1,
            rng: Default::default(),
            test_state: Default::default(),
            user_args_offset: 1,
            output_buffer: None,
        }
    }

    /// Attaches the original source text so runtime errors can render
    /// ariadne source snippets instead of a bare message.
    pub fn with_source_file(mut self, source: SourceFile) -> Self {
        self.source = Some(source);
        self
    }

    /// The `std::test` registry (cases, results, skips). The `rl test`
    /// runner reads it after each case driver to report verdicts.
    pub fn test_state(&mut self) -> &mut rl_std_core::TestState<VmValue> {
        &mut self.test_state
    }

    /// Sets the source text on an already-constructed [`Vm`] (the builder
    /// form [`Vm::with_source_file`] consumes `self`, which doesn't work for
    /// the REPL's persistent `Vm`). Runtime errors render ariadne snippets
    /// against this text.
    pub fn set_source_file(&mut self, source: SourceFile) {
        self.source = Some(source);
    }

    /// Clears per-execution transient state - the value stack, locals, and
    /// scope table - while preserving globals and native side-tables. The
    /// REPL calls this after a runtime error so the next input starts from a
    /// clean stack instead of reusing a torn-down one.
    pub fn reset_transient(&mut self) {
        self.stack.clear();
        self.locals.clear();
        self.scope_starts.clear();
        self.current_chunk = std::ptr::null();
        self.current_ip = 0;
    }

    /// Attaches a [`LineIndex`] so runtime errors can still report a
    /// precise `file:line:col` location when no source text is available
    /// (see the `line_index` field docs). No-op when `source` is also set -
    /// [`Error::report_to_stderr`] always prefers the full ariadne snippet.
    pub fn with_line_index(mut self, index: LineIndex) -> Self {
        self.line_index = Some(index);
        self
    }

    /// Builds a [`Reason::Runtime`] error anchored at the currently
    /// executing instruction, with source (or a line-index location)
    /// attached when known.
    pub fn err(&self, message: impl Into<String>) -> VmError {
        let err = Error::at(Reason::Runtime, message, self.cur_span());
        self.attach_location(err)
    }

    /// [`Span`] of the instruction currently executing. Native functions
    /// (which don't receive a `Span` argument) can use this to anchor errors
    /// or re-enter the interpreter via [`Vm::call_value`] at the right spot.
    pub fn current_span(&self) -> Span {
        self.cur_span()
    }

    /// Resolves the span of the currently-executing instruction from the
    /// lazily-tracked `current_ip`/`current_chunk`, without copying it on
    /// every instruction. Falls back to a dummy span when no chunk is
    /// running (e.g. a native function invoked before `Vm::run`).
    fn cur_span(&self) -> Span {
        if self.current_chunk.is_null() {
            return Span::dummy();
        }
        unsafe { (*self.current_chunk).span_at(self.current_ip) }
    }

    /// Original source attached via [`Vm::with_source_file`], if any.
    pub(crate) fn source_file(&self) -> Option<&SourceFile> {
        self.source.as_ref()
    }

    /// Re-anchors an error built without span/source context - e.g. deep
    /// inside the generic native-function binding machinery, which has no
    /// access to the VM - at the currently executing instruction. Used at
    /// the single point where a native call's `Result` rejoins the main
    /// loop, so every native-function error still gets a correct source
    /// snippet without threading a `Span` through `FromValue`/`IntoNativeFn`.
    pub fn annotate(&self, e: VmError) -> VmError {
        let e = e.with_span(self.cur_span());
        self.attach_location(e)
    }

    /// Shared by [`Vm::err`] and [`Vm::annotate`]: attaches full source
    /// when available, otherwise falls back to a `LineIndex`-derived
    /// `file:line:col` location, otherwise leaves the error as-is (plain
    /// message only).
    fn attach_location(&self, e: VmError) -> VmError {
        if let Some(file) = &self.source {
            return e.with_source_file(file);
        }
        if let Some(index) = &self.line_index {
            return e.with_location_from(index);
        }
        e
    }

    pub fn run_and_return(&mut self, chunk: &Chunk) -> Result<VmValue, VmError> {
        self.run(chunk)?;
        Ok(self.stack.pop().unwrap_or(VmValue::Null))
    }

    /// Calls an arbitrary callable `VmValue` (a user function, closure, or
    /// native function) with the given arguments and returns its result.
    ///
    /// Used by native stdlib modules (e.g. `std::gui`) that need to invoke
    /// an rl-lang callback value from Rust - outside the normal `Call`
    /// opcode dispatch path, e.g. from an egui event callback. Dispatches
    /// directly on the callee instead of building a synthetic chunk:
    /// native functions are invoked synchronously, and user functions /
    /// closures get their own call frame executed to completion via the
    /// shared [`Vm::run_frames`] loop. `self.stack`, `self.locals`, and
    /// `self.scope_starts` are ordinary fields, so this is safe to call
    /// reentrantly from inside an already-running dispatch loop (e.g. from
    /// within a native function's own body).
    pub fn call_value(
        &mut self,
        callee: &VmValue,
        args: &[VmValue],
        _span: Span,
    ) -> Result<VmValue, VmError> {
        self.invoke_callable(callee, args)
    }

    /// Direct callable dispatch backing [`Vm::call_value`] (synchronous
    /// invocation). Binds the arguments into a fresh scope and pushes a
    /// single call frame for user functions and closures; runs native
    /// functions inline.
    fn invoke_callable(&mut self, callee: &VmValue, args: &[VmValue]) -> Result<VmValue, VmError> {
        match callee {
            VmValue::Native(native) => {
                let result = match native {
                    crate::values::VmNative::Std(h) => (h.thunk)(self, args.to_vec(), ()),
                    crate::values::VmNative::Legacy(f) => (f.func)(self, args.to_vec()),
                };
                result.map_err(|e| self.annotate(e))
            }
            VmValue::Function(func) => {
                if args.len() != func.arity {
                    return Err(self.err(format!(
                        "{} expects {} args, got {}",
                        func.name,
                        func.arity,
                        args.len()
                    )));
                }
                let base = self.locals.len();
                self.locals.resize(base + args.len(), VmValue::Null);
                for (i, arg) in args.iter().enumerate() {
                    self.locals[base + i] = arg.clone();
                }
                self.scope_starts.push(base);
                let scope_base = self.scope_starts.len() - 1;
                self.run_call_frame(FrameSource::Func(func.clone()), scope_base)
            }
            VmValue::Closure {
                func,
                captured,
                capture_start,
            } => {
                if args.len() != func.arity {
                    return Err(self.err(format!(
                        "closure expects {} args, got {}",
                        func.arity,
                        args.len()
                    )));
                }
                let base = self.locals.len();
                self.locals
                    .resize(base + (*capture_start) as usize, VmValue::Null);
                self.locals.extend_from_slice(captured);
                let params_start = base + (*capture_start) as usize + captured.len();
                self.locals.resize(params_start + args.len(), VmValue::Null);
                for (i, arg) in args.iter().enumerate() {
                    self.locals[params_start + i] = arg.clone();
                }
                self.scope_starts.push(base);
                let scope_base = self.scope_starts.len() - 1;
                self.run_call_frame(FrameSource::Func(func.clone()), scope_base)
            }
            other => Err(self.err(format!("cannot call {other:?}"))),
        }
    }

    /// Executes a single fresh call frame to completion, returning the value
    /// its `Return` left on the stack. Used by [`Vm::invoke_callable`] to
    /// run a user function / closure synchronously without routing through
    /// the `Call` opcode.
    fn run_call_frame(
        &mut self,
        source: FrameSource<'_>,
        scope_base: usize,
    ) -> Result<VmValue, VmError> {
        let frames = vec![CallFrame {
            source,
            ip: 0,
            scope_base,
            frame_base: self.scope_starts.get(scope_base).copied().unwrap_or(0),
        }];
        self.run_frames(frames)?;
        Ok(self.stack.pop().unwrap_or(VmValue::Null))
    }

    /// Vm entry function
    pub fn run(&mut self, chunk: &Chunk) -> Result<(), VmError> {
        let frames = vec![CallFrame {
            source: FrameSource::Top(chunk),
            ip: 0,
            scope_base: self.scope_starts.len(),
            frame_base: 0,
        }];
        self.run_frames(frames)
    }

    /// Runs a stack of call frames to completion.
    ///
    /// The dispatch loop lives here so it can be reused both for whole
    /// programs (`run`) and for synchronous callable invocations
    /// (`call_value`), which avoids constructing a synthetic chunk on every
    /// higher-order callback call.
    fn run_frames(&mut self, mut frames: Vec<CallFrame>) -> Result<(), VmError> {
        let mut cur_chunk: *const Chunk = frames[0].source.chunk();
        self.current_chunk = cur_chunk;
        let mut ip: usize = 0;
        let mut scope_base: usize = frames[0].scope_base;
        // `self.scope_starts[scope_base]` - the locals base of the current
        // call frame. Constant for the frame's lifetime (only ever written by
        // the frame's first `PushScope`), so cache it instead of re-indexing
        // the scope table on every local access.
        let mut frame_base: usize = frames[0].frame_base;

        macro_rules! chunk {
            () => {
                unsafe { &*cur_chunk }
            };
        }
        macro_rules! read_u16 {
            () => {{
                // SAFETY: `ip` and `ip + 1` index the current chunk's operand
                // bytes, which the compiler always writes in full pairs (see
                // `Chunk::write_u16`), so both are in bounds here.
                unsafe { (&*cur_chunk).read_u16_unchecked(ip) }
            }};
        }

        loop {
            if ip >= chunk!().code.len() {
                self.stack.push(VmValue::Null);
                frames.last_mut().unwrap().ip = ip;
                if !self.finish_call(&mut frames)? {
                    return Ok(());
                }
                let top = frames.last().unwrap();
                cur_chunk = top.source.chunk();
                self.current_chunk = cur_chunk;
                ip = top.ip;
                scope_base = top.scope_base;
                frame_base = top.frame_base;
                continue;
            }

            // SAFETY: `ip < code.len()` was checked above and the bytecode is
            // compiler-emitted, so the byte is a valid opcode (see
            // `OpCode::from_u8_unchecked`).
            let op = unsafe { OpCode::from_u8_unchecked(*(&*cur_chunk).code.as_ptr().add(ip)) };
            ip += 1;
            self.current_ip = ip - 1;

            match op {
                OpCode::Const => {
                    let idx = read_u16!() as usize;
                    ip += 2;
                    // SAFETY: `idx` was emitted by the compiler and always
                    // points at a valid constant slot.
                    let val = unsafe { (&*cur_chunk).constants.get_unchecked(idx).clone() };
                    self.stack.push(val);
                }

                OpCode::Add => binary_arith!(self, checked_add, +, "+"),
                OpCode::Sub => binary_arith!(self, checked_sub, -, "-"),
                OpCode::Mul => binary_arith!(self, checked_mul, *, "*"),
                OpCode::Div => self.binary_div()?,

                OpCode::Negate => {
                    let v = self.pop()?;
                    let out = match v {
                        VmValue::Int(n) => VmValue::Int(-n),
                        VmValue::SInt(n) => VmValue::SInt(-n),
                        VmValue::BSByte(n) => VmValue::BSByte(-n),
                        VmValue::SByte(n) => VmValue::SByte(-n),
                        VmValue::Float(n) => VmValue::Float(-n),
                        VmValue::SFloat(n) => VmValue::SFloat(-n),
                        other => return Err(self.err(format!("cannot negate {other:?}"))),
                    };
                    self.stack.push(out);
                }
                OpCode::Not => {
                    let v = self.pop()?;
                    let out = match v {
                        VmValue::Bool(b) => VmValue::Bool(!b),
                        other => return Err(self.err(format!("cannot apply ! to {other:?}"))),
                    };
                    self.stack.push(out);
                }
                OpCode::Eq => {
                    let (a, b) = self.pop_two_unchecked();
                    self.stack.push(VmValue::Bool(a == b));
                }
                OpCode::NotEq => {
                    let (a, b) = self.pop_two_unchecked();
                    self.stack.push(VmValue::Bool(a != b));
                }
                OpCode::Less => self.binary_cmp(|o| o.is_lt())?,
                OpCode::LessEq => self.binary_cmp(|o| o.is_le())?,
                OpCode::Greater => self.binary_cmp(|o| o.is_gt())?,
                OpCode::GreaterEq => self.binary_cmp(|o| o.is_ge())?,

                OpCode::GetLocal => {
                    let flat = read_u16!() as usize;
                    ip += 2;
                    let val = self.locals[frame_base + flat].clone();
                    self.stack.push(val);
                }
                OpCode::SetLocal => {
                    let flat = read_u16!() as usize;
                    ip += 2;
                    let val = self
                        .stack
                        .last()
                        .cloned()
                        .ok_or_else(|| self.err("stack underflow on assignment"))?;
                    self.locals[frame_base + flat] = val;
                }
                OpCode::GetGlobal => {
                    let slot = read_u16!() as usize;
                    ip += 2;
                    let val =
                        self.globals.get(slot).cloned().ok_or_else(|| {
                            self.err(format!("read of undefined global slot {slot}"))
                        })?;
                    self.stack.push(val);
                }
                OpCode::SetGlobal => {
                    let slot = read_u16!() as usize;
                    ip += 2;
                    let val = self
                        .stack
                        .last()
                        .cloned()
                        .ok_or_else(|| self.err("stack underflow on assignment"))?;
                    if slot >= self.globals.len() {
                        return Err(self.err(format!("assignment to undefined global slot {slot}")));
                    }
                    self.globals[slot] = val;
                }
                OpCode::DefineLocal => {
                    let slot = read_u16!() as usize;
                    ip += 2;
                    let val = self.pop()?;

                    if self.scope_starts.len() == scope_base {
                        if slot >= self.globals.len() {
                            self.globals.resize(slot + 1, VmValue::Null);
                        }
                        self.globals[slot] = val;
                    } else {
                        if frame_base + slot >= self.locals.len() {
                            self.locals.resize(frame_base + slot + 1, VmValue::Null);
                        }
                        self.locals[frame_base + slot] = val;
                    }
                }
                OpCode::Pop => {
                    self.pop()?;
                }

                OpCode::Return => {
                    let ret = self.pop().ok();
                    self.stack.push(ret.unwrap_or(VmValue::Null));
                    frames.last_mut().unwrap().ip = ip;
                    if !self.finish_call(&mut frames)? {
                        return Ok(());
                    }
                    let top = frames.last().unwrap();
                    cur_chunk = top.source.chunk();
                    self.current_chunk = cur_chunk;
                    ip = top.ip;
                    scope_base = top.scope_base;
                    frame_base = top.frame_base;
                }

                OpCode::PushScope => {
                    let base = self.locals.len();
                    self.scope_starts.push(base);
                    if self.scope_starts.len() - 1 == scope_base {
                        frame_base = base;
                    }
                }
                OpCode::PopScope => {
                    let num_active = self.scope_starts.len() - scope_base;
                    let min_active = if frames.len() > 1 { 1 } else { 0 };
                    if num_active <= min_active {
                        return Err(self.err("cannot pop the base call frame"));
                    }
                    let start = self.scope_starts.pop().unwrap();
                    self.locals.truncate(start);
                }

                OpCode::Jump => {
                    let offset = read_u16!() as usize;
                    ip += 2;
                    ip += offset;
                }
                OpCode::JumpIfFalse => {
                    let offset = read_u16!() as usize;
                    ip += 2;
                    match self.pop()? {
                        VmValue::Bool(false) => ip += offset,
                        VmValue::Bool(true) => {}
                        other => {
                            return Err(
                                self.err(format!("if/while condition must be bool, got {other:?}"))
                            );
                        }
                    }
                }
                OpCode::Loop => {
                    let offset = read_u16!() as usize;
                    ip += 2;
                    ip -= offset;
                }

                OpCode::Call => {
                    let arg_count = read_u16!() as usize;
                    ip += 2;

                    let callee_idx = self.stack.len() - 1 - arg_count;
                    match self.stack[callee_idx].clone() {
                        VmValue::Function(func) => {
                            let base = self.locals.len();
                            self.locals.resize(base + arg_count, VmValue::Null);
                            for i in (0..arg_count).rev() {
                                self.locals[base + i] = self.pop()?;
                            }
                            self.pop()?; // discard the callee itself

                            if arg_count != func.arity {
                                return Err(self.err(format!(
                                    "{} expects {} args, got {}",
                                    func.name, func.arity, arg_count
                                )));
                            }

                            self.scope_starts.push(base);
                            let new_scope_base = self.scope_starts.len() - 1;

                            frames.last_mut().unwrap().ip = ip;
                            cur_chunk = &func.chunk as *const Chunk;
                            self.current_chunk = cur_chunk;
                            frames.push(CallFrame {
                                source: FrameSource::Func(func),
                                ip: 0,
                                scope_base: new_scope_base,
                                frame_base: base,
                            });
                            ip = 0;
                            scope_base = new_scope_base;
                            frame_base = base;
                        }

                        VmValue::Native(native) => {
                            let mut call_args = Vec::with_capacity(arg_count);
                            for _ in 0..arg_count {
                                call_args.push(self.pop()?);
                            }
                            call_args.reverse();
                            self.pop()?; // discard the callee itself

                            let result = match native {
                                crate::values::VmNative::Std(h) => (h.thunk)(self, call_args, ()),
                                crate::values::VmNative::Legacy(f) => (f.func)(self, call_args),
                            }
                            .map_err(|e| self.annotate(e))?;
                            self.stack.push(result);
                        }

                        VmValue::Closure {
                            func,
                            captured,
                            capture_start,
                        } => {
                            if arg_count != func.arity {
                                return Err(self.err(format!(
                                    "closure expects {} args, got {}",
                                    func.arity, arg_count
                                )));
                            }
                            let base = self.locals.len();
                            self.locals
                                .resize(base + capture_start as usize, VmValue::Null);
                            self.locals.extend_from_slice(&captured);
                            let params_start = base + capture_start as usize + captured.len();
                            self.locals.resize(params_start + arg_count, VmValue::Null);
                            for i in (0..arg_count).rev() {
                                self.locals[params_start + i] = self.pop()?;
                            }
                            self.pop()?; // discard the closure itself

                            self.scope_starts.push(base);
                            let new_scope_base = self.scope_starts.len() - 1;

                            frames.last_mut().unwrap().ip = ip;
                            cur_chunk = &func.chunk as *const Chunk;
                            self.current_chunk = cur_chunk;
                            frames.push(CallFrame {
                                source: FrameSource::Func(func),
                                ip: 0,
                                scope_base: new_scope_base,
                                frame_base: base,
                            });
                            ip = 0;
                            scope_base = new_scope_base;
                            frame_base = base;
                        }

                        other => return Err(self.err(format!("cannot call {other:?}"))),
                    }
                }

                OpCode::Ok => {
                    let v = self.pop()?;
                    self.stack.push(VmValue::Ok(Box::new(v)));
                }

                OpCode::Err => {
                    let v = self.pop()?;
                    self.stack.push(VmValue::Err(Box::new(v)));
                }

                OpCode::Propagate => {
                    let v = self.pop()?;
                    match v {
                        VmValue::Ok(inner) => self.stack.push(*inner),
                        VmValue::Err(_) => {
                            self.stack.push(v);
                            frames.last_mut().unwrap().ip = ip;
                            if !self.finish_call(&mut frames)? {
                                return Ok(());
                            }
                            let top = frames.last().unwrap();
                            cur_chunk = top.source.chunk();
                            self.current_chunk = cur_chunk;
                            ip = top.ip;
                            scope_base = top.scope_base;
                            frame_base = top.frame_base;
                        }
                        other => self.stack.push(other),
                    }
                }

                OpCode::Error => {
                    let v = self.pop()?;
                    self.stack.push(VmValue::Error(Box::new(v)));
                }

                OpCode::BuildArr => {
                    let count = read_u16!() as usize;
                    ip += 2;
                    if self.stack.len() < count {
                        return Err(self.err("stack underflow building array"));
                    }
                    let items = self.stack.split_off(self.stack.len() - count);
                    self.stack.push(VmValue::Arr(Rc::new(items)));
                }

                OpCode::BuildTuple => {
                    let count = read_u16!() as usize;
                    ip += 2;
                    if self.stack.len() < count {
                        return Err(self.err("stack underflow building tuple"));
                    }
                    let items = self.stack.split_off(self.stack.len() - count);
                    self.stack.push(VmValue::Tuple(Rc::new(items)));
                }

                OpCode::Index => {
                    let index = self.pop()?;
                    let arr = self.pop()?;
                    let elem = self.index_get(&arr, &index)?;
                    self.stack.push(elem);
                }

                OpCode::ArrLen => {
                    let arr = self.pop()?;
                    match arr {
                        VmValue::Arr(items) => self.stack.push(VmValue::Int(items.len() as i64)),
                        other => {
                            return Err(
                                self.err(format!("cannot get length of {}", other.type_name()))
                            );
                        }
                    }
                }

                OpCode::ArrSet => {
                    let value = self.pop()?;
                    let index = self.pop()?;
                    let arr = self.pop()?;
                    let updated = self.index_set(arr, &index, value)?;
                    self.stack.push(updated);
                }

                OpCode::BuildSet => {
                    let count = read_u16!() as usize;
                    ip += 2;
                    if self.stack.len() < count {
                        return Err(self.err("stack underflow building set"));
                    }
                    let items = self.stack.split_off(self.stack.len() - count);
                    let mut set = HashSet::with_capacity(count);
                    for v in items {
                        let key = VmMapKey::from_value(&v).ok_or_else(|| {
                            self.err(format!("type {} cannot be a set element", v.type_name()))
                        })?;
                        set.insert(key);
                    }
                    self.stack.push(VmValue::Set(Rc::new(RefCell::new(set))));
                }

                OpCode::BuildMap => {
                    let count = read_u16!() as usize; // number of entries
                    ip += 2;
                    if self.stack.len() < count * 2 {
                        return Err(self.err("stack underflow building map"));
                    }
                    let flat = self.stack.split_off(self.stack.len() - count * 2);
                    let mut map = HashMap::with_capacity(count);
                    for pair in flat.as_chunks::<2>().0 {
                        let key = VmMapKey::from_value(&pair[0]).ok_or_else(|| {
                            self.err(format!(
                                "type {} cannot be used as a map key",
                                pair[0].type_name()
                            ))
                        })?;
                        map.insert(key, pair[1].clone());
                    }
                    self.stack.push(VmValue::Map(Rc::new(RefCell::new(map))));
                }

                OpCode::BuildRecord => {
                    let name_idx = read_u16!() as usize;
                    ip += 2;
                    let fields_idx = read_u16!() as usize;
                    ip += 2;
                    let count = read_u16!() as usize;
                    ip += 2;

                    let VmValue::Str(name) = chunk!().constants[name_idx].clone() else {
                        return Err(self.err("corrupt bytecode: struct name is not a string"));
                    };
                    let VmValue::Arr(field_names) = chunk!().constants[fields_idx].clone() else {
                        return Err(self.err("corrupt bytecode: struct field list is not an array"));
                    };
                    if self.stack.len() < count {
                        return Err(self.err("stack underflow building struct"));
                    }
                    let values = self.stack.split_off(self.stack.len() - count);
                    let fields = field_names
                        .iter()
                        .zip(values)
                        .map(|(fname, val)| {
                            let VmValue::Str(fname) = fname else {
                                unreachable!("field name constant must be a string");
                            };
                            (fname.clone(), val)
                        })
                        .collect();
                    self.stack.push(VmValue::Record {
                        name,
                        fields: RecordFields::new(fields),
                    });
                }

                OpCode::FieldGet => {
                    let field_idx = read_u16!() as usize;
                    ip += 2;
                    let VmValue::Str(field) = chunk!().constants[field_idx].clone() else {
                        return Err(self.err("corrupt bytecode: field name is not a string"));
                    };
                    let target = self.pop()?;
                    let VmValue::Record { name, fields } = &target else {
                        return Err(self.err(format!(
                            "cannot access field `{}` on {}",
                            field,
                            target.type_name()
                        )));
                    };
                    let value = fields.get(&field).ok_or_else(|| {
                        self.err(format!("record `{}` has no field `{}`", name, field))
                    })?;
                    self.stack.push(value);
                }

                OpCode::FieldSet => {
                    let field_idx = read_u16!() as usize;
                    ip += 2;
                    let VmValue::Str(field) = chunk!().constants[field_idx].clone() else {
                        return Err(self.err("corrupt bytecode: field name is not a string"));
                    };
                    let value = self.pop()?;
                    let target = self.pop()?;
                    let VmValue::Record { name, fields } = &target else {
                        return Err(self.err(format!(
                            "cannot assign field `{}` on {}",
                            field,
                            target.type_name()
                        )));
                    };
                    if !fields.has(&field) {
                        return Err(self.err(format!("record `{}` has no field `{}`", name, field)));
                    }
                    fields.set(&field, value.clone());
                    self.stack.push(value);
                }
                OpCode::BuildClosure => {
                    let const_idx = read_u16!() as usize;
                    ip += 2;
                    let capture_start = read_u16!();
                    ip += 2;
                    let VmValue::Function(func) = chunk!().constants[const_idx].clone() else {
                        return Err(
                            self.err("corrupt bytecode: closure template is not a function")
                        );
                    };

                    let captured = if self.scope_starts.len() == scope_base {
                        Vec::new()
                    } else {
                        let frame_base = self.scope_starts[scope_base];
                        let start = frame_base + capture_start as usize;
                        if start > self.locals.len() {
                            return Err(self.err(format!(
                                "corrupt bytecode: closure capture_start {capture_start} exceeds live locals ({} available)",
                                self.locals.len() - frame_base
                            )));
                        }
                        self.locals[start..].to_vec()
                    };

                    self.stack.push(VmValue::Closure {
                        func,
                        captured: Rc::new(captured),
                        capture_start,
                    });
                }

                OpCode::RegisterMethod => {
                    let key_idx = read_u16!() as usize;
                    ip += 2;
                    let func_idx = read_u16!() as usize;
                    ip += 2;

                    let VmValue::Str(key) = chunk!().constants[key_idx].clone() else {
                        return Err(self.err("corrupt bytecode: method key is not a string"));
                    };
                    let VmValue::Function(func) = chunk!().constants[func_idx].clone() else {
                        return Err(self.err("corrupt bytecode: method body is not a function"));
                    };
                    self.impl_methods.insert(key.to_string(), func);
                }

                OpCode::RegisterStdlibMethod => {
                    let key_idx = read_u16!() as usize;
                    ip += 2;
                    let value_idx = read_u16!() as usize;
                    ip += 2;

                    let VmValue::Str(key) = chunk!().constants[key_idx].clone() else {
                        return Err(self.err("corrupt bytecode: method key is not a string"));
                    };
                    let VmValue::Native(native) = chunk!().constants[value_idx].clone() else {
                        return Err(self.err("corrupt bytecode: stdlib fallback is not a native"));
                    };
                    self.stdlib_methods.insert(key.to_string(), native);
                }

                OpCode::RegisterUserMethod => {
                    let key_idx = read_u16!() as usize;
                    ip += 2;
                    let func_idx = read_u16!() as usize;
                    ip += 2;

                    let VmValue::Str(key) = chunk!().constants[key_idx].clone() else {
                        return Err(self.err("corrupt bytecode: method key is not a string"));
                    };
                    let VmValue::Function(func) = chunk!().constants[func_idx].clone() else {
                        return Err(
                            self.err("corrupt bytecode: user method body is not a function")
                        );
                    };
                    self.user_methods.insert(key.to_string(), func);
                }

                OpCode::LookupAssoc => {
                    let key_idx = read_u16!() as usize;
                    ip += 2;

                    let VmValue::Str(key) = chunk!().constants[key_idx].clone() else {
                        return Err(self.err("corrupt bytecode: method key is not a string"));
                    };
                    let func = self
                        .impl_methods
                        .get(&*key)
                        .cloned()
                        .ok_or_else(|| self.err(format!("undefined function {key}")))?;
                    self.stack.push(VmValue::Function(func));
                }

                OpCode::LookupMethod => {
                    let name_idx = read_u16!() as usize;
                    ip += 2;

                    let VmValue::Str(method) = chunk!().constants[name_idx].clone() else {
                        return Err(self.err("corrupt bytecode: method name is not a string"));
                    };
                    let caller = self
                        .stack
                        .last()
                        .ok_or_else(|| self.err("stack underflow on method call"))?
                        .clone();
                    let insert_pos = self.stack.len() - 1;

                    // Dispatch order mirrors the interpreter's `MethodCall`
                    // handler (`evaluator.rs`): a record's `impl` method wins,
                    // then the method name is resolved as a free function with
                    // the receiver as its first argument - imported stdlib
                    // functions first, then named user functions.
                    let resolved: Option<VmValue> = match &caller {
                        VmValue::Record { name, .. } => self
                            .impl_methods
                            .get(&format!("{name}::{method}"))
                            .map(|func| VmValue::Function(func.clone())),
                        _ => None,
                    }
                    .or_else(|| {
                        self.stdlib_methods
                            .get(&*method)
                            .map(|n| VmValue::Native(n.clone()))
                            .or_else(|| {
                                self.user_methods
                                    .get(&*method)
                                    .map(|f| VmValue::Function(f.clone()))
                            })
                    });

                    match resolved {
                        Some(callee) => {
                            self.stack.insert(insert_pos, callee);
                        }
                        None => match &caller {
                            VmValue::Record { name, .. } => {
                                return Err(
                                    self.err(format!("record `{name}` has no method `{method}`"))
                                );
                            }
                            other => {
                                return Err(self.err(format!(
                                    "cannot call method `{method}` on {}",
                                    other.type_name()
                                )));
                            }
                        },
                    }
                }

                OpCode::Cast => {
                    let code = read_u16!() as usize;
                    ip += 2;
                    let value = self.pop()?;
                    let cast = self.cast(value, code)?;
                    self.stack.push(cast);
                }

                OpCode::IsKind => {
                    let kind = read_u16!();
                    ip += 2;
                    let name_idx = read_u16!() as usize;
                    ip += 2;
                    let inner_kind = read_u16!();
                    ip += 2;
                    let inner_name_idx = read_u16!() as usize;
                    ip += 2;
                    let value = self.pop()?;
                    let name = match &chunk!().constants[name_idx] {
                        VmValue::Str(s) => s.clone(),
                        _ => {
                            return Err(self.err("corrupt bytecode: is-kind name is not a string"));
                        }
                    };
                    let inner_name = match &chunk!().constants[inner_name_idx] {
                        VmValue::Str(s) => s.clone(),
                        _ => {
                            return Err(self.err("corrupt bytecode: is-kind name is not a string"));
                        }
                    };
                    let matched =
                        Self::value_is_kind(&value, kind, &name, inner_kind, &inner_name);
                    self.stack.push(VmValue::Bool(matched));
                }
            }
        }
    }

    fn finish_call(&mut self, frames: &mut Vec<CallFrame>) -> Result<bool, VmError> {
        let finished = frames.pop().expect("frame stack must not be empty");
        if finished.scope_base < self.scope_starts.len() {
            let cut = self.scope_starts[finished.scope_base];
            self.scope_starts.truncate(finished.scope_base);
            self.locals.truncate(cut);
        }
        Ok(!frames.is_empty())
    }

    fn index_as_usize(&self, index: &VmValue, len: usize) -> Result<usize, VmError> {
        let i = match index {
            VmValue::Int(n) if *n < 0_i64 => {
                return Err(self.err(format!("array index must be postive int, got {}", n)));
            }
            VmValue::Int(n) => *n as usize,
            VmValue::Byte(b) => *b as usize,
            VmValue::UInt(u) => *u as usize,
            other => {
                return Err(self.err(format!(
                    "array index must be int or byte, got {}",
                    other.type_name()
                )));
            }
        };
        if i >= len {
            return Err(self.err(format!("array index out of bounds: {i} (len {len})")));
        }
        Ok(i)
    }

    fn index_get(&self, arr: &VmValue, index: &VmValue) -> Result<VmValue, VmError> {
        match arr {
            VmValue::Arr(items) | VmValue::Tuple(items) => {
                let i = self.index_as_usize(index, items.len())?;
                Ok(items[i].clone())
            }
            VmValue::Map(entries) => {
                let key = VmMapKey::from_value(index).ok_or_else(|| {
                    self.err(format!(
                        "type {} cannot be used as a map key",
                        index.type_name()
                    ))
                })?;
                entries
                    .borrow()
                    .get(&key)
                    .cloned()
                    .ok_or_else(|| self.err(format!("key {} not found in map", index)))
            }
            other => Err(self.err(format!("cannot index into {}", other.type_name()))),
        }
    }

    fn index_set(&self, arr: VmValue, index: &VmValue, value: VmValue) -> Result<VmValue, VmError> {
        match arr {
            VmValue::Arr(items) => {
                let i = self.index_as_usize(index, items.len())?;
                let mut items = Rc::try_unwrap(items).unwrap_or_else(|rc| (*rc).clone());
                items[i] = value;
                Ok(VmValue::Arr(Rc::new(items)))
            }
            VmValue::Map(entries) => {
                let key = VmMapKey::from_value(index).ok_or_else(|| {
                    self.err(format!(
                        "type {} cannot be used as a map key",
                        index.type_name()
                    ))
                })?;
                entries.borrow_mut().insert(key, value);
                Ok(VmValue::Map(entries))
            }
            other => Err(self.err(format!("cannot index into {}", other.type_name()))),
        }
    }

    /// Runs `value as <type>` for the given numeric target `code`
    /// (see `CastTarget` in `compiler.rs`). Sources are widened to `i128`/`f64`,
    /// then narrowed via checked `try_from` into the target type.
    /// Runtime shape test for `OpCode::IsKind`, mirroring
    /// [`VmValue::type_name`] exactly (same variant mapping, so `is`
    /// and `__type_of` can never disagree). Containers test shape only;
    /// `result[T]` needs an `Ok` payload matching the inner kind.
    fn value_is_kind(
        value: &VmValue,
        kind: u16,
        name: &str,
        inner_kind: u16,
        inner_name: &str,
    ) -> bool {
        match kind {
            0 => matches!(value, VmValue::Null),
            1 => matches!(value, VmValue::Int(_)),
            2 => matches!(value, VmValue::UInt(_)),
            3 => matches!(value, VmValue::SInt(_)),
            4 => matches!(value, VmValue::SUInt(_)),
            5 => matches!(value, VmValue::Float(_)),
            6 => matches!(value, VmValue::SFloat(_)),
            7 => matches!(value, VmValue::Bool(_)),
            8 => matches!(value, VmValue::Str(_)),
            9 => matches!(value, VmValue::Char(_)),
            10 => matches!(value, VmValue::Byte(_)),
            11 => matches!(value, VmValue::SByte(_)),
            12 => matches!(value, VmValue::BByte(_)),
            13 => matches!(value, VmValue::BSByte(_)),
            14 => matches!(value, VmValue::Arr(_)),
            15 => matches!(value, VmValue::Map(_)),
            16 => matches!(value, VmValue::Set(_)),
            17 => matches!(value, VmValue::Tuple(_)),
            18 => matches!(value, VmValue::Record { name: n, .. } if n.as_ref() == name),
            19 => matches!(value, VmValue::Tag { name: n, .. } if n.as_ref() == name),
            20 => matches!(
                value,
                VmValue::Function(_) | VmValue::Native(_) | VmValue::Closure { .. }
            ),
            21 => matches!(value, VmValue::Handle { .. }),
            22 => match value {
                VmValue::Ok(inner) => {
                    Self::value_is_kind(inner, inner_kind, inner_name, 0, "")
                }
                _ => false,
            },
            23 => matches!(value, VmValue::Error(_)),
            _ => false,
        }
    }

    fn cast(&self, value: VmValue, code: usize) -> Result<VmValue, VmError> {
        fn as_i128(v: &VmValue) -> Option<i128> {
            match v {
                VmValue::Int(n) => Some(*n as i128),
                VmValue::SInt(n) => Some(*n as i128),
                VmValue::SUInt(n) => Some(*n as i128),
                VmValue::Byte(n) => Some(*n as i128),
                VmValue::SByte(n) => Some(*n as i128),
                VmValue::BByte(n) => Some(*n as i128),
                VmValue::BSByte(n) => Some(*n as i128),
                VmValue::Float(f) => Some(*f as i128),
                VmValue::SFloat(f) => Some(*f as i128),
                _ => None,
            }
        }

        fn as_f64(v: &VmValue) -> Option<f64> {
            match v {
                VmValue::Int(n) => Some(*n as f64),
                VmValue::SInt(n) => Some(*n as f64),
                VmValue::SUInt(n) => Some(*n as f64),
                VmValue::Byte(n) => Some(*n as f64),
                VmValue::SByte(n) => Some(*n as f64),
                VmValue::BByte(n) => Some(*n as f64),
                VmValue::BSByte(n) => Some(*n as f64),
                VmValue::Float(f) => Some(*f),
                VmValue::SFloat(f) => Some(*f as f64),
                _ => None,
            }
        }

        let bad_cast = || {
            self.err(format!(
                "invalid cast: cannot cast {}:{} to {:?}",
                value.type_name(),
                value,
                cast_target_name(code)
            ))
        };

        match code {
            // Int
            0 => as_i128(&value)
                .map(|n| VmValue::Int(n as i64))
                .ok_or_else(bad_cast),
            // Float
            1 => as_f64(&value).map(VmValue::Float).ok_or_else(bad_cast),
            // UInt
            2 => as_i128(&value)
                .ok_or_else(bad_cast)
                .and_then(|n| u64::try_from(n).map(VmValue::UInt).map_err(|_| bad_cast())),
            // SFloat
            3 => as_f64(&value)
                .map(|f| VmValue::SFloat(f as f32))
                .ok_or_else(bad_cast),
            // SUInt
            4 => as_i128(&value)
                .ok_or_else(bad_cast)
                .and_then(|n| u32::try_from(n).map(VmValue::SUInt).map_err(|_| bad_cast())),
            // SInt
            5 => as_i128(&value)
                .ok_or_else(bad_cast)
                .and_then(|n| i32::try_from(n).map(VmValue::SInt).map_err(|_| bad_cast())),
            // BByte
            6 => as_i128(&value)
                .ok_or_else(bad_cast)
                .and_then(|n| u16::try_from(n).map(VmValue::BByte).map_err(|_| bad_cast())),
            // BSByte
            7 => as_i128(&value).ok_or_else(bad_cast).and_then(|n| {
                i16::try_from(n)
                    .map(VmValue::BSByte)
                    .map_err(|_| bad_cast())
            }),
            // Byte
            8 => as_i128(&value)
                .ok_or_else(bad_cast)
                .and_then(|n| u8::try_from(n).map(VmValue::Byte).map_err(|_| bad_cast())),
            // SByte
            9 => as_i128(&value)
                .ok_or_else(bad_cast)
                .and_then(|n| i8::try_from(n).map(VmValue::SByte).map_err(|_| bad_cast())),
            other => Err(self.err(format!("corrupt bytecode: unknown cast target {other}"))),
        }
    }

    #[inline(always)]
    fn pop_unchecked(&mut self) -> VmValue {
        debug_assert!(!self.stack.is_empty(), "stack underflow");
        let new_len = self.stack.len() - 1;
        unsafe {
            self.stack.set_len(new_len);
            std::ptr::read(self.stack.as_ptr().add(new_len))
        }
    }
    #[inline(always)]
    fn pop_two_unchecked(&mut self) -> (VmValue, VmValue) {
        let b = self.pop_unchecked();
        let a = self.pop_unchecked();
        (a, b)
    }

    /// Helper functions that wraps the Vec::pop to return valid VmError or VmValue
    fn pop(&mut self) -> Result<VmValue, VmError> {
        self.stack.pop().ok_or_else(|| self.err("stack underflow"))
    }

    /// Helper function for arth operations
    /// handles /
    fn binary_div(&mut self) -> Result<(), VmError> {
        let (a, b) = self.pop_two_unchecked();
        macro_rules! int_div {
            ($self:expr, $variant:ident, $a:expr, $b:expr) => {{
                if $b == 0 {
                    return Err($self.err("division by zero"));
                }
                VmValue::$variant(
                    $a.checked_div($b)
                        .ok_or_else(|| $self.err(format!("integer overflow: {} / {}", $a, $b)))?,
                )
            }};
        }
        let out = match (a, b) {
            (VmValue::Int(a), VmValue::Int(b)) => int_div!(self, Int, a, b),
            (VmValue::UInt(a), VmValue::UInt(b)) => int_div!(self, UInt, a, b),
            (VmValue::SInt(a), VmValue::SInt(b)) => int_div!(self, SInt, a, b),
            (VmValue::SUInt(a), VmValue::SUInt(b)) => int_div!(self, SUInt, a, b),
            (VmValue::BByte(a), VmValue::BByte(b)) => int_div!(self, BByte, a, b),
            (VmValue::BSByte(a), VmValue::BSByte(b)) => int_div!(self, BSByte, a, b),
            (VmValue::Byte(a), VmValue::Byte(b)) => int_div!(self, Byte, a, b),
            (VmValue::SByte(a), VmValue::SByte(b)) => int_div!(self, SByte, a, b),
            (VmValue::Float(a), VmValue::Float(b)) => VmValue::Float(a / b),
            (VmValue::SFloat(a), VmValue::SFloat(b)) => VmValue::SFloat(a / b),
            (a, b) => return Err(self.err(format!("cannot divide {a:?} by {b:?}"))),
        };
        self.stack.push(out);
        Ok(())
    }

    /// Helper function for comparsion operations
    /// accepts every numeric VmValue variant (ints [Int, UInt, SInt, SUInt,
    /// BByte, BSByte, Byte, SByte], floats [Float, SFloat])
    /// handles >, <, >=, <=
    fn binary_cmp(&mut self, pred: fn(std::cmp::Ordering) -> bool) -> Result<(), VmError> {
        let (a, b) = self.pop_two_unchecked();
        let ord = match (&a, &b) {
            (VmValue::Int(a), VmValue::Int(b)) => a.partial_cmp(b),
            (VmValue::UInt(a), VmValue::UInt(b)) => a.partial_cmp(b),
            (VmValue::SInt(a), VmValue::SInt(b)) => a.partial_cmp(b),
            (VmValue::SUInt(a), VmValue::SUInt(b)) => a.partial_cmp(b),
            (VmValue::BByte(a), VmValue::BByte(b)) => a.partial_cmp(b),
            (VmValue::BSByte(a), VmValue::BSByte(b)) => a.partial_cmp(b),
            (VmValue::Byte(a), VmValue::Byte(b)) => a.partial_cmp(b),
            (VmValue::SByte(a), VmValue::SByte(b)) => a.partial_cmp(b),
            (VmValue::Float(a), VmValue::Float(b)) => a.partial_cmp(b),
            (VmValue::SFloat(a), VmValue::SFloat(b)) => a.partial_cmp(b),
            _ => return Err(self.err(format!("cannot compare {a:?} and {b:?}"))),
        }
        .ok_or_else(|| self.err("comparison produced no ordering (NaN?)"))?;
        self.stack.push(VmValue::Bool(pred(ord)));
        Ok(())
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

/// Maps a `CastTarget` code to its `TypeAnnotation` debug name, used in the
/// `invalid cast` error message (matches the interpreter's `{:?}` output).
fn cast_target_name(code: usize) -> &'static str {
    match code {
        0 => "Int",
        1 => "Float",
        2 => "UInt",
        3 => "SFloat",
        4 => "SUInt",
        5 => "SInt",
        6 => "BByte",
        7 => "BSByte",
        8 => "Byte",
        9 => "SByte",
        _ => "unknown",
    }
}
