use std::collections::HashMap;

use crate::{
    parsing::*,
    intermediate_representation::*
};

pub fn build_bytecode_at_assignment(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    value: &AbstractSyntaxNode
) {
    match value.item_ref() {
        AbstractSyntaxNodeItem::ProcedureCall { name, args, .. } => 
            build_bytecode_at_procedure_call_with_assignment(ir, assignment_map, assignment_name, name, args),
        AbstractSyntaxNodeItem::Literal(literal) => 
            build_bytecode_at_assignment_to_literal(ir, assignment_map, assignment_name, literal),
        _ => todo!()
    }
}

fn build_bytecode_at_assignment_to_literal(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    literal: &ResolvableLiteral
) {
    let offset = get_assignment_offset(assignment_map, assignment_name);
    
    let instruction = match get_resolved_literal(literal) {
        ResolvedLiteral::UnsignedInt8(_) => todo!("assignment to literal u8"),
        ResolvedLiteral::SignedInt8(_) => todo!("assignment to literal s8"),
        ResolvedLiteral::UnsignedInt16(_) => todo!("assignment to literal u16"),
        ResolvedLiteral::SignedInt16(_) => todo!("assignment to literal s16"),
        ResolvedLiteral::UnsignedInt32(value) => move_value_to_reg_plus_offset_32_instruction(value, base_pointer_register(), offset),
        ResolvedLiteral::SignedInt32(value) => move_value_to_reg_plus_offset_32_instruction(value as u32, base_pointer_register(), offset),
        ResolvedLiteral::UnsignedInt64(value) => move_value_to_reg_plus_offset_64_instruction(value, base_pointer_register(), offset),
        ResolvedLiteral::SignedInt64(value) => move_value_to_reg_plus_offset_64_instruction(value as u64, base_pointer_register(), offset),
        ResolvedLiteral::Float32(_) => todo!("assignment to literal float32"),
        ResolvedLiteral::Float64(_) => todo!("assignment to literal float64"),
        ResolvedLiteral::String(_) => todo!("assignment to literal string"),
    };

    add_byte_code(&mut ir.byte_code, instruction);
}

pub type AssignmentMap = HashMap<String, isize>;

pub fn get_assignment_map(statements: &AbstractSyntaxChildNodes) -> AssignmentMap {
    let mut position = 0;
    let mut assignment_map = AssignmentMap::default();
    
    for statement in statements {
        match statement.item_ref() {
            AbstractSyntaxNodeItem::Assignment { name, .. } => {
                position = position - 8;
                assignment_map.insert(name.clone(), position);
            }
        _ => {}
        }
    }

    assignment_map
}

pub fn get_full_assignment_storage_size(assignment_map: &AssignmentMap) -> u8 {
    (assignment_map.len() * 8) as u8
}

pub fn get_assignment_offset(assignment_map: &AssignmentMap, assignment_name: &str) -> u8 {
    if let Some(position) = assignment_map.get(assignment_name) {
        return *position as u8;
    }
    panic!("Assignment not found in map")
}
