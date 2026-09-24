//! The `rl test` runner: discovers `!#[test]` functions, executes setup,
//! cases (inits/finals semantics unchanged), teardown, and reports
//! pass/fail/skip with counts for the exit code. `rl run` never executes
//! tests; this module is the only entry point that does.
//!
//! Property cases (`cases(N)`) generate inputs from parameter types and
//! refinements (plan #349): ints bounded (narrowed, never filtered),
//! floats, bools, strings, and arrays thereof; anything else errors
//! as explicitly out of scope. Failures shrink greedily to minimal
//! inputs. Generation is deterministic (fixed seed) so failures replay.

use rl_ast::{Ast, statements::Statement};
use rl_utils::source::SourceFile;
use rl_vm::{Compiler, TestTarget, Vm};

/// Outcome counts for one `rl test` run.
pub struct TestReport {
    pub ran: usize,
    pub ok: usize,
    pub failed: usize,
    pub skipped: usize,
    pub failures: Vec<String>,
}

/// Matches `--match` against the test name, group, register, or full name.
fn matches(target: &TestTarget, pattern: &str) -> bool {
    target.name.contains(pattern)
        || target
            .params
            .group
            .as_deref()
            .is_some_and(|g| g.contains(pattern))
        || target
            .params
            .register
            .as_deref()
            .is_some_and(|r| r.contains(pattern))
        || target_full_name(target).contains(pattern)
}

fn target_full_name(target: &TestTarget) -> String {
    match &target.params.group {
        Some(g) => format!("{g}::{}", target.name),
        None => target.name.clone(),
    }
}

/// Runs one zero-argument driver against `vm`, recording a hard failure
/// (and returning its message) when the driver itself raises.
fn run_driver(vm: &mut Vm, compiler: &mut Compiler, slot: u16, span: rl_utils::span::Span, context: &str) -> Option<String> {
    let driver = match compiler.compile_call(slot, span, false) {
        Ok(d) => d,
        Err(e) => return Some(format!("[{context}] compile error: {}", e.message())),
    };
    match vm.run_and_return(&driver) {
        Ok(_) => None,
        Err(e) => Some(format!("[{context}] error: {}", e.message())),
    }
}

/// Discovers, filters, and runs every test. Prints per-test verdicts plus a
/// summary; the caller maps `failed > 0` to a non-zero exit.
pub fn run_tests(
    source: &SourceFile,
    arena: &Ast,
    resolved: &[Statement],
    match_pattern: Option<&str>,
) -> TestReport {
    let plan = Compiler::scan_tests(resolved);
    let mut compiler = Compiler::new(arena).with_source_file(source.clone());

    // Definitions execute once; every case observes shared global state,
    // exactly like a running program.
    let mut vm = Vm::new().with_source_file(source.clone());
    let mut report = TestReport {
        ran: 0,
        ok: 0,
        failed: 0,
        skipped: 0,
        failures: Vec::new(),
    };
    let definitions = match compiler.compile_definitions(resolved) {
        Ok(d) => d,
        Err(e) => {
            e.report_to_stderr();
            std::process::exit(1);
        }
    };
    if let Err(e) = vm.run(&definitions) {
        e.report_to_stderr();
        std::process::exit(1);
    }

    for (slot, span, _) in &plan.inits {
        if let Some(msg) = run_driver(&mut vm, &mut compiler, *slot, *span, "init") {
            println!("{msg}");
            report.failures.push(msg);
            report.failed += 1;
            return report;
        }
    }

    for target in &plan.tests {
        if let Some(pat) = match_pattern
            && !matches(target, pat)
        {
            continue;
        }
        let full = target_full_name(target);
        report.ran += 1;
        if let Some(n) = target.params.cases {
            run_property_case(&mut vm, &mut compiler, &plan, target, &full, n, &mut report);
            continue;
        }
        // Attribute failing asserts to this case (cleared afterwards so
        // top-level code between cases stays unattributed).
        vm.test_state().current = Some(full.clone());
        let failures_before = vm.test_state().failures.len();
        let skipped_before = vm.test_state().skipped.len();
        let mut hard_error: Option<String> = None;
        for (slot, span) in &plan.setups {
            if let Some(msg) = run_driver(&mut vm, &mut compiler, *slot, *span, &full) {
                hard_error = Some(msg);
                break;
            }
        }
        if hard_error.is_none() {
            hard_error = run_driver(&mut vm, &mut compiler, target.slot, target.span, &full);
        }
        for (slot, span) in &plan.teardowns {
            if let Some(msg) = run_driver(&mut vm, &mut compiler, *slot, *span, &full) {
                hard_error = Some(msg);
                break;
            }
        }
        vm.test_state().current = None;
        let st = vm.test_state();
        if st.skipped.len() > skipped_before {
            report.skipped += 1;
            for entry in &st.skipped[skipped_before..] {
                println!("SKIP {entry}");
            }
        } else if hard_error.is_some() || st.failures.len() > failures_before {
            report.failed += 1;
            println!("FAIL {full}");
            if let Some(msg) = hard_error {
                // Surfaces again in the summary via st.failures.
                st.failures.push(msg);
            }
        } else {
            report.ok += 1;
            println!("ok {full}");
        }
    }

    for (slot, span, _) in &plan.finals {
        if let Some(msg) = run_driver(&mut vm, &mut compiler, *slot, *span, "final") {
            println!("{msg}");
            report.failures.push(msg);
            report.failed += 1;
        }
    }

    let st = vm.test_state();
    println!(
        "ran {} tests: {} ok, {} failed, {} skipped ({} assertions passed, {} failed)",
        report.ran, report.ok, report.failed, report.skipped, st.passed, st.failed
    );
    for failure in st.failures.clone() {
        println!("{failure}");
        report.failures.push(failure);
    }
    report
}

// ---- property generation (cases(N), #349) ---------------------------------

/// Deterministic xorshift64* stream: property inputs replay identically
/// across runs, so a reported failure reproduces bit-for-bit.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next() % n
        }
    }

    fn range_i64(&mut self, lo: i64, hi: i64) -> i64 {
        if lo >= hi {
            return lo;
        }
        let span = (hi as u64).wrapping_sub(lo as u64).wrapping_add(1);
        lo.wrapping_add(self.below(span) as i64)
    }
}

use rl_ast::statements::{ParamRefinement, RefineOp, RefineOperand, TypeAnnotation};
use rl_vm::VmValue;

/// Looks up an already-generated parameter value by name for
/// cross-parameter refinements (`balance: >=amt`).
fn bound_value<'a>(bound: &'a [(String, VmValue)], name: &str) -> Option<&'a VmValue> {
    bound.iter().find(|(n, _)| n == name).map(|(_, v)| v)
}

fn as_i64(v: &VmValue) -> Option<i64> {
    match v {
        VmValue::Int(i) => Some(*i),
        VmValue::UInt(u) => i64::try_from(*u).ok(),
        VmValue::SInt(i) => Some(*i as i64),
        VmValue::SUInt(u) => Some(*u as i64),
        VmValue::Byte(b) => Some(*b as i64),
        VmValue::SByte(b) => Some(*b as i64),
        VmValue::BByte(b) => Some(*b as i64),
        VmValue::BSByte(b) => Some(*b as i64),
        _ => None,
    }
}

/// Generates one input for a parameter type, narrowed (never filtered) by
/// its refinement. `bound` holds already-generated parameters for
/// cross-parameter bounds. Errors for types outside the initial scope
/// (maps, sets, tuples, records, chars, functions, handles, results).
fn generate_for_param(
    ty: &TypeAnnotation,
    refinement: Option<&ParamRefinement>,
    bound: &[(String, VmValue)],
    rng: &mut Rng,
) -> Result<VmValue, String> {
    const LO: i64 = -100;
    const HI: i64 = 100;
    // Equality refinements pin the value outright.
    if let Some(r) = refinement
        && r.op == RefineOp::Eq
    {
        match &r.operand {
            RefineOperand::Integer(v) => return num_from_ty(ty, *v),
            RefineOperand::Bool(b) => {
                if matches!(
                    ty,
                    TypeAnnotation::Bool | TypeAnnotation::CBool
                ) {
                    return Ok(VmValue::Bool(*b));
                }
            }
            RefineOperand::Str(s) => {
                if matches!(
                    ty,
                    TypeAnnotation::String | TypeAnnotation::CString
                ) {
                    return Ok(VmValue::Str(s.as_str().into()));
                }
            }
            RefineOperand::Param(_) => {}
        }
    }
    match ty {
        TypeAnnotation::Int
        | TypeAnnotation::CInt
        | TypeAnnotation::SInt
        | TypeAnnotation::CSInt
        | TypeAnnotation::Byte
        | TypeAnnotation::CByte
        | TypeAnnotation::SByte
        | TypeAnnotation::CSByte
        | TypeAnnotation::BByte
        | TypeAnnotation::CBByte
        | TypeAnnotation::BSByte
        | TypeAnnotation::CBSByte => {
            let (mut lo, mut hi) = (LO, HI);
            if let Some(r) = refinement {
                let bound_int = match &r.operand {
                    RefineOperand::Integer(v) => Some(*v),
                    RefineOperand::Param(p) => bound_value(bound, p).and_then(as_i64),
                    _ => None,
                };
                if let Some(v) = bound_int {
                    match r.op {
                        RefineOp::Gt => lo = lo.max(v.saturating_add(1)),
                        RefineOp::Ge => lo = lo.max(v),
                        RefineOp::Lt => hi = hi.min(v.saturating_sub(1)),
                        RefineOp::Le => hi = hi.min(v),
                        RefineOp::Eq => unreachable!(),
                        RefineOp::Ne => {}
                    }
                }
                if r.op == RefineOp::Ne
                    && let RefineOperand::Integer(v) = &r.operand
                {
                    for _ in 0..10 {
                        let candidate = num_from_ty(ty, rng.range_i64(lo, hi))?;
                        if as_i64(&candidate) != Some(*v) {
                            return Ok(candidate);
                        }
                    }
                }
            }
            if lo > hi {
                return Err(format!("empty generation range ({lo}..={hi})"));
            }
            num_from_ty(ty, rng.range_i64(lo, hi))
        }
        TypeAnnotation::UInt
        | TypeAnnotation::CUInt
        | TypeAnnotation::SUInt
        | TypeAnnotation::CSUInt => {
            let v = rng.range_i64(0, HI);
            num_from_ty(ty, v)
        }
        TypeAnnotation::Float | TypeAnnotation::CFloat | TypeAnnotation::SFloat | TypeAnnotation::CSFloat => {
            let v = rng.range_i64(-10000, 10000) as f64 / 100.0;
            match ty {
                TypeAnnotation::SFloat | TypeAnnotation::CSFloat => Ok(VmValue::SFloat(v as f32)),
                _ => Ok(VmValue::Float(v)),
            }
        }
        TypeAnnotation::Bool | TypeAnnotation::CBool => Ok(VmValue::Bool(rng.below(2) == 0)),
        TypeAnnotation::String | TypeAnnotation::CString => {
            const ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
            let len = rng.below(9) as usize;
            let s: String = (0..len).map(|_| ALPHA[rng.below(ALPHA.len() as u64) as usize] as char).collect();
            Ok(VmValue::Str(s.as_str().into()))
        }
        TypeAnnotation::Array(inner) | TypeAnnotation::CArray(inner) => {
            let len = rng.below(4) as usize;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(generate_for_param(inner, None, bound, rng)?);
            }
            Ok(VmValue::Arr(items.into()))
        }
        TypeAnnotation::Null => Ok(VmValue::Null),
        other => Err(format!("cannot generate inputs for type {other:?}")),
    }
}

/// Builds a numeric value of type `ty` holding `v` (wrapping on overflow,
/// matching unchecked generator semantics).
fn num_from_ty(ty: &TypeAnnotation, v: i64) -> Result<VmValue, String> {
    Ok(match ty {
        TypeAnnotation::Int | TypeAnnotation::CInt => VmValue::Int(v),
        TypeAnnotation::UInt | TypeAnnotation::CUInt => VmValue::UInt(v as u64),
        TypeAnnotation::SInt | TypeAnnotation::CSInt => VmValue::SInt(v as i32),
        TypeAnnotation::SUInt | TypeAnnotation::CSUInt => VmValue::SUInt(v as u32),
        TypeAnnotation::Byte | TypeAnnotation::CByte => VmValue::Byte(v as u8),
        TypeAnnotation::SByte | TypeAnnotation::CSByte => VmValue::SByte(v as i8),
        TypeAnnotation::BByte | TypeAnnotation::CBByte => VmValue::BByte(v as u16),
        TypeAnnotation::BSByte | TypeAnnotation::CBSByte => VmValue::BSByte(v as i16),
        _ => return Err(format!("not a numeric type: {ty:?}")),
    })
}

/// Greedy shrinking: each position tries smaller candidates until no
/// progress (cap 50 rounds). Strings halve, arrays drop half, numbers
/// move toward zero.
fn shrink_candidates(v: &VmValue) -> Vec<VmValue> {
    match v {
        VmValue::Int(i) => [0, i / 2, i - 1, i + 1]
            .into_iter()
            .filter(|c| *c != *i)
            .map(VmValue::Int)
            .collect(),
        VmValue::UInt(u) => [0, u / 2, u.saturating_sub(1)]
            .into_iter()
            .filter(|c| *c != *u)
            .map(VmValue::UInt)
            .collect(),
        VmValue::Float(f) => [0.0, f / 2.0]
            .into_iter()
            .filter(|c| *c != *f)
            .map(VmValue::Float)
            .collect(),
        VmValue::Bool(b) => vec![VmValue::Bool(!b)],
        VmValue::Str(s) => {
            let half = s.len() / 2;
            let mut out = vec![VmValue::Str("".into())];
            if half > 0 && half < s.len() {
                out.push(VmValue::Str(s[..half].into()));
            }
            out
        }
        VmValue::Arr(items) => {
            let mut out = vec![VmValue::Arr(std::rc::Rc::new(Vec::new()))];
            if items.len() > 1 {
                out.push(VmValue::Arr(std::rc::Rc::new(items[..items.len() / 2].to_vec())));
            }
            out
        }
        _ => Vec::new(),
    }
}

fn format_inputs(args: &[VmValue]) -> String {
    args.iter()
        .map(|v| format!("{v}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Runs one property case (`cases(n)`): generates `n` inputs, invokes the
/// case per input with setups/teardowns, shrinks the first failure to
/// minimal inputs. Assertion counts are snapshotted and restored so
/// iterations never pollute the totals: a failing property records one
/// summary failure with its minimal inputs.
#[allow(clippy::too_many_arguments)]
fn run_property_case(
    vm: &mut Vm,
    compiler: &mut Compiler,
    plan: &rl_vm::TestPlan,
    target: &TestTarget,
    full: &str,
    n: u64,
    report: &mut TestReport,
) {
    use rl_vm::VmValue;
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let st = vm.test_state();
    let (p0, f0, fl0, s0) = (st.passed, st.failed, st.failures.len(), st.skipped.len());
    vm.test_state().current = Some(full.to_string());

    // One iteration: generate, run setups + case + teardowns, report a
    // failure as Some((inputs, message)). Iteration noise is always
    // truncated back; the case records one summary failure at the end.
    let run_once = |vm: &mut Vm,
                        compiler: &mut Compiler,
                        args: &[VmValue]|
     -> Option<(Vec<VmValue>, String)> {
        let invoke = |vm: &mut Vm, compiler: &mut Compiler| -> Result<VmValue, String> {
            let driver = compiler
                .compile_call_with_args(target.slot, target.span, args.to_vec())
                .map_err(|e| e.message().to_string())?;
            vm.run_and_return(&driver).map_err(|e| e.message().to_string())
        };
        for (slot, span) in &plan.setups {
            if let Some(msg) = run_driver(vm, compiler, *slot, *span, full) {
                return Some((args.to_vec(), format!("setup failed: {msg}")));
            }
        }
        let failures_before = vm.test_state().failures.len();
        let outcome = invoke(vm, compiler);
        for (slot, span) in &plan.teardowns {
            if let Some(msg) = run_driver(vm, compiler, *slot, *span, full) {
                return Some((args.to_vec(), format!("teardown failed: {msg}")));
            }
        }
        let found = match outcome {
            Err(msg) => Some((args.to_vec(), msg)),
            Ok(VmValue::Bool(false)) => {
                Some((args.to_vec(), "case returned false".to_string()))
            }
            Ok(_) => {
                if vm.test_state().failures.len() > failures_before {
                    let msg = vm.test_state().failures.last().cloned().unwrap_or_default();
                    Some((args.to_vec(), msg))
                } else {
                    None
                }
            }
        };
        vm.test_state().failures.truncate(failures_before);
        found
    };

    let mut failing: Option<(Vec<VmValue>, String)> = None;
    for _ in 0..n {
        let mut bound = Vec::new();
        let mut args = Vec::new();
        let mut gen_error: Option<String> = None;
        for p in &target.fn_params {
            match generate_for_param(&p.param_type, p.refinement.as_ref(), &bound, &mut rng) {
                Ok(v) => {
                    bound.push((p.param_name.clone(), v.clone()));
                    args.push(v);
                }
                Err(e) => {
                    gen_error = Some(format!("cannot generate `{}`: {e}", p.param_name));
                    break;
                }
            }
        }
        if let Some(e) = gen_error {
            failing = Some((args, e));
            break;
        }
        if let Some(found) = run_once(vm, compiler, &args) {
            failing = Some(found);
            break;
        }
    }

    // Greedy shrinking toward minimal failing inputs (cap 50 rounds).
    if let Some((inputs, msg)) = failing {
        let mut best = inputs;
        let mut best_msg = msg;
        for _ in 0..50 {
            let mut progressed = false;
            for i in 0..best.len() {
                let mut trial = best.clone();
                let mut shrunk = false;
                for candidate in shrink_candidates(&best[i]) {
                    trial[i] = candidate;
                    if run_once(vm, compiler, &trial.clone()).is_some() {
                        best = trial.clone();
                        shrunk = true;
                        progressed = true;
                        break;
                    }
                }
                if shrunk {
                    break;
                }
            }
            if !progressed {
                break;
            }
        }
        best_msg = format!("{} (inputs: {})", best_msg, format_inputs(&best));
        // Restore iteration noise; record one summary failure.
        // Assert records already carry their context; driver and
        // generation errors need it added.
        if !best_msg.starts_with('[') {
            best_msg = format!("[{full}] {best_msg}");
        }
        let st = vm.test_state();
        st.passed = p0;
        st.failed = f0;
        st.failures.truncate(fl0);
        st.skipped.truncate(s0);
        st.current = None;
        let st = vm.test_state();
        st.failed += 1;
        st.failures.push(best_msg);
        report.failed += 1;
        println!("FAIL {full}");
    } else {
        // Passing iterations contribute their assertions; drop nothing.
        // Skips inside property bodies mark the whole case skipped.
        let skipped = vm.test_state().skipped.len() > s0;
        vm.test_state().current = None;
        if skipped {
            report.skipped += 1;
            let st = vm.test_state();
            for entry in &st.skipped[s0..] {
                println!("SKIP {entry}");
            }
        } else {
            report.ok += 1;
            println!("ok {full}");
        }
    }
}
