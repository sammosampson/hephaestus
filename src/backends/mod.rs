mod llvm;
mod x64;

pub use llvm::*;
pub use x64::*;

use crate::{
    acting::*,
    compilation::*,
    intermediate_representation::*
};

pub trait BackendBuild : Send + Clone + 'static {
    fn build_backend(&mut self, ir: IntermediateRepresentation);
}


pub struct BackendActor<TBackend: BackendBuild>(TBackend);

pub fn create_backend_actor<TBackend: BackendBuild>(backend: TBackend) -> BackendActor<TBackend> {
    BackendActor(backend)
}

impl<TBackend: BackendBuild> Actor<CompilationMessage> for BackendActor<TBackend> {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::BuildBackend { code, compiler } =>
                build_backend_from_ir(&mut self.0, code, &compiler),
            _ => continue_listening_after_receive()
        }
    }
}

fn build_backend_from_ir<TBackend: BackendBuild>(backend: &mut TBackend, ir: IntermediateRepresentation, compiler: &CompilationActorHandle) -> AfterReceiveAction {
    let id = ir.id;
    backend.build_backend(ir);
    send_message_to_actor(compiler, create_backend_built_event(id));
    shutdown_after_receive()
}
