use crate::parsing::*;
use crate::compilation::*;
use crate::threading::*;
use crate::typing::*;
use crate::types::*;
use crate::utilities::*;

pub fn perform_typing_for_known_target_type_expression(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    expr: &mut AbstractSyntaxNode,
    known_target_type: &RuntimeTypePointer
) { 
    match expr.item_mut() {
        AbstractSyntaxNodeItem::Literal(literal) => {
            perform_typing_for_known_target_type_expression_literal(literal, known_target_type);
        },
        AbstractSyntaxNodeItem::Identifier { name, scope }  => {
            perform_typing_for_expression_identifier(ctx, type_repository, local_type_map, name, scope);
        },
        AbstractSyntaxNodeItem::BinaryExpr { lhs, rhs, expression_type, ..} => {
            perform_typing_for_expression_expression(ctx, type_repository, local_type_map, lhs, rhs, expression_type);
        },
        AbstractSyntaxNodeItem::ProcedureCall { name, args, procedure_call_type } => {
            perform_typing_for_expression_procedure_call(ctx, type_repository, local_type_map, name, args, procedure_call_type);
        },
        AbstractSyntaxNodeItem::MemberExpr { instance, member, member_expression_type } => {
            perform_typing_for_member_expression(ctx, type_repository, local_type_map, instance, member, member_expression_type);
        },
        _ => {}
    }
}

fn perform_typing_for_known_target_type_expression_literal(
    literal: &mut ResolvableLiteral,
    known_target_type: &RuntimeTypePointer
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
                            _ => panic!("type size in bytes invalid")
                        };
                        if let Some(resolved) = resolved {
                            *literal = resolved_resolvable_literal(resolved);
                        } else {
                            panic!("literal value is too large for target type")
                        }
                    } else {
                        panic!("target type size is not resolved")
                    }
                } else {
                    panic!("literal value is not for target type")
                }
            },
            UnresolvedLiteral::Float(number) => { 
                if let RuntimeTypeItem::Float = known_target_type.item {
                    if let TypeSize::Resolved { size_in_bytes } = known_target_type.size {
                        let resolved = match size_in_bytes {
                            4 => resolve_to_float_32_literal_if_possible(number),
                            8 => resolve_to_float_64_literal_if_possible(number),
                            _ => panic!("type size in bytes invalid")
                        };
                        if let Some(resolved) = resolved {
                            *literal = resolved_resolvable_literal(resolved);
                        } else {
                            panic!("literal value is too large for target type")
                        }
                    } else {
                        panic!("target type size is not resolved")
                    }
                } else {
                    panic!("literal value is not for target type")
                }
            },
            UnresolvedLiteral::String(value) => {
                match known_target_type.item {
                    RuntimeTypeItem::String { .. }  => {
                        *literal = resolved_resolvable_literal(resolved_string_literal(value.clone()));
                    }, 
                    _ => panic!("literal value is not for target type")
                }
            },
        }
    } else {
        panic!("literal should not be resolved at this point");
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
    expr: &mut AbstractSyntaxNode
)  -> OptionalRuntimeTypePointer { 
    match expr.item_mut() {
        AbstractSyntaxNodeItem::ForeignSystemLibrary{ library } =>
            perform_typing_for_inferred_type_expression(ctx, type_repository, local_type_map, library),
        AbstractSyntaxNodeItem::Literal(literal) =>
            perform_typing_for_inferred_type_expression_literal(literal),
        AbstractSyntaxNodeItem::Identifier { name, scope} =>
            perform_typing_for_expression_identifier(ctx, type_repository, local_type_map, name, scope),
        AbstractSyntaxNodeItem::BinaryExpr { lhs, rhs, expression_type: type_id, ..} =>
            perform_typing_for_expression_expression(ctx, type_repository, local_type_map, lhs, rhs, type_id),
        AbstractSyntaxNodeItem::ProcedureCall { name, args, procedure_call_type: type_id } =>
            perform_typing_for_expression_procedure_call(ctx, type_repository, local_type_map, name, args, type_id),
        AbstractSyntaxNodeItem::Cast { cast_type, expr} =>
            perform_typing_for_expression_cast(ctx, type_repository, local_type_map, cast_type, expr),
        AbstractSyntaxNodeItem::MemberExpr { instance, member, member_expression_type } =>
            perform_typing_for_member_expression(ctx, type_repository, local_type_map, instance, member, member_expression_type),
        _ => None
    }
}
fn perform_typing_for_inferred_type_expression_literal(literal: &mut ResolvableLiteral) -> OptionalRuntimeTypePointer {
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
    panic!("literal should not be resaolved at this point")
}

fn perform_typing_for_expression_identifier(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    name: &mut String,
    scope: &mut Scope
) -> OptionalRuntimeTypePointer {
    if let Some(local_identifier_type) = get_type_for_identifier(local_type_map, &name) {
        *scope = local_scope();
        return Some(local_identifier_type.clone());
    }
    *scope = global_scope();
    get_global_type_for_identifier(ctx, type_repository, name)
}

fn get_global_type_for_identifier(ctx: &CompilationMessageContext, type_repository: &CompilationActorHandle, name: &mut String) -> OptionalRuntimeTypePointer {
    let global_type = find_type_by_name(ctx, type_repository, name);
    if let Some(global_type) = try_get_constant_definition_runtime_type_item(&global_type.item) {
        return Some(global_type);
    }
    None
}

fn perform_typing_for_expression_expression(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    lhs: &mut AbstractSyntaxNode,
    rhs: &mut AbstractSyntaxNode,
    type_id: &mut ResolvableType
) -> OptionalRuntimeTypePointer {
    let lhs_resolved_type = perform_typing_for_inferred_type_expression(ctx, type_repository, local_type_map, lhs);
    let rhs_resolved_type = perform_typing_for_inferred_type_expression(ctx, type_repository, local_type_map, rhs);
    
    if lhs_resolved_type != rhs_resolved_type {
        todo!("deal with different types on either side of expression")
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
    type_id: &mut ResolvableType
) -> OptionalRuntimeTypePointer {
    let resolved_types = perform_typing_for_procedure_call_return_first_return_type(
        ctx,
        type_repository,
        local_type_map,
        args,
        name,
        type_id
    );
    
    if resolved_types.len() > 0 {
        return Some(resolved_types.first().unwrap().clone());
    }

    todo!()
}

fn perform_typing_for_expression_cast(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    local_type_map: &IdentifierTypeLookup,
    cast_type: &mut ResolvableType,
    expr: &mut AbstractSyntaxNode
) -> OptionalRuntimeTypePointer {
    if let Some(resolved_cast_type) = try_get_resolved_runtime_type_pointer(&cast_type) {
        perform_typing_for_known_target_type_expression(ctx, type_repository, local_type_map, expr, &resolved_cast_type);
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
    member_expression_type: &mut ResolvableType
) -> OptionalRuntimeTypePointer {
    
    if let Some(instance_type) = perform_typing_for_member_expression_instance(
        ctx,
        type_repository,
        local_type_map,
        instance
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
    instance: &mut AbstractSyntaxNode
) -> OptionalRuntimeTypePointer {
    
    match instance.item_mut() {
        AbstractSyntaxNodeItem::Instance { name, instance_type, scope} => {
            if let Some(resolved_member_instance_type) = perform_typing_for_expression_identifier(ctx, type_repository, local_type_map, name, scope) {
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