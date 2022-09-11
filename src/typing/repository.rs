use std::collections::*;
use crate::acting::*;
use crate::compilation::*;
use super::*;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FindTypeCriteria { 
    name: String,
    args: RuntimeTypeIds
}

pub fn create_find_type_criteria(name: String, args: RuntimeTypeIds) -> FindTypeCriteria {
    FindTypeCriteria { name, args }
}

pub fn find_type(criteria: FindTypeCriteria, ctx: &CompilationMessageContext, type_repository: &CompilationActorHandle) -> ResolvedType {
    send_find_type_request(type_repository, criteria, ctx);  
    await_type_found_response(ctx)
}

fn send_find_type_request(type_repository: &ActorHandle<CompilationMessage>, criteria: FindTypeCriteria, ctx: &ActorContext<CompilationMessage>) {
    send_message_to_actor(type_repository, create_find_type_request(criteria, create_self_handle(ctx)))
}

fn await_type_found_response(ctx: &ActorContext<CompilationMessage>) -> ResolvedType {
    let mut result = None;
    
    await_message(ctx, |message| {
        let resolved_type = try_get_type_found_compilation_message(message);
        if resolved_type.is_some() {
            result = resolved_type;
            return true;
        }
        false
    });

    if let Some(result) = result {
        return result
    }

    todo!("wait and send back type when it exists")
}

pub struct TypeRepositoryActor { 
    type_map: HashMap<FindTypeCriteria, ResolvedType> 
}

pub fn create_type_repository_actor() -> TypeRepositoryActor {
    TypeRepositoryActor { 
        type_map: HashMap::default()
    }
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

fn handle_find_type(repository: &mut TypeRepositoryActor, criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> AfterReceiveAction {
    let resolved_type = repository.type_map.get(&criteria);

    if let Some(resolved_type) = resolved_type {
        send_message_to_actor(&respond_to, create_type_found_event(resolved_type.clone()));
    }

    continue_listening_after_receive()
}

fn handle_add_resolved_type(repository: &mut TypeRepositoryActor, resolved_type: ResolvedType) -> AfterReceiveAction {
    let criteria = match &resolved_type.item {
        TypeItem::ProcedureDefinition { arg_types, .. } => 
            create_find_type_criteria(resolved_type.name.clone(), arg_types.clone()),
        _ => todo!("add other types")
    };
    
    repository.type_map.insert(criteria, resolved_type);
    
    continue_listening_after_receive()
}