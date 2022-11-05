
use crate::{
    compilation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*,
    errors::*,
    utilities::*,
};

use log::*;

pub fn handle_file_parsed_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> (
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    parse_result: FileParseResult,
    ctx: &ActorContext<CompilationMessage>
) -> AfterReceiveAction {
    
    match parse_result {
        FileParseResult::CompilationUnits { units, .. } => process_parsed_compilation_units_in_error_state(compiler, units, ctx),
        FileParseResult::NotFound(filename) => process_parse_file_not_found_in_error_state(compiler, filename)
    }
}

fn process_parsed_compilation_units_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    units: CompilationUnits,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    for unit in units {
        handle_after_compile_in_error_state(compiler, unit.id, &unit.filename, unit.errors, ctx);
    }
    continue_listening_after_receive()
}

fn process_parse_file_not_found_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    filename: String
) -> AfterReceiveAction {
    handle_any_errors_in_error_state(compiler, "", &create_errors_for_file_not_found(filename));
    continue_listening_after_receive()
}

pub fn handle_backend_built_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    id: CompilationUnitId,
    result: BackendErrorResult,
    ctx: &ActorContext<CompilationMessage>
) -> AfterReceiveAction {

    let errors = create_errors_for_backend_error_result(result);
    handle_after_compile_in_error_state(compiler, id, "", errors, ctx)
}

pub fn create_errors_for_backend_error_result(result: BackendErrorResult) -> CompilationErrors {
    let mut errors = create_compilation_errors();
    if let Err(error) = result {
        add_compilation_error(&mut errors, create_compilation_error(backend_error(error), no_position()));
    }
    errors
}

pub fn handle_after_compile_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    id: CompilationUnitId,
    filename: &str,
    errors: CompilationErrors,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    debug!("handling after compile in error state for {:?}", id);
    
    handle_any_errors_in_error_state(compiler, filename, &errors);

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

pub fn handle_any_errors<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    filename: &str,
    errors: &CompilationErrors
) -> bool {
    
    if errors.len() == 0 {
        return false;
    }

    send_message_to_actor(
        &compiler.type_repository, 
        create_shutdown_command()
    );

    send_message_to_actor(
        &compiler.error_reporter, 
        create_report_errors_command(string(filename), errors.clone())
    );
    
    compiler.errors_exist = true;

    return true
}

fn handle_any_errors_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,  
    filename: &str,
    errors: &CompilationErrors,
) {
    if errors.len() > 0 {
        send_message_to_actor(
            &compiler.error_reporter, 
            create_report_errors_command(string(filename), errors.clone())
        );
    }
}

fn create_errors_for_file_not_found(filename: String) -> CompilationErrors {
    let mut errors = create_compilation_errors();
    add_compilation_error(&mut errors, create_compilation_error(file_not_found_error(filename), no_position()));
    errors
}

