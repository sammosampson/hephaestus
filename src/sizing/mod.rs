
use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;

pub struct SizingActor;

pub fn create_sizing_actor() -> SizingActor {
    SizingActor
}

impl Actor<CompilationMessage> for SizingActor {
    fn receive(&mut self, message: CompilationMessage, ctx: &ActorContext<CompilationMessage>) -> AfterReceiveAction {
        match message {
            CompilationMessage::PerformSizing { unit, type_repository, compiler} => 
                handle_perform_sizing(unit, ctx, &type_repository, compiler),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_perform_sizing(
    mut unit: CompilationUnit, 
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle, 
    compiler: CompilationActorHandle
) -> AfterReceiveAction {
    perform_sizing(ctx, type_repository, &mut unit);
    send_message_to_actor(&compiler, create_unit_sized_event(unit));    
    shutdown_after_receive()
}

pub fn perform_sizing(
    _ctx: &CompilationMessageContext,
    _type_repository: &CompilationActorHandle,
    _unit: &mut CompilationUnit
) {
}