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
    fn receive(&mut self, message: CompilationMessage, ctx: &ActorContext<CompilationMessage>) -> AfterReceiveAction {
        match message {
            CompilationMessage::ReportErrors { errors } => report_errors(errors),
            CompilationMessage::ShutDown => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
}

fn report_errors(errors: CompilationErrors) -> AfterReceiveAction {
    for error in errors {
        report_error(error);
    }
    continue_listening_after_receive()
}

fn report_error(error: CompilationError) {
    match error.item {
        CompilationErrorItem::None => todo!(),
        CompilationErrorItem::ParseError(_) => todo!(),
        CompilationErrorItem::TypeInferenceError(_) => todo!(),
        CompilationErrorItem::IntermediateRepresentationError(_) => todo!(),
        CompilationErrorItem::ToDo { function, text } => todo!(),
        CompilationErrorItem::ShutDownRequested => todo!(),
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum CompilationErrorItem {
    None,
    ParseError(ParseError),
    TypeInferenceError(TypeInferenceError),
    IntermediateRepresentationError(IntermediateRepresentationError),
    ToDo{function: String, text: String, },
    ShutDownRequested,
}

pub fn no_error() -> CompilationErrorItem {
    CompilationErrorItem::None
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

pub fn todo(errors: &mut CompilationErrors, function: &str, text: &str) {
    add_compilation_error(errors, create_compilation_error(todo_error(function, text), no_position()));
}

#[derive(PartialEq, Debug, Clone)]
pub struct CompilationError {
    item: CompilationErrorItem,
    position: SourceFilePosition,
}

pub fn create_compilation_error(item: CompilationErrorItem, position: SourceFilePosition) -> CompilationError {
    CompilationError {
        item,
        position,
    }
}

pub type CompilationErrors = Vec<CompilationError>;

pub fn create_compilation_errors() -> CompilationErrors {
    vec!()
}

pub fn add_compilation_error(errors: &mut CompilationErrors, error: CompilationError) {
    errors.push(error);
}