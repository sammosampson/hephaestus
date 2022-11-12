use crate::{
    compilation::*,
    intermediate_representation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*,
};

pub fn build_backend<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    ctx: &CompilationMessageContext,
    backend: TBackend,
    unit: CompilationUnit,
    code: IntermediateRepresentation) {
    start_compilation_phase(&mut compiler.statistics, backend_build_compilation_phase(), unit.id);

    let byte_code_runner = start_backend_actor(
        create_self_handle(ctx), 
        compiler.error_reporter.clone(),
        backend,
        ctx
    );
    
    send_build_backend_command_to_actor(byte_code_runner, code, compiler.errors_have_occurred);
}

fn start_backend_actor<TBackend: BackendBuild>(
    compiler: CompilationActorHandle,
    error_reporter: CompilationActorHandle,
    backend: TBackend,
    ctx: &CompilationMessageContext
) -> CompilationActorHandle {
    let (byte_code_runner, ..) = start_actor(
        ctx, 
        create_backend_actor(compiler, error_reporter, backend)
    );
    byte_code_runner
}

fn send_build_backend_command_to_actor(byte_code_runner: CompilationActorHandle, code: IntermediateRepresentation, has_prior_errors: bool) {
    send_message_to_actor(
        &byte_code_runner, 
        create_build_backend_command(code,  has_prior_errors)
    );
}

pub fn handle_backend_built<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    id: CompilationUnitId,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {    
    end_compilation_phase(&mut compiler.statistics, backend_build_compilation_phase(), id, ctx);
    continue_listening_after_receive()
}