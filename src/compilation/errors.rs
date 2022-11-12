
use crate::{
    compilation::*,
    file_system::*,
    acting::*,
    backends::*,
    errors::*,
};



pub fn handle_errors_reported<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> (
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>
) -> AfterReceiveAction {
    compiler.errors_have_occurred = true;
    continue_listening_after_receive()
}

pub fn report_errors(
    error_reporter: &CompilationActorHandle,
    compiler: CompilationActorHandle,
    errors: CompilationErrors
) {
    send_message_to_actor(
        error_reporter, 
        create_report_errors_command(errors, compiler)
    );
}