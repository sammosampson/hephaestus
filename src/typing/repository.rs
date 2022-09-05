use crate::acting::*;
use crate::compilation::*;
use super::*;

#[derive(Clone)]
pub struct FindTypeCriteria;

pub fn find_type(criteria: FindTypeCriteria, ctx: &CompilationMessageContext, type_repository: &CompilationActorHandle) -> ResolvedTypeId {
    send_find_type_request(type_repository, criteria, ctx);  
    await_type_found_response(ctx)
}

fn send_find_type_request(type_repository: &ActorHandle<CompilationMessage>, criteria: FindTypeCriteria, ctx: &ActorContext<CompilationMessage>) {
    send_message_to_actor(type_repository, create_find_type_request(criteria, create_self_handle(ctx)))
}

fn await_type_found_response(ctx: &ActorContext<CompilationMessage>)-> ResolvedTypeId {
    let mut result = ResolvedTypeId::NotResolved;
    
    await_message(ctx, |message| {
        if let CompilationMessage::TypeFound(resolved_type) = message {
            result = resolved_type;
            return true;
        }
        false
    });

    result
}

pub struct TypeRepositoryActor;

pub fn create_type_repository_actor() -> TypeRepositoryActor { TypeRepositoryActor }

impl Actor<CompilationMessage> for TypeRepositoryActor {
    fn receive(&self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::FindType { criteria, respond_to } => handle_find_type(criteria, respond_to),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_find_type(_criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> AfterReceiveAction {
    send_message_to_actor(&respond_to, create_type_found_event(ResolvedTypeId::BuiltInType(BuiltInType::Float32)));
    continue_listening_after_receive()
}