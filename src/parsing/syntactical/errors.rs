use crate::parsing::*;

#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    ExpectedFileName,
    ExpectedOpenBrace,
    ExpectedArgName,
    ExpectedArgInitialise,
    ExpectedArgSeparator,
    ExpectedType,
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

pub fn expected_open_brace_error() -> ParseError {
    ParseError::ExpectedOpenBrace
}

pub fn expected_arg_name_error() -> ParseError {
    ParseError::ExpectedArgName
}

pub fn expected_initialise_assignment_error() -> ParseError {
    ParseError::ExpectedArgInitialise
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
