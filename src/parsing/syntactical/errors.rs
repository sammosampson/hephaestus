use crate::parsing::*;

#[derive(PartialEq, Debug, Clone)]
pub enum AbstractSyntaxParseError {
    FileNotFoundError(String),
    ExpectedFileName,
    ExpectedOpenBrace,
    ExpectedArgName,
    ExpectedArgInitialise,
    ExpectedArgSeparator,
    ExpectedType,
    TokenisationError(SourceTokenError),
    Unimplemented
}

pub fn create_error_node(error: AbstractSyntaxParseError, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(AbstractSyntaxNodeItem::Error(error), position)
}

pub fn tokenisation_error(error: SourceTokenError) -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::TokenisationError(error)
}

pub fn unimplemented_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::Unimplemented
}

pub fn expected_open_brace_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedOpenBrace
}

pub fn expected_arg_name_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedArgName
}

pub fn expected_initialise_assignment_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedArgInitialise
}

pub fn expected_arg_separator_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedArgSeparator
}

pub fn expected_type_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedType
}

pub fn expected_file_name_error() -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::ExpectedFileName
}

pub fn file_not_found_error(file_name: String) -> AbstractSyntaxParseError {
    AbstractSyntaxParseError::FileNotFoundError(file_name)
}
