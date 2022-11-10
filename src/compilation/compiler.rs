use crate::{
    compilation::*,
    parsing::*,
    typing::*,
    acting::*,
    file_system::*,
    intermediate_representation::*,
    backends::*,
    types::*,
    errors::*,
};

use log::*;

#[derive(Clone, Debug)]
pub enum CompilationMessage {
    Compile(String),
    ParseFile(String, CompilationActorHandle),
    FileParsed(FileParseResult),
    PerformTyping { unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle, has_prior_errors: bool },
    UnitTyped { resolved_types: RuntimeTypePointers, unit: CompilationUnit, errors: CompilationErrors },
    PerformSizing { unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle, has_prior_errors: bool },
    UnitSized { unit: CompilationUnit, errors: CompilationErrors },
    FindType { criteria: FindTypeCriteria, respond_to: CompilationActorHandle },
    TypeFound(RuntimeTypePointer),
    TypeRequestReleaseDueToError(CompilationErrorItem),
    AddResolvedType(RuntimeTypePointer),
    BuildByteCode { unit: CompilationUnit, compiler: CompilationActorHandle, has_prior_errors: bool },
    ByteCodeBuilt { unit: CompilationUnit, errors: CompilationErrors, code: IntermediateRepresentation },
    BuildBackend { code: IntermediateRepresentation, compiler: CompilationActorHandle, has_prior_errors: bool },
    BackendBuilt { id: CompilationUnitId, result: BackendErrorResult },
    CompilationComplete,
    ReportErrors { errors: CompilationErrors },
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

pub fn create_perform_typing_command(unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle, has_prior_errors: bool) -> CompilationMessage {
    CompilationMessage::PerformTyping { unit, type_repository, compiler, has_prior_errors }
}

pub fn create_perform_sizing_command(unit: CompilationUnit, type_repository: CompilationActorHandle, compiler: CompilationActorHandle, has_prior_errors: bool) -> CompilationMessage {
    CompilationMessage::PerformSizing { unit, type_repository, compiler, has_prior_errors }
}

pub fn create_find_type_request(criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::FindType { criteria, respond_to }
}

pub fn create_file_parsed_event(parse_result: FileParseResult) -> CompilationMessage {
    CompilationMessage::FileParsed(parse_result)
}

pub fn create_unit_typed_event(resolved_types: RuntimeTypePointers, unit: CompilationUnit, errors: CompilationErrors) -> CompilationMessage {
    CompilationMessage::UnitTyped { resolved_types, unit, errors }
}

pub fn create_unit_sized_event(unit: CompilationUnit, errors: CompilationErrors) -> CompilationMessage {
    CompilationMessage::UnitSized { unit, errors }
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

pub fn create_build_byte_code_command(unit: CompilationUnit, compiler: CompilationActorHandle, has_prior_errors: bool) -> CompilationMessage {
    CompilationMessage::BuildByteCode { unit, compiler, has_prior_errors }
}

pub fn create_byte_code_built_event(unit: CompilationUnit, errors: CompilationErrors, code: IntermediateRepresentation) -> CompilationMessage {
    CompilationMessage::ByteCodeBuilt { unit, errors, code }
}

pub fn create_build_backend_command(code: IntermediateRepresentation, compiler: CompilationActorHandle, has_prior_errors: bool) -> CompilationMessage {
    CompilationMessage::BuildBackend { code, compiler, has_prior_errors }
}

pub fn create_backend_built_event(id: CompilationUnitId, result: BackendErrorResult) -> CompilationMessage {
    CompilationMessage::BackendBuilt { id, result }
}

pub fn create_compilation_complete_event() -> CompilationMessage {
    CompilationMessage::CompilationComplete
}

pub fn create_report_errors_command(errors: CompilationErrors) -> CompilationMessage {
    CompilationMessage::ReportErrors { errors }
}

pub fn create_shutdown_command() -> CompilationMessage {
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
    
    start_compilation(compiler_handle, file_name);
    await_shutdown(&compiler_shutdown_notifier);
}

fn start_compilation(compiler_handle: ActorHandle<CompilationMessage>, file_name: String) {
    send_message_to_actor(
        &compiler_handle, 
        create_compile_command(file_name)
    );
}

pub struct CompilerActor<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> {
    pub statistics: Statistics,
    pub type_repository: CompilationActorHandle,
    pub error_reporter: CompilationActorHandle,
    pub reader: TReader,
    pub backend: TBackend,
    pub message_wire_tap: TMessageWireTap
}

fn create_compiler_actor<TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage>(
    type_repository: CompilationActorHandle,
    error_reporter: CompilationActorHandle,
    reader: TReader,
    backend: TBackend, 
    message_wire_tap: TMessageWireTap
) -> CompilerActor<TReader, TBackend, TMessageWireTap> {
    CompilerActor {
        statistics: create_statistics(),
        type_repository,
        error_reporter, 
        reader,
        backend,
        message_wire_tap
    }
}

impl <TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> Actor<CompilationMessage> for CompilerActor<TReader, TBackend, TMessageWireTap> {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        self.message_wire_tap.tap(&message);
        match message {
            CompilationMessage::Compile(file_name) =>
                handle_compile(file_name, ctx, self.reader.clone()),
            CompilationMessage::FileParsed(parse_result) =>
                handle_file_parsed(self, parse_result, ctx),
            CompilationMessage::UnitTyped { resolved_types, unit, errors } => 
                handle_unit_typed(self, unit, errors, resolved_types, ctx),
            CompilationMessage::UnitSized { unit, errors } => 
                handle_unit_sized(self, unit, errors, ctx),
            CompilationMessage::ByteCodeBuilt { code, unit, errors } => 
                handle_byte_code_built(self, unit, errors, code, ctx, self.backend.clone()),
            CompilationMessage::BackendBuilt { id, result } => 
                handle_backend_built(self, id, result, ctx),
            CompilationMessage::CompilationComplete => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
}

pub fn handle_compile<TReader: FileRead>(
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
    
    parse_file(parser_handle, file_name, compiler_handle);
    continue_listening_after_receive()
}

fn parse_file(parser_handle: ActorHandle<CompilationMessage>, file_name: String, compiler_handle: ActorHandle<CompilationMessage>) {
    send_message_to_actor(
        &parser_handle, 
        create_parse_file_command(file_name, compiler_handle)
    );
}

