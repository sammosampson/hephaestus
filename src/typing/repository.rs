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

pub struct TypeRepositoryActor(ResolvedTypes);

pub fn create_type_repository_actor() -> TypeRepositoryActor {
    TypeRepositoryActor(vec!())
}

impl Actor<CompilationMessage> for TypeRepositoryActor {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::FindType { criteria, respond_to } =>
                handle_find_type(self, criteria, respond_to),
            CompilationMessage::AddResolvedType(resolved_type) => 
                handle_add_resolved_type(self, resolved_type),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_find_type(repository: &mut TypeRepositoryActor, _criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> AfterReceiveAction {
    send_message_to_actor(&respond_to, create_type_found_event(repository.0[0].id.clone()));
    continue_listening_after_receive()
}

fn handle_add_resolved_type(repository: &mut TypeRepositoryActor, resolved_type: ResolvedType) -> AfterReceiveAction {
    repository.0.push(resolved_type);
    continue_listening_after_receive()
}