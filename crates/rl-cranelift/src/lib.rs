use cranelift_codegen::ir::{AbiParam, InstBuilder, types};
use cranelift_codegen::isa;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

use rl_vm::{Chunk, OpCode, VmValue};

#[derive(Debug)]
pub struct CraneliftError(pub String);

pub fn run_chunk(chunk: &Chunk) -> Result<i64, CraneliftError> {
    let mut flag_builder = settings::builder();
    flag_builder
        .set("use_colocated_libcalls", "false")
        .map_err(|e| CraneliftError(e.to_string()))?;
    flag_builder
        .set("is_pic", "false")
        .map_err(|e| CraneliftError(e.to_string()))?;

    let isa_builder =
        isa::lookup(target_lexicon::Triple::host()).map_err(|e| CraneliftError(e.to_string()))?;
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .map_err(|e| CraneliftError(e.to_string()))?;

    let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    let mut module = JITModule::new(builder);

    let mut ctx = module.make_context();
    ctx.func.signature.returns.push(AbiParam::new(types::I64));

    let mut fn_builder_ctx = FunctionBuilderContext::new();
    {
        let mut fb = FunctionBuilder::new(&mut ctx.func, &mut fn_builder_ctx);
        let block = fb.create_block();
        fb.switch_to_block(block);
        fb.seal_block(block);

        let mut value_stack = Vec::new();
        let mut ip = 0usize;

        while ip < chunk.code.len() {
            let op = OpCode::from_u8_unchecked(chunk.code[ip]);
            ip += 1;

            match op {
                OpCode::Const => {
                    let idx = chunk.read_u16(ip) as usize;
                    ip += 2;
                    let n = int_const(&chunk.constants[idx])?;
                    value_stack.push(fb.ins().iconst(types::I64, n));
                }
                OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => {
                    let b = value_stack
                        .pop()
                        .ok_or_else(|| CraneliftError("stack underflow".into()))?;
                    let a = value_stack
                        .pop()
                        .ok_or_else(|| CraneliftError("stack underflow".into()))?;
                    let result = match op {
                        OpCode::Add => fb.ins().iadd(a, b),
                        OpCode::Sub => fb.ins().isub(a, b),
                        OpCode::Mul => fb.ins().imul(a, b),
                        OpCode::Div => fb.ins().sdiv(a, b),
                        _ => unreachable!(),
                    };
                    value_stack.push(result);
                }
                OpCode::Negate => {
                    let a = value_stack
                        .pop()
                        .ok_or_else(|| CraneliftError("stack underflow".into()))?;
                    value_stack.push(fb.ins().ineg(a));
                }
                OpCode::Return => {
                    let ret = value_stack
                        .pop()
                        .unwrap_or_else(|| fb.ins().iconst(types::I64, 0));
                    fb.ins().return_(&[ret]);
                    break;
                }
                other => {
                    return Err(CraneliftError(format!(
                        "--cranelift: unsupported opcode {:?} (only supports \
                         Const/Add/Sub/Mul/Div/Negate/Return over ints)",
                        other
                    )));
                }
            }
        }
        fb.finalize();
    }
    println!("{}", ctx.func.display());

    let func_id = module
        .declare_function("rl_main", Linkage::Export, &ctx.func.signature)
        .map_err(|e| CraneliftError(e.to_string()))?;
    module
        .define_function(func_id, &mut ctx)
        .map_err(|e| CraneliftError(e.to_string()))?;
    module.clear_context(&mut ctx);
    module
        .finalize_definitions()
        .map_err(|e| CraneliftError(e.to_string()))?;

    let code_ptr = module.get_finalized_function(func_id);
    let result = unsafe {
        let f: extern "C" fn() -> i64 = std::mem::transmute(code_ptr);
        f()
    };

    Ok(result)
}

fn int_const(v: &VmValue) -> Result<i64, CraneliftError> {
    match v {
        VmValue::Int(n) => Ok(*n),
        other => Err(CraneliftError(format!(
            "--cranelift: unsupported constant {:?}",
            other
        ))),
    }
}
