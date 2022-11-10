
use crate::{
    compilation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*,
    errors::*,
    utilities::*,
};

pub fn create_errors_for_backend_error_result(result: BackendErrorResult) -> CompilationErrors {
    let mut errors = create_compilation_errors(empty_string());
    if let Err(error) = result {
        add_compilation_error(&mut errors, compilation_error(backend_error(error), no_position()));
    }
    errors
}

pub fn handle_any_errors<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    errors: &CompilationErrors
) -> bool {
    
    if !are_any_compilation_errors(errors) {
        return false;
    }

    report_on_errors(compiler, errors);

    return true
}

fn report_on_errors<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    errors: &CompilationErrors
) {
    send_message_to_actor(
        &compiler.error_reporter, 
        create_report_errors_command(errors.clone())
    );
}
