
use crate::{
    parsing::*,
    acting::*,
    file_system::*,
    intermediate_representation::*,
    backends::*,
    compilation::*,
};

pub fn start_bytecode_creation_actor(ctx: &CompilationMessageContext) -> CompilationActorHandle {
    let (intemediate_representation_handle, ..) = start_actor(
        ctx, 
        create_intemediate_representation_actor()
    );
    intemediate_representation_handle
}

pub fn perform_bytecode_creation(intemediate_representation_handle: CompilationActorHandle, unit: CompilationUnit, compiler_handle: CompilationActorHandle) {
    send_message_to_actor(
        &intemediate_representation_handle, 
        create_build_byte_code_command(unit, compiler_handle)
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
    
    if handle_any_errors(compiler, &unit.filename, &unit.errors) {
        return continue_listening_after_receive();
    }
    
    start_compilation_phase_in_statistics(&mut compiler.statistics, backend_build_compilation_phase(), unit.id);
    let byte_code_runner = start_backend_actor(ctx, backend);
    let compiler_handle = create_self_handle(&ctx);
    build_backend(byte_code_runner, code, compiler_handle);

    continue_listening_after_receive()
}