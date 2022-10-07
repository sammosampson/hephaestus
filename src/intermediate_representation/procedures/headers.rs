use crate::{
    intermediate_representation::*,
    utilities::*

};

pub fn build_bytecode_at_procedure_header(ir: &mut IntermediateRepresentation, name: &str) {
    ir.top_level_symbol = string(name);
}