use crate::{
    compilation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*
};

use log::*;

pub fn parse_file<TReader: FileRead>(file_name: String, ctx: &CompilationMessageContext, error_reporter: &CompilationActorHandle, reader: TReader) {
    debug!("handling compile for: {:?}", &file_name);

    let parser_handle = start_parser_actor(ctx, error_reporter, reader);

    send_parse_file_command_to_actor(parser_handle, file_name);
}

fn start_parser_actor<TReader: FileRead>(ctx: &CompilationMessageContext, error_reporter: &CompilationActorHandle, reader: TReader) -> CompilationActorHandle {
    let (parser_handle, ..) = start_actor(
        ctx, 
        create_parser_actor(create_self_handle(ctx), error_reporter.clone(), reader)
    );
    parser_handle
}

fn send_parse_file_command_to_actor(parser_handle: CompilationActorHandle, file_name: String) {
    send_message_to_actor(
        &parser_handle, 
        create_parse_file_command(file_name)
    );
}

pub fn handle_file_parsed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    units: CompilationUnits,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    register_units_with_statistics(&mut compiler.statistics, &units);    

    for unit in units {
        perform_typing(compiler, unit, ctx);
    }

    continue_listening_after_receive()
}



