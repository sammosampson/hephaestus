use crate::typing::*;
use crate::utilities::*;

pub fn perform_typing_for_procedure_header(
    unit_id: CompilationUnitId,
    name: &str,
    resolved_types: &mut RuntimeTypePointers,
    args: &mut AbstractSyntaxChildNodes,
    return_args: &mut AbstractSyntaxChildNodes
) {
    let mut arg_types = vec!();
    let mut return_arg_types = vec!();
    
    for arg in args {
        match arg.item_mut() {
            AbstractSyntaxNodeItem::Declaration { arg_type: type_id, .. } => 
                try_parse_resolved_runtime_type_pointer(type_id, &mut arg_types),
            item => panic!("{:?} is not viable procedure header arg", item)
        }
    }

    for return_type in return_args {
        match return_type.item_mut() {
            AbstractSyntaxNodeItem::Type(resolvable_type) => 
                try_parse_resolved_runtime_type_pointer(resolvable_type, &mut return_arg_types),
            item => panic!("{:?} is not viable procedure header return arg", item)
        }
    }

    resolved_types.push(create_procedure_definition_type(unit_id, name, arg_types, return_arg_types));  
}

fn create_procedure_definition_type(
    unit_id: CompilationUnitId,
    name: &str,
    arg_types: RuntimeTypePointers,
    return_arg_types: RuntimeTypePointers
) -> RuntimeTypePointer {
    create_shareable(
        create_type(
            user_defined_runtime_type_id(unit_id),
            string(&name),
            procedure_definition_type_item(arg_types, return_arg_types),
            not_required_type_size()
        )
    )
}

fn try_parse_resolved_runtime_type_pointer(arg_type: &ResolvableType, type_ids: &mut RuntimeTypePointers) {
    if let Some(resolved_runtime_type_pointer) = try_get_resolved_runtime_type_pointer(arg_type) {
        type_ids.push(resolved_runtime_type_pointer);
    }
}
