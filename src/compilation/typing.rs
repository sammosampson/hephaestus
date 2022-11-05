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
        send_message_to_actor(&compiler.type_repository, create_add_resolved_type_command(resolved_type));
    }
    
    let (sizing_handle, ..) = start_actor(
        ctx, 
        create_sizing_actor()
    );

    let compiler_handle = create_self_handle(&ctx);

    send_message_to_actor(
        &sizing_handle, 
        create_perform_sizing_command(unit, compiler.type_repository.clone(), compiler_handle)
    );

    continue_listening_after_receive()
}
