use crate::{
    compilation::*,
    parsing::*,
    sizing::*,
    acting::*,
    file_system::*,
    backends::*,
    errors::*,
};

pub fn start_sizing_actor(ctx: &CompilationMessageContext) -> CompilationActorHandle {
    let (sizing_handle, ..) = start_actor(
        ctx, 
        create_sizing_actor()
    );
    sizing_handle
}

pub fn perform_sizing(
    type_repository: CompilationActorHandle,
    sizing_handle: CompilationActorHandle,
    compiler_handle: CompilationActorHandle,
    unit: CompilationUnit
) {
    send_message_to_actor(
        &sizing_handle, 
        create_perform_sizing_command(unit, type_repository, compiler_handle)
    );
}

pub fn handle_unit_sized<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    errors: CompilationErrors,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    end_compilation_phase_in_statistics(&mut compiler.statistics, sizing_compilation_phase(), unit.id, ctx);
    
    if handle_any_errors(compiler, &errors) {
        return continue_listening_after_receive();
    }
    
    start_compilation_phase_in_statistics(&mut compiler.statistics, bytecode_creation_compilation_phase(), unit.id);
    let intemediate_representation_handle = start_bytecode_creation_actor(ctx);
    let compiler_handle = create_self_handle(&ctx);
    perform_bytecode_creation(intemediate_representation_handle, unit, compiler_handle);

    continue_listening_after_receive()
}