use crate::{
    parsing::*,
    intermediate_representation::*,
    typing::*
};

pub fn build_bytecode_at_procedure_call_with_assignment(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    call_name: &str,
    args: &AbstractSyntaxChildNodes
) {
    reserve_shadow_stack_space(ir);
    build_bytecode_at_procedure_call_arguments(args, assignment_map, ir);
    call_external_function(ir, call_name);
    move_procedure_call_return_value_into_storage(ir, assignment_map, assignment_name);
    release_shadow_stack_space(ir);
}

pub fn build_bytecode_at_procedure_call(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    name: &str,
    args: &AbstractSyntaxChildNodes
) {
    reserve_shadow_stack_space(ir);
    build_bytecode_at_procedure_call_arguments(args, assignment_map, ir);
    call_external_function(ir, name);
    release_shadow_stack_space(ir);
}

fn build_bytecode_at_procedure_call_arguments(args: &AbstractSyntaxChildNodes, assignment_map: &AssignmentMap, ir: &mut IntermediateRepresentation) {
    for arg_index in 0..args.len() {
        build_bytecode_at_procedure_call_argument(ir, assignment_map, &args[arg_index], arg_index);
    }
}

fn build_bytecode_at_procedure_call_argument(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    arg: &AbstractSyntaxNode,
    arg_index: usize
) {
    match arg.item_ref() {
        AbstractSyntaxNodeItem::Argument { expr, arg_type, .. } =>
            build_bytecode_at_procedure_call_argument_expression(ir, assignment_map, expr, arg_type, arg_index),
        _ => todo!()
    }    
}

fn build_bytecode_at_procedure_call_argument_expression(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    expr: &AbstractSyntaxNode,
    arg_type: &ResolvableType,
    arg_index: usize
) {
    match expr.item_ref() {
        AbstractSyntaxNodeItem::Literal(literal) =>
            build_bytecode_at_procedure_call_argument_literal(ir, &get_resolved_literal(literal), arg_index),
        AbstractSyntaxNodeItem::Identifier(name) =>
            build_bytecode_at_procedure_call_argument_identifier(ir, assignment_map, name, arg_type, arg_index),
        _ => todo!()
    }    
}


fn build_bytecode_at_procedure_call_argument_literal(ir: &mut IntermediateRepresentation, literal: &ResolvedLiteral, arg_index: usize) {
    match literal {
        ResolvedLiteral::UnsignedInt32(value) => {
            add_byte_code(
                &mut ir.byte_code, 
                move_value_to_reg_32_instruction(*value, call_arg_register(arg_index))
            );
        },
        ResolvedLiteral::SignedInt64(value) => {
            add_byte_code(
                &mut ir.byte_code, 
                move_value_to_reg_64_instruction(*value as u64, call_arg_register(arg_index))
            );
        },
        ResolvedLiteral::String(value) => {
            let data_item_pointer = add_data_item(&mut ir.data, string_data_item(string(&value)));
            add_symbol(&mut ir.symbols, data_section_item(data_section_item_name(data_item_pointer), data_item_pointer));
            add_byte_code(
                &mut ir.byte_code, 
                load_data_section_address_to_reg_64(data_item_pointer, call_arg_register(arg_index))
            );
        },
        _ =>  todo!("Other literals as call args")
    }
}


fn build_bytecode_at_procedure_call_argument_identifier(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    identifier_name: &str,
    arg_type: &ResolvableType,
    arg_index: usize
) {
    let offset = get_assignment_offset(assignment_map, identifier_name);
    
    if let Some(arg_type) = try_get_resolved_runtime_type_pointer(arg_type) {
        if let Some((built_in_arg_type, _is_pointer)) = try_get_built_in_type(&arg_type.id) {
            let instruction = match built_in_arg_type {
                BuiltInType::UnsignedInt8 => todo!("identifier call arg u8"),
                BuiltInType::SignedInt8 => todo!("identifier call arg s8"),
                BuiltInType::UnsignedInt16 => todo!("identifier call arg u16"),
                BuiltInType::SignedInt16 => todo!("identifier call arg s16"),
                BuiltInType::UnsignedInt32 => move_reg_plus_offset_to_reg_32_instruction(base_pointer_register(), offset, call_arg_register(arg_index)),
                BuiltInType::SignedInt32 => move_reg_plus_offset_to_reg_32_instruction(base_pointer_register(), offset, call_arg_register(arg_index)),
                BuiltInType::UnsignedInt64 => move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), offset, call_arg_register(arg_index)),
                BuiltInType::SignedInt64 => move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), offset, call_arg_register(arg_index)),
                BuiltInType::Float32 => todo!("identifier call arg float32"),
                BuiltInType::Float64 => todo!("identifier call arg float64"),
                BuiltInType::String => todo!("identifier call arg string"),
                BuiltInType::Boolean => todo!(),
                BuiltInType::Void => todo!(),
            };
            add_byte_code(&mut ir.byte_code, instruction);
            return;
        }
        todo!("Non built in typed identifier call arg")
    }
    panic!("Unresolved type for identifier call arg")
}

fn move_procedure_call_return_value_into_storage(ir: &mut IntermediateRepresentation, assignment_map: &AssignmentMap, assignment_name: &str) {
    add_byte_code(
        &mut ir.byte_code, 
        move_reg_to_reg_plus_offset_32_instruction(
            call_return_arg_register(0), 
            base_pointer_register(),
            get_assignment_offset(assignment_map, assignment_name)
        )
    );
}

fn call_external_function(ir: &mut IntermediateRepresentation, name: &str) {
    let call_name_symbol_index = add_symbol(&mut ir.symbols, foreign_external(string(name)));
    add_byte_code(
        &mut ir.byte_code,
        call_to_symbol_instruction(call_name_symbol_index)
    );
}

fn reserve_shadow_stack_space(ir: &mut IntermediateRepresentation) {
    add_byte_code(
        &mut ir.byte_code,
        sub_value_from_reg_8_instruction(32, stack_pointer_register())
    );
}

fn release_shadow_stack_space(ir: &mut IntermediateRepresentation) {
    add_byte_code(
        &mut ir.byte_code,
        add_value_to_reg_8_instruction(32, stack_pointer_register())
    );
}