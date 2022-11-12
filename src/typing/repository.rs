use std::collections::*;
use log::debug;

use crate::acting::*;
use crate::compilation::*;
use crate::types::*;
use crate::errors::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FindTypeCriteria { 
    name: String,
    args: RuntimeTypePointers
}

pub fn find_type_by_name(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    name: &mut String
) -> RuntimeTypePointerResult {
    find_type_by_name_and_args(ctx, type_repository, name, vec!())
}


pub fn find_type_by_name_and_args(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    name: &mut String,
    arg_types: RuntimeTypePointers    
) -> RuntimeTypePointerResult {
    find_type_from_criteria(
        create_find_type_criteria_with_name_and_args(name.to_string(), arg_types),
        ctx,
        type_repository
    )
}


pub fn create_find_type_criteria_with_name(name: String) -> FindTypeCriteria {
    create_find_type_criteria_with_name_and_args(name, vec!())
}

pub fn create_find_type_criteria_with_name_and_args(name: String, args: RuntimeTypePointers) -> FindTypeCriteria {
    FindTypeCriteria { name, args }
}

pub fn find_type_from_criteria(criteria: FindTypeCriteria, ctx: &CompilationMessageContext, type_repository: &CompilationActorHandle) -> RuntimeTypePointerResult {
    send_find_type_request(type_repository, criteria, ctx);  
    await_type_found_response(ctx)
}

fn send_find_type_request(type_repository: &CompilationActorHandle, criteria: FindTypeCriteria, ctx: &CompilationMessageContext) {
    debug!("finding type: {:?}", criteria.name);
    send_message_to_actor(type_repository, create_find_type_request(criteria, create_self_handle(ctx)))
}

fn await_type_found_response(ctx: &CompilationMessageContext) -> RuntimeTypePointerResult {    
    let mut result = Err(no_error());
    
    await_message(ctx, |message| {
        match message {
            CompilationMessage::TypeFound(resolved_type) => {
                result = Ok(resolved_type);
                true
            },
            CompilationMessage::TypeRequestReleaseDueToError(error) => {
                result = Err(error);
                true
            },
            _ => false
        }
    });

    result
}

type RuntimeTypeMap = HashMap<FindTypeCriteria, RuntimeTypePointer>;

fn create_type_map() -> RuntimeTypeMap {
    HashMap::default()
}

struct FindTypeRequest {
    criteria: FindTypeCriteria,
    respond_to: CompilationActorHandle
}

fn find_type_request(criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> FindTypeRequest {
    FindTypeRequest {
        criteria,
        respond_to
    }
}

type FindTypeRequests = Vec<FindTypeRequest>;

fn create_find_type_requests() -> FindTypeRequests {
    vec!()
}

pub struct TypeRepositoryActor { 
    type_map: RuntimeTypeMap,
    find_type_requests: FindTypeRequests 
}

pub fn create_type_repository_actor() -> TypeRepositoryActor {
    TypeRepositoryActor { 
        type_map: create_type_map(),
        find_type_requests: create_find_type_requests()
    }
}

impl Actor<CompilationMessage> for TypeRepositoryActor {
    fn receive(&mut self, message: CompilationMessage, _ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::FindType { criteria, respond_to } =>
                handle_find_type(self, criteria, respond_to),
            CompilationMessage::AddResolvedType(resolved_type) => 
                handle_add_resolved_type(self, resolved_type),
            CompilationMessage::ReleaseAllTypeRequests =>
                handle_release_all_type_requests(self),
            CompilationMessage::ShutDown => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
}

fn handle_find_type(repository: &mut TypeRepositoryActor, criteria: FindTypeCriteria, respond_to: CompilationActorHandle) -> AfterReceiveAction {
    add_find_type_request(repository, find_type_request(criteria, respond_to));
    service_find_type_requests(repository);
    continue_listening_after_receive()
}

fn handle_add_resolved_type(repository: &mut TypeRepositoryActor, resolved_type: RuntimeTypePointer) -> AfterReceiveAction {
    match parse_find_type_criteria(&resolved_type) {
        Ok(criteria) => add_resolved_type(repository, criteria, resolved_type), 
        Err(error) => release_all_type_requests_due_to_error(repository, error)
    };
    service_find_type_requests(repository);
    continue_listening_after_receive()
}

fn handle_release_all_type_requests(repository: &mut TypeRepositoryActor) -> AfterReceiveAction {
    release_all_type_requests_due_to_error(repository, shutdown_requested_error());
    continue_listening_after_receive()
}


fn add_find_type_request(repository: &mut TypeRepositoryActor,  request: FindTypeRequest) {
    repository.find_type_requests.push(request);
}

fn service_find_type_requests(repository: &mut TypeRepositoryActor) {
    let mut removals = vec!();
    
    for index in 0..repository.find_type_requests.len() {
        let request = &repository.find_type_requests[index];
        if service_find_type_request(repository, request) {
            removals.push(index);
        }
    }

    for index in removals {
        remove_find_type_request(repository, index);
    }
}

fn remove_find_type_request(repository: &mut TypeRepositoryActor, index: usize) -> FindTypeRequest {
    repository.find_type_requests.swap_remove(index)
}

fn service_find_type_request(repository: &TypeRepositoryActor, request: &FindTypeRequest) -> bool {
    let resolved_type = repository.type_map.get(&request.criteria);

    if let Some(resolved_type) = resolved_type {
        send_message_to_actor(&request.respond_to, create_type_found_event(resolved_type.clone()));
        return true;
    }
    false
}

fn release_all_type_requests_due_to_error(repository: &mut TypeRepositoryActor, error: CompilationErrorItem) {
    for request in &repository.find_type_requests {
        release_type_request_due_to_error(request, error.clone())
    }
}

fn release_type_request_due_to_error(request: &FindTypeRequest, error: CompilationErrorItem) {
    send_message_to_actor(&request.respond_to, type_request_released_due_to_error_event(error));
}

pub type FindTypeCriteriaResult = Result<FindTypeCriteria, CompilationErrorItem>;

pub fn parse_find_type_criteria(resolved_type: &RuntimeTypePointer) -> FindTypeCriteriaResult {
    match &resolved_type.item {
        RuntimeTypeItem::ProcedureDefinition { arg_types, .. } => 
            Ok(create_find_type_criteria_with_name_and_args(resolved_type.name.clone(), arg_types.clone())),
        RuntimeTypeItem::ConstantDefinition { .. } => 
            Ok(create_find_type_criteria_with_name(resolved_type.name.clone())),
        _ => Err(todo_error(function!(), "parse criteria for other types"))
    }
}

fn add_resolved_type(repository: &mut TypeRepositoryActor, criteria: FindTypeCriteria, resolved_type: RuntimeTypePointer) {
    repository.type_map.insert(criteria, resolved_type);
}
