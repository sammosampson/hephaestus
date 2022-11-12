
use crate::{
    parsing::*,
    acting::*,
    file_system::*,
    intermediate_representation::*,
    backends::*,
    compilation::*
};

pub fn perform_byte_code_creation<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    ctx: &CompilationMessageContext
) {
    start_compilation_phase(&mut compiler.statistics, byte_code_creation_compilation_phase(unit.id));

    let intemediate_representation_handle = start_byte_code_creation_actor(
        ctx,
        &compiler.error_reporter
    );

    send_build_byte_code_command_to_actor(intemediate_representation_handle, unit, compiler.errors_have_occurred);
}

fn start_byte_code_creation_actor(ctx: &CompilationMessageContext, error_reporter: &CompilationActorHandle) -> CompilationActorHandle {
    let (intemediate_representation_handle, ..) = start_actor(
        ctx, 
        create_intemediate_representation_actor(create_self_handle(ctx), error_reporter.clone())
    );
    intemediate_representation_handle
}

fn send_build_byte_code_command_to_actor(
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

    end_compilation_phase(&mut compiler.statistics, byte_code_creation_compilation_phase(unit.id), ctx);
    
    build_backend(compiler, ctx, backend, unit, code);

    continue_listening_after_receive()
}