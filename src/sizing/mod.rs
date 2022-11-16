
use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;
use crate::utilities::*;

pub struct SizingActor {    
    compiler: CompilationActorHandle,
    type_repository: CompilationActorHandle
}

pub fn create_sizing_actor(compiler: CompilationActorHandle, type_repository: CompilationActorHandle) -> SizingActor {
    SizingActor {      
        compiler,  
        type_repository
    }
}

impl Actor<CompilationMessage> for SizingActor {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::PerformSizing { unit, has_prior_errors } => 
                handle_perform_sizing(self.compiler.clone(), &self.type_repository, ctx, unit, has_prior_errors),
            _ => continue_listening_after_receive()
        }
    }
    
    fn get_type_name(&self) -> String {
        string_type_name::<SizingActor>()
    }
}

fn handle_perform_sizing(
    compiler: CompilationActorHandle,
    type_repository: &CompilationActorHandle, 
    ctx: &CompilationMessageContext,
    mut unit: CompilationUnit, 
    has_prior_errors: bool
) -> AfterReceiveAction {
    perform_sizing(ctx, type_repository, &mut unit, has_prior_errors);
    notify_compiler_unit_is_sized(&compiler, unit);    
    shutdown_after_receive()
}

fn notify_compiler_unit_is_sized(compiler: &CompilationActorHandle, unit: CompilationUnit) {
    send_message_to_actor(compiler, create_unit_sized_event(unit));
}

pub fn perform_sizing(
    _ctx: &CompilationMessageContext,
    _type_repository: &CompilationActorHandle,
    _unit: &mut CompilationUnit,
    _has_prior_errors: bool
) {
}