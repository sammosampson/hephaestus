use crate::{
    compilation::*,
    parsing::*,
    typing::*,
    acting::*,
    file_system::*,
    backends::*,
    types::*
};

#[derive(Clone, Debug)]
pub enum TypeRequestCircuitBreakReason {
    CompilationError(CompilationErrorItem),
    TypesNotFound
}

pub fn compilation_error_type_request_circuit_break_reason(error: CompilationErrorItem) -> TypeRequestCircuitBreakReason {
    TypeRequestCircuitBreakReason::CompilationError(error)
}

pub fn types_not_found_type_request_circuit_break_reason() -> TypeRequestCircuitBreakReason {
    TypeRequestCircuitBreakReason::TypesNotFound
}

pub fn perform_typing<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    unit: CompilationUnit,
    ctx: &CompilationMessageContext
) {
    start_compilation_phase(&mut compiler.statistics, typing_compilation_phase(unit.id));

    let typing_handle = start_typing_actor(
        create_self_handle(ctx), 
        compiler.type_repository.clone(),
        compiler.error_reporter.clone(),
        unit.id,
        ctx
    );
    
    send_perform_typing_command_to_actor(typing_handle, unit, compiler.errors_have_occurred);
}

fn start_typing_actor(
    compiler: CompilationActorHandle,
    type_repository: CompilationActorHandle,
    error_reporter: CompilationActorHandle,
    unit_id: CompilationUnitId,
    ctx: &CompilationMessageContext
) -> CompilationActorHandle {
    let (typing_handle, ..) = start_actor(
        &ctx, 
        create_typing_actor(compiler, type_repository, error_reporter, unit_id)
    );
    typing_handle
}

fn send_perform_typing_command_to_actor(
    typing_handle: CompilationActorHandle,
    unit: CompilationUnit,
    has_prior_errors: bool
) {
    send_message_to_actor(
        &typing_handle, 
        create_perform_typing_command(
            unit, 
            has_prior_errors
        )
    );
}

pub fn circuit_break_all_type_requests (type_repository: &CompilationActorHandle, reason: TypeRequestCircuitBreakReason) {
    send_message_to_actor(type_repository, circuit_break_all_type_requests_command(reason));
}

pub fn handle_unit_typed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    resolved_types: RuntimeTypePointers,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    for resolved_type in resolved_types {
        add_resolved_type(compiler.type_repository.clone(), resolved_type);
    }
    
    end_compilation_phase(&mut compiler.statistics, &compiler.type_repository, typing_compilation_phase(unit.id), ctx);

    perform_sizing(compiler, unit, ctx);

    continue_listening_after_receive()
}

pub fn handle_type_find_requested<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    awaiting_unit_id: CompilationUnitId
) -> AfterReceiveAction {
    await_unit_in_statistics(&mut compiler.statistics, &compiler.type_repository, awaiting_unit_id);
    continue_listening_after_receive()
}

fn add_resolved_type(
    type_repository: CompilationActorHandle,
    resolved_type: RuntimeTypePointer
) {
    send_message_to_actor(&type_repository, create_add_resolved_type_command(resolved_type));
}
