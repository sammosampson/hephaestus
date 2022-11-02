use crate::parsing::*;
use crate::compilation::*;
use crate::threading::*;
use crate::typing::*;
use crate::types::*;
use crate::utilities::*;
use crate::errors::*;

pub fn perform_typing_for_constant(
    unit_id: CompilationUnitId,
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    resolved_types: &mut RuntimeTypePointers,
    name: &str,
    value: &mut AbstractSyntaxNode,
    constant_type: &mut ResolvableType,
    errors: &mut CompilationErrors
) {
    if let Some(resolved_constant_type) = try_get_resolved_runtime_type_pointer(constant_type) {
        perform_typing_for_known_type_constant(ctx, type_repository, value, &resolved_constant_type, errors);
    } else {
        perform_typing_for_inferred_constant(ctx, type_repository, value, constant_type, errors);
    }

    if let Some(resolved_constant_type) = try_get_resolved_runtime_type_pointer(constant_type) {
        resolved_types.push(create_constant_definition_type(unit_id, name, resolved_constant_type));  
    }
}

fn create_constant_definition_type(unit_id: CompilationUnitId, name: &str, constant_type: RuntimeTypePointer) -> RuntimeTypePointer {
    create_shareable(
        create_type(
            user_defined_runtime_type_id(unit_id),
            string(&name),
            constant_definition_type_item(constant_type),
            not_required_type_size()
        )
    )
}

fn perform_typing_for_inferred_constant(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    value: &mut AbstractSyntaxNode,
    constant_type: &mut ResolvableType,
    errors: &mut CompilationErrors
) {
    if let Some(expression_type) = perform_typing_for_inferred_type_expression(
        ctx,
        type_repository, 
        &create_identifier_type_lookup(), 
        value,
        errors
    ) {
        *constant_type = resolved_resolvable_type(expression_type);
    }
}

fn perform_typing_for_known_type_constant(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    value: &mut AbstractSyntaxNode,
    constant_type: &RuntimeTypePointer,
    errors: &mut CompilationErrors
) {
    perform_typing_for_known_target_type_expression(
        ctx,
        type_repository, 
        &create_identifier_type_lookup(), 
        value,
        constant_type,
        errors
    );
}
