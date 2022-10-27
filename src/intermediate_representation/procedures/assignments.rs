use core::panic;
use std::collections::HashMap;

use crate::{
    parsing::*,
    typing::*,
    intermediate_representation::*
};

pub fn build_bytecode_at_variable_declaration(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    value: &AbstractSyntaxNode
) {
    match value.item_ref() {
        AbstractSyntaxNodeItem::ProcedureCall { name, args, .. } => 
            build_bytecode_at_variable_assignment_to_procedure_call(ir, assignment_map, assignment_name, name, args),
        AbstractSyntaxNodeItem::Literal(literal) => 
            build_bytecode_at_variable_assignment_to_literal(ir, assignment_map, assignment_name, literal),
        AbstractSyntaxNodeItem::Identifier { name, ..} => 
            build_bytecode_at_variable_assignment_to_identifier(ir, assignment_map, assignment_name, name),
        AbstractSyntaxNodeItem::Null =>  
            build_bytecode_at_variable_assignment_to_null(ir, assignment_map, assignment_name),
        AbstractSyntaxNodeItem::Cast { expr, .. } =>  
            build_bytecode_at_variable_declaration(ir, assignment_map, assignment_name, expr),
        AbstractSyntaxNodeItem::MemberExpr { instance, member, .. } =>  
            build_bytecode_at_variable_assignment_to_member_expr(ir, assignment_map, assignment_name, instance, member),
        item => todo!("implementation needed for {:?}", item)
    }
}


fn build_bytecode_at_variable_assignment_to_literal(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    literal: &ResolvableLiteral
) {
    let assignment_offset = get_assignment_offset(assignment_map, assignment_name);
    match get_resolved_literal(literal) {
        ResolvedLiteral::UnsignedInt8(_) => todo!("assignment to literal u8"),
        ResolvedLiteral::SignedInt8(_) => todo!("assignment to literal s8"),
        ResolvedLiteral::UnsignedInt16(_) => todo!("assignment to literal u16"),
        ResolvedLiteral::SignedInt16(_) => todo!("assignment to literal s16"),
        ResolvedLiteral::UnsignedInt32(value) => add_byte_code(&mut ir.byte_code, move_value_to_reg_plus_offset_32_instruction(value, base_pointer_register(), assignment_offset)),
        ResolvedLiteral::SignedInt32(value) => add_byte_code(&mut ir.byte_code, move_value_to_reg_plus_offset_32_instruction(value as u32, base_pointer_register(), assignment_offset)),
        ResolvedLiteral::UnsignedInt64(value) => add_byte_code(&mut ir.byte_code, move_value_to_reg_plus_offset_64_instruction(value, base_pointer_register(), assignment_offset)),
        ResolvedLiteral::SignedInt64(value) => add_byte_code(&mut ir.byte_code, move_value_to_reg_plus_offset_64_instruction(value as u64, base_pointer_register(), assignment_offset)),
        ResolvedLiteral::Float32(_) => todo!("assignment to literal float32"),
        ResolvedLiteral::Float64(_) => todo!("assignment to literal float64"),
        ResolvedLiteral::String(value) => build_bytecode_at_variable_assignment_to_literal_string(ir, assignment_offset, value),
    };
}

fn build_bytecode_at_variable_assignment_to_literal_string(
    ir: &mut IntermediateRepresentation,
    assignment_offset: u8,
    value: String,
) {    
    store_string_count_member_value(ir, &value, assignment_offset);
    let data_item_pointer = store_string_literal_in_data_section_and_add_symbol(ir, &value);
    store_string_data_member_value(ir, data_item_pointer, assignment_offset);
}

fn store_string_count_member_value(ir: &mut IntermediateRepresentation, value: &str, assignment_offset: u8) {
    add_byte_code(
        &mut ir.byte_code, 
        move_value_to_reg_plus_offset_64_instruction(value.len() as u64, base_pointer_register(), assignment_offset)
    )
}

fn store_string_data_member_value(ir: &mut IntermediateRepresentation, data_item_pointer: u32, assignment_offset: u8) {
    add_byte_code(
        &mut ir.byte_code, 
        load_data_section_address_to_reg_64(data_item_pointer, call_arg_register(0))
    );
    add_byte_code(
        &mut ir.byte_code, 
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(0), base_pointer_register(), assignment_offset + 8)
    );
}

fn build_bytecode_at_variable_assignment_to_identifier(
    _ir: &mut IntermediateRepresentation,
    _assignment_map: &AssignmentMap,
    _assignment_name: &str,
    _name: &str
) {
    println!("build_bytecode_at_assignment_to_identifier: {}, {}", _assignment_name, _name);
}

fn build_bytecode_at_variable_assignment_to_null(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str
) {
    let offset = get_assignment_offset(assignment_map, assignment_name);
    let instruction = move_value_to_reg_plus_offset_64_instruction(0, base_pointer_register(), offset);
    add_byte_code(&mut ir.byte_code, instruction);
}


fn build_bytecode_at_variable_assignment_to_member_expr(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    instance: &AbstractSyntaxNode,
    member: &AbstractSyntaxNode
) {
    if let AbstractSyntaxNodeItem::Instance { name, instance_type, .. } = instance.item_ref() {
        let instance_offset = get_assignment_offset(assignment_map, name);
        add_byte_code(
            &mut ir.byte_code, 
            move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), instance_offset, standard_register(0))
        );

        if let AbstractSyntaxNodeItem::Member { name, .. } = member.item_ref() {
            
            let member_offset = get_instance_member_offset(instance_type, name) as u8;
            add_byte_code(
                &mut ir.byte_code, 
                move_reg_plus_offset_to_reg_64_instruction(standard_register(0), member_offset, standard_register(1))
            );

            let assignment_offset = get_assignment_offset(assignment_map, assignment_name);
            add_byte_code(
                &mut ir.byte_code, 
                move_reg_to_reg_plus_offset_64_instruction(standard_register(1), base_pointer_register(), assignment_offset)
            );
        }
    } else {
        panic!("member expr instance is not instance");
    }
}

pub type Assignments = HashMap<String, isize>;

#[derive(Default)]
pub struct AssignmentMap {
    assignments: Assignments,
    total_size: usize
}

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
            AbstractSyntaxNodeItem::VariableDeclaration { name, variable_type, ..} => {
                let size = get_type_size_from_resolvable_type(variable_type) as isize;
                position = position - size;
                assignment_map.total_size += size as usize;
                add_assignment_to_map(assignment_map, name.clone(), position);
            }
        _ => {}
        }
    }
}

fn add_args_to_assignment_map(assignment_map: &mut AssignmentMap, args: &AbstractSyntaxChildNodes) {
    let mut position = 0;
    for statement in args {
        match statement.item_ref() {
            AbstractSyntaxNodeItem::MemberDeclaration { name, .. } => {
                position = position + 8;
                add_assignment_to_map(assignment_map, name.clone(), position);
            }
        _ => {}
        }
    }
}

fn add_assignment_to_map(assignment_map: &mut AssignmentMap, name: String, position: isize) {
    assignment_map.assignments.insert(name, position);
}

pub fn get_full_assignment_storage_size(assignment_map: &AssignmentMap) -> u8 {
    assignment_map.total_size as u8
}

pub fn get_assignment_offset(assignment_map: &AssignmentMap, assignment_name: &str) -> u8 {
    if let Some(position) = assignment_map.assignments.get(assignment_name) {
        return *position as u8;
    } else {
        panic!("No offset found for assignment")
    }
}
