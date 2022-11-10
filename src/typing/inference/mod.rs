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
use crate::errors::*;

pub struct TypingActor;

pub fn create_typing_actor() -> TypingActor {
    TypingActor
}

impl Actor<CompilationMessage> for TypingActor {
    fn receive(&mut self, message: CompilationMessage, ctx: &CompilationMessageContext) -> AfterReceiveAction {
        match message {
            CompilationMessage::PerformTyping { unit, type_repository, compiler, has_prior_errors } => 
                handle_perform_typing(unit, ctx, &type_repository, &compiler, has_prior_errors),
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
    compiler: &CompilationActorHandle,
    has_prior_errors: bool
) -> AfterReceiveAction {
    let mut errors = create_compilation_errors(unit.filename.clone());
    let resolved_types = perform_typing(ctx, type_repository, &mut unit, &mut errors, has_prior_errors);
    notify_compiler_unit_has_been_typed(compiler, resolved_types, unit, errors);    
    shutdown_after_receive()
}

fn notify_compiler_unit_has_been_typed(
    compiler: &CompilationActorHandle,
    resolved_types: RuntimeTypePointers,
    unit: CompilationUnit,
    errors: CompilationErrors
) {
    send_message_to_actor(compiler, create_unit_typed_event(resolved_types, unit, errors));
}

pub fn perform_typing(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    unit: &mut CompilationUnit,
    errors: &mut CompilationErrors,
    has_prior_errors: bool
) -> RuntimeTypePointers {
    let mut resolved_types = vec!();

    if has_prior_errors {
        return resolved_types;
    }

    match unit.tree.item_mut() {
        AbstractSyntaxNodeItem::Run { expr } => {
            perform_typing_for_inferred_type_expression(ctx, type_repository, &create_identifier_type_lookup(), expr, errors);        
        },
        AbstractSyntaxNodeItem::Constant { name, value, constant_type } => {
            perform_typing_for_constant(unit.id, ctx, type_repository, &mut resolved_types, name, value, constant_type, errors);        
        },
        AbstractSyntaxNodeItem::ProcedureHeader { name, args, return_args, .. } => {
            perform_typing_for_procedure_header(unit.id, name, &mut resolved_types, args, return_args, errors);                      
        },
        AbstractSyntaxNodeItem::ProcedureBody { args, return_types, statements, .. } => {
            perform_typing_for_procedure_body(ctx, type_repository, args, return_types, statements, errors);
        },
        _ => {}
    };
    resolved_types
    
}