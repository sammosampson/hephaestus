mod procedures;
mod expressions;
use crate::parsing::*;
use crate::typing::*;
use crate::acting::*;
use crate::compilation::*;
use crate::tests::acting::*;

pub fn run_typing_on_unit(typing_repository: CompilationActorHandle, unit: CompilationUnit) -> (ResolvedTypes, CompilationUnit) {
    let (
        message_receiver_handle, 
        message_receiver
    ) = create_test_message_receiver_actor();
    
    let (typing_actor, ..) = start_singleton_actor(create_typing_actor());
        
    send_message_to_actor(
        &typing_actor, 
        create_perform_typing_command(unit, typing_repository, message_receiver_handle)
    );

    let next_message = message_receiver.into_iter().next().unwrap();

    let result = match next_message {
        CompilationMessage::UnitTyped(resolved_types, compilation_unit) => Some((resolved_types, compilation_unit)),
        _ => None
    };

    result.unwrap()
}

pub fn start_type_repository_actor() -> CompilationActorHandle {
    let (handle, _) = start_singleton_actor(create_type_repository_actor());
    handle
}

pub fn add_resolved_type(typing_repository: &CompilationActorHandle, resolved_type: ResolvedType) {
    send_message_to_actor(
        typing_repository, 
        create_add_resolved_type_command(resolved_type)
    );
}

pub fn create_procedure_definition_type(name: &str, arg_types: RuntimeTypeIds, return_types: RuntimeTypeIds) -> ResolvedType {
    let other_proc_type = create_type(
        create_compilation_unit_id(), 
        name.to_string(),
        procedure_definition_type_item(arg_types, return_types)
    );
    other_proc_type
}

pub fn create_procedure_definition_type_with_no_args(name: &str) -> ResolvedType {
    create_procedure_definition_type(
        name,
        vec!(), 
        vec!()
    )
}