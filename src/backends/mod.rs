mod llvm;
mod x64;
mod errors;

pub use llvm::*;
pub use x64::*;
pub use errors::*;

use crate::{
    acting::*,
    compilation::*,
    intermediate_representation::*,
    parsing::*,
    utilities::*,
    errors::*
};

pub trait BackendBuild : Send + Clone + 'static {
    fn build_backend(&mut self, ir: IntermediateRepresentation, has_prior_errors: bool) -> BackendErrorResult;
}


pub struct BackendActor<TBackend: BackendBuild> {
    compiler: CompilationActorHandle,
    error_reporter: CompilationActorHandle,
    backend: TBackend,
}

pub fn create_backend_actor<TBackend: BackendBuild>(compiler: CompilationActorHandle, error_reporter: CompilationActorHandle, backend: TBackend) -> BackendActor<TBackend> {
    BackendActor {
        compiler,
        error_reporter,
        backend
    }
}

impl<TBackend: BackendBuild> Actor<CompilationMessage> for BackendActor<TBackend> {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::BuildBackend { code, has_prior_errors } =>
                build_backend_from_ir(&mut self.backend, code, &self.error_reporter, &self.compiler, has_prior_errors),
            _ => continue_listening_after_receive()
        }
    }
}

fn build_backend_from_ir<TBackend: BackendBuild>(
    backend: &mut TBackend,
    ir: IntermediateRepresentation,
    compiler: &CompilationActorHandle,
    error_reporter: &CompilationActorHandle,
    has_prior_errors: bool
) -> AfterReceiveAction {
    let id = ir.id;
    let result = backend.build_backend(ir, has_prior_errors);
    report_errors(error_reporter, compiler.clone(), create_errors_for_backend_error_result(result));
    send_message_to_actor(compiler, create_backend_built_event(id));
    shutdown_after_receive()
}

fn create_errors_for_backend_error_result(result: BackendErrorResult) -> CompilationErrors {
    let mut errors = create_compilation_errors(empty_string());
    if let Err(error) = result {
        add_compilation_error(&mut errors, compilation_error(backend_error(error), no_position()));
    }
    errors
}
