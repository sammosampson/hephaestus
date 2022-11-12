use crate::{
    compilation::*,
    parsing::*,
    sizing::*,
    acting::*,
    file_system::*,
    backends::*
};

pub fn start_sizing_actor(compiler: CompilationActorHandle, type_repository: CompilationActorHandle, ctx: &CompilationMessageContext) -> CompilationActorHandle {
    let (sizing_handle, ..) = start_actor(
        ctx, 
        create_sizing_actor(compiler, type_repository)
    );
    sizing_handle
}

pub fn perform_sizing(
    sizing_handle: CompilationActorHandle,
    unit: CompilationUnit,
    has_prior_errors: bool
) {
    send_message_to_actor(
        &sizing_handle, 
        create_perform_sizing_command(unit, has_prior_errors)
    );
}

pub fn handle_unit_sized<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    end_compilation_phase_in_statistics(&mut compiler.statistics, sizing_compilation_phase(), unit.id, ctx);
    
    start_compilation_phase_in_statistics(&mut compiler.statistics, bytecode_creation_compilation_phase(), unit.id);
    let intemediate_representation_handle = start_bytecode_creation_actor(ctx, &compiler.error_reporter);
    perform_bytecode_creation(intemediate_representation_handle, unit, compiler.errors_have_occurred);

    continue_listening_after_receive()
}