use crate::{
    acting::*,
    compilation::*,
};

pub struct ByteCodeRunnerActor;

pub fn create_byte_code_runner_actor() -> ByteCodeRunnerActor {
    ByteCodeRunnerActor
}

impl Actor<CompilationMessage> for ByteCodeRunnerActor {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::RunByteCode { byte_code, compiler } =>
                run_byte_code(byte_code, &compiler),
            _ => continue_listening_after_receive()
        }
    }
}

