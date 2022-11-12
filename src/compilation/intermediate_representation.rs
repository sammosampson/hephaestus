
use crate::{
    parsing::*,
    acting::*,
    file_system::*,
    intermediate_representation::*,
    backends::*,
    compilation::*
};

pub fn start_bytecode_creation_actor(ctx: &CompilationMessageContext, error_reporter: &CompilationActorHandle) -> CompilationActorHandle {
    let (intemediate_representation_handle, ..) = start_actor(
        ctx, 
        create_intemediate_representation_actor(create_self_handle(ctx), error_reporter.clone())
    );
    intemediate_representation_handle
}

pub fn perform_bytecode_creation(
    intemediate_representation_handle: CompilationActorHandle,
    unit: CompilationUnit,
    has_prior_errors: bool
) {
    send_message_to_actor(
        &intemediate_representation_handle, 
        create_build_byte_code_command(unit, has_prior_errors)
    );
}

pub fn handle_byte_code_built<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    unit: CompilationUnit,
    code: IntermediateRepresentation,
    ctx: &CompilationMessageContext,
    backend: TBackend
) -> AfterReceiveAction {

    end_compilation_phase_in_statistics(&mut compiler.statistics, bytecode_creation_compilation_phase(), unit.id, ctx);
    
    start_compilation_phase_in_statistics(&mut compiler.statistics, backend_build_compilation_phase(), unit.id);
    let byte_code_runner = start_backend_actor(
        create_self_handle(ctx), 
        compiler.error_reporter.clone(),
        backend,
        ctx
    );
    build_backend(byte_code_runner, code, compiler.errors_have_occurred);

    continue_listening_after_receive()
}