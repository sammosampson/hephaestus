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
    
        let mut visitor = create_procedure_body_bytecode_build_ast_node_visitor(&mut self.ir);
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

pub struct ProcedureBodyByteCodeBuildAstNodeVisitor<'a> {
    ir: &'a mut IntermediateRepresentation,
    current_arg_count: usize
}

fn create_procedure_body_bytecode_build_ast_node_visitor<'a>(ir: &'a mut IntermediateRepresentation) -> ProcedureBodyByteCodeBuildAstNodeVisitor<'a> {
    ProcedureBodyByteCodeBuildAstNodeVisitor { 
        ir,
        current_arg_count: 0
    }
}

impl<'a> AbstractSyntaxProcedureBodyNodeVisitor for ProcedureBodyByteCodeBuildAstNodeVisitor<'a> {
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
    }

    fn visit_procedure_call(
        &mut self,
        _name: &mut String,
        _args: &mut AbstractSyntaxChildNodes,
        _type_id: &mut ResolvableType
    ) {
    }

    fn visit_assignment(
        &mut self,
        _name: &mut String,
        _value: &mut AbstractSyntaxNode,
        _type_id: &mut ResolvableType
    ) {
    }

    fn visit_return_statement(&mut self, _args: &mut AbstractSyntaxChildNodes) {
    }
}