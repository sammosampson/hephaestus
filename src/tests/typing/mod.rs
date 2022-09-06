mod procedures;
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