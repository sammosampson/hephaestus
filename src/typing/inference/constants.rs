use crate::parsing::*;
use crate::compilation::*;
use crate::typing::*;


pub fn perform_typing_for_constant(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    value: &mut AbstractSyntaxNode,
    constant_type: &mut ResolvableType
) {
    if let Some(resolved_constant_type) = try_get_resolved_runtime_type_pointer(constant_type) {
        perform_typing_for_known_type_constant(ctx, type_repository, value, &resolved_constant_type);
    } else {
        perform_typing_for_inferred_constant(ctx, type_repository, value, constant_type);
    }
}

fn perform_typing_for_inferred_constant(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    value: &mut AbstractSyntaxNode,
    constant_type: &mut ResolvableType
) {
    if let Some(expression_type) = perform_typing_for_inferred_type_expression(
        ctx,
        type_repository, 
        &create_local_type_map(), 
        value
    ) {
        *constant_type = resolved_resolvable_type(expression_type);
    }
}

fn perform_typing_for_known_type_constant(
    ctx: &CompilationMessageContext,
    type_repository: &CompilationActorHandle,
    value: &mut AbstractSyntaxNode,
    constant_type: &RuntimeTypePointer
) {
    perform_typing_for_known_target_type_expression(
        ctx,
        type_repository, 
        &create_local_type_map(), 
        value,
        constant_type
    );
}
