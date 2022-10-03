mod header;
mod body;
pub use header::*;
pub use body::*;

use crate::parsing::*;
use crate::acting::*;
use crate::compilation::*;
use crate::typing::*;

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
            perform_typing_for_expression(ctx, type_repository, &create_local_type_map(), expr);        
        },
        AbstractSyntaxNodeItem::ProcedureHeader { name, args, return_args, .. } => {
            perform_typing_for_procedure_header(unit.id, name, &mut resolved_types, args, return_args);                      
        },
        AbstractSyntaxNodeItem::ProcedureBody { args, return_types, statements, .. } => {
            perform_typing_for_procedure_body(ctx, type_repository, args, return_types, statements);
        },
        _ => {}
    };
    resolved_types
    
}
