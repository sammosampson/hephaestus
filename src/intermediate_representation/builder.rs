use crate::{
    acting::*,
    compilation::*,
    parsing::*,
    intermediate_representation::*
};

pub struct IntemediateRepresentationActor;

pub fn create_intemediate_representation_actor() -> IntemediateRepresentationActor {
    IntemediateRepresentationActor
}

impl Actor<CompilationMessage> for IntemediateRepresentationActor {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::BuildByteCode { mut unit, compiler } =>
                build_bytecode(&mut unit, &compiler),
            _ => continue_listening_after_receive()
        }
    }
}

fn build_bytecode(unit: &mut CompilationUnit, compiler: &CompilationActorHandle) -> AfterReceiveAction {
    
    let mut ir = create_intermediate_representation(unit.id, unit.filename.clone());    
    build_bytecode_at_root(unit, &mut ir);
    send_message_to_actor(compiler, create_byte_code_built_event(ir));
    shutdown_after_receive()
}

fn build_bytecode_at_root(unit: &CompilationUnit, ir: &mut IntermediateRepresentation) {
    match unit.tree.item_ref() {
        AbstractSyntaxNodeItem::ProcedureHeader { name, .. } =>
            build_bytecode_at_procedure_header(ir, name),
        AbstractSyntaxNodeItem::ProcedureBody { name, args, statements, .. } =>
            build_bytecode_at_procedure_body(ir, name, args, statements),
        AbstractSyntaxNodeItem::Constant { name, value} =>
            build_bytecode_at_top_root_const(ir, name, value),
        _ => todo!()
    }    
}

fn build_bytecode_at_top_root_const(ir: &mut IntermediateRepresentation, name: &str, value: &AbstractSyntaxNode) {
    ir.top_level_symbol = string(name);

    match value.item_ref() {
        AbstractSyntaxNodeItem::ForeignSystemLibrary { library } =>
            build_bytecode_at_foreign_system_library_const(ir, library),
        AbstractSyntaxNodeItem::Literal(literal) => build_bytecode_at_literal_const(ir, &get_resolved_literal(literal)),        
        _ => todo!()
    }
}

fn build_bytecode_at_foreign_system_library_const(ir: &mut IntermediateRepresentation, library: &AbstractSyntaxNode) {
    match library.item_ref() {
        AbstractSyntaxNodeItem::Literal(literal) =>
            build_bytecode_at_foreign_system_library_literal_const(ir, &get_resolved_literal(literal)),
        _ => todo!("foreign system library none literals")
    }
}

fn build_bytecode_at_foreign_system_library_literal_const(ir: &mut IntermediateRepresentation, library: &ResolvedLiteral) {
    match library {
        ResolvedLiteral::String(value) =>
            add_foreign_library_reference(&mut ir.foreign_libraries, string(value)),
        _ => todo!("foreign system library none string literals")
    }
}

fn build_bytecode_at_literal_const(ir: &mut IntermediateRepresentation, library: &ResolvedLiteral) {
    match library {
        ResolvedLiteral::UnsignedInt32(number) => todo!("const int number literals"),
        _ => todo!("const none int number literals")
    }
}

fn build_bytecode_at_procedure_header(ir: &mut IntermediateRepresentation, name: &str) {
    ir.top_level_symbol = string(name);
}

fn build_bytecode_at_procedure_body(
    ir: &mut IntermediateRepresentation, 
    name: &str,
    args: &AbstractSyntaxChildNodes,
    statements: &AbstractSyntaxChildNodes
) {
    ir.top_level_symbol = string(&name);

    add_symbol(&mut ir.symbols, external_code_label(string(&name), 0));

    add_byte_codes(
        &mut ir.byte_code,
        vec!(
            push_reg_64_instruction(base_pointer_register()),
            move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        )
    );

    for arg_index in 0..args.len() {
        build_bytecode_at_procedure_body_argument_declaration(ir, arg_index);
    }

    for statement in statements {
        build_bytecode_at_procedure_body_statement(ir, statement);
    }

    add_byte_codes(
        &mut ir.byte_code, 
        vec!(
            move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
            pop_reg_64_instruction(base_pointer_register()),
            ret_instruction()
        )
    );
}

fn build_bytecode_at_procedure_body_argument_declaration(ir: &mut IntermediateRepresentation, arg_index: usize) {
    add_byte_code(
        &mut ir.byte_code,
        move_reg_to_reg_plus_offset_64_instruction(
            call_arg_register(arg_index), 
            base_pointer_register(), 
            (16 + (arg_index * 8)) as u8
        )
    );
}

fn build_bytecode_at_procedure_body_statement(ir: &mut IntermediateRepresentation, statement: &AbstractSyntaxNode) {
    match statement.item_ref() {
        AbstractSyntaxNodeItem::ProcedureCall { name, args, .. } => 
            build_bytecode_at_procedure_call(ir, name, args),
        AbstractSyntaxNodeItem::Assignment { name, value, .. } => 
            build_bytecode_at_assignment(ir, name, value),
        _ => todo!()
    }
}

fn build_bytecode_at_assignment(ir: &mut IntermediateRepresentation, assignment_name: &str, value: &AbstractSyntaxNode) {
    match value.item_ref() {
        AbstractSyntaxNodeItem::ProcedureCall { name, args, .. } => 
            build_bytecode_at_procedure_call_with_assignment(ir, assignment_name, name, args),
        _ => todo!()
    }
}

fn build_bytecode_at_procedure_call_with_assignment(ir: &mut IntermediateRepresentation, _assignment_name: &str, call_name: &str, args: &AbstractSyntaxChildNodes) {
    reserve_shadow_stack_space(ir);
    build_bytecode_at_procedure_call_arguments(args, ir);
    call_external_function(ir, call_name);
    move_procedure_call_return_value_into_storage(ir);
    release_shadow_stack_space(ir);
}

fn build_bytecode_at_procedure_call(ir: &mut IntermediateRepresentation, name: &str, args: &AbstractSyntaxChildNodes) {
    reserve_shadow_stack_space(ir);
    build_bytecode_at_procedure_call_arguments(args, ir);
    call_external_function(ir, name);
    release_shadow_stack_space(ir);
}

fn build_bytecode_at_procedure_call_arguments(args: &AbstractSyntaxChildNodes, ir: &mut IntermediateRepresentation) {
    for arg_index in 0..args.len() {
        build_bytecode_at_procedure_call_argument(ir, &args[arg_index], arg_index);
    }
}

fn build_bytecode_at_procedure_call_argument(ir: &mut IntermediateRepresentation, arg: &AbstractSyntaxNode, arg_index: usize) {
    match arg.item_ref() {
        AbstractSyntaxNodeItem::Argument { expr, .. } =>
            build_bytecode_at_procedure_call_argument_expression(ir, expr, arg_index),
        _ => todo!()
    }    
}

fn build_bytecode_at_procedure_call_argument_expression(ir: &mut IntermediateRepresentation,  expr: &AbstractSyntaxNode, arg_index: usize) {
    match expr.item_ref() {
        AbstractSyntaxNodeItem::Literal(literal) =>
            build_bytecode_at_procedure_call_argument_literal(ir, &get_resolved_literal(literal), arg_index),
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

fn move_procedure_call_return_value_into_storage(ir: &mut IntermediateRepresentation) {
    add_byte_code(
        &mut ir.byte_code, 
        move_reg_to_reg_plus_offset_32_instruction(
            call_return_arg_register(0), 
            base_pointer_register(),
            0xF8
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
