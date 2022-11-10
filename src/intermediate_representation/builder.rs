use crate::{
    acting::*,
    compilation::*,
    parsing::*,
    intermediate_representation::*,
    errors::*
};

pub struct IntemediateRepresentationActor;

pub fn create_intemediate_representation_actor() -> IntemediateRepresentationActor {
    IntemediateRepresentationActor
}

impl Actor<CompilationMessage> for IntemediateRepresentationActor {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::BuildByteCode { unit, compiler, has_prior_errors } =>
                build_bytecode(unit, &compiler, has_prior_errors),
            _ => continue_listening_after_receive()
        }
    }
}

fn build_bytecode(mut unit: CompilationUnit, compiler: &CompilationActorHandle, has_prior_errors: bool) -> AfterReceiveAction {    
    let mut ir = create_intermediate_representation(unit.id, unit.filename.clone());
    let mut errors = create_compilation_errors(unit.filename.clone());
    build_bytecode_at_root(&mut unit, &mut errors, &mut ir, has_prior_errors);
    notify_compiler_byte_code_built_for_unit(compiler, unit, errors, ir);
    shutdown_after_receive()
}


fn notify_compiler_byte_code_built_for_unit(
    compiler: &CompilationActorHandle,
    unit: CompilationUnit,
    errors: CompilationErrors,
    ir: IntermediateRepresentation
) {
    send_message_to_actor(compiler, create_byte_code_built_event(unit, errors, ir));
}

fn build_bytecode_at_root(unit: &mut CompilationUnit, errors: &mut CompilationErrors, ir: &mut IntermediateRepresentation, has_prior_errors: bool) {
    if has_prior_errors {
        return;
    }

    match unit.tree.item_ref() {
        AbstractSyntaxNodeItem::ProcedureHeader { name, .. } =>
            build_bytecode_at_procedure_header(ir, name),
        AbstractSyntaxNodeItem::ProcedureBody { name, args, statements, .. } =>
            build_bytecode_at_procedure_body(ir, name, args, statements, errors),
        AbstractSyntaxNodeItem::Constant { name, value, ..} =>
            build_bytecode_at_top_root_const(ir, name, value, errors),
        AbstractSyntaxNodeItem::Struct { name, ..} =>
            todo(errors, function!(), &format!("struct bytecode coming soon {}", name)),
        AbstractSyntaxNodeItem::Error => {},
        item =>
            todo(errors, function!(), &format!("Other root bytecode: {:?}", item))
    }    
}