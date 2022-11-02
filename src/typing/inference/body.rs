
use crate::parsing::*;
use crate::compilation::*;
use crate::typing::*;
use crate::types::*;
use crate::errors::*;

pub type LocalTypes = RuntimeTypePointers;

pub fn perform_typing_for_procedure_body(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    args: &mut AbstractSyntaxChildNodes,
    return_types: &mut AbstractSyntaxChildNodes,
    statements: &mut AbstractSyntaxChildNodes, 
    errors: &mut CompilationErrors
) {
    let mut local_type_map = create_identifier_type_lookup();
    let mut local_return_types = vec!();

    for arg in args {
        let arg_position = arg.position.clone();
        match arg.item_mut() {
            AbstractSyntaxNodeItem::MemberDeclaration { name, member_type: type_id } => 
                perform_typing_for_procedure_body_argument_declaration(&mut local_type_map, name, type_id),
                _ => add_type_inference_error(errors, not_viable_procedure_body_argument_error(), arg_position)
        }
    }

    for return_type in return_types {
        let return_type_position = return_type.position.clone();
        match return_type.item_mut() {
            AbstractSyntaxNodeItem::Type(resolvable_type) => 
                perform_typing_for_procedure_body_return_type_declaration(&mut local_return_types, resolvable_type),
                _ => add_type_inference_error(errors, not_viable_procedure_body_return_type_error(), return_type_position) 
        }
    }
    
    for statement in statements {
        let statement_position = statement.position.clone();
        match statement.item_mut() {
            AbstractSyntaxNodeItem::ProcedureCall { name, args, procedure_call_type: type_id} => 
                perform_typing_for_procedure_body_procedure_call(ctx, type_repository, &mut local_type_map, name, args, type_id, statement_position, errors),
            AbstractSyntaxNodeItem::VariableDeclaration { name, value, variable_type: type_id } => 
                perform_typing_for_procedure_body_assignment(ctx, type_repository, &mut local_type_map, name, value, type_id, errors),
            AbstractSyntaxNodeItem::Return { args } => {
                perform_typing_for_procedure_body_return_args(ctx, type_repository, &mut local_type_map, args, statement_position, &local_return_types, errors);
            },
            _ => add_type_inference_error(errors, not_viable_procedure_body_statement_error(), statement_position) 
        }
    }    
}

fn perform_typing_for_procedure_body_return_args(ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &mut IdentifierTypeLookup,
    args: &mut AbstractSyntaxChildNodes,
    args_position: SourceFilePosition,
    local_return_types: &RuntimeTypePointers, 
    errors: &mut CompilationErrors
) {
    perform_typing_for_known_target_type_args(ctx, type_repository, local_type_map, local_return_types, args, args_position, errors);
}

fn perform_typing_for_procedure_body_argument_declaration(local_type_map: &mut IdentifierTypeLookup, name: &mut String, arg_type: &mut ResolvableType) {
    if let Some(resolved_type) = try_get_resolved_runtime_type_pointer(arg_type) {
        add_to_identifier_type_lookup(local_type_map, name.clone(), resolved_type);    
    }
}

fn perform_typing_for_procedure_body_return_type_declaration(local_return_types: &mut LocalTypes, return_type: &mut ResolvableType) {      
    if let Some(resolved_type) = try_get_resolved_runtime_type_pointer(return_type) {
        local_return_types.push(resolved_type);
    }
}

fn perform_typing_for_procedure_body_procedure_call(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &mut IdentifierTypeLookup,
    name: &mut String,
    args: &mut AbstractSyntaxChildNodes,
    type_id: &mut ResolvableType,
    position: SourceFilePosition,
    errors: &mut CompilationErrors
) {   
    perform_typing_for_procedure_call_return_first_return_type(ctx, type_repository, local_type_map, args, name, type_id, position, errors);
}

fn perform_typing_for_procedure_body_assignment(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &mut IdentifierTypeLookup,
    name: &mut String,
    value: &mut AbstractSyntaxNode,
    assignment_type: &mut ResolvableType,
    errors: &mut CompilationErrors
) {
    if let Some(resolved_assignment_type) = try_get_resolved_runtime_type_pointer(assignment_type) {
        perform_typing_for_known_type_procedure_body_assignment(ctx, type_repository, local_type_map, name, value, &resolved_assignment_type, errors);
    } else {
        perform_typing_for_inferred_procedure_body_assignment(ctx, type_repository, local_type_map, name, value, assignment_type, errors);
    }
}

fn perform_typing_for_known_type_procedure_body_assignment(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &mut IdentifierTypeLookup,
    name: &mut String,
    value: &mut AbstractSyntaxNode,
    resolved_type: &RuntimeTypePointer,
    errors: &mut CompilationErrors
) {
    perform_typing_for_known_target_type_expression(ctx, type_repository, local_type_map, value, resolved_type, errors);
    add_to_identifier_type_lookup(local_type_map, name.clone(), resolved_type.clone());    
}

fn perform_typing_for_inferred_procedure_body_assignment(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &mut IdentifierTypeLookup,
    name: &mut String,
    value: &mut AbstractSyntaxNode,
    resolvable_type: &mut ResolvableType,
    errors: &mut CompilationErrors
) {
    let resolved_type = perform_typing_for_inferred_type_expression(ctx, type_repository, local_type_map, value, errors);
    
    if let Some(resolved_type) = resolved_type {
        *resolvable_type = resolved_resolvable_type(resolved_type.clone());
        add_to_identifier_type_lookup(local_type_map, name.clone(), resolved_type);
    }
}

pub fn perform_typing_for_procedure_call_return_first_return_type(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    args: &mut AbstractSyntaxChildNodes,
    name: &mut String, 
    type_id: &mut ResolvableType,
    position: SourceFilePosition,
    errors: &mut CompilationErrors
) -> RuntimeTypePointers {  
    let resolved_arg_types = perform_typing_for_unknown_target_type_args(ctx, type_repository, local_type_map, args, errors);
    
    match find_type_by_name_and_args(ctx, type_repository, name, resolved_arg_types) {
        Ok(resolved_type) => {
            *type_id = resolved_resolvable_type(resolved_type.clone());
            if let Some((_arg_types, return_types)) = try_get_procedure_definition_runtime_type_item(&resolved_type.item) {
                return return_types;
            }
        }        
        Err(error) => {
            add_compilation_error(errors, create_compilation_error(error, position));
        },
    }       

    vec!()
}
fn perform_typing_for_unknown_target_type_args(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    args: &mut AbstractSyntaxChildNodes,
    errors: &mut CompilationErrors
)-> RuntimeTypePointers {
    let mut resolved_types = vec!();

    for arg in args {
        perform_typing_for_unknown_target_type_arg(ctx, type_repository, local_type_map, &mut resolved_types, arg, errors);
    }

    resolved_types
}

fn perform_typing_for_unknown_target_type_arg(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    resolved_types: &mut RuntimeTypePointers,
    arg: &mut AbstractSyntaxNode,
    errors: &mut CompilationErrors
) {
    match arg.item_mut() {
        AbstractSyntaxNodeItem::Argument { expr, arg_type } => {
            if let Some(resolved_type) = perform_typing_for_inferred_type_expression(ctx, type_repository, local_type_map, expr, errors) {
                *arg_type = resolved_resolvable_type(resolved_type.clone());
                resolved_types.push(resolved_type);
            }
        }
        _ => {}
    }
}

fn perform_typing_for_known_target_type_args(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    known_target_types: &RuntimeTypePointers,
    args: &mut AbstractSyntaxChildNodes,
    position: SourceFilePosition,
    errors: &mut CompilationErrors
) {
    if args.len() != known_target_types.len() {
        add_type_inference_error(errors, args_and_known_types_are_not_same_length_error(), position);
    }

    for arg_index in 0..args.len() {
        perform_typing_for_known_target_type_arg(
            ctx,
            type_repository,
            local_type_map,
            &known_target_types[arg_index],
            &mut args[arg_index],
            errors
        );
    }
}

fn perform_typing_for_known_target_type_arg(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    known_target_type: &RuntimeTypePointer,
    arg: &mut AbstractSyntaxNode,
    errors: &mut CompilationErrors
) {
    match arg.item_mut() {
        AbstractSyntaxNodeItem::Argument { expr, arg_type } => {
            perform_typing_for_known_target_type_expression(ctx, type_repository, local_type_map, expr, known_target_type, errors);
            *arg_type = resolved_resolvable_type(known_target_type.clone());
        }
        _ => {}
    }
}