use crate::{
    compilation::*,
    intermediate_representation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*,
};

pub fn start_backend_actor<TBackend: BackendBuild>(ctx: &CompilationMessageContext, backend: TBackend) -> CompilationActorHandle {
    let (byte_code_runner, ..) = start_actor(
        ctx, 
        create_backend_actor(backend)
    );
    byte_code_runner
}

pub fn build_backend(byte_code_runner: CompilationActorHandle, code: IntermediateRepresentation, compiler_handle: CompilationActorHandle, has_prior_errors: bool) {
    send_message_to_actor(
        &byte_code_runner, 
        create_build_backend_command(code, compiler_handle, has_prior_errors)
    );
}

pub fn handle_backend_built<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    id: CompilationUnitId,
    result: BackendErrorResult,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {    
    end_compilation_phase_in_statistics(&mut compiler.statistics, backend_build_compilation_phase(), id, ctx);
    
    if handle_any_errors(compiler, &create_errors_for_backend_error_result(result)) {
        return continue_listening_after_receive();
    }
    
    continue_listening_after_receive()
}