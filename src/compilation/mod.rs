mod compiler;
mod parsing;
mod typing;
mod sizing;
mod intermediate_representation;
mod backends;
mod errors;
mod statistics;

pub use compiler::*;
pub use parsing::*;
pub use typing::*;
pub use sizing::*;
pub use intermediate_representation::*;
pub use backends::*;
pub use errors::*;
pub use statistics::*;

use crate::{
    parsing::*,
    intermediate_representation::*,
    types::*,
    errors::*,
    typing::*,
    acting::*
};

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
    ReleaseAllTypeRequests,
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

pub fn create_release_all_type_requests_command() -> CompilationMessage {
    CompilationMessage::ReleaseAllTypeRequests
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
