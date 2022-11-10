mod llvm;
mod x64;
mod errors;

pub use llvm::*;
pub use x64::*;
pub use errors::*;

use crate::{
    acting::*,
    compilation::*,
    intermediate_representation::*
};

pub trait BackendBuild : Send + Clone + 'static {
    fn build_backend(&mut self, ir: IntermediateRepresentation, has_prior_errors: bool) -> BackendErrorResult;
}


pub struct BackendActor<TBackend: BackendBuild>(TBackend);

pub fn create_backend_actor<TBackend: BackendBuild>(backend: TBackend) -> BackendActor<TBackend> {
    BackendActor(backend)
}

impl<TBackend: BackendBuild> Actor<CompilationMessage> for BackendActor<TBackend> {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::BuildBackend { code, compiler, has_prior_errors } =>
                build_backend_from_ir(&mut self.0, code, &compiler, has_prior_errors),
            _ => continue_listening_after_receive()
        }
    }
}

fn build_backend_from_ir<TBackend: BackendBuild>(
    backend: &mut TBackend,
    ir: IntermediateRepresentation,
    compiler: &CompilationActorHandle,
    has_prior_errors: bool
) -> AfterReceiveAction {
    let id = ir.id;
    let result = backend.build_backend(ir, has_prior_errors);
    send_message_to_actor(compiler, create_backend_built_event(id, result));
    shutdown_after_receive()
}
