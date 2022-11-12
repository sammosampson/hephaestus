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
    ParseFile(String),
    FileParsed { file_name: String, units: CompilationUnits },
    PerformTyping { unit: CompilationUnit, has_prior_errors: bool },
    UnitTyped { resolved_types: RuntimeTypePointers, unit: CompilationUnit },
    PerformSizing { unit: CompilationUnit, has_prior_errors: bool },
    UnitSized { unit: CompilationUnit },
    FindType { criteria: FindTypeCriteria, respond_to: CompilationActorHandle },
    TypeFound(RuntimeTypePointer),
    TypeRequestReleaseDueToError(CompilationErrorItem),
    AddResolvedType(RuntimeTypePointer),
    BuildByteCode { unit: CompilationUnit, has_prior_errors: bool },
    ByteCodeBuilt { unit: CompilationUnit, code: IntermediateRepresentation },
    BuildBackend { code: IntermediateRepresentation, has_prior_errors: bool },
    BackendBuilt { id: CompilationUnitId },
    CompilationComplete,
    ReportErrors { errors: CompilationErrors, compiler: CompilationActorHandle },
    ErrorsReported,
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

pub fn create_parse_file_command(file_name: String) -> CompilationMessage {
    CompilationMessage::ParseFile(file_name)
}

pub fn create_perform_typing_command(unit: CompilationUnit, has_prior_errors: bool) -> CompilationMessage {
    CompilationMessage::PerformTyping { unit, has_prior_errors }
}

pub fn create_perform_sizing_command(unit: CompilationUnit, has_prior_errors: bool) -> CompilationMessage {
    CompilationMessage::PerformSizing { unit, has_prior_errors }
}

pub fn create_find_type_request(criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::FindType { criteria, respond_to }
}

pub fn create_file_parsed_event(file_name: String, units: CompilationUnits) -> CompilationMessage {
    CompilationMessage::FileParsed { file_name, units }
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

pub fn create_build_byte_code_command(unit: CompilationUnit, has_prior_errors: bool) -> CompilationMessage {
    CompilationMessage::BuildByteCode { unit, has_prior_errors }
}

pub fn create_byte_code_built_event(unit: CompilationUnit, code: IntermediateRepresentation) -> CompilationMessage {
    CompilationMessage::ByteCodeBuilt { unit, code }
}

pub fn create_build_backend_command(code: IntermediateRepresentation, has_prior_errors: bool) -> CompilationMessage {
    CompilationMessage::BuildBackend { code, has_prior_errors }
}

pub fn create_backend_built_event(id: CompilationUnitId) -> CompilationMessage {
    CompilationMessage::BackendBuilt { id }
}

pub fn create_compilation_complete_event() -> CompilationMessage {
    CompilationMessage::CompilationComplete
}

pub fn create_report_errors_command(errors: CompilationErrors, compiler: CompilationActorHandle) -> CompilationMessage {
    CompilationMessage::ReportErrors { errors, compiler }
}

pub fn create_errors_reported_event() -> CompilationMessage {
    CompilationMessage::ErrorsReported
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
    let type_repository = create_type_repository_actor();
    let error_reporter = create_error_reporter_actor();
    let (type_repository_handle, ..) = start_singleton_actor(type_repository);
    let (error_reporter_handle, ..) = start_singleton_actor(error_reporter);
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
    pub message_wire_tap: TMessageWireTap,
    pub errors_have_occurred: bool
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
        message_wire_tap,
        errors_have_occurred: false
    }
}

impl <TReader: FileRead, TBackend: BackendBuild, TMessageWireTap: WireTapCompilationMessage> Actor<CompilationMessage> for CompilerActor<TReader, TBackend, TMessageWireTap> {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        self.message_wire_tap.tap(&message);
        match message {
            CompilationMessage::Compile(file_name) =>
                handle_compile(&self.error_reporter, self.reader.clone(), ctx, file_name ),
            CompilationMessage::FileParsed { units, .. } =>
                handle_file_parsed(self, units, ctx),
            CompilationMessage::UnitTyped { resolved_types, unit } => 
                handle_unit_typed(self, unit, resolved_types, ctx),
            CompilationMessage::UnitSized { unit } => 
                handle_unit_sized(self, unit, ctx),
            CompilationMessage::ByteCodeBuilt { code, unit  } => 
                handle_byte_code_built(self, unit, code, ctx, self.backend.clone()),
            CompilationMessage::BackendBuilt { id, .. } => 
                handle_backend_built(self, id, ctx),
            CompilationMessage::ErrorsReported => 
                handle_errors_reported(self),
            CompilationMessage::CompilationComplete => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
}

pub fn handle_compile<TReader: FileRead>(
    error_reporter: &CompilationActorHandle,
    reader: TReader,
    ctx: &CompilationMessageContext,
    file_name: String,
) -> AfterReceiveAction {

    debug!("handling compile for: {:?}", &file_name);
        
    let (parser_handle, ..) = start_actor(
        ctx, 
        create_parser_actor(create_self_handle(ctx), error_reporter.clone(), reader)
    );

    parse_file(parser_handle, file_name);
    continue_listening_after_receive()
}

fn parse_file(parser_handle: ActorHandle<CompilationMessage>, file_name: String) {
    send_message_to_actor(
        &parser_handle, 
        create_parse_file_command(file_name)
    );
}

