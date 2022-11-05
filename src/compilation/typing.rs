use crate::{
    compilation::*,
    parsing::*,
    sizing::*,
    acting::*,
    file_system::*,
    backends::*,
    types::*,
};

use log::*;

pub fn handle_unit_typed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    resolved_types: RuntimeTypePointers,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    debug!("handling unit typed for {:?}", unit.id);

    if handle_any_errors(compiler, &unit.filename, &unit.errors) {
        return continue_listening_after_receive();
    }
    
    for resolved_type in resolved_types {
        add_resolved_type(compiler, resolved_type);
    }
    
    let sizing_handle = start_sizing_actor(ctx);

    let compiler_handle = create_self_handle(&ctx);

    perform_sizing(compiler, unit, sizing_handle, compiler_handle);

    continue_listening_after_receive()
}

fn add_resolved_type<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    resolved_type: RuntimeTypePointer
) {
    send_message_to_actor(&compiler.type_repository, create_add_resolved_type_command(resolved_type));
}

fn start_sizing_actor(ctx: &ActorContext<CompilationMessage>) -> ActorHandle<CompilationMessage> {
    let (sizing_handle, ..) = start_actor(
        ctx, 
        create_sizing_actor()
    );
    sizing_handle
}

fn perform_sizing<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    sizing_handle: ActorHandle<CompilationMessage>,
    compiler_handle: ActorHandle<CompilationMessage>
) {
    send_message_to_actor(
        &sizing_handle, 
        create_perform_sizing_command(unit, compiler.type_repository.clone(), compiler_handle)
    );
}
