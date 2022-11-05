use crate::{
    compilation::*,
    parsing::*,
    typing::*,
    acting::*,
    file_system::*,
    backends::*,
    errors::*,
};

use log::*;

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

    debug!("process parsed compilation units for {:?} units", units.len());
    
    for unit in &units {
        register_compilation_requested(&mut compiler.compilation_units_requested_list, unit.id);
    }

    debug!("unit requsted list is now {:?}", &compiler.compilation_units_requested_list.keys());
    

    for unit in units {
        let (typing_handle, ..) = start_actor(
            &ctx, 
            create_typing_actor()
        );
        
        send_message_to_actor(
            &typing_handle, 
            create_perform_typing_command(
                unit, 
                compiler.type_repository.clone(), 
                create_self_handle(ctx)
            )
        );
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

