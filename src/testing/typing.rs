use crate::parsing::*;
use crate::compilation::*;
use crate::types::*;
use crate::typing::*;
use crate::testing::*;
use crate::acting::*;

pub fn run_typing_on_unit(typing_repository: CompilationActorHandle, unit: CompilationUnit) -> (RuntimeTypePointers, CompilationUnit) {
    let (
        message_receiver_handle, 
        message_receiver
    ) = create_test_message_receiver_actor();
    
    let (error_reporter, ..) = create_test_message_receiver_actor();
    
    let (typing_actor, ..) = start_singleton_actor(
        create_typing_actor(message_receiver_handle, typing_repository, error_reporter, unit.id),
        
    );
        
    send_message_to_actor(
        &typing_actor, 
        create_perform_typing_command(unit, false)
    );

    let next_message = message_receiver.into_iter().next().unwrap();

    let result = match next_message {
        CompilationMessage::UnitTyped { resolved_types, unit, .. } => Some((resolved_types, unit)),
        _ => None
    };

    result.unwrap()
}