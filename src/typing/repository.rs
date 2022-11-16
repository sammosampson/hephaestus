use std::collections::*;
use log::debug;

use crate::CompilationUnitId;
use crate::acting::*;
use crate::compilation::*;
use crate::types::*;
use crate::typing::*;
use crate::errors::*;
use crate::utilities::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FindTypeCriteria { 
    name: String,
    args: RuntimeTypePointers
}

pub fn find_type_by_name(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    name: &mut String,
    caller_unit_id: CompilationUnitId,
    compiler: CompilationActorHandle 
) -> RuntimeTypePointerResult {
    find_type_by_name_and_args(ctx, type_repository, name, vec!(), caller_unit_id, compiler)
}


pub fn find_type_by_name_and_args(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    name: &mut String,
    arg_types: RuntimeTypePointers,
    caller_unit_id: CompilationUnitId,
    compiler: CompilationActorHandle  
) -> RuntimeTypePointerResult {
    find_type_from_criteria(
        create_find_type_criteria_with_name_and_args(name.to_string(), arg_types),
        caller_unit_id, 
        ctx,
        type_repository,
        compiler
    )
}

pub fn create_find_type_criteria_with_name(name: String) -> FindTypeCriteria {
    create_find_type_criteria_with_name_and_args(name, vec!())
}

pub fn create_find_type_criteria_with_name_and_args(name: String, args: RuntimeTypePointers) -> FindTypeCriteria {
    FindTypeCriteria { name, args }
}

pub fn find_type_from_criteria(
    criteria: FindTypeCriteria,
    caller_unit_id: CompilationUnitId,
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    compiler: CompilationActorHandle
) -> RuntimeTypePointerResult {
    log_finding_type(&criteria);
    send_find_type_request(type_repository, criteria, caller_unit_id, ctx, compiler);  
    await_type_found_response(ctx)
}

fn send_find_type_request(
    type_repository: &CompilationActorHandle,
    criteria: FindTypeCriteria,
    caller_unit_id: CompilationUnitId,
    ctx: &CompilationMessageContext,
    compiler: CompilationActorHandle
) {
    send_message_to_actor(
        type_repository, 
        create_find_type_request(
            criteria, 
            find_type_caller(create_self_handle(ctx), caller_unit_id),
            compiler
        )
    )
}

fn log_finding_type(criteria: &FindTypeCriteria) {
    debug!("finding type: {:?}", criteria.name);
}

fn await_type_found_response(ctx: &CompilationMessageContext) -> RuntimeTypePointerResult {    
    let mut result = Err(no_error());
    
    await_message(ctx, |message| {
        match message {
            CompilationMessage::TypeFound(resolved_type) => {
                result = Ok(resolved_type);
                true
            },
            CompilationMessage::CircuitBreakTypeRequest(reason) => {
                result = match reason {
                    TypeRequestCircuitBreakReason::CompilationError(error) => Err(error),
                    TypeRequestCircuitBreakReason::TypesNotFound => Err(type_inference_error(type_cannot_be_found_error())),
                };
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
    respond_to: FindTypeCaller
}

#[derive(Clone, Debug)]
pub struct FindTypeCaller {
    caller: CompilationActorHandle,
    caller_unit_id: CompilationUnitId
}

pub fn find_type_caller(caller: CompilationActorHandle, caller_unit_id: CompilationUnitId) -> FindTypeCaller {
    FindTypeCaller {
        caller,
        caller_unit_id
    }
}

fn find_type_request(criteria: FindTypeCriteria, respond_to: FindTypeCaller) -> FindTypeRequest {
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
            CompilationMessage::FindType { criteria, respond_to, compiler } =>
                handle_find_type(self, criteria, respond_to, compiler),
            CompilationMessage::AddResolvedType(resolved_type) => 
                handle_add_resolved_type(self, resolved_type),
            CompilationMessage::CircuitBreakAllTypeRequests(reason) =>
                handle_release_all_type_requests(self, reason),
            CompilationMessage::ShutDown => shutdown_after_receive(),
            _ => continue_listening_after_receive()
        }
    }
    
    fn get_type_name(&self) -> String {
        string_type_name::<TypeRepositoryActor>()
    }
}

fn handle_find_type(repository: &mut TypeRepositoryActor, criteria: FindTypeCriteria, respond_to: FindTypeCaller, compiler: CompilationActorHandle) -> AfterReceiveAction {
    let awaiting_unit_id = respond_to.caller_unit_id;
    add_find_type_request(repository, find_type_request(criteria, respond_to));
    notify_compiler_of_find_requested(compiler, awaiting_unit_id);
    service_find_type_requests(repository);
    continue_listening_after_receive()
}

fn handle_add_resolved_type(repository: &mut TypeRepositoryActor, resolved_type: RuntimeTypePointer) -> AfterReceiveAction {
    match parse_find_type_criteria(&resolved_type) {
        Ok(criteria) => add_resolved_type(repository, criteria, resolved_type), 
        Err(error) => release_all_type_requests(repository, compilation_error_type_request_circuit_break_reason(error))
    };
    service_find_type_requests(repository);
    continue_listening_after_receive()
}

fn handle_release_all_type_requests(repository: &mut TypeRepositoryActor, reason: TypeRequestCircuitBreakReason ) -> AfterReceiveAction {
    release_all_type_requests(repository, reason);
    continue_listening_after_receive()
}

fn add_find_type_request(repository: &mut TypeRepositoryActor,  request: FindTypeRequest) {
    repository.find_type_requests.push(request);
}

fn notify_compiler_of_find_requested(compiler: CompilationActorHandle, awaiting_unit_id: CompilationUnitId) {
    send_message_to_actor(
        &compiler,
        create_type_find_requested_event(awaiting_unit_id)
    );
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
        send_message_to_actor(&request.respond_to.caller, create_type_found_event(resolved_type.clone()));
        return true;
    }
    false
}

fn release_all_type_requests(repository: &mut TypeRepositoryActor, reason: TypeRequestCircuitBreakReason) {
    for request in &repository.find_type_requests {
        release_type_request(request, reason.clone())
    }
    repository.find_type_requests.clear();
}

fn release_type_request(request: &FindTypeRequest, reason: TypeRequestCircuitBreakReason) {
    send_message_to_actor(&request.respond_to.caller, circuit_break_type_request(reason));
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
