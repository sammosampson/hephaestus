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

fn build_bytecode_at_root(unit: &mut CompilationUnit, ir: &mut IntermediateRepresentation) {
    match unit.tree.item_ref() {
        AbstractSyntaxNodeItem::ProcedureHeader { name, .. } =>
            build_bytecode_at_procedure_header(ir, name),
        AbstractSyntaxNodeItem::ProcedureBody { name, args, statements, .. } =>
            build_bytecode_at_procedure_body(ir, name, args, statements, &mut unit.errors),
        AbstractSyntaxNodeItem::Constant { name, value, ..} =>
            build_bytecode_at_top_root_const(ir, name, value, &mut unit.errors),
        AbstractSyntaxNodeItem::Struct { name, ..} =>
            todo(&mut unit.errors, function!(), &format!("struct bytecode coming soon {}", name)),
        item =>
            todo(&mut unit.errors, function!(), &format!("Other root bytecode: {:?}", item))
    }    
}