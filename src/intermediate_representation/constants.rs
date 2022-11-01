use crate::{
    parsing::*,
    compilation::*,
    intermediate_representation::*,
    strings::*
};

pub fn build_bytecode_at_top_root_const(ir: &mut IntermediateRepresentation, name: &str, value: &AbstractSyntaxNode, errors: &mut CompilationErrors) {
    ir.top_level_symbol = string(name);
    let const_position = value.position.clone();
    match value.item_ref() {
        AbstractSyntaxNodeItem::ForeignSystemLibrary { library } =>
            build_bytecode_at_foreign_system_library_const(ir, library, errors),
        AbstractSyntaxNodeItem::Literal(literal) => {
            if let Some(resolved_literal) = try_get_resolved_literal(literal) {
                build_bytecode_at_literal_const(ir, name, &resolved_literal, errors);
            } else {
                add_intermediate_representation_error(errors, literal_not_resolved_error(), const_position);
            }
        },        
        _ => todo(errors, function!(), "other top level const types"),
    }
}

fn build_bytecode_at_foreign_system_library_const(ir: &mut IntermediateRepresentation, library: &AbstractSyntaxNode, errors: &mut CompilationErrors) {
    let library_position = library.position.clone();
    match library.item_ref() {
        AbstractSyntaxNodeItem::Literal(literal) => {
            if let Some(resolved_literal) = try_get_resolved_literal(literal) {
                build_bytecode_at_foreign_system_library_literal_const(ir, &resolved_literal, errors);
            } else {
                add_intermediate_representation_error(errors, literal_not_resolved_error(), library_position);
            }
        },
        _ => todo(errors, function!(), "foreign system library none literals")
    }
}

fn build_bytecode_at_foreign_system_library_literal_const(ir: &mut IntermediateRepresentation, library: &ResolvedLiteral, errors: &mut CompilationErrors) {
    match library {
        ResolvedLiteral::String(value) =>
            add_foreign_library_reference(&mut ir.foreign_libraries, byte_string_to_string(value)),
            _ => todo(errors, function!(),"foreign system library none string literals")
    }
}

fn build_bytecode_at_literal_const(ir: &mut IntermediateRepresentation, const_name: &str, library: &ResolvedLiteral, errors: &mut CompilationErrors) {
    match library {
        ResolvedLiteral::SignedInt64(number) => {
            add_symbol(&mut ir.symbols, absolute_external_64(string(const_name), *number as u64));
        },
        ResolvedLiteral::SignedInt32(number) => {
            add_symbol(&mut ir.symbols, absolute_external_32(string(const_name), *number as u32));
        },
        _ => todo(errors, function!(), "const none signed int number literals")
    }
}
