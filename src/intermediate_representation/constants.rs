use crate::{
    parsing::*,
    intermediate_representation::*
};

pub fn build_bytecode_at_top_root_const(ir: &mut IntermediateRepresentation, name: &str, value: &AbstractSyntaxNode) {
    ir.top_level_symbol = string(name);
    match value.item_ref() {
        AbstractSyntaxNodeItem::ForeignSystemLibrary { library } =>
            build_bytecode_at_foreign_system_library_const(ir, library),
        AbstractSyntaxNodeItem::Literal(literal) =>
            build_bytecode_at_literal_const(ir, name, &get_resolved_literal(literal)),        
        _ => todo!()
    }
}

fn build_bytecode_at_foreign_system_library_const(ir: &mut IntermediateRepresentation, library: &AbstractSyntaxNode) {
    match library.item_ref() {
        AbstractSyntaxNodeItem::Literal(literal) =>
            build_bytecode_at_foreign_system_library_literal_const(ir, &get_resolved_literal(literal)),
        _ => todo!("foreign system library none literals")
    }
}

fn build_bytecode_at_foreign_system_library_literal_const(ir: &mut IntermediateRepresentation, library: &ResolvedLiteral) {
    match library {
        ResolvedLiteral::String(value) =>
            add_foreign_library_reference(&mut ir.foreign_libraries, string(value)),
        _ => todo!("foreign system library none string literals")
    }
}

fn build_bytecode_at_literal_const(ir: &mut IntermediateRepresentation, const_name: &str, library: &ResolvedLiteral) {
    match library {
        ResolvedLiteral::SignedInt64(number) => {
            add_symbol(&mut ir.symbols, absolute_external_64(string(const_name), *number as u64));
        },
        ResolvedLiteral::SignedInt32(number) => {
            add_symbol(&mut ir.symbols, absolute_external_32(string(const_name), *number as u32));
        },
        _ => todo!("const none signed int number literals")
    }
}
