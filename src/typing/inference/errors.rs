use crate::compilation::*;
use crate::parsing::*;

#[derive(PartialEq, Debug, Clone)]
pub enum TypeInferenceError {
    ArgsAndKnownTypesAreNotSameLength,
    NotViableProcedureBodyStatement,
    NotViableProcedureHeaderArgument,
    NotViableProcedureHeaderReturnType,
    NotViableProcedureBodyArgument,
    NotViableProcedureBodyReturnType,
    TargetTypeSizeIsNotResolved,
    LiteralValueIsNotForTargetType,
    LiteralValueIsTooLargeForTargetType,
    LiteralShouldNotBeResolved,
    TypeSizeInBytesInvalidError(usize),
    TypeCanNotBeFound,
}

pub fn args_and_known_types_are_not_same_length_error() ->  TypeInferenceError {
    TypeInferenceError::ArgsAndKnownTypesAreNotSameLength
}

pub fn not_viable_procedure_body_statement_error() -> TypeInferenceError {
    TypeInferenceError::NotViableProcedureBodyStatement
}

pub fn not_viable_procedure_header_argument_error() -> TypeInferenceError {
    TypeInferenceError::NotViableProcedureHeaderArgument
}

pub fn not_viable_procedure_header_return_type_error() -> TypeInferenceError {
    TypeInferenceError::NotViableProcedureHeaderReturnType
}

pub fn not_viable_procedure_body_return_type_error() -> TypeInferenceError {
    TypeInferenceError::NotViableProcedureBodyReturnType
}

pub fn not_viable_procedure_body_argument_error() -> TypeInferenceError {
    TypeInferenceError::NotViableProcedureBodyArgument
}

pub fn target_type_size_is_not_resolved_error() -> TypeInferenceError {
    TypeInferenceError::TargetTypeSizeIsNotResolved
}

pub fn literal_value_is_not_for_target_type_error() -> TypeInferenceError {
    TypeInferenceError::LiteralValueIsNotForTargetType
}

pub fn literal_value_is_too_large_for_target_type_error() -> TypeInferenceError {
    TypeInferenceError::LiteralValueIsTooLargeForTargetType
}

pub fn literal_should_not_be_resolved_error() -> TypeInferenceError {
    TypeInferenceError::LiteralShouldNotBeResolved
}

pub fn type_size_in_bytes_invalid_error(size: usize) -> TypeInferenceError {
    TypeInferenceError::TypeSizeInBytesInvalidError(size)
}

pub fn type_cannot_be_found_error() -> TypeInferenceError {
    TypeInferenceError::TypeCanNotBeFound
}

pub fn add_type_inference_error(errors: &mut CompilationErrors, error: TypeInferenceError, position: SourceFilePosition) {
    add_compilation_error(errors, create_compilation_error(type_inference_error(error), position));
}


