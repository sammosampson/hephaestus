use std::collections::HashMap;

use crate::{
    parsing::*,
    typing::*,
    sizing::*,
    acting::*,
    file_system::*,
    intermediate_representation::*,
    backends::*,
    types::*,
    errors::*,
    utilities::*
};

use log::*;

#[derive(Clone, Debug)]
pub enum CompilationMessage {
    Compile(String),
    ParseFile(String, CompilationActorHandle),
    FileParsed(FileParseResult),
    PerformTyping { unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle },
    UnitTyped { resolved_types: RuntimeTypePointers, unit: CompilationUnit },
    PerformSizing { unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle },
    UnitSized { unit: CompilationUnit },
    FindType { criteria: FindTypeCriteria, respond_to: CompilationActorHandle },
    TypeFound(RuntimeTypePointer),
    TypeRequestReleaseDueToError(CompilationErrorItem),
    AddResolvedType(RuntimeTypePointer),
    BuildByteCode { unit: CompilationUnit, compiler: CompilationActorHandle },
    ByteCodeBuilt { unit: CompilationUnit, code: IntermediateRepresentation },
    BuildBackend { code: IntermediateRepresentation, compiler: CompilationActorHandle },
    BackendBuilt { id: CompilationUnitId, result: BackendErrorResult },
    CompilationComplete,
    ReportErrors { filename: String, errors: CompilationErrors },
    ShutDown,
}

pub trait WireTapCompilationMessage : Send + 'static {
    fn tap(&mut self, message: &CompilationMessage);
}

pub struct NullCompilationMessageWireTap;

impl WireTapCompilationMessage for NullCompilationMessageWireTap {
    fn tap(&mut self, _message: &CompilationMessage) {
    }
}

pub fn create_null_message_wire_tap() -> NullCompilationMessageWireTap {
    NullCompilationMessageWireTap
}

pub type CompilationActorHandle = ActorHandle<CompilationMessage>;
pub type CompilationMessageContext = ActorContext<CompilationMessage>;

fn create_compile_command(file_name: String) -> CompilationMessage {
    CompilationMessage::Compile(file_name)
}

pub fn create_parse_file_command(file_name: String, handle: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::ParseFile(file_name, handle)
}

pub fn create_perform_typing_command(unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::PerformTyping { unit, type_repository, compiler }
}

pub fn create_perform_sizing_command(unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::PerformSizing { unit, type_repository, compiler }
}

pub fn create_find_type_request(criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::FindType { criteria, respond_to }
}

pub fn create_file_parsed_event(parse_result: FileParseResult) -> CompilationMessage {
    CompilationMessage::FileParsed(parse_result)
}

pub fn create_unit_typed_event(resolved_types: RuntimeTypePointers, unit: CompilationUnit) -> CompilationMessage {
    CompilationMessage::UnitTyped { resolved_types, unit }
}

pub fn create_unit_sized_event(unit: CompilationUnit) -> CompilationMessage {
    CompilationMessage::UnitSized { unit }
}

pub fn create_type_found_event(resolved_type: RuntimeTypePointer) -> CompilationMessage {
    CompilationMessage::TypeFound(resolved_type)
}

pub fn type_request_released_due_to_error_event(error: CompilationErrorItem) -> CompilationMessage {
    CompilationMessage::TypeRequestReleaseDueToError(error)
}

pub fn create_add_resolved_type_command(resolved_type: RuntimeTypePointer) -> CompilationMessage {
    CompilationMessage::AddResolvedType(resolved_type)
}

pub fn create_build_byte_code_command(unit: CompilationUnit, compiler: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::BuildByteCode { unit, compiler }
}

pub fn create_byte_code_built_event(unit: CompilationUnit, code: IntermediateRepresentation) -> CompilationMessage {
    CompilationMessage::ByteCodeBuilt { unit, code }
}

pub fn create_build_backend_command(code: IntermediateRepresentation, compiler: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::BuildBackend { code, compiler }
}

pub fn create_backend_built_event(id: CompilationUnitId, result: BackendErrorResult) -> CompilationMessage {
    CompilationMessage::BackendBuilt { id, result }
}

fn create_compilation_complete_event() -> CompilationMessage {
    CompilationMessage::CompilationComplete
}


fn create_report_errors_command(filename: String, errors: CompilationErrors) -> CompilationMessage {
    CompilationMessage::ReportErrors { filename, errors }
}

fn create_shutdown_command() -> CompilationMessage {
    CompilationMessage::ShutDown
}

pub fn compile<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    file_name: String,
    reader: TReader,
    backend: TBackend,
    message_wire_tap: TMessageWireTap
) {
    let (type_repository_handle, ..) = start_singleton_actor(create_type_repository_actor());
    let (error_reporter_handle, ..) = start_singleton_actor(create_error_reporter_actor());
    let (compiler_handle, compiler_shutdown_notifier) = start_singleton_actor(
        create_compiler_actor(type_repository_handle, error_reporter_handle, reader, backend, message_wire_tap)
    );
    
    send_message_to_actor(
        &compiler_handle, 
        create_compile_command(file_name)
    );

    await_shutdown(&compiler_shutdown_notifier);
}

type CompilationUnitsRequestedList = HashMap<CompilationUnitId, CompilationUnitId>;

fn create_compilation_units_requested_list() -> HashMap<CompilationUnitId, CompilationUnitId> {
    HashMap::default()
}

fn register_compilation_requested(lookup: &mut CompilationUnitsRequestedList, id: CompilationUnitId) {
    lookup.insert(id, id);
}

fn remove_unit_from_compilation_requested_list(lookup: &mut CompilationUnitsRequestedList, id: &CompilationUnitId) {
    lookup.remove(id);
}

fn compilation_requested_list_is_empty(lookup: &CompilationUnitsRequestedList) -> bool {
    lookup.is_empty()
}

struct CompilerActor<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> {
    compilation_units_requested_list: CompilationUnitsRequestedList,
    type_repository: CompilationActorHandle,
    error_reporter: CompilationActorHandle,
    reader: TReader,
    backend: TBackend,
    message_wire_tap: TMessageWireTap,
    errors_exist: bool
}

fn create_compiler_actor<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    type_repository: CompilationActorHandle,
    error_reporter: CompilationActorHandle,
    reader: TReader,
    backend: TBackend, 
    message_wire_tap: TMessageWireTap
) -> CompilerActor<TReader, TBackend, TMessageWireTap> {
    CompilerActor {
        compilation_units_requested_list: create_compilation_units_requested_list(),
        type_repository,
        error_reporter, 
        reader,
        backend,
        message_wire_tap,
        errors_exist: false
    }
}

impl <TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> Actor<CompilationMessage> for CompilerActor<TReader, TBackend, TMessageWireTap> {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        self.message_wire_tap.tap(&message);

        if self.errors_exist {
            return self.error_state_handling(message, ctx)
        }

        self.normal_state_handling(message, ctx)
    }
}

impl<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> CompilerActor<TReader, TBackend, TMessageWireTap> {
    fn normal_state_handling(&mut self, message: CompilationMessage, ctx: &ActorContext<CompilationMessage>) -> AfterReceiveAction {
        match message {
            CompilationMessage::Compile(file_name) =>
                handle_compile(file_name, ctx, self.reader.clone()),
            CompilationMessage::FileParsed(parse_result) =>
                handle_file_parsed(self, parse_result, ctx),
            CompilationMessage::UnitTyped { resolved_types, unit } => 
                handle_unit_typed(self, unit, resolved_types, ctx),
            CompilationMessage::UnitSized { unit } => 
                handle_unit_sized(self, unit, ctx),
            CompilationMessage::ByteCodeBuilt { code, unit } => 
                handle_byte_code_built(self, unit, code, ctx, self.backend.clone()),
            CompilationMessage::BackendBuilt { id, result } => 
                handle_backend_built(self, id, result, ctx),
            CompilationMessage::CompilationComplete => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }

    fn error_state_handling(&mut self, message: CompilationMessage, ctx: &ActorContext<CompilationMessage>) -> AfterReceiveAction {
        match message {
            CompilationMessage::Compile(file_name) =>
                handle_compile(file_name, ctx, self.reader.clone()),
            CompilationMessage::FileParsed(parse_result) =>
                handle_file_parsed_in_error_state(self, parse_result, ctx),
            CompilationMessage::UnitTyped { unit, .. } => 
                handle_after_compile_in_error_state(self, unit.id, &unit.filename, unit.errors, ctx),
            CompilationMessage::UnitSized { unit } => 
                handle_after_compile_in_error_state(self, unit.id, &unit.filename, unit.errors, ctx),
            CompilationMessage::ByteCodeBuilt { unit, .. } => 
                handle_after_compile_in_error_state(self, unit.id, &unit.filename, unit.errors, ctx),
            CompilationMessage::BackendBuilt { id, result } => 
                handle_backend_built_in_error_state(self, id, result, ctx),
            CompilationMessage::CompilationComplete => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_file_parsed_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> (
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

fn handle_backend_built_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    id: CompilationUnitId,
    result: BackendErrorResult,
    ctx: &ActorContext<CompilationMessage>
) -> AfterReceiveAction {

    let errors = create_errors_for_backend_error_result(result);
    handle_after_compile_in_error_state(compiler, id, "", errors, ctx)
}

fn create_errors_for_backend_error_result(result: BackendErrorResult) -> CompilationErrors {
    let mut errors = create_compilation_errors();
    if let Err(error) = result {
        add_compilation_error(&mut errors, create_compilation_error(backend_error(error), no_position()));
    }
    errors
}

fn handle_after_compile_in_error_state<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
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

fn handle_any_errors<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
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

fn handle_compile<TReader: FileRead>(
    file_name: String,
    ctx: &CompilationMessageContext,
    reader: TReader
) -> AfterReceiveAction {

    debug!("handling compile for: {:?}", &file_name);
        
    let (parser_handle, ..) = start_actor(
        ctx, 
        create_parser_actor(reader)
    );

    let compiler_handle = create_self_handle(ctx);
    
    send_message_to_actor(
        &parser_handle, 
        create_parse_file_command(file_name, compiler_handle)
    );

    continue_listening_after_receive()
}

fn handle_file_parsed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    parse_result: FileParseResult,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    match parse_result {
        FileParseResult::CompilationUnits { units, .. } => process_parsed_compilation_units(compiler, units, ctx),
        FileParseResult::NotFound(filename) => process_parse_file_not_found(compiler, filename)
    }
}

fn handle_unit_typed<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    resolved_types: RuntimeTypePointers,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    debug!("handling unit typed for {:?}", unit.id);

    if handle_any_errors(compiler, &unit.filename, &unit.errors) {
        return continue_listening_after_receive();
    }
    
    for resolved_type in resolved_types {
        send_message_to_actor(&compiler.type_repository, create_add_resolved_type_command(resolved_type));
    }
    
    let (sizing_handle, ..) = start_actor(
        ctx, 
        create_sizing_actor()
    );

    let compiler_handle = create_self_handle(&ctx);

    send_message_to_actor(
        &sizing_handle, 
        create_perform_sizing_command(unit, compiler.type_repository.clone(), compiler_handle)
    );

    continue_listening_after_receive()
}

fn handle_unit_sized<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>, 
    unit: CompilationUnit,
    ctx: &CompilationMessageContext
) -> AfterReceiveAction {
    
    debug!("handling unit sized for {:?}", unit.id);

    if handle_any_errors(compiler, &unit.filename, &unit.errors) {
        return continue_listening_after_receive();
    }
    
    let (intemediate_representation_handle, ..) = start_actor(
        ctx, 
        create_intemediate_representation_actor()
    );

    let compiler_handle = create_self_handle(&ctx);

    send_message_to_actor(
        &intemediate_representation_handle, 
        create_build_byte_code_command(unit, compiler_handle)
    );

    continue_listening_after_receive()
}

fn handle_byte_code_built<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    compiler: &mut CompilerActor<TReader, TBackend, TMessageWireTap>,
    unit: CompilationUnit,
    code: IntermediateRepresentation,
    ctx: &CompilationMessageContext,
    backend: TBackend
) -> AfterReceiveAction {

    debug!("handling byte code built for {:?} {:?}", code.top_level_symbol, code.id);

    if handle_any_errors(compiler, &unit.filename, &unit.errors) {
        return continue_listening_after_receive();
    }
    
    let (byte_code_runner, ..) = start_actor(
        ctx, 
        create_backend_actor(backend)
    );

    let compiler_handle = create_self_handle(&ctx);

    send_message_to_actor(
        &byte_code_runner, 
        create_build_backend_command(code, compiler_handle)
    );

    continue_listening_after_receive()
}

fn handle_backend_built<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
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

