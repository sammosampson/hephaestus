#![allow(dead_code)]

mod builder;
mod procedures;
mod constants;
mod strings;
mod errors;

use std::ops::{Deref, Add};

pub use builder::*;
pub use procedures::*;
pub use constants::*;
pub use strings::*;
pub use errors::*;

use crate::{
    parsing::*,
    utilities::*, 
    strings::*,
    types::*
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InstructionValue {
    Byte(u8),
    Word(u16),
    DoubleWord(u32),
    QuadWord(u64),
    Float(f32),
    LargeFloat(f64),
    Unsupported
}

pub fn instruction_value_8(value: u8) -> InstructionValue {
    InstructionValue::Byte(value)
}

pub fn instruction_value_32(value: u32) -> InstructionValue {
    InstructionValue::DoubleWord(value)
}

pub fn instruction_value_64(value: u64) -> InstructionValue {
    InstructionValue::QuadWord(value)
}

impl From<&ResolvedLiteral> for InstructionValue {
    fn from(from: &ResolvedLiteral) -> Self {
        match from {
            ResolvedLiteral::UnsignedInt8(value) => InstructionValue::Byte(*value),
            ResolvedLiteral::SignedInt8(value) => InstructionValue::Byte(*value as u8),
            ResolvedLiteral::UnsignedInt16(value) => InstructionValue::Word(*value),
            ResolvedLiteral::SignedInt16(value) => InstructionValue::Word(*value as u16),
            ResolvedLiteral::UnsignedInt32(value) => InstructionValue::DoubleWord(*value),
            ResolvedLiteral::SignedInt32(value) => InstructionValue::DoubleWord(*value as u32),
            ResolvedLiteral::UnsignedInt64(value) => InstructionValue::QuadWord(*value),
            ResolvedLiteral::SignedInt64(value) => InstructionValue::QuadWord(*value as u64),
            ResolvedLiteral::Float32(value) => InstructionValue::Float(*value),
            ResolvedLiteral::Float64(value) => InstructionValue::LargeFloat(*value),
            ResolvedLiteral::String(_) => InstructionValue::Unsupported,
        }
    }
}

pub fn resolved_literal_to_instruction_value(from: &ResolvedLiteral) -> InstructionValue {
    from.into()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterSize {
    Byte,
    Word,
    DoubleWord,
    QuadWord, 
    Float,
    LargeFloat
}

pub fn register_size_32() -> RegisterSize {
    RegisterSize::DoubleWord
}

pub fn register_size_64() -> RegisterSize {
    RegisterSize::QuadWord
}

impl From<BuiltInType> for RegisterSize {
    fn from(from: BuiltInType) -> Self {
        match from {
            BuiltInType::UnsignedInt8 => RegisterSize::Byte,
            BuiltInType::SignedInt8 => RegisterSize::Byte,
            BuiltInType::UnsignedInt16 => RegisterSize::Word,
            BuiltInType::SignedInt16 => RegisterSize::Word,
            BuiltInType::UnsignedInt32 => RegisterSize::DoubleWord,
            BuiltInType::SignedInt32 => RegisterSize::DoubleWord,
            BuiltInType::UnsignedInt64 => RegisterSize::QuadWord,
            BuiltInType::SignedInt64 => RegisterSize::QuadWord,
            BuiltInType::Float32 => RegisterSize::Float,
            BuiltInType::Float64 => RegisterSize::LargeFloat,
            BuiltInType::String => RegisterSize::QuadWord,
            BuiltInType::Boolean => RegisterSize::Byte,
            BuiltInType::Void => RegisterSize::QuadWord,
        }
    }
}

pub fn built_in_type_to_register_size(from: BuiltInType) -> RegisterSize {
    from.into()
}


pub fn resolved_type_to_register_size(from: &RuntimeTypePointer) -> Option<RegisterSize> {
    if let Some((built_in_arg_type, ..)) = try_get_built_in_type(&from.id) {
        return Some(built_in_type_to_register_size(built_in_arg_type));
    }
    None
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ByteCodeInstruction {
    Unsupported,
    Unimplemented,
    CallToSymbol(SymbolIndex),
    AddValueToReg8 { value: u8, to: ByteCodeRegister },
    SubValueFromReg8 { value: u8, from: ByteCodeRegister },
    MoveSymbolToReg32 { symbol_index: SymbolIndex, to: ByteCodeRegister },
    MoveValueToReg32 { value: u32, to: ByteCodeRegister },
    MoveValueToReg64 { value: u64, to: ByteCodeRegister },
    MoveRegToReg64 { from: ByteCodeRegister, to: ByteCodeRegister },
    MoveValueToRegPlusOffset32 { value: u32, to: ByteCodeRegister, offset: AddressOffset },
    MoveValueToRegPlusOffset64 { value: u64, to: ByteCodeRegister, offset: AddressOffset },
    MoveRegToRegPlusOffset32 { from: ByteCodeRegister, to: ByteCodeRegister, offset: AddressOffset },
    MoveRegToRegPlusOffset64 { from: ByteCodeRegister, to: ByteCodeRegister, offset: AddressOffset },
    MoveRegPlusOffsetToReg32 { from: ByteCodeRegister, offset: AddressOffset, to: ByteCodeRegister },
    MoveRegPlusOffsetToReg64 { from: ByteCodeRegister, offset: AddressOffset, to: ByteCodeRegister },
    LoadDataSectionAddressToReg64 { data_section_offset: DataSectionOffset, to: ByteCodeRegister },
    LoadAddressInRegPlusOffsetToReg64 { from: ByteCodeRegister, offset: AddressOffset, to: ByteCodeRegister },
    PushReg64(ByteCodeRegister),
    PopReg64(ByteCodeRegister),
    ZeroReg64(ByteCodeRegister),
    Return
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SymbolIndex(u32);

impl Deref for SymbolIndex {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn symbol_index(value: u32) -> SymbolIndex {
    SymbolIndex(value)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddressOffset(u8);

impl Deref for AddressOffset {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add<u8> for AddressOffset {
    type Output = AddressOffset;

    fn add(self, rhs: u8) -> Self::Output {
        address_offset(self.0 + rhs)
    }
}

pub fn address_offset(value: u8) -> AddressOffset {
    AddressOffset(value)
}

pub fn negative_address_offset(value: u8) -> AddressOffset {
    AddressOffset(-(value as i8) as u8)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DataSectionOffset(u32);

impl Deref for DataSectionOffset {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn data_section_offset(value: u32) -> DataSectionOffset {
    DataSectionOffset(value)
}


pub fn call_to_symbol_instruction(symbol_index: SymbolIndex) -> ByteCodeInstruction {
    ByteCodeInstruction::CallToSymbol(symbol_index)
}

pub fn push_reg_instruction(register_size: RegisterSize, register: ByteCodeRegister) -> ByteCodeInstruction {
    match register_size {
        RegisterSize::Byte => ByteCodeInstruction::Unimplemented,
        RegisterSize::Word => ByteCodeInstruction::Unimplemented,
        RegisterSize::DoubleWord => ByteCodeInstruction::Unimplemented,
        RegisterSize::Float => ByteCodeInstruction::Unimplemented,
        RegisterSize::LargeFloat => ByteCodeInstruction::Unimplemented,
        RegisterSize::QuadWord => ByteCodeInstruction::PushReg64(register),
    }
}

pub fn pop_reg_instruction(register_size: RegisterSize, register: ByteCodeRegister) -> ByteCodeInstruction {
    match register_size {
        RegisterSize::Byte => ByteCodeInstruction::Unimplemented,
        RegisterSize::Word => ByteCodeInstruction::Unimplemented,
        RegisterSize::Float => ByteCodeInstruction::Unimplemented,
        RegisterSize::LargeFloat => ByteCodeInstruction::Unimplemented,
        RegisterSize::DoubleWord => ByteCodeInstruction::Unimplemented,
        RegisterSize::QuadWord => ByteCodeInstruction::PopReg64(register)
    }
}

pub fn add_value_to_reg_instruction(value: InstructionValue, to: ByteCodeRegister) -> ByteCodeInstruction {
    match value {
        InstructionValue::Unsupported => ByteCodeInstruction::Unimplemented,
        InstructionValue::Float(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::LargeFloat(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::Byte(value) => ByteCodeInstruction::AddValueToReg8 { value, to },
        InstructionValue::Word(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::DoubleWord(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::QuadWord(_) =>  ByteCodeInstruction::Unimplemented,
    }
}

pub fn sub_value_from_reg_instruction(value: InstructionValue, from: ByteCodeRegister) -> ByteCodeInstruction {
    match value {
        InstructionValue::Unsupported => ByteCodeInstruction::Unimplemented,
        InstructionValue::Float(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::LargeFloat(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::Byte(value) => ByteCodeInstruction::SubValueFromReg8 { value, from },
        InstructionValue::Word(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::DoubleWord(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::QuadWord(_) =>  ByteCodeInstruction::Unimplemented
    }
}

pub fn move_symbol_to_reg_instruction(register_size: RegisterSize, symbol_index: SymbolIndex, to: ByteCodeRegister) -> ByteCodeInstruction {
    match register_size {
        RegisterSize::Byte => ByteCodeInstruction::Unimplemented,
        RegisterSize::Word => ByteCodeInstruction::Unimplemented,
        RegisterSize::Float => ByteCodeInstruction::Unimplemented,
        RegisterSize::LargeFloat => ByteCodeInstruction::Unimplemented,
        RegisterSize::DoubleWord => ByteCodeInstruction::MoveSymbolToReg32 { symbol_index, to },
        RegisterSize::QuadWord => ByteCodeInstruction::Unimplemented
    }
}

pub fn move_value_to_reg_instruction(value: InstructionValue, to: ByteCodeRegister) -> ByteCodeInstruction {
    match value {
        InstructionValue::Unsupported => ByteCodeInstruction::Unimplemented,
        InstructionValue::Float(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::LargeFloat(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::Byte(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::Word(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::DoubleWord(value) => ByteCodeInstruction::MoveValueToReg32 { value, to },
        InstructionValue::QuadWord(value) =>  ByteCodeInstruction::MoveValueToReg64 { value, to }
    }
}

pub fn move_reg_to_reg_instruction(register_size: RegisterSize, from: ByteCodeRegister, to: ByteCodeRegister) -> ByteCodeInstruction {
    match register_size {
        RegisterSize::Byte => ByteCodeInstruction::Unimplemented,
        RegisterSize::Word => ByteCodeInstruction::Unimplemented,
        RegisterSize::Float => ByteCodeInstruction::Unimplemented,
        RegisterSize::LargeFloat => ByteCodeInstruction::Unimplemented,
        RegisterSize::DoubleWord => ByteCodeInstruction::Unimplemented,
        RegisterSize::QuadWord => ByteCodeInstruction::MoveRegToReg64 { from, to }
    }
}

pub fn move_value_to_reg_plus_offset_instruction(value: InstructionValue, to: ByteCodeRegister, offset: AddressOffset) -> ByteCodeInstruction {
    match value {
        InstructionValue::Unsupported => ByteCodeInstruction::Unimplemented,
        InstructionValue::Float(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::LargeFloat(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::Byte(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::Word(_) => ByteCodeInstruction::Unimplemented,
        InstructionValue::DoubleWord(value) => ByteCodeInstruction::MoveValueToRegPlusOffset32 { value, to, offset },
        InstructionValue::QuadWord(value) => ByteCodeInstruction::MoveValueToRegPlusOffset64 { value, to, offset }
    }
}

pub fn move_reg_to_reg_plus_offset_instruction(register_size: RegisterSize, from: ByteCodeRegister, to: ByteCodeRegister, offset: AddressOffset) -> ByteCodeInstruction {
    match register_size {
        RegisterSize::Byte => ByteCodeInstruction::Unimplemented,
        RegisterSize::Word => ByteCodeInstruction::Unimplemented,
        RegisterSize::Float => ByteCodeInstruction::Unimplemented,
        RegisterSize::LargeFloat => ByteCodeInstruction::Unimplemented,
        RegisterSize::DoubleWord => ByteCodeInstruction::MoveRegToRegPlusOffset32 { from, to, offset },
        RegisterSize::QuadWord => ByteCodeInstruction::MoveRegToRegPlusOffset64 { from, to, offset },
    }
}

pub fn move_reg_plus_offset_to_reg_instruction(register_size: RegisterSize, from: ByteCodeRegister, offset: AddressOffset, to: ByteCodeRegister) -> ByteCodeInstruction {
    match register_size {
        RegisterSize::Byte => ByteCodeInstruction::Unimplemented,
        RegisterSize::Word => ByteCodeInstruction::Unimplemented,
        RegisterSize::Float => ByteCodeInstruction::Unimplemented,
        RegisterSize::LargeFloat => ByteCodeInstruction::Unimplemented,
        RegisterSize::DoubleWord => ByteCodeInstruction::MoveRegPlusOffsetToReg32 { from, offset, to },
        RegisterSize::QuadWord => ByteCodeInstruction::MoveRegPlusOffsetToReg64 { from, offset, to },
    }    
}

pub fn load_data_section_address_to_reg(register_size: RegisterSize, data_section_offset: DataSectionOffset, to: ByteCodeRegister) -> ByteCodeInstruction {
    match register_size {
        RegisterSize::QuadWord => ByteCodeInstruction::LoadDataSectionAddressToReg64 { data_section_offset, to },
        _ => ByteCodeInstruction::Unsupported
    }
}

pub fn load_address_in_reg_plus_offset_to_reg(register_size: RegisterSize, from: ByteCodeRegister, offset: AddressOffset, to: ByteCodeRegister) -> ByteCodeInstruction {
    match register_size {
        RegisterSize::QuadWord => ByteCodeInstruction::LoadAddressInRegPlusOffsetToReg64 { from, offset, to },
        _ => ByteCodeInstruction::Unsupported
    }
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

pub fn add_symbol(symbols: &mut ByteCodeSymbols, symbol: ByteCodeSymbol) -> SymbolIndex {
    symbols.push(symbol);
    symbol_index((symbols.len() - 1) as u32)
}

pub fn data_section_item_name(data_item_pointer: DataSectionOffset) -> String {
    format!("ds{}", *data_item_pointer)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteCodeDataItem {
    String { value: ByteString },
    Pointer { value: u64 },
    QuadWord { value: u64 }
}

pub type ByteCodeDataItems = Vec<ByteCodeDataItem>;

#[derive(Default, Debug, Clone)]
pub struct ByteCodeData {
    size: u32,
    pub items: ByteCodeDataItems
}

pub fn string_data_item(value: ByteString) -> ByteCodeDataItem{
    ByteCodeDataItem::String { value }
}
pub fn quad_word_data_item(value: u64) -> ByteCodeDataItem{
    ByteCodeDataItem::QuadWord { value }
}

pub fn pointer_data_item(value: u64) -> ByteCodeDataItem{
    ByteCodeDataItem::Pointer { value }
}

pub fn add_data_item(data: &mut ByteCodeData, item: ByteCodeDataItem) -> DataSectionOffset {
    let pointer = data.size;
    data.size += get_byte_code_data_item_size(&item);
    data.items.push(item);
    data_section_offset(pointer)
}

fn get_byte_code_data_item_size(item: &ByteCodeDataItem) -> u32 {
    match item {
        ByteCodeDataItem::String { value } => value.len() as u32,
        ByteCodeDataItem::Pointer { .. } => 8,
        ByteCodeDataItem::QuadWord { .. } => 8,
    }
}
