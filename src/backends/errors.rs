use crate::utilities::*;

#[derive(Clone, Debug, PartialEq)]
pub enum BackendError {
    UnsupportedInstruction,
    UnimplementedInstruction,
    UnimplementedFeature(String),
    RegisterNotAvailable(usize),
}

pub type BackendErrorResult = Result<(), BackendError>;

pub fn unsupported_instruction_error() -> BackendError {
    BackendError::UnsupportedInstruction
}

pub fn unimplemented_instruction_error() -> BackendError {
    BackendError::UnimplementedInstruction
}

pub fn todo_error(text: &str) -> BackendError {
    BackendError::UnimplementedFeature(string(text))
}

pub fn register_not_available_error(r: usize) -> BackendError {
    BackendError::RegisterNotAvailable(r)
}
