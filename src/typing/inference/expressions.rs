use crate::parsing::*;
use crate::compilation::*;
use crate::threading::*;
use crate::typing::*;
use crate::types::*;
use crate::utilities::*;
use crate::errors::*;

pub fn perform_typing_for_known_target_type_expression(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    expr: &mut AbstractSyntaxNode,
    known_target_type: &RuntimeTypePointer,
    errors: &mut CompilationErrors
) { 
    let expr_position = expr.position.clone();
    
    match expr.item_mut() {
        AbstractSyntaxNodeItem::Literal(literal) => {
            perform_typing_for_known_target_type_expression_literal(literal, known_target_type, expr_position, errors);
        },
        AbstractSyntaxNodeItem::Identifier { name, scope }  => {
            perform_typing_for_expression_identifier(ctx, type_repository, local_type_map, name, scope, expr_position, errors);
        },
        AbstractSyntaxNodeItem::BinaryExpr { lhs, rhs, expression_type, ..} => {
            perform_typing_for_expression_expression(ctx, type_repository, local_type_map, lhs, rhs, expression_type, errors);
        },
        AbstractSyntaxNodeItem::ProcedureCall { name, args, procedure_call_type } => {
            perform_typing_for_expression_procedure_call(ctx, type_repository, local_type_map, name, args, procedure_call_type, expr_position, errors);
        },
        AbstractSyntaxNodeItem::MemberExpr { instance, member, member_expression_type } => {
            perform_typing_for_member_expression(ctx, type_repository, local_type_map, instance, member, member_expression_type, errors);
        },
        _ => {}
    }
}

fn perform_typing_for_known_target_type_expression_literal(
    literal: &mut ResolvableLiteral,
    known_target_type: &RuntimeTypePointer, 
    literal_position: SourceFilePosition,
    errors: &mut CompilationErrors
)  {
    if let ResolvableLiteral::Unresolved(unresolved_literal) = literal {
        match unresolved_literal {
            UnresolvedLiteral::Int(value) => {
                if let RuntimeTypeItem::Int { is_signed } = known_target_type.item {
                    if let TypeSize::Resolved { size_in_bytes } = known_target_type.size {
                        let resolved = match size_in_bytes {
                            1 => resolve_to_int_8_literal_if_possible(value, is_signed),
                            2 => resolve_to_int_16_literal_if_possible(value, is_signed),
                            4 => resolve_to_int_32_literal_if_possible(value, is_signed),
                            8 => resolve_to_int_64_literal_if_possible(value, is_signed),
                            n => {
                                add_type_inference_error(errors, type_size_in_bytes_invalid_error(n), literal_position);
                                None
                            }
                        };
                        if let Some(resolved) = resolved {
                            *literal = resolved_resolvable_literal(resolved);
                        } else {
                            add_type_inference_error(errors, literal_value_is_too_large_for_target_type_error(), literal_position);
                        }
                    } else {
                        add_type_inference_error(errors, target_type_size_is_not_resolved_error(), literal_position);
                    }
                } else {
                    add_type_inference_error(errors, literal_value_is_not_for_target_type_error(), literal_position);
                }
            },
            UnresolvedLiteral::Float(number) => { 
                if let RuntimeTypeItem::Float = known_target_type.item {
                    if let TypeSize::Resolved { size_in_bytes } = known_target_type.size {
                        let resolved = match size_in_bytes {
                            4 => resolve_to_float_32_literal_if_possible(number),
                            8 => resolve_to_float_64_literal_if_possible(number),
                            n => {
                                add_type_inference_error(errors, type_size_in_bytes_invalid_error(n), literal_position);
                                None
                            }
                        };
                        if let Some(resolved) = resolved {
                            *literal = resolved_resolvable_literal(resolved);
                        } else {
                            add_type_inference_error(errors, literal_value_is_too_large_for_target_type_error(), literal_position);
                        }
                    } else {
                        add_type_inference_error(errors, target_type_size_is_not_resolved_error(), literal_position);
                    }
                } else {
                    add_type_inference_error(errors, literal_value_is_not_for_target_type_error(), literal_position);
                }
            },
            UnresolvedLiteral::String(value) => {
                match known_target_type.item {
                    RuntimeTypeItem::String { .. }  => {
                        *literal = resolved_resolvable_literal(resolved_string_literal(value.clone()));
                    }, 
                    _ => add_type_inference_error(errors, literal_value_is_not_for_target_type_error(), literal_position)
                }
            },
        }
    } else {
        add_type_inference_error(errors, literal_should_not_be_resolved_error(), literal_position);
    }
}

fn resolve_to_int_8_literal_if_possible(number: &str, is_signed: bool) -> Option<ResolvedLiteral> {
    if is_signed {
        if let Ok(converted_number) = parse_signed_8_from_string(number) {
            return Some(resolved_signed_int_8_literal(converted_number));
        }
    } else {
        if let Ok(converted_number) = parse_unsigned_8_from_string(number) {
            return Some(resolved_unsigned_int_8_literal(converted_number));
        }
    }
    None
}

fn resolve_to_int_16_literal_if_possible(number: &str, is_signed: bool) -> Option<ResolvedLiteral> {
    if is_signed {
        if let Ok(converted_number) = parse_signed_16_from_string(number) {
            return Some(resolved_signed_int_16_literal(converted_number));
        }
    } else {
        if let Ok(converted_number) = parse_unsigned_16_from_string(number) {
            return Some(resolved_unsigned_int_16_literal(converted_number));
        }
    }
    None
}

fn resolve_to_int_32_literal_if_possible(number: &str, is_signed: bool) -> Option<ResolvedLiteral> {
    if is_signed {
        if let Ok(converted_number) = parse_signed_32_from_string(number) {
            return Some(resolved_signed_int_32_literal(converted_number));
        }
    } else {
        if let Ok(converted_number) = parse_unsigned_32_from_string(number) {
            return Some(resolved_unsigned_int_32_literal(converted_number));
        }
    }
    None
}

fn resolve_to_int_64_literal_if_possible(number: &str, is_signed: bool) -> Option<ResolvedLiteral> {
    if is_signed {
        if let Ok(converted_number) = parse_signed_64_from_string(number) {
            return Some(resolved_signed_int_64_literal(converted_number));
        }
    } else {
        if let Ok(converted_number) = parse_unsigned_64_from_string(number) {
            return Some(resolved_unsigned_int_64_literal(converted_number));
        }
    }
    None
}

fn resolve_to_float_32_literal_if_possible(number: &str) -> Option<ResolvedLiteral> {
    if let Ok(converted_number) = parse_float_32_from_string(number) {
        return Some(resolved_float_32_literal(converted_number));
    }
    None
}

fn resolve_to_float_64_literal_if_possible(number: &str) -> Option<ResolvedLiteral> {
    if let Ok(converted_number) = parse_float_64_from_string(number) {
        return Some(resolved_float_64_literal(converted_number));
    }
    None
}

pub fn perform_typing_for_inferred_type_expression(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    expr: &mut AbstractSyntaxNode,
    errors: &mut CompilationErrors
)  -> OptionalRuntimeTypePointer { 
    let expr_position = expr.position.clone();

    match expr.item_mut() {
        AbstractSyntaxNodeItem::ForeignSystemLibrary{ library } =>
            perform_typing_for_inferred_type_expression(ctx, type_repository, local_type_map, library, errors),
        AbstractSyntaxNodeItem::Literal(literal) =>
            perform_typing_for_inferred_type_expression_literal(literal, expr_position, errors),
        AbstractSyntaxNodeItem::Identifier { name, scope} =>
            perform_typing_for_expression_identifier(ctx, type_repository, local_type_map, name, scope, expr_position, errors),
        AbstractSyntaxNodeItem::BinaryExpr { lhs, rhs, expression_type: type_id, ..} =>
            perform_typing_for_expression_expression(ctx, type_repository, local_type_map, lhs, rhs, type_id, errors),
        AbstractSyntaxNodeItem::ProcedureCall { name, args, procedure_call_type: type_id } =>
            perform_typing_for_expression_procedure_call(ctx, type_repository, local_type_map, name, args, type_id, expr_position, errors),
        AbstractSyntaxNodeItem::Cast { cast_type, expr} =>
            perform_typing_for_expression_cast(ctx, type_repository, local_type_map, cast_type, expr, errors),
        AbstractSyntaxNodeItem::MemberExpr { instance, member, member_expression_type } =>
            perform_typing_for_member_expression(ctx, type_repository, local_type_map, instance, member, member_expression_type, errors),
        _ => None
    }
}
fn perform_typing_for_inferred_type_expression_literal(
    literal: &mut ResolvableLiteral,
    literal_position: SourceFilePosition,   
    errors: &mut CompilationErrors
) -> OptionalRuntimeTypePointer {
    if let ResolvableLiteral::Unresolved(unresolved_literal) = literal {
        match unresolved_literal {
            UnresolvedLiteral::Int(value) => {
                *literal = resolved_resolvable_literal(resolve_to_int_64_literal_if_possible(value, true).unwrap());
                return Some(create_shareable(signed_int_64_runtime_type()));
            },
            UnresolvedLiteral::Float(value) => {
                *literal = resolved_resolvable_literal(resolve_to_float_32_literal_if_possible(value).unwrap());
                return Some(create_shareable(float_32_runtime_type()));
            }
            UnresolvedLiteral::String(value) => {
                *literal = resolved_resolvable_literal(resolved_string_literal(value.clone()));
                return Some(create_shareable(string_runtime_type()));
            },
        }
    }
    add_type_inference_error(errors, literal_should_not_be_resolved_error(), literal_position);
    None
}

fn perform_typing_for_expression_identifier(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    name: &mut String,
    scope: &mut Scope,
    identifier_position: SourceFilePosition,   
    errors: &mut CompilationErrors
) -> OptionalRuntimeTypePointer {
    if let Some(local_identifier_type) = get_type_for_identifier(local_type_map, &name) {
        *scope = local_scope();
        return Some(local_identifier_type.clone());
    }
    *scope = global_scope();
    get_global_type_for_identifier(ctx, type_repository, name, identifier_position, errors)
}

fn get_global_type_for_identifier(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    name: &mut String,
    identifier_position: SourceFilePosition,   
    errors: &mut CompilationErrors
) -> OptionalRuntimeTypePointer {

    match find_type_by_name(ctx, type_repository, name) {
        Ok(global_type) => {
            if let Some(global_type) = try_get_constant_definition_runtime_type_item(&global_type.item) {
                return Some(global_type);
            }        
        },
        Err(error) => {
            add_compilation_error(errors, compilation_error(error, identifier_position));
        },
    }
    
    None
}

fn perform_typing_for_expression_expression(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    lhs: &mut AbstractSyntaxNode,
    rhs: &mut AbstractSyntaxNode,
    type_id: &mut ResolvableType,
    errors: &mut CompilationErrors
) -> OptionalRuntimeTypePointer {
    let lhs_resolved_type = perform_typing_for_inferred_type_expression(ctx, type_repository, local_type_map, lhs, errors);
    let rhs_resolved_type = perform_typing_for_inferred_type_expression(ctx, type_repository, local_type_map, rhs, errors);
    
    if lhs_resolved_type != rhs_resolved_type {
        todo(errors, function!(), "deal with different types on either side of expression");
    }

    if let Some(resolved_type) = lhs_resolved_type {
        *type_id = resolved_resolvable_type(resolved_type.clone());
        return Some(resolved_type);
    }

    None
}

fn perform_typing_for_expression_procedure_call(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    name: &mut String,
    args: &mut AbstractSyntaxChildNodes,
    type_id: &mut ResolvableType,
    position: SourceFilePosition,
    errors: &mut CompilationErrors
) -> OptionalRuntimeTypePointer {
    let resolved_types = perform_typing_for_procedure_call_return_first_return_type(
        ctx,
        type_repository,
        local_type_map,
        args,
        name,
        type_id,
        position,
        errors
    );
    
    if resolved_types.len() > 0 {
        return Some(resolved_types.first().unwrap().clone());
    }

    todo(errors, function!(), "Deal with no resolved types returned");
    None
}

fn perform_typing_for_expression_cast(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    cast_type: &mut ResolvableType,
    expr: &mut AbstractSyntaxNode,
    errors: &mut CompilationErrors
) -> OptionalRuntimeTypePointer {
    if let Some(resolved_cast_type) = try_get_resolved_runtime_type_pointer(&cast_type) {
        perform_typing_for_known_target_type_expression(
            ctx,
            type_repository,
            local_type_map,
            expr,
            &resolved_cast_type,
            errors
        );
        return Some(resolved_cast_type.clone());
    }
    None
}

fn perform_typing_for_member_expression(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    instance: &mut AbstractSyntaxNode,
    member: &mut AbstractSyntaxNode,
    member_expression_type: &mut ResolvableType,
    errors: &mut CompilationErrors
) -> OptionalRuntimeTypePointer {
    
    if let Some(instance_type) = perform_typing_for_member_expression_instance(
        ctx,
        type_repository,
        local_type_map,
        instance,
        errors
    ) {
        if let Some(resolved_member_expression_type) = perform_typing_for_member_expression_member(&instance_type, member) {
            *member_expression_type = resolved_resolvable_type(resolved_member_expression_type.clone());
            return Some(resolved_member_expression_type);
        }
    }
    None
}

fn perform_typing_for_member_expression_instance(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    instance: &mut AbstractSyntaxNode,
    errors: &mut CompilationErrors
) -> OptionalRuntimeTypePointer {
    let instance_position = instance.position.clone();
    match instance.item_mut() {
        AbstractSyntaxNodeItem::Instance { name, instance_type, scope} => {
            if let Some(resolved_member_instance_type) = perform_typing_for_expression_identifier(ctx, type_repository, local_type_map, name, scope, instance_position, errors) {
                *instance_type = resolved_resolvable_type(resolved_member_instance_type.clone());
                return Some(resolved_member_instance_type);
            }
            return None;
        },
        _ => None
    }
}

fn perform_typing_for_member_expression_member(
    instance_type: &RuntimeTypePointer,
    member: &mut AbstractSyntaxNode
) -> OptionalRuntimeTypePointer {
    
    match member.item_mut() {
        AbstractSyntaxNodeItem::Member { name, member_type } =>
        perform_typing_for_member_expression_member_member(instance_type, name, member_type),
        _ => None
    }
}

fn perform_typing_for_member_expression_member_member(
    instance_type: &RuntimeTypePointer,
    name: &mut String,
    member_type: &mut ResolvableType
) -> OptionalRuntimeTypePointer {
    match &instance_type.item {
        RuntimeTypeItem::String { members } => {
            if let Some(field_type) = get_type_of_member_by_member_name(members, name) {
                *member_type = resolved_resolvable_type(field_type.clone());
                return Some(field_type);
            }
            None

        },
        _ => None
    }
}
