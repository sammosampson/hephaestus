
use crate::{
    parsing::*,
    acting::*,
    file_system::*,
    intermediate_representation::*,
    backends::*,
    compilation::*,
};

use log::*;

pub fn handle_byte_code_built<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    unit: CompilationUnit,
    code: IntermediateRepresentation,
    ctx: &CompilationMessageContext,
    backend: TBackend
) -> AfterReceiveAction {

    debug!("handling byte code built for {:?} {:?}", code.top_level_symbol, code.id);

    if handle_any_errors(compiler, &unit.filename, &unit.errors) {
        return continue_listening_after_receive();
    }
    
    let byte_code_runner = start_backend_actor(ctx, backend);

    let compiler_handle = create_self_handle(&ctx);

    build_backend(byte_code_runner, code, compiler_handle);
    continue_listening_after_receive()
}

fn start_backend_actor<TBackend: BackendBuild>(ctx: &ActorContext<CompilationMessage>, backend: TBackend) -> ActorHandle<CompilationMessage> {
    let (byte_code_runner, ..) = start_actor(
        ctx, 
        create_backend_actor(backend)
    );
    byte_code_runner
}

fn build_backend(byte_code_runner: ActorHandle<CompilationMessage>, code: IntermediateRepresentation, compiler_handle: ActorHandle<CompilationMessage>) {
    send_message_to_actor(
        &byte_code_runner, 
        create_build_backend_command(code, compiler_handle)
    );
}
