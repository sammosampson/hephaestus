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
            CompilationMessage::AssembleByteCode { mut unit, compiler } =>
                assemble_bytecode(&mut unit, &compiler),
            _ => continue_listening_after_receive()
        }
    }
}

fn assemble_bytecode(unit: &mut CompilationUnit, compiler: &CompilationActorHandle) -> AfterReceiveAction {
    let mut visitor = create_root_assembly_ast_node_visitor();

    apply_visitor_to_ast_root(&mut unit.tree, &mut visitor);

    send_message_to_actor(
        compiler, 
        create_byte_code_assembled_event(create_intermediate_representation(unit.id, visitor.generated_code)));

    shutdown_after_receive()
}

pub struct RootAssemblyAstNodeVisitor {
    generated_code: ByteCodeInstructionStream
}

fn create_root_assembly_ast_node_visitor() -> RootAssemblyAstNodeVisitor {
    RootAssemblyAstNodeVisitor { generated_code: vec!() }
}

impl AbstractSyntaxRootNodeVisitor for RootAssemblyAstNodeVisitor {
    fn visit_run(&mut self, _expr: &mut AbstractSyntaxNode) {
    }

    fn visit_procedure_header(
        &mut self,
        _name: &mut String,
        _args: &mut AbstractSyntaxChildNodes,
        _return_types: &mut AbstractSyntaxChildNodes,
        _body: &mut ProcedureBodyReference
    ) {
    }

    fn visit_procedure_body(
        &mut self, 
        _args: &mut AbstractSyntaxChildNodes,
        _return_types: &mut AbstractSyntaxChildNodes,
        _statements: &mut AbstractSyntaxChildNodes
    ) {
        self.generated_code.push(
            ByteCodeInstruction::AssignToNumericLiteral { 
                to: ByteCodeValue::Register(0),
                from: ByteCodeValue::S64(1)
            }
        );
        self.generated_code.push(ByteCodeInstruction::Return);
    }
}