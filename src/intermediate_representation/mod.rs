#![allow(dead_code)]

mod builder;
mod procedures;
mod constants;
mod strings;

pub use builder::*;
pub use procedures::*;
pub use constants::*;
pub use strings::*;

use crate::{
    parsing::CompilationUnitId,
    utilities::*
};

pub type ForeignLibraryReferences = Vec<String>;

pub fn add_foreign_library_reference(references: &mut ForeignLibraryReferences, reference: String) {
    references.push(reference);
}

#[derive(Debug, Clone)]
pub struct IntermediateRepresentation {
    pub id: CompilationUnitId,
    pub filename: String,
    pub top_level_symbol: String,
    pub byte_code: ByteCodeInstructionStream,
    pub symbols: ByteCodeSymbols,
    pub data: ByteCodeData,
    pub foreign_libraries: ForeignLibraryReferences
}

pub fn create_intermediate_representation(id: CompilationUnitId, filename: String) -> IntermediateRepresentation {
    IntermediateRepresentation {
        id,
        filename,
        top_level_symbol: string(""),
        byte_code: vec!(),
        symbols: vec!(),
        data: ByteCodeData::default(),
        foreign_libraries: vec!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ByteCodeRegister {
    Standard(usize),
    CallArg(usize),
    CallReturnArg(usize),
    StackPointer,
    BasePointer
}

pub fn standard_register(number: usize) -> ByteCodeRegister {
    ByteCodeRegister::Standard(number)
}

pub fn call_arg_register(number: usize) -> ByteCodeRegister {
    ByteCodeRegister::CallArg(number)
}

pub fn call_return_arg_register(number: usize) -> ByteCodeRegister {
    ByteCodeRegister::CallReturnArg(number)
}

pub fn base_pointer_register() -> ByteCodeRegister {
    ByteCodeRegister::BasePointer
}

pub fn stack_pointer_register() -> ByteCodeRegister {
    ByteCodeRegister::StackPointer
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ByteCodeInstruction {
    CallToSymbol(u32),
    AddValueToReg8 { value: u8, to: ByteCodeRegister },
    SubValueFromReg8 { value: u8, from: ByteCodeRegister },
    MoveSymbolToReg32 { symbol_index: u32, to: ByteCodeRegister },
    MoveValueToReg32 { value: u32, to: ByteCodeRegister },
    MoveValueToReg64 { value: u64, to: ByteCodeRegister },
    MoveRegToReg64 { from: ByteCodeRegister, to: ByteCodeRegister },
    MoveValueToRegPlusOffset32 { value: u32, to: ByteCodeRegister, offset: u8 },
    MoveValueToRegPlusOffset64 { value: u64, to: ByteCodeRegister, offset: u8 },
    MoveRegToRegPlusOffset32 { from: ByteCodeRegister, to: ByteCodeRegister, offset: u8 },
    MoveRegToRegPlusOffset64 { from: ByteCodeRegister, to: ByteCodeRegister, offset: u8 },
    MoveRegPlusOffsetToReg32 { from: ByteCodeRegister, offset: u8, to: ByteCodeRegister },
    MoveRegPlusOffsetToReg64 { from: ByteCodeRegister, offset: u8, to: ByteCodeRegister },
    LoadDataSectionAddressToReg64 { data_section_offset: u32, to: ByteCodeRegister },
    LoadAddressInRegPlusOffsetToReg64 { from: ByteCodeRegister, offset: u8, to: ByteCodeRegister },
    PushReg64(ByteCodeRegister),
    PopReg64(ByteCodeRegister),
    ZeroReg64(ByteCodeRegister),
    Return
}

pub fn call_to_symbol_instruction(symbol_index: u32) -> ByteCodeInstruction {
    ByteCodeInstruction::CallToSymbol(symbol_index)
}

pub fn push_reg_64_instruction(register: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::PushReg64(register)
}

pub fn pop_reg_64_instruction(register: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::PopReg64(register)
}

pub fn add_value_to_reg_8_instruction(value: u8, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::AddValueToReg8 { value, to }
}

pub fn sub_value_from_reg_8_instruction(value: u8, from: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::SubValueFromReg8 { value, from }
}

pub fn move_symbol_to_reg_32_instruction(symbol_index: u32, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveSymbolToReg32 { symbol_index, to }
}

pub fn move_value_to_reg_32_instruction(value: u32, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveValueToReg32 { value, to }
}

pub fn move_value_to_reg_64_instruction(value: u64, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveValueToReg64 { value, to }
}

pub fn move_reg_to_reg_64_instruction(from: ByteCodeRegister, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegToReg64 { from, to }
}

pub fn move_value_to_reg_plus_offset_32_instruction(value: u32, to: ByteCodeRegister, offset: u8) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveValueToRegPlusOffset32 { value, to, offset }
}

pub fn move_value_to_reg_plus_offset_64_instruction(value: u64, to: ByteCodeRegister, offset: u8) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveValueToRegPlusOffset64 { value, to, offset }
}

pub fn move_reg_to_reg_plus_offset_64_instruction(from: ByteCodeRegister, to: ByteCodeRegister, offset: u8) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegToRegPlusOffset64 { from, to, offset }
}

pub fn move_reg_to_reg_plus_offset_32_instruction(from: ByteCodeRegister, to: ByteCodeRegister, offset: u8) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegToRegPlusOffset32 { from, to, offset }
}

pub fn move_reg_plus_offset_to_reg_32_instruction(from: ByteCodeRegister, offset: u8, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegPlusOffsetToReg32 { from, offset, to }
}

pub fn move_reg_plus_offset_to_reg_64_instruction(from: ByteCodeRegister, offset: u8, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegPlusOffsetToReg64 { from, offset, to }
}

pub fn load_data_section_address_to_reg_64(data_section_offset: u32, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::LoadDataSectionAddressToReg64 { data_section_offset, to }
}

pub fn load_address_in_reg_plus_offset_to_reg_64(from: ByteCodeRegister, offset: u8, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::LoadAddressInRegPlusOffsetToReg64 { from, offset, to }
}

pub fn ret_instruction() -> ByteCodeInstruction {
    ByteCodeInstruction::Return
}
pub type ByteCodeInstructionStream = Vec<ByteCodeInstruction>;

pub fn add_byte_code(byte_code_stream: &mut Vec<ByteCodeInstruction>, instruction: ByteCodeInstruction) {
    byte_code_stream.push(instruction);
}

pub fn add_byte_codes(byte_code_stream: &mut Vec<ByteCodeInstruction>, mut instructions: Vec<ByteCodeInstruction>) {
    byte_code_stream.append(&mut instructions);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteCodeSymbol {
    DataSectionItem { name: String, value: u32 },
    ForeignExternal { name: String },
    AbsoluteExternal32 { name: String, value: u32 },
    AbsoluteExternal64 { name: String, value: u64 },
    ExternalCodeLabel { name: String, position: u32 },
}

pub type ByteCodeSymbols = Vec<ByteCodeSymbol>;

pub fn data_section_item(name: String, value: u32) -> ByteCodeSymbol{
    ByteCodeSymbol::DataSectionItem { name, value }
}

pub fn foreign_external(name: String) -> ByteCodeSymbol{
    ByteCodeSymbol::ForeignExternal { name }
}

pub fn absolute_external_32(name: String, value: u32) -> ByteCodeSymbol{
    ByteCodeSymbol::AbsoluteExternal32 { name, value }
}

pub fn absolute_external_64(name: String, value: u64) -> ByteCodeSymbol{
    ByteCodeSymbol::AbsoluteExternal64 { name, value }
}

pub fn external_code_label(name: String, position: u32) -> ByteCodeSymbol{
    ByteCodeSymbol::ExternalCodeLabel { name, position }
}

pub fn add_symbol(symbols: &mut ByteCodeSymbols, symbol: ByteCodeSymbol) -> u32 {
    symbols.push(symbol);
    (symbols.len() - 1) as u32
}

pub fn data_section_item_name(data_item_pointer: u32) -> String {
    format!("ds{}", data_item_pointer)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteCodeDataItem {
    String { value: String },
    QuadWord { value: u64 }
}

pub type ByteCodeDataItems = Vec<ByteCodeDataItem>;

#[derive(Default, Debug, Clone)]
pub struct ByteCodeData {
    size: u32,
    pub items: ByteCodeDataItems
}

pub fn string_data_item(value: String) -> ByteCodeDataItem{
    ByteCodeDataItem::String { value }
}
pub fn quad_word_data_item(value: u64) -> ByteCodeDataItem{
    ByteCodeDataItem::QuadWord { value }
}

pub fn add_data_item(data: &mut ByteCodeData, item: ByteCodeDataItem) -> u32 {
    let pointer = data.size;
    data.size += get_byte_code_data_item_size(&item);
    data.items.push(item);
    pointer
}

fn get_byte_code_data_item_size(item: &ByteCodeDataItem) -> u32 {
    match item {
        ByteCodeDataItem::String { value } => value.len() as u32,
        ByteCodeDataItem::QuadWord { .. } => 8,
    }
}
