use crate::{
    compilation::*,
    parsing::*,
    acting::*,
    file_system::*,
    intermediate_representation::*,
    backends::*,
};

use log::*;

pub fn handle_unit_sized<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    debug!("handling unit sized for {:?}", unit.id);

    if handle_any_errors(compiler, &unit.filename, &unit.errors) {
        return continue_listening_after_receive();
    }
    
    let intemediate_representation_handle = start_sizing_actor(ctx);

    let compiler_handle = create_self_handle(&ctx);

    perform_sizing(intemediate_representation_handle, unit, compiler_handle);
    continue_listening_after_receive()
}

fn start_sizing_actor(ctx: &ActorContext<CompilationMessage>) -> ActorHandle<CompilationMessage> {
    let (intemediate_representation_handle, ..) = start_actor(
        ctx, 
        create_intemediate_representation_actor()
    );
    intemediate_representation_handle
}

fn perform_sizing(intemediate_representation_handle: ActorHandle<CompilationMessage>, unit: CompilationUnit, compiler_handle: ActorHandle<CompilationMessage>) {
    send_message_to_actor(
        &intemediate_representation_handle, 
        create_build_byte_code_command(unit, compiler_handle)
    );
}