use crate::backends::*;
use crate::strings::*;
use crate::utilities::*;

fn advance_data_section(coff: &mut Coff, amount: u32) {
    coff.data_section_header.size_of_section += amount;
    coff.data_section_header.pointer_to_relocations += amount;
    coff.text_section_header.pointer_to_section += amount;
    coff.text_section_header.pointer_to_relocations += amount;
    coff.header.pointer_to_symbol_table += amount;
    set_current_timestamp(coff);
}

pub fn add_string_to_data_section(coff: &mut Coff, to_add: &ByteString) -> u32 {
    let pointer = coff.data_section_header.size_of_section; 
    let mut string_bytes = to_add.clone();
    advance_data_section(coff, string_bytes.len() as u32);
    coff.data_section.append(&mut string_bytes);
    pointer
}

pub fn add_quad_word_to_data_section(coff: &mut Coff, to_add: &u64) -> u32 {
    let pointer = coff.data_section_header.size_of_section; 
    let mut bytes = u64_to_bytes(to_add);
    advance_data_section(coff, bytes.len() as u32);
    coff.data_section.append(&mut bytes);
    pointer
}

pub fn add_pointer_to_data_section_and_make_relocation(coff: &mut Coff, to_add: &u64) -> u32 {
    let pointer = add_quad_word_to_data_section(coff, to_add);
    add_relocation_for_data_pointer(coff, pointer);
    pointer
}

fn add_relocation_for_data_pointer(coff: &mut Coff, pointer: u32) {
    add_data_section_relocation_entry(coff, relocation_entry(pointer, 2, IMAGE_REL_AMD64_ADDR64));
}

pub fn add_entry_to_text_section(coff: &mut Coff, entry: u8) {
    coff.text_section.push(entry);
    coff.text_section_header.size_of_section += 1;
    coff.text_section_header.pointer_to_relocations += 1;
    coff.header.pointer_to_symbol_table += 1;
    set_current_timestamp(coff);
}

pub fn add_entries_to_text_section(coff: &mut Coff, entries: Vec<u8>) {
    for entry in entries {
        add_entry_to_text_section(coff, entry);
    }
}