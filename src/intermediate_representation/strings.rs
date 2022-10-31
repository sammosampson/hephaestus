use crate::{intermediate_representation::*, strings::*};

pub fn store_string_literal_in_data_section_and_add_symbol(ir: &mut IntermediateRepresentation, value: &ByteString) -> DataSectionOffset {
    let literal_data_item_pointer = add_data_item(&mut ir.data, string_data_item(value.clone()));
    add_symbol(&mut ir.symbols, data_section_item(data_section_item_name(literal_data_item_pointer), *literal_data_item_pointer));
    literal_data_item_pointer
}

pub fn store_string_in_data_section_and_add_symbol(ir: &mut IntermediateRepresentation, string_length: usize, string_literal_data_item_pointer: DataSectionOffset) -> DataSectionOffset {
    let string_data_item_pointer = add_data_item(&mut ir.data, quad_word_data_item(string_length as u64));
    add_symbol(&mut ir.symbols, data_section_item(data_section_item_name(string_data_item_pointer), *string_data_item_pointer));
    add_data_item(&mut ir.data, pointer_data_item(*string_literal_data_item_pointer as u64));
    string_data_item_pointer
}