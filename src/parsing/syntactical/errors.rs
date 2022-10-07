use crate::parsing::*;

#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    ExpectedFileName,
    ExpectedLibraryName,
    ExpectedForeignLibraryIdentifier,
    ExpectedArgName,
    ExpectedAssignmentInitialise,
    ExpectedAssignmentAssignValue,
    ExpectedArgSeparator,
    ExpectedOperator,
    ExpectedType,
    UnexpectedDirective,
    TokenisationError(SourceTokenError),
    Unimplemented
}

pub fn create_error_node(error: ParseError, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(AbstractSyntaxNodeItem::Error(error), position)
}

pub fn tokenisation_error(error: SourceTokenError) -> ParseError {
    ParseError::TokenisationError(error)
}

pub fn unimplemented_error() -> ParseError {
    ParseError::Unimplemented
}

pub fn expected_operator_error() -> ParseError {
    ParseError::ExpectedOperator
}

pub fn expected_foreign_library_identifier_error() -> ParseError {
    ParseError::ExpectedForeignLibraryIdentifier
}

pub fn expected_arg_name_error() -> ParseError {
    ParseError::ExpectedArgName
}

pub fn expected_initialise_assignment_error() -> ParseError {
    ParseError::ExpectedAssignmentInitialise
}

pub fn expected_assign_value_assignment_error() -> ParseError {
    ParseError::ExpectedAssignmentAssignValue
}


pub fn expected_arg_separator_error() -> ParseError {
    ParseError::ExpectedArgSeparator
}

pub fn expected_type_error() -> ParseError {
    ParseError::ExpectedType
}

pub fn expected_file_name_error() -> ParseError {
    ParseError::ExpectedFileName
}

pub fn expected_library_name_error() -> ParseError {
    ParseError::ExpectedLibraryName
}

pub fn unexpected_directive_error() -> ParseError {
    ParseError::UnexpectedDirective
}
