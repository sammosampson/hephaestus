use crate::{
    compilation::*,
    parsing::*,
    typing::*,
    acting::*,
    file_system::*,
    backends::*,
    types::*
};

pub fn start_typing_actor(compiler: CompilationActorHandle, type_repository: CompilationActorHandle, error_reporter: CompilationActorHandle, ctx: &CompilationMessageContext) -> CompilationActorHandle {
    let (typing_handle, ..) = start_actor(
        &ctx, 
        create_typing_actor(compiler, type_repository, error_reporter)
    );
    typing_handle
}

pub fn perform_typing(
    typing_handle: CompilationActorHandle,
    unit: CompilationUnit,
    has_prior_errors: bool
) {
    send_message_to_actor(
        &typing_handle, 
        create_perform_typing_command(
            unit, 
            has_prior_errors
        )
    );
}
pub fn handle_unit_typed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    resolved_types: RuntimeTypePointers,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    end_compilation_phase_in_statistics(&mut compiler.statistics, typing_compilation_phase(), unit.id, ctx);

    for resolved_type in resolved_types {
        add_resolved_type(compiler.type_repository.clone(), resolved_type);
    }
    
    start_compilation_phase_in_statistics(&mut compiler.statistics, sizing_compilation_phase(), unit.id);
    let sizing_handle = start_sizing_actor(create_self_handle(&ctx), compiler.type_repository.clone(), ctx);
    perform_sizing(sizing_handle, unit, compiler.errors_have_occurred);

    continue_listening_after_receive()
}

fn add_resolved_type(
    type_repository: CompilationActorHandle,
    resolved_type: RuntimeTypePointer
) {
    send_message_to_actor(&type_repository, create_add_resolved_type_command(resolved_type));
}
