use crate::{
    compilation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*,
};

use log::*;

pub fn handle_backend_built<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    id: CompilationUnitId,
    result: BackendErrorResult,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    debug!("handling byte code ran for {:?}", id);

    if handle_any_errors(compiler, "", &create_errors_for_backend_error_result(result)) {
        return continue_listening_after_receive();
    }
    
    remove_unit_from_compilation_requested_list(
        &mut compiler.compilation_units_requested_list,
        &id
    );

    debug!("unit requsted list is now {:?}", &compiler.compilation_units_requested_list.keys());
    
    if compilation_requested_list_is_empty(&compiler.compilation_units_requested_list) {
        send_message_to_actor(&create_self_handle(ctx), create_compilation_complete_event());
    }

    continue_listening_after_receive()
}
