use crate::{
    acting::*,
    compilation::*,
    running::*
};

pub struct ByteCodeRunnerActor;

pub fn create_byte_code_runner_actor() -> ByteCodeRunnerActor {
    ByteCodeRunnerActor
}

impl Actor<CompilationMessage> for ByteCodeRunnerActor {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::RunByteCode { code, compiler } =>
                run_byte_code(code, &compiler),
            _ => continue_listening_after_receive()
        }
    }
}

fn run_byte_code(code: RunnableCompileTimeCode, compiler: &CompilationActorHandle) -> AfterReceiveAction {
    let id = code.to_run_id;
    let mut vm = create_virtual_machine(code);
    run_virtual_machine(&mut vm);

    send_message_to_actor(compiler, create_byte_code_ran_event(id));
    
    shutdown_after_receive()
}

