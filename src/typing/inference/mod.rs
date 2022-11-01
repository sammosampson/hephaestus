mod header;
mod body;
mod constants;
mod expressions;
mod errors;

pub use header::*;
pub use body::*;
pub use constants::*;
pub use expressions::*;
pub use errors::*;

use std::collections::*;
use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;
use crate::types::*;

pub struct TypingActor;

pub fn create_typing_actor() -> TypingActor {
    TypingActor
}

impl Actor<CompilationMessage> for TypingActor {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::PerformTyping { unit, type_repository, compiler} => 
                handle_perform_typing(unit, ctx, &type_repository, compiler),
            _ => continue_listening_after_receive()
        }
    }
}

pub type IdentifierTypeLookup = HashMap<String, RuntimeTypePointer>;

pub fn add_to_identifier_type_lookup(map: &mut IdentifierTypeLookup, identifier: String, resolved_type: RuntimeTypePointer) {
    map.insert(identifier, resolved_type);
}

pub fn get_type_for_identifier<'a>(map: &'a IdentifierTypeLookup, identifier: &str) -> Option<&'a RuntimeTypePointer> {
    map.get(identifier)
}

pub fn create_identifier_type_lookup() -> IdentifierTypeLookup {
    IdentifierTypeLookup::default()
}

fn handle_perform_typing(
    mut unit: CompilationUnit, 
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle, 
    compiler: CompilationActorHandle
) -> AfterReceiveAction {
    let resolved_types = perform_typing(ctx, type_repository, &mut unit);
    send_message_to_actor(&compiler, create_unit_typed_event(resolved_types, unit));    
    shutdown_after_receive()
}

pub fn perform_typing(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    unit: &mut CompilationUnit
) -> RuntimeTypePointers {
    let mut resolved_types = vec!();

    match unit.tree.item_mut() {
        AbstractSyntaxNodeItem::Run { expr } => {
            perform_typing_for_inferred_type_expression(ctx, type_repository, &create_identifier_type_lookup(), expr, &mut unit.errors);        
        },
        AbstractSyntaxNodeItem::Constant { name, value, constant_type } => {
            perform_typing_for_constant(unit.id, ctx, type_repository, &mut resolved_types, name, value, constant_type, &mut unit.errors);        
        },
        AbstractSyntaxNodeItem::ProcedureHeader { name, args, return_args, .. } => {
            perform_typing_for_procedure_header(unit.id, name, &mut resolved_types, args, return_args, &mut unit.errors);                      
        },
        AbstractSyntaxNodeItem::ProcedureBody { args, return_types, statements, .. } => {
            perform_typing_for_procedure_body(ctx, type_repository, args, return_types, statements, &mut unit.errors);
        },
        _ => {}
    };
    resolved_types
    
}