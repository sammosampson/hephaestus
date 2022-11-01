use crate::{
    parsing::*,
    compilation::*
};

#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    ExpectedFileName,
    ExpectedLibraryName,
    ExpectedForeignLibraryIdentifier,
    ExpectedIdentifier,
    ExpectedDeclarationName,
    ExpectedAssignmentInitialise,
    ExpectedAssignmentAssignValue,
    ExpectedArgSeparator,
    ExpectedEnclosure(Enclosure),
    ExpectedOperator,
    ExpectedType,
    ExpectedLineTerminator,
    UnexpectedDirective,
    TokenisationError(SourceTokenError),
    Unimplemented
}

pub fn create_error_node(error: ParseError, position: SourceFilePosition) -> AbstractSyntaxNode {
    create_node(AbstractSyntaxNodeItem::Error(error), position)
}

pub fn create_error_and_error_node(errors: &mut CompilationErrors, error: ParseError, position: SourceFilePosition) -> AbstractSyntaxNode {
    add_compilation_error(
        errors,
        create_compilation_error(CompilationErrorItem::ParseError(error.clone()), position.clone())
    );
    create_error_node(error, position)
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

pub fn expected_identifier_error() -> ParseError {
    ParseError::ExpectedIdentifier
}

pub fn expected_declaration_name_error() -> ParseError {
    ParseError::ExpectedDeclarationName
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

pub fn expected_open_paren_error() -> ParseError {
    ParseError::ExpectedEnclosure(Enclosure::Parentheses(EnclosureType::Open))
}

pub fn expected_close_paren_error() -> ParseError {
    ParseError::ExpectedEnclosure(Enclosure::Parentheses(EnclosureType::Close))
}

pub fn expected_line_terminator_error() -> ParseError {
    ParseError::ExpectedLineTerminator
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
