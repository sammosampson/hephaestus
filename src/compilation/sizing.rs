use crate::{
    compilation::*,
    parsing::*,
    sizing::*,
    acting::*,
    file_system::*,
    backends::*
};

pub fn perform_sizing<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    ctx: &CompilationMessageContext
) {
    start_compilation_phase(&mut compiler.statistics, sizing_compilation_phase(unit.id));

    let sizing_handle = start_sizing_actor(
        create_self_handle(&ctx), 
        compiler.type_repository.clone(),
        ctx
    );

    send_perform_sizing_command_to_actor(sizing_handle, unit, compiler.errors_have_occurred);
}

fn start_sizing_actor(compiler: CompilationActorHandle, type_repository: CompilationActorHandle, ctx: &CompilationMessageContext) -> CompilationActorHandle {
    let (sizing_handle, ..) = start_actor(
        ctx, 
        create_sizing_actor(compiler, type_repository)
    );
    sizing_handle
}

fn send_perform_sizing_command_to_actor(
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
    
    end_compilation_phase(&mut compiler.statistics, &compiler.type_repository, sizing_compilation_phase(unit.id), ctx);
    
    perform_byte_code_creation(compiler, unit, ctx);

    continue_listening_after_receive()
}