use crate::{
    acting::*,
    compilation::*,
    parsing::*,
    intermediate_representation::*,
    typing::*
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
    
    let mut visitor = create_root_bytecode_build_ast_node_visitor(unit);

    apply_visitor_to_ast_root(&mut unit.tree, &mut visitor);

    send_message_to_actor(
            compiler, 
            create_byte_code_built_event(visitor.ir));
    shutdown_after_receive()
}

pub struct RootByteCodeBuildAstNodeVisitor {
    ir: IntermediateRepresentation
}

fn create_root_bytecode_build_ast_node_visitor(unit: &mut CompilationUnit) -> RootByteCodeBuildAstNodeVisitor {
    RootByteCodeBuildAstNodeVisitor { 
        ir: create_intermediate_representation(unit.id, unit.filename.clone())
    }
}

impl AbstractSyntaxRootNodeVisitor for RootByteCodeBuildAstNodeVisitor {
    fn visit_run(&mut self, _expr: &mut AbstractSyntaxNode) {
        todo!("run at top level")
    }

    fn visit_const(&mut self, name: &mut String, value: &mut AbstractSyntaxNode) {
        
    }

    fn visit_procedure_header(
        &mut self,
        name: &mut String,
        _args: &mut AbstractSyntaxChildNodes,
        _return_types: &mut AbstractSyntaxChildNodes,
        _body: &mut ProcedureBodyReference
    ) {
        self.ir.top_level_symbol = name.clone();
    }

    fn visit_procedure_body(
        &mut self, 
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        return_types: &mut AbstractSyntaxChildNodes,
        statements: &mut AbstractSyntaxChildNodes
    ) {
        self.ir.top_level_symbol = string(&name);

        add_symbol(&mut self.ir.symbols, external_code_label(string(&name), 0x0));

        add_byte_codes(
            &mut self.ir.byte_code,
            vec!(
                push_reg_64_instruction(base_pointer_register()),
                move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
            )
        );
    
        let mut visitor = create_procedure_body_visitor(&mut self.ir);
        apply_visitor_to_ast_procedure_body(args, return_types, statements, &mut visitor);

        add_byte_codes(
            &mut self.ir.byte_code, 
            vec!(
                move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
                pop_reg_64_instruction(base_pointer_register()),
                ret_instruction()
            )
        );
    }

}

pub struct ProcedureBodyVisitor<'a> {
    ir: &'a mut IntermediateRepresentation,
    current_arg_count: usize
}

fn create_procedure_body_visitor<'a>(ir: &'a mut IntermediateRepresentation) -> ProcedureBodyVisitor<'a> {
    ProcedureBodyVisitor { 
        ir,
        current_arg_count: 0
    }
}

impl<'a> AbstractSyntaxProcedureBodyNodeVisitor for ProcedureBodyVisitor<'a> {
    fn visit_argument_declaration(
        &mut self,
        _name: &mut String,
        _type_id: &mut ResolvableType
    ) {
        add_byte_code(
            &mut self.ir.byte_code,
            move_reg_to_reg_plus_offset_64_instruction(
                call_arg_register(self.current_arg_count), 
                base_pointer_register(), 
                (16 + (self.current_arg_count * 8)) as u8
            )
        );
        
        self.current_arg_count += 1;
    }

    fn visit_return_type_declaration(&mut self, _return_type: &mut ResolvableType) {
        todo!("return type from proc body")
    }

    fn visit_procedure_call(
        &mut self,
        name: &mut String,
        args: &mut AbstractSyntaxChildNodes,
        _type_id: &mut ResolvableType
    ) {
        add_byte_code(
            &mut self.ir.byte_code,
            sub_value_from_reg_8_instruction(32, stack_pointer_register())
        );

        let mut visitor = create_args_visitor(self.ir);
        apply_visitor_to_ast_args(args, &mut visitor);  

        let call_name_symbol_index = add_symbol(&mut self.ir.symbols, foreign_external(string(&name)));
    
        add_byte_code(
            &mut self.ir.byte_code,
            call_to_symbol_instruction(call_name_symbol_index)
        );

        add_byte_code(
            &mut self.ir.byte_code,
            add_value_to_reg_8_instruction(32, stack_pointer_register())
        );
    }

    fn visit_assignment(
        &mut self,
        _name: &mut String,
        _value: &mut AbstractSyntaxNode,
        _type_id: &mut ResolvableType
    ) {
        todo!("assignment in proc body")
    }

    fn visit_return_statement(&mut self, _args: &mut AbstractSyntaxChildNodes) {
    }
}

struct ProcedureCallArgVisitor <'a> { 
    ir: &'a mut IntermediateRepresentation,
    current_arg_count: usize
}

fn create_args_visitor<'a>(ir: &'a mut IntermediateRepresentation) -> ProcedureCallArgVisitor::<'a> {
    ProcedureCallArgVisitor { 
        ir,
        current_arg_count: 0
    }
}

impl <'a> AbstractSyntaxArgumentsNodeVisitor for ProcedureCallArgVisitor<'a> {
    fn visit_argument(&mut self, expr: &mut AbstractSyntaxNode, _arg_type: &mut ResolvableType) {
        let mut visitor = create_arg_expression_visitor(self.ir, self.current_arg_count);
        apply_visitor_to_ast_expression(expr, &mut visitor);
        self.current_arg_count += 1;
    }
}

struct ProcedureCallArgExpressionVisitor <'a> { 
    ir: &'a mut IntermediateRepresentation,
    current_arg_count: usize
}

fn create_arg_expression_visitor<'a>(ir: &'a mut IntermediateRepresentation, current_arg_count: usize) -> ProcedureCallArgExpressionVisitor::<'a> {
    ProcedureCallArgExpressionVisitor { 
        ir,
        current_arg_count
    }
}

impl <'a> AbstractSyntaxExpressionNodeVisitor for ProcedureCallArgExpressionVisitor<'a> {
    fn visit_literal(&mut self, literal: &mut Literal) {
        match literal {
            Literal::UnsignedInt(value) => {
                add_byte_code(
                    &mut self.ir.byte_code, 
                    move_value_to_reg_32_instruction(*value as u32, call_arg_register(self.current_arg_count))
                );
            },
            Literal::String(value) => {
                let data_item_pointer = add_data_item(&mut self.ir.data, string_data_item(string(&value)));
                add_symbol(&mut self.ir.symbols, data_section_item(data_section_item_name(data_item_pointer), data_item_pointer));
                add_byte_code(
                    &mut self.ir.byte_code, 
                    load_data_section_address_to_reg_64(data_item_pointer, call_arg_register(self.current_arg_count))
                );
            },
            _ =>  todo!("float literals as call args")
        }
    }

    fn visit_identifier(&mut self, _name: &mut String) {
        todo!("identifiers as call args")
    }

    fn visit_expression(
        &mut self,
        _op: &mut AbstractSyntaxNode,
        _lhs: &mut AbstractSyntaxNode,
        _rhs: &mut AbstractSyntaxNode,
        _type_id: &mut ResolvableType
    ) {
        todo!("expressions as call args")
    }

    fn visit_procedure_call(
        &mut self,
        _name: &mut String,
        _args: &mut AbstractSyntaxChildNodes,
        _type_id: &mut ResolvableType
    ) {
        todo!("proc call as call args")
    }

    fn visit_foreign_system_library(&mut self, library: &mut AbstractSyntaxNode) {
        panic!("foreign system library as call args")
    }
}
