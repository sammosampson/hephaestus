#![allow(dead_code)]

mod interpreter;
mod byte_code_runner;

use std::collections::*;

pub use interpreter::*;
pub use byte_code_runner::*;

use crate::{
    parsing::*,
    threading::*,
    intermediate_representation::*
};

pub type RunnableCompileTimeCodeLookup = HashMap<CompilationUnitId, Shareable<ByteCodeInstructionStream>>;

#[derive(Clone, Debug)]
pub struct RunnableCompileTimeCode {
    to_run_id: CompilationUnitId,
    blocks: RunnableCompileTimeCodeLookup
}

pub fn create_runnable_compile_time_code(to_run_id: CompilationUnitId) -> RunnableCompileTimeCode {
    RunnableCompileTimeCode {
        to_run_id,
        blocks: HashMap::default()
    }
}


pub fn get_code_block_to_run(code: &RunnableCompileTimeCode) -> Shareable<ByteCodeInstructionStream> {
    get_code_block(code, &code.to_run_id)
}

pub fn get_code_block(code: &RunnableCompileTimeCode, id: &CompilationUnitId) -> Shareable<ByteCodeInstructionStream> {
    code.blocks.get(id).unwrap().clone()
}
