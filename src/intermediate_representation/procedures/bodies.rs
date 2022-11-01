use crate::{
    parsing::*,
    intermediate_representation::*,
    compilation::*
};

pub fn build_bytecode_at_procedure_body(
    ir: &mut IntermediateRepresentation, 
    name: &str,
    args: &AbstractSyntaxChildNodes,
    statements: &AbstractSyntaxChildNodes,
    errors: &mut CompilationErrors
) {
    ir.top_level_symbol = string(&name);
    let assignment_map = get_assignment_map(args, statements, errors);
    store_procedure_name_as_external_symbol(ir, name);
    build_bytecode_for_procedure_prologue(ir);
    build_bytecode_for_procedure_argument_shadow_storage(args, ir);
    build_bytecode_for_procedure_assignments_storage_reservation(ir, &assignment_map);
    build_bytecode_for_procedure_body_statements(ir, &assignment_map, statements, errors);
    build_bytecode_for_procedure_epilogue(ir);
}

fn build_bytecode_for_procedure_argument_shadow_storage(args: &AbstractSyntaxChildNodes, ir: &mut IntermediateRepresentation) {
    for arg_index in 0..args.len() {
        build_bytecode_at_procedure_argument_shadow_storage(ir, arg_index);
    }
}

fn build_bytecode_at_procedure_argument_shadow_storage(ir: &mut IntermediateRepresentation, arg_index: usize) {
    add_byte_code(
        &mut ir.byte_code,
        move_reg_to_reg_plus_offset_instruction(
            register_size_64(),
            call_arg_register(arg_index), 
            base_pointer_register(), 
            address_offset((16 + (arg_index * 8)) as u8)
        )
    );
}

fn build_bytecode_for_procedure_assignments_storage_reservation(ir: &mut IntermediateRepresentation, assignment_map: &AssignmentMap) {
    let assignment_storage_size = get_full_assignment_storage_size(assignment_map);
    
    if assignment_storage_size == 0 {
        return;
    }

    add_byte_code(
        &mut ir.byte_code,
        sub_value_from_reg_instruction(instruction_value_8(assignment_storage_size), stack_pointer_register())
    );
}

fn build_bytecode_for_procedure_body_statements(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    statements: &AbstractSyntaxChildNodes,
    errors: &mut CompilationErrors
) {
    for statement in statements {
        build_bytecode_at_procedure_body_statement(ir, assignment_map, statement, errors);
    }
}

fn build_bytecode_at_procedure_body_statement(
    ir: &mut IntermediateRepresentation,
    assignment_map: &AssignmentMap,
    statement: &AbstractSyntaxNode,
    errors: &mut CompilationErrors
) {
    match statement.item_ref() {
        AbstractSyntaxNodeItem::ProcedureCall { name, args, .. } => 
            build_bytecode_at_procedure_call(ir, assignment_map, name, args, errors),
        AbstractSyntaxNodeItem::VariableDeclaration { name, value, .. } => 
            build_bytecode_at_variable_declaration(ir, assignment_map, name, statement.position.clone(), value, errors),
        _ => todo(errors, function!(), "Other procedure body statement types")
    }
}

fn store_procedure_name_as_external_symbol(ir: &mut IntermediateRepresentation, name: &str) {
    add_symbol(&mut ir.symbols, external_code_label(string(&name), 0));
}

fn build_bytecode_for_procedure_prologue(ir: &mut IntermediateRepresentation) {
    add_byte_codes(
        &mut ir.byte_code,
        vec!(
            push_reg_instruction(register_size_64(), base_pointer_register()),
            move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),
        )
    );
}

fn build_bytecode_for_procedure_epilogue(ir: &mut IntermediateRepresentation) {
    add_byte_codes(
        &mut ir.byte_code, 
        vec!(
            move_reg_to_reg_instruction(register_size_64(),base_pointer_register(), stack_pointer_register()),
            pop_reg_instruction(register_size_64(), base_pointer_register()),
            ret_instruction()
        )
    );
}
