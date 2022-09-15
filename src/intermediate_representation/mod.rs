mod assembler;

pub use assembler::*;

use crate::parsing::CompilationUnitId;

#[derive(Debug, Clone)]
pub struct IntermediateRepresentation {
    pub id: CompilationUnitId,
    pub byte_code: ByteCodeInstructionStream
}

pub fn create_intermediate_representation(id: CompilationUnitId, byte_code: ByteCodeInstructionStream) -> IntermediateRepresentation {
    IntermediateRepresentation { id, byte_code }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteCodeValue {
    Register(usize),
    S64(i64)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteCodeInstruction {
    AssignToNumericLiteral { to: ByteCodeValue, from: ByteCodeValue },
    Return
}

pub type ByteCodeInstructionStream = Vec<ByteCodeInstruction>;
