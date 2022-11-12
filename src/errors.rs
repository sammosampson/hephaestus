use crate::BackendError;
use crate::acting::*;
use crate::compilation::*;
use crate::parsing::*;
use crate::typing::*;
use crate::intermediate_representation::*;
use crate::utilities::*;

pub struct ErrorReporterActor;

pub fn create_error_reporter_actor() -> ErrorReporterActor {
    ErrorReporterActor
}

impl Actor<CompilationMessage> for ErrorReporterActor {
    fn receive(&mut self, message: CompilationMessage, _ctx: &ActorContext<CompilationMessage>) -> AfterReceiveAction {
        match message {
            CompilationMessage::ReportErrors { errors, compiler} => report_errors(&errors, &compiler),
            CompilationMessage::ShutDown => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
}

fn report_errors(errors: &CompilationErrors, compiler: &CompilationActorHandle) -> AfterReceiveAction {
    for error in &errors.items {
        report_error(&errors.filename, error);
    }
    notify_compiler_errors_reported(compiler);
    continue_listening_after_receive()
}

fn notify_compiler_errors_reported(compiler: &CompilationActorHandle) {
    send_message_to_actor(compiler, create_errors_reported_event());
}


fn report_error(filename: &str, error: &CompilationError) {
    match &error.item {
        CompilationErrorItem::None => {},
        CompilationErrorItem::ParseError(parser_error) => report_parser_error(filename, parser_error, error.position),
        CompilationErrorItem::TypeInferenceError(type_error) => report_type_inference_error(filename, type_error, error.position),
        CompilationErrorItem::IntermediateRepresentationError(ir_error) => report_intermediate_representation_error(filename, ir_error, error.position),
        CompilationErrorItem::ToDo { function, text } => report_todo_error(filename, function, text, error.position),
        CompilationErrorItem::ShutDownRequested => {},
        CompilationErrorItem::FileNotFound(filename) => report_file_not_found_error(filename),
        CompilationErrorItem::BackendError(backend_error) => report_backend_error(backend_error),
    }
}

fn report_parser_error(filename: &str, error: &ParseError, position: SourceFilePosition) {
    match error {
        ParseError::ExpectedFileName => output_error(filename, "expected filename", position),
        ParseError::ExpectedLibraryName => output_error(filename, "expected library name", position),
        ParseError::ExpectedForeignLibraryIdentifier => output_error(filename, "expected foreign library identifier", position),
        ParseError::ExpectedIdentifier => output_error(filename, "expected identifier", position),
        ParseError::ExpectedDeclarationName => output_error(filename, "expected declaration name", position),
        ParseError::ExpectedAssignmentInitialise => output_error(filename, "expected assignment initialise", position),
        ParseError::ExpectedAssignmentAssignValue => output_error(filename, "expected assignment assign value", position),
        ParseError::ExpectedArgSeparator => output_error(filename, "expected argument separator", position),
        ParseError::ExpectedEnclosure(enclosure) => report_expected_enclosure_error(filename, enclosure, position),
        ParseError::ExpectedOperator => output_error(filename, "expected operator", position),
        ParseError::ExpectedType => output_error(filename, "expected type", position),
        ParseError::ExpectedLineTerminator => output_error(filename, "expected line terminator", position),
        ParseError::UnexpectedDirective => output_error(filename, "unexpected directive", position),
        ParseError::TokenisationError(token_error) => report_token_error_error(filename, token_error, position),
        ParseError::Unimplemented => output_error(filename, "unimplemented", position),
    }
}

fn report_expected_enclosure_error(filename: &str, enclosure: &Enclosure, position: SourceFilePosition) {
    match enclosure {
        Enclosure::Brace(enclosure) => report_brace_error_error(filename, enclosure, position),
        Enclosure::Parentheses(enclosure) => report_parentheses_error_error(filename, enclosure, position),
    }
}

fn report_brace_error_error(filename: &str, enclosure: &EnclosureType, position: SourceFilePosition) {
    match enclosure {
        EnclosureType::Open => output_error(filename, "expected opening brace", position),
        EnclosureType::Close => output_error(filename, "expected closing brace", position),
    }
}

fn report_parentheses_error_error(filename: &str, enclosure: &EnclosureType, position: SourceFilePosition) {
    match enclosure {
        EnclosureType::Open => output_error(filename, "expected opening parentheses", position),
        EnclosureType::Close => output_error(filename, "expected closing parentheses", position),
    }
}

fn report_token_error_error(filename: &str, token_error: &SourceTokenError, position: SourceFilePosition) {
    match token_error {
        SourceTokenError::UnknownToken(character) => output_error(filename, &format!("unknown token {}", character), position),
        SourceTokenError::UnknownDirective(directive) => output_error(filename, &format!("unknown directive {}", directive), position),
    }
}

fn report_type_inference_error(filename: &str, error: &TypeInferenceError, position: SourceFilePosition) {
    match error {
        TypeInferenceError::ArgsAndKnownTypesAreNotSameLength => output_error(filename, "arguments and known types are not same length", position),
        TypeInferenceError::NotViableProcedureBodyStatement => output_error(filename, "non viable procedure body statement", position),
        TypeInferenceError::NotViableProcedureHeaderArgument => output_error(filename, "non viable procedure header argument", position),
        TypeInferenceError::NotViableProcedureHeaderReturnType => output_error(filename, "non viable procedure header return type", position),
        TypeInferenceError::NotViableProcedureBodyArgument => output_error(filename, "non viable procedure body argument", position),
        TypeInferenceError::NotViableProcedureBodyReturnType => output_error(filename, "non viable procedure body return type", position),
        TypeInferenceError::TargetTypeSizeIsNotResolved => output_error(filename, "target type size not resolved", position),
        TypeInferenceError::LiteralValueIsNotForTargetType => output_error(filename, "literal value is not for target type", position),
        TypeInferenceError::LiteralValueIsTooLargeForTargetType => output_error(filename, "literal value is too large for target type", position),
        TypeInferenceError::LiteralShouldNotBeResolved => output_error(filename, "literal should not resolved", position),
        TypeInferenceError::TypeSizeInBytesInvalidError(size) => output_error(filename, &format!("type size {} in bytes invalid", size), position),
        TypeInferenceError::TypeCanNotBeFound => output_error(filename, "type cannot be found", position),
    }
}

fn report_intermediate_representation_error(filename: &str, error: &IntermediateRepresentationError, position: SourceFilePosition) {
    match error {
        IntermediateRepresentationError::LiteralNotResolved => output_error(filename, "literal not resolved", position),
        IntermediateRepresentationError::ExpectedArgument => output_error(filename, "expected argument", position),
        IntermediateRepresentationError::NoOffsetFound => output_error(filename, "no offset found", position),
        IntermediateRepresentationError::ExpectedMember => output_error(filename, "expected member", position),
        IntermediateRepresentationError::ExpectedInstance => output_error(filename, "expected instance", position),
        IntermediateRepresentationError::TypeNotResolved => output_error(filename, "type not resolved", position),
        IntermediateRepresentationError::ScopeNotKnown => output_error(filename, "scope not known", position),
        IntermediateRepresentationError::NoAssignmentFound => output_error(filename, "no assignment found", position),
        IntermediateRepresentationError::RegisterSizeNotResolved => output_error(filename, "register size not resolved", position),
    }
}

fn report_todo_error(filename: &str, function: &str, text: &str, position: SourceFilePosition) {
    output_error(filename, &format!("TODO: {} - {}", function, text), position);
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

fn output_error(filename: &str, text: &str, position: SourceFilePosition) {
    println!("{}", text);
    println!("file: {}, line: {}, column: {}", filename, position.line, position.col);
}

#[derive(PartialEq, Debug, Clone)]
pub enum CompilationErrorItem {
    None,
    FileNotFound(String),
    ParseError(ParseError),
    TypeInferenceError(TypeInferenceError),
    IntermediateRepresentationError(IntermediateRepresentationError),
    BackendError(BackendError),
    ToDo{function: String, text: String, },
    ShutDownRequested,
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

pub fn shutdown_requested_error() -> CompilationErrorItem {
    CompilationErrorItem::ShutDownRequested
}

pub fn backend_error(error: BackendError) -> CompilationErrorItem {
    CompilationErrorItem::BackendError(error)
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
