//! [`VmRuntime`] - the [`Runtime`] implementation binding the shared `rl-std`
//! stdlib to the bytecode VM's [`VmValue`] / [`Vm`].
//!
//! The VM does not thread a call span into native functions (`Span = ()`); it
//! re-anchors any error at the call site via [`Vm::annotate`] after the native
//! returns. Compound-value type annotations are ignored (the VM does not track
//! `items_type`).

use crate::values::{VmMapKey, VmValue};
use crate::vm_logic::Vm;
use rl_ast::statements::TypeAnnotation;
use rl_std_core::Runtime;
use rl_utils::errors::Error;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Zero-sized marker binding the shared stdlib to the VM.
pub struct VmRuntime;

impl Runtime for VmRuntime {
    type Value = VmValue;
    type Cx = Vm;
    type Span = ();

    fn error(cx: &Self::Cx, msg: impl Into<String>, _span: Self::Span) -> Error {
        // `Vm::err` anchors at the currently-executing instruction and attaches
        // source; the call-site `Vm::annotate` re-anchors identically.
        cx.err(msg)
    }

    fn as_i64(v: &Self::Value) -> Option<i64> {
        match v {
            VmValue::Int(x) => Some(*x),
            VmValue::UInt(x) => Some(*x as i64),
            VmValue::Byte(x) => Some(*x as i64),
            VmValue::SByte(x) => Some(*x as i64),
            VmValue::BByte(x) => Some(*x as i64),
            VmValue::BSByte(x) => Some(*x as i64),
            VmValue::SInt(x) => Some(*x as i64),
            VmValue::SUInt(x) => Some(*x as i64),
            _ => None,
        }
    }
    fn as_u64(v: &Self::Value) -> Option<u64> {
        if let VmValue::UInt(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_i32(v: &Self::Value) -> Option<i32> {
        if let VmValue::SInt(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_u32(v: &Self::Value) -> Option<u32> {
        if let VmValue::SUInt(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_i16(v: &Self::Value) -> Option<i16> {
        if let VmValue::BSByte(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_u16(v: &Self::Value) -> Option<u16> {
        if let VmValue::BByte(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_i8(v: &Self::Value) -> Option<i8> {
        if let VmValue::SByte(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_u8(v: &Self::Value) -> Option<u8> {
        if let VmValue::Byte(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_f64(v: &Self::Value) -> Option<f64> {
        match v {
            VmValue::Float(x) => Some(*x),
            VmValue::SFloat(x) => Some(*x as f64),
            VmValue::Int(x) => Some(*x as f64),
            VmValue::UInt(x) => Some(*x as f64),
            VmValue::Byte(x) => Some(*x as f64),
            VmValue::SByte(x) => Some(*x as f64),
            VmValue::BByte(x) => Some(*x as f64),
            VmValue::BSByte(x) => Some(*x as f64),
            VmValue::SInt(x) => Some(*x as f64),
            VmValue::SUInt(x) => Some(*x as f64),
            _ => None,
        }
    }
    fn as_f32(v: &Self::Value) -> Option<f32> {
        if let VmValue::SFloat(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_bool(v: &Self::Value) -> Option<bool> {
        if let VmValue::Bool(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_char(v: &Self::Value) -> Option<char> {
        if let VmValue::Char(x) = v {
            Some(*x)
        } else {
            None
        }
    }
    fn as_str(v: &Self::Value) -> Option<&str> {
        if let VmValue::Str(s) = v {
            Some(s)
        } else {
            None
        }
    }

    fn type_name(v: &Self::Value) -> &'static str {
        v.type_name()
    }
    fn display(v: &Self::Value) -> String {
        v.to_string()
    }
    fn is_callable(v: &Self::Value) -> bool {
        matches!(
            v,
            VmValue::Function(_) | VmValue::Native(_) | VmValue::Closure { .. }
        )
    }

    fn from_i64(x: i64) -> Self::Value {
        VmValue::Int(x)
    }
    fn from_u64(x: u64) -> Self::Value {
        VmValue::UInt(x)
    }
    fn from_i32(x: i32) -> Self::Value {
        VmValue::SInt(x)
    }
    fn from_u32(x: u32) -> Self::Value {
        VmValue::SUInt(x)
    }
    fn from_i16(x: i16) -> Self::Value {
        VmValue::BSByte(x)
    }
    fn from_u16(x: u16) -> Self::Value {
        VmValue::BByte(x)
    }
    fn from_i8(x: i8) -> Self::Value {
        VmValue::SByte(x)
    }
    fn from_u8(x: u8) -> Self::Value {
        VmValue::Byte(x)
    }
    fn from_f64(x: f64) -> Self::Value {
        VmValue::Float(x)
    }
    fn from_f32(x: f32) -> Self::Value {
        VmValue::SFloat(x)
    }
    fn from_bool(x: bool) -> Self::Value {
        VmValue::Bool(x)
    }
    fn from_char(x: char) -> Self::Value {
        VmValue::Char(x)
    }
    fn from_string(x: String) -> Self::Value {
        VmValue::Str(Rc::from(x.as_str()))
    }
    fn null() -> Self::Value {
        VmValue::Null
    }

    fn ok(v: Self::Value) -> Self::Value {
        VmValue::Ok(Box::new(v))
    }
    fn err(v: Self::Value) -> Self::Value {
        VmValue::Err(Box::new(v))
    }
    fn error_value(v: Self::Value) -> Self::Value {
        VmValue::Error(Box::new(v))
    }

    fn array(items: Vec<Self::Value>, _elem: TypeAnnotation) -> Self::Value {
        VmValue::Arr(Rc::new(items))
    }
    fn tuple(items: Vec<Self::Value>) -> Self::Value {
        VmValue::Tuple(Rc::new(items))
    }
    fn map(
        entries: Vec<(Self::Value, Self::Value)>,
        _key: TypeAnnotation,
        _val: TypeAnnotation,
    ) -> Self::Value {
        let mut m = HashMap::new();
        for (k, v) in entries {
            if let Some(key) = crate::values::VmMapKey::from_value(&k) {
                m.insert(key, v);
            }
        }
        VmValue::Map(Rc::new(RefCell::new(m)))
    }
    fn set(items: Vec<Self::Value>, _elem: TypeAnnotation) -> Self::Value {
        let mut s = HashSet::new();
        for item in items {
            if let Some(key) = crate::values::VmMapKey::from_value(&item) {
                s.insert(key);
            }
        }
        VmValue::Set(Rc::new(RefCell::new(s)))
    }
    fn as_array(v: &Self::Value) -> Option<(&[Self::Value], TypeAnnotation)> {
        match v {
            VmValue::Arr(items) => Some((&items[..], TypeAnnotation::Infer)),
            _ => None,
        }
    }
    fn as_tuple(v: &Self::Value) -> Option<&[Self::Value]> {
        match v {
            VmValue::Tuple(items) => Some(&items[..]),
            _ => None,
        }
    }
    fn as_ok_inner(v: &Self::Value) -> Option<Self::Value> {
        if let VmValue::Ok(b) = v {
            Some((**b).clone())
        } else {
            None
        }
    }
    fn as_err_inner(v: &Self::Value) -> Option<Self::Value> {
        if let VmValue::Err(b) = v {
            Some((**b).clone())
        } else {
            None
        }
    }
    fn as_error_inner(v: &Self::Value) -> Option<Self::Value> {
        if let VmValue::Error(b) = v {
            Some((**b).clone())
        } else {
            None
        }
    }
    fn as_set(v: &Self::Value) -> Option<(Vec<Self::Value>, TypeAnnotation)> {
        match v {
            VmValue::Set(rc) => Some((
                rc.borrow().iter().map(|k| k.clone().into_value()).collect(),
                TypeAnnotation::Infer,
            )),
            _ => None,
        }
    }
    fn as_map(
        v: &Self::Value,
    ) -> Option<(
        Vec<(Self::Value, Self::Value)>,
        TypeAnnotation,
        TypeAnnotation,
    )> {
        match v {
            VmValue::Map(rc) => Some((
                rc.borrow()
                    .iter()
                    .map(|(k, val)| (k.clone().into_value(), val.clone()))
                    .collect(),
                TypeAnnotation::Infer,
                TypeAnnotation::Infer,
            )),
            _ => None,
        }
    }
    fn is_valid_key(v: &Self::Value) -> bool {
        crate::values::VmMapKey::from_value(v).is_some()
    }
    fn keys_equal(a: &Self::Value, b: &Self::Value) -> bool {
        match (
            crate::values::VmMapKey::from_value(a),
            crate::values::VmMapKey::from_value(b),
        ) {
            (Some(x), Some(y)) => x == y,
            _ => false,
        }
    }
    fn values_equal(a: &Self::Value, b: &Self::Value) -> bool {
        a == b
    }

    fn set_insert(v: &Self::Value, item: &Self::Value) -> Option<bool> {
        match v {
            VmValue::Set(items) => {
                let key = VmMapKey::from_value(item)?;
                Some(items.borrow_mut().insert(key))
            }
            _ => None,
        }
    }
    fn set_remove(v: &Self::Value, item: &Self::Value) -> Option<bool> {
        match v {
            VmValue::Set(items) => {
                let key = VmMapKey::from_value(item)?;
                Some(items.borrow_mut().remove(&key))
            }
            _ => None,
        }
    }
    fn set_contains(v: &Self::Value, item: &Self::Value) -> Option<bool> {
        match v {
            VmValue::Set(items) => {
                let key = VmMapKey::from_value(item)?;
                Some(items.borrow().contains(&key))
            }
            _ => None,
        }
    }
    fn set_len(v: &Self::Value) -> Option<usize> {
        match v {
            VmValue::Set(items) => Some(items.borrow().len()),
            _ => None,
        }
    }
    fn set_element_type(v: &Self::Value) -> Option<TypeAnnotation> {
        match v {
            VmValue::Set(_) => Some(TypeAnnotation::Infer),
            _ => None,
        }
    }

    fn map_insert(v: &Self::Value, key: &Self::Value, value: &Self::Value) -> bool {
        match v {
            VmValue::Map(entries) => {
                if let Some(key) = VmMapKey::from_value(key) {
                    entries.borrow_mut().insert(key, value.clone());
                }
                true
            }
            _ => false,
        }
    }
    fn map_get(v: &Self::Value, key: &Self::Value) -> Option<Option<Self::Value>> {
        match v {
            VmValue::Map(entries) => {
                let key = VmMapKey::from_value(key)?;
                Some(entries.borrow().get(&key).cloned())
            }
            _ => None,
        }
    }
    fn map_remove(v: &Self::Value, key: &Self::Value) -> Option<Option<Self::Value>> {
        match v {
            VmValue::Map(entries) => {
                let key = VmMapKey::from_value(key)?;
                Some(entries.borrow_mut().remove(&key))
            }
            _ => None,
        }
    }
    fn map_contains(v: &Self::Value, key: &Self::Value) -> Option<bool> {
        match v {
            VmValue::Map(entries) => {
                let key = VmMapKey::from_value(key)?;
                Some(entries.borrow().contains_key(&key))
            }
            _ => None,
        }
    }
    fn map_len(v: &Self::Value) -> Option<usize> {
        match v {
            VmValue::Map(entries) => Some(entries.borrow().len()),
            _ => None,
        }
    }
    fn map_key_value_types(v: &Self::Value) -> Option<(TypeAnnotation, TypeAnnotation)> {
        match v {
            VmValue::Map(_) => Some((TypeAnnotation::Infer, TypeAnnotation::Infer)),
            _ => None,
        }
    }
    fn map_clear(v: &Self::Value) -> bool {
        match v {
            VmValue::Map(entries) => {
                entries.borrow_mut().clear();
                true
            }
            _ => false,
        }
    }
    fn map_for_each<F: FnMut(Self::Value, Self::Value)>(v: &Self::Value, mut f: F) -> bool {
        match v {
            VmValue::Map(entries) => {
                for (k, val) in entries.borrow().iter() {
                    f(k.clone().into_value(), val.clone());
                }
                true
            }
            _ => false,
        }
    }

    fn value_type(_v: &Self::Value) -> TypeAnnotation {
        // The VM does not track element types, so it never performs the
        // interpreter's container element-type checks.
        TypeAnnotation::Infer
    }
    fn types_compatible(_actual: &TypeAnnotation, _expected: &TypeAnnotation) -> bool {
        true
    }

    fn call_value(
        cx: &mut Self::Cx,
        callee: &Self::Value,
        args: &[Self::Value],
        _span: Self::Span,
    ) -> Result<Self::Value, Error> {
        cx.call_value(callee, args, rl_utils::span::Span::dummy())
    }
    fn callable_return_type(_v: &Self::Value) -> Option<TypeAnnotation> {
        None
    }

    fn rng(cx: &mut Self::Cx) -> &mut rl_std_core::Xoshiro256 {
        &mut cx.rng
    }
    fn test_state(cx: &mut Self::Cx) -> &mut rl_std_core::TestState<Self::Value> {
        &mut cx.test_state
    }
    fn output_buffer(cx: &mut Self::Cx) -> &mut Option<String> {
        &mut cx.output_buffer
    }
    fn user_args_offset(cx: &Self::Cx) -> usize {
        cx.user_args_offset
    }
    fn as_handle(v: &Self::Value, kind: rl_ast::statements::HandleKind) -> Option<u64> {
        match v {
            VmValue::Handle { kind: k, id } if *k == kind => Some(*id),
            _ => None,
        }
    }
    fn make_handle(kind: rl_ast::statements::HandleKind, id: u64) -> Self::Value {
        VmValue::Handle { kind, id }
    }
}

impl rl_std::net::NetStore for VmRuntime {
    fn net_insert(cx: &mut Vm, h: rl_std::net::NetHandle) -> u64 {
        let id = cx.net_next_handle;
        cx.net_next_handle += 1;
        cx.net_handles.insert(id, h);
        id
    }
    fn net_get(cx: &Vm, id: u64) -> Option<&rl_std::net::NetHandle> {
        cx.net_handles.get(&id)
    }
    fn net_get_mut(cx: &mut Vm, id: u64) -> Option<&mut rl_std::net::NetHandle> {
        cx.net_handles.get_mut(&id)
    }
    fn net_remove(cx: &mut Vm, id: u64) -> Option<rl_std::net::NetHandle> {
        cx.net_handles.remove(&id)
    }
}

impl rl_std::c::CStore for VmRuntime {
    fn c_insert(cx: &mut Vm, h: rl_std::c::CHandle) -> u64 {
        let id = cx.c_next_handle;
        cx.c_next_handle += 1;
        cx.c_handles.insert(id, h);
        id
    }
    fn c_get(cx: &Vm, id: u64) -> Option<&rl_std::c::CHandle> {
        cx.c_handles.get(&id)
    }
    fn c_get_mut(cx: &mut Vm, id: u64) -> Option<&mut rl_std::c::CHandle> {
        cx.c_handles.get_mut(&id)
    }
    fn c_remove(cx: &mut Vm, id: u64) -> Option<rl_std::c::CHandle> {
        cx.c_handles.remove(&id)
    }
}

impl rl_std::http::HttpStore for VmRuntime {
    fn http_insert(cx: &mut Vm, h: rl_std::http::HttpHandle) -> u64 {
        let id = cx.http_next_handle;
        cx.http_next_handle += 1;
        cx.http_handles.insert(id, h);
        id
    }
    fn http_get(cx: &Vm, id: u64) -> Option<&rl_std::http::HttpHandle> {
        cx.http_handles.get(&id)
    }
    fn http_get_mut(cx: &mut Vm, id: u64) -> Option<&mut rl_std::http::HttpHandle> {
        cx.http_handles.get_mut(&id)
    }
    fn http_remove(cx: &mut Vm, id: u64) -> Option<rl_std::http::HttpHandle> {
        cx.http_handles.remove(&id)
    }
}

impl rl_std::audio::AudioStore for VmRuntime {
    fn audio_insert(cx: &mut Vm, h: rl_std::audio::AudioHandle) -> u64 {
        let id = cx.audio_next_handle;
        cx.audio_next_handle += 1;
        cx.audio_handles.insert(id, h);
        id
    }
    fn audio_get(cx: &Vm, id: u64) -> Option<&rl_std::audio::AudioHandle> {
        cx.audio_handles.get(&id)
    }
    fn audio_get_mut(cx: &mut Vm, id: u64) -> Option<&mut rl_std::audio::AudioHandle> {
        cx.audio_handles.get_mut(&id)
    }
    fn audio_remove(cx: &mut Vm, id: u64) -> Option<rl_std::audio::AudioHandle> {
        cx.audio_handles.remove(&id)
    }
    fn audio_output_device(cx: &mut Vm) -> &mut Option<String> {
        &mut cx.audio_output_device
    }
    fn audio_master_volume(cx: &mut Vm) -> &mut f32 {
        &mut cx.audio_master_volume
    }
    fn audio_handles_values<'a>(
        cx: &'a Vm,
    ) -> Box<dyn Iterator<Item = &'a rl_std::audio::AudioHandle> + 'a> {
        Box::new(cx.audio_handles.values())
    }
}

impl rl_std::gui::GuiStore for VmRuntime {
    fn gui_handles(
        cx: &mut Vm,
    ) -> &mut std::collections::HashMap<u64, rl_std::gui::GuiHandle<VmValue>> {
        &mut cx.gui_handles
    }
    fn gui_handles_ref(
        cx: &Vm,
    ) -> &std::collections::HashMap<u64, rl_std::gui::GuiHandle<VmValue>> {
        &cx.gui_handles
    }
    fn gui_next_handle(cx: &mut Vm) -> &mut u64 {
        &mut cx.gui_next_handle
    }
    fn gui_quit_requested(cx: &mut Vm) -> &mut bool {
        &mut cx.gui_quit_requested
    }
}

impl rl_std::io::IoStore for VmRuntime {
    fn io_insert(cx: &mut Vm, h: rl_std::io::IoFileHandle) -> u64 {
        let id = cx.io_next_handle;
        cx.io_next_handle += 1;
        cx.io_handles.insert(id, h);
        id
    }
    fn io_get(cx: &Vm, id: u64) -> Option<&rl_std::io::IoFileHandle> {
        cx.io_handles.get(&id)
    }
    fn io_get_mut(cx: &mut Vm, id: u64) -> Option<&mut rl_std::io::IoFileHandle> {
        cx.io_handles.get_mut(&id)
    }
    fn io_remove(cx: &mut Vm, id: u64) -> Option<rl_std::io::IoFileHandle> {
        cx.io_handles.remove(&id)
    }
}

impl rl_std::core::BufStore for VmRuntime {
    fn buf_insert(cx: &mut Vm, buf: Vec<u8>) -> u64 {
        let id = cx.buf_next_handle;
        cx.buf_next_handle += 1;
        cx.buf_handles.insert(id, buf);
        id
    }
    fn buf_get(cx: &Vm, id: u64) -> Option<&Vec<u8>> {
        cx.buf_handles.get(&id)
    }
    fn buf_get_mut(cx: &mut Vm, id: u64) -> Option<&mut Vec<u8>> {
        cx.buf_handles.get_mut(&id)
    }
    fn buf_remove(cx: &mut Vm, id: u64) -> Option<Vec<u8>> {
        cx.buf_handles.remove(&id)
    }
}

/// One unit of worker-thread work: RL source plus the channel its
/// progress and outcome travel on.
struct PoolJob {
    source: String,
    out: std::sync::mpsc::Sender<rl_std::core::ThreadMsg>,
}

/// Process-wide worker pool: fixed threads (CPU count, at least 2),
/// bounded queue (256). Saturation fails the spawn loudly instead of
/// growing memory without limit.
struct ThreadPool {
    tx: std::sync::mpsc::SyncSender<PoolJob>,
}

fn global_pool() -> &'static ThreadPool {
    static POOL: std::sync::OnceLock<ThreadPool> = std::sync::OnceLock::new();
    POOL.get_or_init(|| {
        let size = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
            .max(2);
        let (tx, rx) = std::sync::mpsc::sync_channel::<PoolJob>(256);
        let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));
        for _ in 0..size {
            let rx = rx.clone();
            std::thread::spawn(move || pool_worker(rx));
        }
        ThreadPool { tx }
    })
}

fn source_hash(source: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut h);
    h.finish()
}

/// Pool worker loop: take jobs until the pool dies. Compiled chunks are
/// memoized per thread by source hash, so repeat spawns of the same
/// source skip lex through compile (running still executes every time).
fn pool_worker(
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<PoolJob>>>,
) {
    thread_local! {
        static CHUNKS: std::cell::RefCell<std::collections::HashMap<u64, crate::Chunk>> =
            std::cell::RefCell::new(std::collections::HashMap::new());
    }
    loop {
        let job = match rx.lock() {
            Ok(rx) => match rx.recv() {
                Ok(job) => job,
                Err(_) => break,
            },
            Err(_) => break,
        };
        run_pooled_job(&job.source, &job.out);
    }

    fn run_pooled_job(
        source: &str,
        out: &std::sync::mpsc::Sender<rl_std::core::ThreadMsg>,
    ) {
        use rl_utils::source::SourceFile;
        let compile = || -> Result<crate::Chunk, String> {
            let file = SourceFile::new("spawn", source.to_string());
            let tokens = match rl_lexer::tokenizer::Tokenizer::lex(file.clone()) {
                Ok(t) => t,
                Err(e) => return Err(format!("spawn: lex failed: {e:?}")),
            };
            let (ast, stmts) =
                match rl_parser::parser_logic::Parser::parse(tokens, file.clone()) {
                    Ok(p) => p,
                    Err(e) => return Err(format!("spawn: parse failed: {e:?}")),
                };
            let mut resolver = rl_resolver::Resolver::new();
            let resolved = resolver.resolve_program(ast, stmts);
            let checker_tokens = match rl_lexer::tokenizer::Tokenizer::lex(file.clone())
            {
                Ok(t) => t,
                Err(e) => return Err(format!("spawn: lex failed: {e:?}")),
            };
            let (checker_ast, checker_stmts) =
                match rl_parser::parser_logic::Parser::parse(checker_tokens, file) {
                    Ok(p) => p,
                    Err(e) => return Err(format!("spawn: parse failed: {e:?}")),
                };
            let mut checker = rl_checker::TypeChecker::new().with_ast_arena(checker_ast);
            let errors = checker.check(&checker_stmts);
            if !errors.is_empty() {
                return Err(format!("spawn: check failed: {:?}", errors[0]));
            }
            match crate::Compiler::new(&resolver.ast_arena).compile(&resolved) {
                Ok(c) => Ok(c),
                Err(e) => Err(format!("spawn: compile failed: {e:?}")),
            }
        };
        let outcome = CHUNKS.with(|cache| {
            use std::collections::hash_map::Entry;
            let mut cache = cache.borrow_mut();
            let chunk = match cache.entry(source_hash(source)) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => match compile() {
                    Ok(c) => v.insert(c),
                    Err(e) => return Err(e),
                },
            };
            // Borrowed across the run: safe because worker code cannot
            // re-enter this function (nested spawn is refused, and nothing
            // else touches the cache).
            let mut vm = crate::Vm::new();
            vm.output_buffer = Some(String::new());
            vm.worker_tx = Some(out.clone());
            let outcome = match vm.run_and_return(chunk) {
                Ok(v) => Ok(v.to_string()),
                Err(e) => Err(format!("spawn: runtime failed: {}", e.message())),
            };
            if let Some(buf) = vm.output_buffer.take().filter(|b| !b.is_empty()) {
                let _ = out.send(rl_std::core::ThreadMsg::Progress(buf));
            }
            outcome
        });
        let _ = out.send(rl_std::core::ThreadMsg::Done(outcome));
    }
}

impl rl_std::core::ThreadStore for VmRuntime {
    fn thread_spawn(cx: &mut Vm, source: String) -> Result<u64, String> {
        if cx.worker_tx.is_some() {
            return Err("__spawn: workers cannot spawn".to_string());
        }
        let id = cx.thread_next_handle;
        cx.thread_next_handle += 1;
        let (tx, rx) = std::sync::mpsc::channel();
        cx.thread_jobs.insert(id, rx);
        match global_pool().tx.try_send(PoolJob { source, out: tx }) {
            Ok(()) => Ok(id),
            Err(_) => {
                cx.thread_jobs.remove(&id);
                Err("__spawn: pool full".to_string())
            }
        }
    }
    fn thread_poll(cx: &mut Vm, id: u64) -> rl_std::core::ThreadPoll {
        use rl_std::core::{ThreadMsg, ThreadPoll};
        let Some(rx) = cx.thread_jobs.get(&id) else {
            return ThreadPoll::Unknown;
        };
        match rx.try_recv() {
            Ok(ThreadMsg::Progress(text)) => ThreadPoll::Message(text),
            Ok(ThreadMsg::Done(outcome)) => {
                cx.thread_jobs.remove(&id);
                ThreadPoll::Done(outcome)
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => ThreadPoll::Pending,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                cx.thread_jobs.remove(&id);
                ThreadPoll::Done(Err("spawn: worker thread died".to_string()))
            }
        }
    }
    fn thread_emit(cx: &mut Self::Cx, text: String) -> bool {
        match &cx.worker_tx {
            Some(tx) => tx.send(rl_std::core::ThreadMsg::Progress(text)).is_ok(),
            None => false,
        }
    }
    fn thread_is_worker(cx: &Self::Cx) -> bool {
        cx.worker_tx.is_some()
    }
}
