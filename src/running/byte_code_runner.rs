use crate::{
    acting::*,
    compilation::*,
    running::*
};

pub struct ByteCodeRunnerActor<T: Interpret> {
    interpreter: T
}

pub fn create_byte_code_runner_actor<T: Interpret>(interpreter: T) -> ByteCodeRunnerActor<T> {
    ByteCodeRunnerActor {
        interpreter
    }
}

impl<T: Interpret> Actor<CompilationMessage> for ByteCodeRunnerActor<T> {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::RunByteCode { code, compiler } =>
                run_byte_code(code, &compiler, &mut self.interpreter),
            _ => continue_listening_after_receive()
        }
    }
}

fn run_byte_code<T: Interpret>(
    code: RunnableCompileTimeCode,
    compiler: &CompilationActorHandle,
    interpreter: &mut T
) -> AfterReceiveAction {
    let id = code.to_run_id;
    
    interpreter.interpret(code);

    send_message_to_actor(compiler, create_byte_code_ran_event(id));
    
    shutdown_after_receive()
}

