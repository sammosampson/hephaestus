
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
) -> ResolvedTypes {
    let mut types = vec!();
    
    match unit.tree.item_mut() {
        AbstractSyntaxNodeItem::ProcedureHeader { 
            name,
            args, 
            return_types, 
            .. 
        } => {
            perform_typing_for_procedure_header(&mut types, unit.id, name.clone(), args, return_types);
        },
        AbstractSyntaxNodeItem::ProcedureBody(statements) => { 
            perform_typing_for_procedure_body(ctx, type_repository, statements);
        }, 
        item => panic!("unimplemented ast item for typing {:?}", item)
    };

    types
}

fn perform_typing_for_procedure_body(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle, 
    statements: &mut AbstractSyntaxChildNodes
) {
    for statement in statements {
        match statement.item_mut() {
            AbstractSyntaxNodeItem::ProcedureCall { name, args, arg_type 
            } => { 
                perform_typing_for_procedure_call(ctx, type_repository, name, args, arg_type);
            },
            item => panic!("unimplemented ast item for typing {:?}", item)
        };
    }
}

fn perform_typing_for_procedure_header(
    types: &mut ResolvedTypes,
    id: CompilationUnitId,
    name: String,
    arguments: &AbstractSyntaxChildNodes,
    return_types: &AbstractSyntaxChildNodes
) {
    types.push(
        create_type(
            id,
            name, 
            create_procedure_defnition_type_item(
                parse_arg_types(arguments),
                parse_return_types(return_types)
            )
        )
    );
}

fn perform_typing_for_procedure_call(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    _name: &str,
    _args: &mut AbstractSyntaxChildNodes,
    arg_type: &mut ResolvableType
) {
    let resolved_type = find_type(FindTypeCriteria, ctx, type_repository);
    *arg_type = ResolvableType::Resolved(resolved_type);
}


fn parse_arg_types(args: &AbstractSyntaxChildNodes) -> ResolvedTypeIds {
    let mut ids = vec!();

    for arg in args {
        parse_arg_type(arg, &mut ids)
    }

    ids
}

fn parse_arg_type(arg: &AbstractSyntaxNode, type_ids: &mut ResolvedTypeIds) {
    if let AbstractSyntaxNodeItem::ArgumentDeclaration { arg_type, .. } = arg.item_ref() {
        if let ResolvableType::Resolved(ResolvedTypeId::BuiltInType(built_in_type)) = arg_type {
            type_ids.push(create_built_in_type_id(built_in_type));
        }
    }
}


fn parse_return_types(args: &AbstractSyntaxChildNodes) -> ResolvedTypeIds {
    let mut ids = vec!();

    for arg in args {
        parse_return_type(arg, &mut ids)
    }

    ids
}

fn parse_return_type(arg: &AbstractSyntaxNode, type_ids: &mut ResolvedTypeIds) {
    if let AbstractSyntaxNodeItem::Type(return_type) = arg.item_ref() {
        if let ResolvableType::Resolved(ResolvedTypeId::BuiltInType(built_in_type)) = return_type {
            type_ids.push(create_built_in_type_id(built_in_type));
        }
    }
}