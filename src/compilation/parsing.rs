use crate::{
    compilation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*,
    errors::*,
};

pub fn handle_file_parsed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    parse_result: FileParseResult,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    match parse_result {
        FileParseResult::CompilationUnits { units, .. } => process_parsed_compilation_units(compiler, units, ctx),
        FileParseResult::NotFound(filename) => process_parse_file_not_found(compiler, filename)
    }
}

fn process_parsed_compilation_units<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    units: CompilationUnits,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {

    register_units_with_statistics(&mut compiler.statistics, &units, ctx);    

    for unit in units {
        start_compilation_phase_in_statistics(&mut compiler.statistics, typing_compilation_phase(), unit.id);
        let typing_handle = start_typing_actor(ctx);
        perform_typing(compiler.type_repository.clone(), typing_handle, unit, ctx);
    }

    continue_listening_after_receive()
}

fn process_parse_file_not_found<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    filename: String
) -> AfterReceiveAction {
    handle_any_errors(compiler, "", &create_errors_for_file_not_found(filename));
    continue_listening_after_receive()
}

fn create_errors_for_file_not_found(filename: String) -> CompilationErrors {
    let mut errors = create_compilation_errors();
    add_compilation_error(&mut errors, create_compilation_error(file_not_found_error(filename), no_position()));
    errors
}

