use core::panic;
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
        AbstractSyntaxNodeItem::Identifier { name, ..} => 
            build_bytecode_at_assignment_to_identifier(ir, assignment_map, assignment_name, name),
        AbstractSyntaxNodeItem::Null =>  
            build_bytecode_at_assignment_to_null(ir, assignment_map, assignment_name),
        item => todo!("implementation needed for {:?}", item)
    }
}

fn build_bytecode_at_assignment_to_literal(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    literal: &ResolvableLiteral
) {
    let offset = get_assignment_offset(assignment_map, assignment_name);
    match get_resolved_literal(literal) {
        ResolvedLiteral::UnsignedInt8(_) => todo!("assignment to literal u8"),
        ResolvedLiteral::SignedInt8(_) => todo!("assignment to literal s8"),
        ResolvedLiteral::UnsignedInt16(_) => todo!("assignment to literal u16"),
        ResolvedLiteral::SignedInt16(_) => todo!("assignment to literal s16"),
        ResolvedLiteral::UnsignedInt32(value) => add_byte_code(&mut ir.byte_code, move_value_to_reg_plus_offset_32_instruction(value, base_pointer_register(), offset)),
        ResolvedLiteral::SignedInt32(value) => add_byte_code(&mut ir.byte_code, move_value_to_reg_plus_offset_32_instruction(value as u32, base_pointer_register(), offset)),
        ResolvedLiteral::UnsignedInt64(value) => add_byte_code(&mut ir.byte_code, move_value_to_reg_plus_offset_64_instruction(value, base_pointer_register(), offset)),
        ResolvedLiteral::SignedInt64(value) => add_byte_code(&mut ir.byte_code, move_value_to_reg_plus_offset_64_instruction(value as u64, base_pointer_register(), offset)),
        ResolvedLiteral::Float32(_) => todo!("assignment to literal float32"),
        ResolvedLiteral::Float64(_) => todo!("assignment to literal float64"),
        ResolvedLiteral::String(_) => println!("assignment to literal string"),
    };
}


fn build_bytecode_at_assignment_to_identifier(
    _ir: &mut IntermediateRepresentation,
    _assignment_map: &AssignmentMap,
    _assignment_name: &str,
    _name: &str
) {
    println!("build_bytecode_at_assignment_to_identifier: {}, {}", _assignment_name, _name);
}

fn build_bytecode_at_assignment_to_null(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str
) {
    let offset = get_assignment_offset(assignment_map, assignment_name);
    let instruction = move_value_to_reg_plus_offset_64_instruction(0, base_pointer_register(), offset);
    add_byte_code(&mut ir.byte_code, instruction);
}

pub type AssignmentMap = HashMap<String, isize>;

pub fn get_assignment_map(args: &AbstractSyntaxChildNodes, statements: &AbstractSyntaxChildNodes) -> AssignmentMap {
    let mut assignment_map = AssignmentMap::default();
    
    add_args_to_assignment_map(&mut assignment_map, args);
    add_statements_to_assignment_map(&mut assignment_map, statements);

    assignment_map
}

fn add_statements_to_assignment_map(assignment_map: &mut AssignmentMap, statements: &AbstractSyntaxChildNodes) {
    let mut position = 0;
    for statement in statements {
        match statement.item_ref() {
            AbstractSyntaxNodeItem::Assignment { name, .. } => {
                position = position - 8;
                assignment_map.insert(name.clone(), position);
            }
        _ => {}
        }
    }
}

fn add_args_to_assignment_map(assignment_map: &mut AssignmentMap, args: &AbstractSyntaxChildNodes) {
    let mut position = 0;
    for statement in args {
        match statement.item_ref() {
            AbstractSyntaxNodeItem::Declaration { name, .. } => {
                position = position + 8;
                assignment_map.insert(name.clone(), position);
            }
        _ => {}
        }
    }
}

pub fn get_full_assignment_storage_size(assignment_map: &AssignmentMap) -> u8 {
    let statement_body_assignments = assignment_map
        .values()
        .filter(| position | **position < 0)
        .count();
    (statement_body_assignments * 8) as u8
}

pub fn get_assignment_offset(assignment_map: &AssignmentMap, assignment_name: &str) -> u8 {
    if let Some(position) = assignment_map.get(assignment_name) {
        return *position as u8;
    } else {
        panic!("No offset found for assignment")
    }
}
