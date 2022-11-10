use crate::{
    compilation::*,
    parsing::*,
    typing::*,
    acting::*,
    file_system::*,
    backends::*,
    types::*,
    errors::*,
};

pub fn start_typing_actor(ctx: &CompilationMessageContext) -> CompilationActorHandle {
    let (typing_handle, ..) = start_actor(
        &ctx, 
        create_typing_actor()
    );
    typing_handle
}

pub fn perform_typing(
    type_repository: CompilationActorHandle,
    typing_handle: CompilationActorHandle,
    unit: CompilationUnit,
    ctx: &CompilationMessageContext,
    has_prior_errors: bool
) {
    send_message_to_actor(
        &typing_handle, 
        create_perform_typing_command(
            unit, 
            type_repository, 
            create_self_handle(ctx),
            has_prior_errors
        )
    );
}
pub fn handle_unit_typed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    errors: CompilationErrors,
    resolved_types: RuntimeTypePointers,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    end_compilation_phase_in_statistics(&mut compiler.statistics, typing_compilation_phase(), unit.id, ctx);

    if handle_any_errors(compiler, &errors) {
        return continue_listening_after_receive();
    }
    
    for resolved_type in resolved_types {
        add_resolved_type(compiler.type_repository.clone(), resolved_type);
    }
    
    start_compilation_phase_in_statistics(&mut compiler.statistics, sizing_compilation_phase(), unit.id);
    let sizing_handle = start_sizing_actor(ctx);
    let compiler_handle = create_self_handle(&ctx);
    perform_sizing(compiler.type_repository.clone(), sizing_handle, compiler_handle, unit, are_any_compilation_errors(&errors));

    continue_listening_after_receive()
}

fn add_resolved_type(
    type_repository: CompilationActorHandle,
    resolved_type: RuntimeTypePointer
) {
    send_message_to_actor(&type_repository, create_add_resolved_type_command(resolved_type));
}
