use crate::errors::*;
use crate::parsing::*;

#[derive(PartialEq, Debug, Clone)]
pub enum IntermediateRepresentationError {
    LiteralNotResolved,
    ExpectedArgument,
    NoOffsetFound,
    ExpectedMember,
    ExpectedInstance,
    TypeNotResolved,
    ScopeNotKnown,
    NoAssignmentFound,
    RegisterSizeNotResolved,
}

pub fn literal_not_resolved_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::LiteralNotResolved
}

pub fn expected_argument_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::ExpectedArgument
}

pub fn no_offset_found_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::NoOffsetFound
}

pub fn exprected_member_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::ExpectedMember
}

pub fn expected_instance_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::ExpectedInstance
}

pub fn type_not_resolved_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::TypeNotResolved
}

pub fn scope_not_known_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::ScopeNotKnown
}

pub fn no_assignment_found_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::NoAssignmentFound
}

pub fn register_size_not_resolved_error() -> IntermediateRepresentationError {
    IntermediateRepresentationError::RegisterSizeNotResolved
}

pub fn add_intermediate_representation_error(errors: &mut CompilationErrors, error: IntermediateRepresentationError, position: SourceFilePosition) {
    add_compilation_error(errors, compilation_error(intermediate_representation_error(error), position));
}

