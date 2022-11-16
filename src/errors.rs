use crate::BackendError;
use crate::acting::*;
use crate::compilation::*;
use crate::parsing::*;
use crate::typing::*;
use crate::intermediate_representation::*;
use crate::utilities::*;
use crate::file_system::*;

pub struct ErrorReporterActor<T: FileRead> {
    file_reader: T
}

pub fn create_error_reporter_actor<T: FileRead>(file_reader: T) -> ErrorReporterActor<T> {
    ErrorReporterActor {
        file_reader
    }
}

impl<T: FileRead> Actor<CompilationMessage> for ErrorReporterActor<T> {
    fn receive(&mut self, message: CompilationMessage, _ctx: &ActorContext<CompilationMessage>) -> AfterReceiveAction {
        match message {
            CompilationMessage::ReportErrors { errors, compiler} => report_errors(&self.file_reader, &errors, &compiler),
            CompilationMessage::ShutDown => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
    
    fn get_type_name(&self) -> String {
        string_type_name::<ErrorReporterActor<T>>()
    }
}

fn report_errors<T: FileRead>(file_reader: &T, errors: &CompilationErrors, compiler: &CompilationActorHandle) -> AfterReceiveAction {
    if !are_any_compilation_errors(errors) {
        return continue_listening_after_receive();
    }
    for error in &errors.items {
        report_error(file_reader, &errors.filename, error);
    }
    notify_compiler_errors_reported(compiler, errors.clone());
    continue_listening_after_receive()
}

fn notify_compiler_errors_reported(compiler: &CompilationActorHandle, errors: CompilationErrors) {
    send_message_to_actor(compiler, create_errors_reported_event(errors));
}


fn report_error<T: FileRead>(file_reader: &T, filename: &str, error: &CompilationError) {
    match &error.item {
        CompilationErrorItem::None => {},
        CompilationErrorItem::ParseError(parser_error) => report_parser_error(file_reader, filename, parser_error, error.position),
        CompilationErrorItem::TypeInferenceError(type_error) => report_type_inference_error(file_reader, filename, type_error, error.position),
        CompilationErrorItem::IntermediateRepresentationError(ir_error) => report_intermediate_representation_error(file_reader, filename, ir_error, error.position),
        CompilationErrorItem::ToDo { function, text } => report_todo_error(file_reader, filename, function, text, error.position),
        CompilationErrorItem::ShutDownRequested => {},
        CompilationErrorItem::FileNotFound(filename) => report_file_not_found_error(filename),
        CompilationErrorItem::BackendError(backend_error) => report_backend_error(backend_error),
    }
}

fn report_parser_error<T: FileRead>(file_reader: &T, filename: &str, error: &ParseError, position: SourceFilePosition) {
    match error {
        ParseError::ExpectedFileName => output_error(file_reader, filename, "expected filename", position),
        ParseError::ExpectedLibraryName => output_error(file_reader, filename, "expected library name", position),
        ParseError::ExpectedForeignLibraryIdentifier => output_error(file_reader, filename, "expected foreign library identifier", position),
        ParseError::ExpectedIdentifier => output_error(file_reader, filename, "expected identifier", position),
        ParseError::ExpectedDeclarationName => output_error(file_reader, filename, "expected declaration name", position),
        ParseError::ExpectedAssignmentInitialise => output_error(file_reader, filename, "expected assignment initialise", position),
        ParseError::ExpectedAssignmentAssignValue => output_error(file_reader, filename, "expected assignment assign value", position),
        ParseError::ExpectedArgSeparator => output_error(file_reader, filename, "expected argument separator", position),
        ParseError::ExpectedEnclosure(enclosure) => report_expected_enclosure_error(file_reader, filename, enclosure, position),
        ParseError::ExpectedOperator => output_error(file_reader, filename, "expected operator", position),
        ParseError::ExpectedType => output_error(file_reader, filename, "expected type", position),
        ParseError::ExpectedLineTerminator => output_error(file_reader, filename, "expected line terminator", position),
        ParseError::UnexpectedDirective => output_error(file_reader, filename, "unexpected directive", position),
        ParseError::TokenisationError(token_error) => report_token_error_error(file_reader, filename, token_error, position),
        ParseError::Unimplemented => output_error(file_reader, filename, "unimplemented", position),
    }
}

fn report_expected_enclosure_error<T: FileRead>(file_reader: &T, filename: &str, enclosure: &Enclosure, position: SourceFilePosition) {
    match enclosure {
        Enclosure::Brace(enclosure) => report_brace_error_error(file_reader, filename, enclosure, position),
        Enclosure::Parentheses(enclosure) => report_parentheses_error_error(file_reader, filename, enclosure, position),
    }
}

fn report_brace_error_error<T: FileRead>(file_reader: &T, filename: &str, enclosure: &EnclosureType, position: SourceFilePosition) {
    match enclosure {
        EnclosureType::Open => output_error(file_reader, filename, "expected opening brace", position),
        EnclosureType::Close => output_error(file_reader, filename, "expected closing brace", position),
    }
}

fn report_parentheses_error_error<T: FileRead>(file_reader: &T, filename: &str, enclosure: &EnclosureType, position: SourceFilePosition) {
    match enclosure {
        EnclosureType::Open => output_error(file_reader, filename, "expected opening parentheses", position),
        EnclosureType::Close => output_error(file_reader, filename, "expected closing parentheses", position),
    }
}

fn report_token_error_error<T: FileRead>(file_reader: &T, filename: &str, token_error: &SourceTokenError, position: SourceFilePosition) {
    match token_error {
        SourceTokenError::UnknownToken(character) => output_error(file_reader, filename, &format!("unknown token {}", character), position),
        SourceTokenError::UnknownDirective(directive) => output_error(file_reader, filename, &format!("unknown directive {}", directive), position),
    }
}

fn report_type_inference_error<T: FileRead>(file_reader: &T, filename: &str, error: &TypeInferenceError, position: SourceFilePosition) {
    match error {
        TypeInferenceError::ArgsAndKnownTypesAreNotSameLength => output_error(file_reader, filename, "arguments and known types are not same length", position),
        TypeInferenceError::NotViableProcedureBodyStatement => output_error(file_reader, filename, "non viable procedure body statement", position),
        TypeInferenceError::NotViableProcedureHeaderArgument => output_error(file_reader, filename, "non viable procedure header argument", position),
        TypeInferenceError::NotViableProcedureHeaderReturnType => output_error(file_reader, filename, "non viable procedure header return type", position),
        TypeInferenceError::NotViableProcedureBodyArgument => output_error(file_reader, filename, "non viable procedure body argument", position),
        TypeInferenceError::NotViableProcedureBodyReturnType => output_error(file_reader, filename, "non viable procedure body return type", position),
        TypeInferenceError::TargetTypeSizeIsNotResolved => output_error(file_reader, filename, "target type size not resolved", position),
        TypeInferenceError::LiteralValueIsNotForTargetType => output_error(file_reader, filename, "literal value is not for target type", position),
        TypeInferenceError::LiteralValueIsTooLargeForTargetType => output_error(file_reader, filename, "literal value is too large for target type", position),
        TypeInferenceError::LiteralShouldNotBeResolved => output_error(file_reader, filename, "literal should not resolved", position),
        TypeInferenceError::TypeSizeInBytesInvalidError(size) => output_error(file_reader, filename, &format!("type size {} in bytes invalid", size), position),
        TypeInferenceError::TypeCanNotBeFound => output_error(file_reader, filename, "type cannot be found", position),
    }
}

fn report_intermediate_representation_error<T: FileRead>(file_reader: &T, filename: &str, error: &IntermediateRepresentationError, position: SourceFilePosition) {
    match error {
        IntermediateRepresentationError::LiteralNotResolved => output_error(file_reader, filename, "literal not resolved", position),
        IntermediateRepresentationError::ExpectedArgument => output_error(file_reader, filename, "expected argument", position),
        IntermediateRepresentationError::NoOffsetFound => output_error(file_reader, filename, "no offset found", position),
        IntermediateRepresentationError::ExpectedMember => output_error(file_reader, filename, "expected member", position),
        IntermediateRepresentationError::ExpectedInstance => output_error(file_reader, filename, "expected instance", position),
        IntermediateRepresentationError::TypeNotResolved => output_error(file_reader, filename, "type not resolved", position),
        IntermediateRepresentationError::ScopeNotKnown => output_error(file_reader, filename, "scope not known", position),
        IntermediateRepresentationError::NoAssignmentFound => output_error(file_reader, filename, "no assignment found", position),
        IntermediateRepresentationError::RegisterSizeNotResolved => output_error(file_reader, filename, "register size not resolved", position),
    }
}

fn report_todo_error<T: FileRead>(file_reader: &T, filename: &str, function: &str, text: &str, position: SourceFilePosition) {
    output_error(file_reader, filename, &format!("TODO: {} - {}", function, text), position);
}

fn report_file_not_found_error(filename: &str) {
    println!("{} not found", filename);
}

fn report_backend_error(backend_error: &BackendError) {
    match backend_error {
        BackendError::UnsupportedInstruction => println!("Unsupported x64 instruction"),
        BackendError::UnimplementedInstruction => println!("Unimplemented x64 instruction"),
        BackendError::UnimplementedFeature(feature) => println!("Unimplemented x64 feature: {}", feature),
        BackendError::RegisterNotAvailable(register) => println!("x64 register: {} not available", register),
    }
}

fn output_error<T: FileRead>(file_reader: &T, filename: &str, text: &str, position: SourceFilePosition) {
    let source_line = get_source_line(file_reader, filename, position);
    println!("{}:", text);
    println!("{}", source_line);
    println!("file: {}, line: {}, column: {}", filename, position.line, position.col);
}

fn get_source_line<'a, T: FileRead>(file_reader: &T, filename: &str, position: SourceFilePosition) -> String {
    file_reader.read_line_from_file(filename, position.line).unwrap_or(empty_string())
}

#[derive(PartialEq, Debug, Clone)]
pub enum CompilationErrorItem {
    None,
    FileNotFound(String),
    ParseError(ParseError),
    TypeInferenceError(TypeInferenceError),
    IntermediateRepresentationError(IntermediateRepresentationError),
    BackendError(BackendError),
    ToDo{ function: String, text: String },
    ShutDownRequested
}

pub fn no_error() -> CompilationErrorItem {
    CompilationErrorItem::None
}

pub fn file_not_found_error(filename: String) -> CompilationErrorItem {
    CompilationErrorItem::FileNotFound(filename)
}

pub fn parser_error(error: ParseError) -> CompilationErrorItem {
    CompilationErrorItem::ParseError(error)
}

pub fn type_inference_error(error: TypeInferenceError) -> CompilationErrorItem {
    CompilationErrorItem::TypeInferenceError(error)
}

pub fn intermediate_representation_error(error: IntermediateRepresentationError) -> CompilationErrorItem {
    CompilationErrorItem::IntermediateRepresentationError(error)
}

pub fn todo_error(function: &str, text: &str) -> CompilationErrorItem {
    CompilationErrorItem::ToDo { function: string(function), text: string(text) }
}

pub fn backend_error(error: BackendError) -> CompilationErrorItem {
    CompilationErrorItem::BackendError(error)
}

pub fn shutdown_requested_error_item() -> CompilationErrorItem {
    CompilationErrorItem::ShutDownRequested
}

pub fn todo(errors: &mut CompilationErrors, function: &str, text: &str) {
    add_compilation_error(errors, compilation_error(todo_error(function, text), no_position()));
}

#[derive(PartialEq, Debug, Clone)]
pub struct CompilationError {
    item: CompilationErrorItem,
    pub position: SourceFilePosition,
}

pub fn compilation_error(item: CompilationErrorItem, position: SourceFilePosition) -> CompilationError {
    CompilationError {
        item,
        position,
    }
}

#[derive(Clone, Debug)]
pub struct CompilationErrors {
    pub filename: String,
    pub items: Vec<CompilationError>
}

pub fn create_compilation_errors(filename: String) -> CompilationErrors {
    CompilationErrors {
        filename,
        items: vec!()
    }
}

pub fn add_compilation_error(errors: &mut CompilationErrors, error: CompilationError) {
    errors.items.push(error);
}

pub fn are_any_compilation_errors(errors: &CompilationErrors) -> bool {
    errors.items.len() > 0
}
