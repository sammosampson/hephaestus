use crate::{
    compilation::*,
    parsing::*,
    acting::*,
    file_system::*,
    backends::*
};

pub fn handle_file_parsed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    units: CompilationUnits,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    register_units_with_statistics(&mut compiler.statistics, &units);    

    for unit in units {
        start_compilation_phase_in_statistics(&mut compiler.statistics, typing_compilation_phase(), unit.id);
        
        let typing_handle = start_typing_actor(
            create_self_handle(ctx), 
            compiler.type_repository.clone(), 
            compiler.error_reporter.clone(),
            ctx
        );
        perform_typing(typing_handle, unit, compiler.errors_have_occurred);
    }

    continue_listening_after_receive()
}

