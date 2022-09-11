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

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteCodeInstruction {
    None,
    R(usize),
    RVAL(i64),
    F(usize),
    FVAL(f64),
    CLF,
    MOV, MOVF,
    STI, STF, LDI, LDF,
    LII, LIF,
    PSH, POP,
    PSHF, POPF,
    ADD, SUB, MUL, DIV,
    FADD, FSUB, FMUL, FDIV,
    JNZ, JEZ, JGZ, JLZ, JMP,
    SHL, SHR,
    BAND, BOR, BNOT, BXOR,
    LAND, LOR, LNOT,
    HLT
}

pub type ByteCodeInstructionStream = Vec<ByteCodeInstruction>;
