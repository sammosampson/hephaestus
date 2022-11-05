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
    
    let (intemediate_representation_handle, ..) = start_actor(
        ctx, 
        create_intemediate_representation_actor()
    );

    let compiler_handle = create_self_handle(&ctx);

    send_message_to_actor(
        &intemediate_representation_handle, 
        create_build_byte_code_command(unit, compiler_handle)
    );

    continue_listening_after_receive()
}