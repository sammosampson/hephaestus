use crate::{
    compilation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*,
    utilities::*
};

pub fn parse_file<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> (
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    file_name: String,
    ctx: &CompilationMessageContext
) {
    start_compilation_phase(&mut compiler.statistics, parsing_compilation_phase(string(&file_name)));

    let parser_handle = start_parser_actor(ctx, &compiler.error_reporter, compiler.reader.clone());

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
    file_name: String,
    units: CompilationUnits,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    end_compilation_phase(&mut compiler.statistics, parsing_compilation_phase(string(&file_name)), ctx);

    for unit in units {
        perform_typing(compiler, unit, ctx);
    }

    continue_listening_after_receive()
}



