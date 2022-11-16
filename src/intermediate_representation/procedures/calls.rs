use crate::{
    parsing::*,
    intermediate_representation::*,
    errors::*,
    types::*
};

pub fn build_bytecode_at_variable_assignment_to_procedure_call(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    assignment_position: SourceFilePosition,
    call_name: &str,
    args: &AbstractSyntaxChildNodes,
    errors: &mut CompilationErrors
) {
    reserve_shadow_stack_space(ir, args.len());
    build_bytecode_at_procedure_call_arguments(args, assignment_map, ir, errors);
    call_external_function(ir, call_name);
    move_procedure_call_return_value_into_storage(ir, assignment_map, assignment_name, assignment_position, errors);
    release_shadow_stack_space(ir, args.len());
}

pub fn build_bytecode_at_procedure_call(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    name: &str,
    args: &AbstractSyntaxChildNodes,
    errors: &mut CompilationErrors
) {
    reserve_shadow_stack_space(ir, args.len());
    build_bytecode_at_procedure_call_arguments(args, assignment_map, ir, errors);
    call_external_function(ir, name);
    release_shadow_stack_space(ir, args.len());
}

fn build_bytecode_at_procedure_call_arguments(
    args: &AbstractSyntaxChildNodes,
    assignment_map: &AssignmentMap,
    ir: &mut IntermediateRepresentation,
    errors: &mut CompilationErrors
) {
    for arg_index in 0..args.len() {
        build_bytecode_at_procedure_call_argument(ir, assignment_map, &args[arg_index], arg_index, errors);
    }
}

fn build_bytecode_at_procedure_call_argument(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    arg: &AbstractSyntaxNode,
    arg_index: usize,
    errors: &mut CompilationErrors
) {
    let arg_position = arg.position.clone();
    match arg.item_ref() {
        AbstractSyntaxNodeItem::Argument { expr, arg_type, .. } =>
            build_bytecode_at_procedure_call_argument_expression(ir, assignment_map, expr, arg_type, arg_index, errors),
        _ => add_intermediate_representation_error(errors, expected_argument_error(), arg_position)
    }    
}

fn build_bytecode_at_procedure_call_argument_expression(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    expr: &AbstractSyntaxNode,
    arg_type: &ResolvableType,
    arg_index: usize,
    errors: &mut CompilationErrors
) {
    let expr_position = expr.position.clone();

    match expr.item_ref() {
        AbstractSyntaxNodeItem::Literal(literal) => {
            if let Some(resolved_literal) = try_get_resolved_literal(literal) {
                build_bytecode_at_procedure_call_argument_literal(ir, &resolved_literal, arg_index, errors)
            } else {
                add_intermediate_representation_error(errors, literal_not_resolved_error(), expr_position);
            }
        },
        AbstractSyntaxNodeItem::Identifier { name, scope} =>
            build_bytecode_at_procedure_call_argument_identifier(ir, assignment_map, name, scope, arg_type, arg_index, expr_position, errors),
        _ => todo(errors, function!(), "Other procedure call arg expression types")
    }    
}

fn build_bytecode_at_procedure_call_argument_literal(
    ir: &mut IntermediateRepresentation,
    literal: &ResolvedLiteral,
    arg_index: usize,
    errors: &mut CompilationErrors
) {
    if arg_index > 3 {
        todo(errors, function!(), "Move fourth or more argument to shadow space");
    }
    
    match literal {
        ResolvedLiteral::String(value) => build_bytecode_at_procedure_call_argument_string_literal(ir, value, arg_index),
        _ => build_bytecode_at_procedure_call_argument_non_string_literal(ir, literal, arg_index)
    }
}

fn build_bytecode_at_procedure_call_argument_non_string_literal(ir: &mut IntermediateRepresentation, literal: &ResolvedLiteral, arg_index: usize) {
    add_byte_code(
        &mut ir.byte_code, 
        move_value_to_reg_instruction(resolved_literal_to_instruction_value(literal), call_arg_register(arg_index))
    );
}

fn build_bytecode_at_procedure_call_argument_string_literal(ir: &mut IntermediateRepresentation, value: &Vec<u8>, arg_index: usize) {
    let string_literal_data_item_pointer = store_string_literal_in_data_section_and_add_symbol(ir, value);
    let string_data_item_pointer = store_string_in_data_section_and_add_symbol(ir, value.len(), string_literal_data_item_pointer);
    add_byte_code(
        &mut ir.byte_code, 
        load_data_section_address_to_reg(
            register_size_64(), 
            string_data_item_pointer, 
            call_arg_register(arg_index)
        )
    );
}

fn build_bytecode_at_procedure_call_argument_identifier(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    name: &str,
    scope: &Scope,
    arg_type: &ResolvableType,
    arg_index: usize,
    arg_position: SourceFilePosition,
    errors: &mut CompilationErrors
) {
    match scope {
        Scope::Unknown => add_intermediate_representation_error(errors, scope_not_known_error(), arg_position),
        Scope::Local => build_bytecode_at_procedure_call_argument_local_identifier(
            ir,
            assignment_map,
            name, 
            arg_position,
            arg_type,
            arg_index,
            errors
        ),
        Scope::Global => build_bytecode_at_procedure_call_argument_global_identifier(ir, name, arg_type, arg_index, arg_position, errors),
    }
}

fn build_bytecode_at_procedure_call_argument_local_identifier(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    identifier_name: &str,
    identifier_position: SourceFilePosition,
    arg_type: &ResolvableType,
    arg_index: usize,
    errors: &mut CompilationErrors
) {
    if let Some(assignment) = get_assignment(assignment_map, identifier_name, identifier_position, errors) {       
        if let Some(arg_type) = try_get_resolved_runtime_type_pointer(arg_type) {
            if let Some((built_in_arg_type, ..)) = try_get_built_in_type(&arg_type.id) {
                let register_size = built_in_type_to_register_size(built_in_arg_type);
                build_bytecode_for_move_variable_to_call_arg_location(ir, register_size, assignment.offset, arg_index);
            } else {
                todo(errors, function!(), "Non built in typed identifier call arg");
            }
        } else {
            add_intermediate_representation_error(errors, type_not_resolved_error(), identifier_position);
        }
    }
}

fn build_bytecode_for_move_variable_to_call_arg_location(ir: &mut IntermediateRepresentation, register_size: RegisterSize, offset: AddressOffset, arg_index: usize) {
    build_bytecode_for_move_variable_to_call_arg_register_instruction(ir, register_size, offset, arg_index);
    build_bytecode_for_move_call_arg_to_shadow_space_if_fourth_or_more(ir, register_size, arg_index);
}

fn build_bytecode_for_move_variable_to_call_arg_register_instruction(
    ir: &mut IntermediateRepresentation,
    register_size: RegisterSize,
    offset: AddressOffset,
    arg_index: usize
) {
    add_byte_code(&mut ir.byte_code, move_variable_to_call_arg_register_instruction(register_size, offset, arg_index))
}

fn build_bytecode_for_move_call_arg_to_shadow_space_if_fourth_or_more(ir: &mut IntermediateRepresentation, register_size: RegisterSize, arg_index: usize) {
    if arg_index > 3 {
        add_byte_code(
            &mut ir.byte_code, 
            move_reg_to_reg_plus_offset_instruction(
                register_size,
                call_arg_register(arg_index),
                stack_pointer_register(), 
                address_offset((arg_index * 8) as u8)
            )
        );
    }
}

fn move_variable_to_call_arg_register_instruction(register_size: RegisterSize, offset: AddressOffset, arg_index: usize) -> ByteCodeInstruction {
    move_reg_plus_offset_to_reg_instruction(register_size, base_pointer_register(), offset, call_arg_register(arg_index))
}

fn build_bytecode_at_procedure_call_argument_global_identifier(
    ir: &mut IntermediateRepresentation,
    identifier_name: &str,
    arg_type: &ResolvableType,
    arg_index: usize,
    arg_position: SourceFilePosition,
    errors: &mut CompilationErrors
) {
    if arg_index > 3 {
        todo(errors, function!(), "Move fourth or more argument to shadow space");
    }

    let symbol_index = add_symbol(&mut ir.symbols, foreign_external(string(identifier_name)));

    if let Some(arg_type) = try_get_resolved_runtime_type_pointer(arg_type) {
        if let Some((built_in_arg_type, _is_pointer)) = try_get_built_in_type(&arg_type.id) {
            let register_size = built_in_type_to_register_size(built_in_arg_type);
            build_bytecode_at_procedure_call_argument_global_identifier_bulit_in_type(ir, register_size, symbol_index, arg_index);
        } else {
            todo(errors, function!(), "Non built in typed identifier call arg");
        }
    } else {
        add_intermediate_representation_error(errors, type_not_resolved_error(), arg_position);
    }
    
}

fn build_bytecode_at_procedure_call_argument_global_identifier_bulit_in_type(
    ir: &mut IntermediateRepresentation,
    register_size: RegisterSize,
    symbol_index: SymbolIndex,
    arg_index: usize
) {
    add_byte_code(
        &mut ir.byte_code, 
        move_symbol_to_reg_instruction(register_size, symbol_index, call_arg_register(arg_index))
    );
}

fn move_procedure_call_return_value_into_storage(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    assignment_name: &str,
    assignment_position: SourceFilePosition,
    errors: &mut CompilationErrors
) {
    if let Some(assignment) = get_assignment(assignment_map, assignment_name, assignment_position, errors) {
        add_byte_code(
            &mut ir.byte_code, 
            move_reg_to_reg_plus_offset_instruction(
                register_size_32(),
                call_return_arg_register(0), 
                base_pointer_register(),
                assignment.offset
            )
        );
    }
}

fn call_external_function(ir: &mut IntermediateRepresentation, name: &str) {
    let call_name_symbol_index = add_symbol(&mut ir.symbols, foreign_external(string(name)));
    add_byte_code(
        &mut ir.byte_code,
        call_to_symbol_instruction(call_name_symbol_index)
    );
}

fn reserve_shadow_stack_space(ir: &mut IntermediateRepresentation, arg_count: usize) {
    add_byte_code(
        &mut ir.byte_code,
        sub_value_from_reg_instruction(instruction_value_8(get_shadow_space_size(arg_count)), stack_pointer_register())
    );
}

fn release_shadow_stack_space(ir: &mut IntermediateRepresentation, arg_count: usize) {
    add_byte_code(
        &mut ir.byte_code,
        add_value_to_reg_instruction(instruction_value_8(get_shadow_space_size(arg_count)), stack_pointer_register())
    );
}

fn get_shadow_space_size(arg_count: usize) -> u8 {
    std::cmp::max(8 * arg_count as u8, 32)
}