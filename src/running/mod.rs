mod virtual_machine;
mod byte_code_runner;

use std::collections::*;

pub use virtual_machine::*;
pub use byte_code_runner::*;

use crate::{parsing::CompilationUnitId, threading::Shareable, intermediate_representation::ByteCodeInstructionStream};

pub type RunnableCompileTimeCodeLookup = HashMap<CompilationUnitId, Shareable<ByteCodeInstructionStream>>;

#[derive(Clone, Debug)]
pub struct RunnableCompileTimeCode {
    to_run_id: CompilationUnitId,
    blocks: RunnableCompileTimeCodeLookup
}

pub fn get_code_block_to_run(code: &RunnableCompileTimeCode) -> Shareable<ByteCodeInstructionStream> {
    get_code_block(code, &code.to_run_id)
}

pub fn get_code_block(code: &RunnableCompileTimeCode, id: &CompilationUnitId) -> Shareable<ByteCodeInstructionStream> {
    code.blocks.get(id).unwrap().clone()
}
