use super::Coff;

use crate::{
    intermediate_representation::*,
    backends::*
};

#[derive(Clone)]
pub struct X64Backend;

impl BackendBuild for X64Backend {
    fn build_backend(&mut self, ir: IntermediateRepresentation, has_prior_errors: bool) -> BackendErrorResult {
        if has_prior_errors {
            return Ok(());
        }
        let mut coff = create_coff();
        let file_name = build_x64_object(&mut coff, ir)?;   
        write_coff_to_file(&coff, &mut create_coff_file(&file_name).unwrap()).unwrap();
        Ok(())
    }
}

pub fn create_x64_backend() -> X64Backend {
    X64Backend
}

fn build_x64_object(coff: &mut Coff, ir: IntermediateRepresentation) -> Result<String, BackendError> {
    for op in ir.byte_code  {
        match op {
            ByteCodeInstruction::Unsupported => return Err(unsupported_instruction_error()),
            ByteCodeInstruction::Unimplemented => return Err(unimplemented_instruction_error()),
            ByteCodeInstruction::CallToSymbol(symbol_index) => 
                add_call_relocatable_addr_op(
                    coff, 
                    relocatable_value(
                        convert_byte_code_to_coff_symbol_index(ir.symbols.len(), symbol_index), 
                    0x0)
                ),
            ByteCodeInstruction::PushReg64(register) => 
                add_push_reg_op(coff, get_register(register)?),
            ByteCodeInstruction::PopReg64(register) =>
                add_pop_reg_op(coff, get_register(register)?),    
            ByteCodeInstruction::MoveSymbolToReg32 { symbol_index, to } => 
                add_mov_dword_relocatable_value_to_reg_op(
                    coff,
                    relocatable_value(convert_byte_code_to_coff_symbol_index(ir.symbols.len(), symbol_index), 0x0), 
                    get_register(to)?
                ),      
            ByteCodeInstruction::MoveValueToReg32 { value, to } => 
                add_mov_dword_value_to_reg_op(coff, value, get_register(to)?),      
            ByteCodeInstruction::MoveValueToReg64 { value, to }  =>
                add_mov_qword_value_to_reg_op(coff, value, get_register(to)?),  
            ByteCodeInstruction::MoveRegToReg64 { from, to } => 
                add_mov_from_qword_reg_to_reg_op(coff, get_register(from)?, get_register(to)?),
            ByteCodeInstruction::MoveValueToRegPlusOffset32 { value, to, offset } => 
                add_mov_dword_value_into_reg_plus_offset_pointer_op(coff, value, get_register(to)?, *offset),
            ByteCodeInstruction::MoveValueToRegPlusOffset64 { value, to, offset } => 
                add_mov_qword_value_into_reg_plus_offset_pointer_op(coff, value, get_register(to)?, *offset),
            ByteCodeInstruction::MoveRegToRegPlusOffset32 { from, to, offset } => 
                add_mov_reg_to_reg_plus_offset_dword_pointer_op(coff, get_register(from)?, get_register(to)?, *offset),
            ByteCodeInstruction::MoveRegToRegPlusOffset64 { from, to, offset } => 
                add_mov_reg_to_reg_plus_offset_qword_pointer_op(coff, get_register(from)?, get_register(to)?, *offset),
            ByteCodeInstruction::MoveRegPlusOffsetToReg32 { from, offset, to } => 
                add_mov_dword_reg_plus_offset_pointer_to_reg_op(coff, get_register(from)?, *offset, get_register(to)?),
            ByteCodeInstruction::MoveRegPlusOffsetToReg64 { from, offset, to } => 
                add_mov_qword_reg_plus_offset_pointer_to_reg_op(coff, get_register(from)?, *offset, get_register(to)?),
            ByteCodeInstruction::SubValueFromReg8 { value, from } => 
                add_sub_byte_value_from_reg_op(coff, value, get_register(from)?),
            ByteCodeInstruction::AddValueToReg8 { value, to } => 
                add_add_byte_value_to_reg_op(coff, value, get_register(to)?),
            ByteCodeInstruction::ZeroReg64(register) =>
                add_xor_qword_reg_into_reg_op(coff, get_register(register)?, get_register(register)?),
            ByteCodeInstruction::Return => add_ret_op(coff),
            ByteCodeInstruction::LoadDataSectionAddressToReg64 { data_section_offset, to } => 
                add_lea_reg_plus_relocatable_offset_pointer_to_reg_op(
                    coff, 
                    REG_IP, 
                    relocatable_value(0x02, *data_section_offset), 
                    get_register(to)?
                ),
            ByteCodeInstruction::LoadAddressInRegPlusOffsetToReg64 { from, offset, to } =>
                add_lea_reg_plus_offset_pointer_to_reg_op(
                    coff, 
                    get_register(from)?, 
                    *offset, 
                    get_register(to)?
                )
        }
    }

    for data_item in ir.data.items {
        match data_item {
            ByteCodeDataItem::String { value } => add_string_to_data_section(coff, &value),
            ByteCodeDataItem::QuadWord { value } => add_quad_word_to_data_section(coff, &value),
            ByteCodeDataItem::Pointer { value } => add_pointer_to_data_section_and_make_relocation(coff, &value),
        };
    }

    add_debug_file_name_symbols(coff, &ir.filename);
    add_data_section_header_symbols(coff);
    add_text_section_header_symbols(coff);
    add_absolute_static_symbol(coff, ".absolut", 0);
    
    for symbol_index in (0..ir.symbols.len()).rev() {
        match &ir.symbols[symbol_index] {
            ByteCodeSymbol::DataSectionItem { name, value } => add_data_section_static_symbol(coff, name, *value),
            ByteCodeSymbol::ForeignExternal { name } => add_foreign_external_symbol(coff, name),
            ByteCodeSymbol::AbsoluteExternal32 { name, value } => add_absolute_external_symbol_32(coff, name, *value),
            ByteCodeSymbol::AbsoluteExternal64 { .. } => return Err(todo_error("64 bit absolute external symbol insertion")),
            ByteCodeSymbol::ExternalCodeLabel { name, position } => add_text_section_external_symbol(coff, name, *position),
        }
    }

    let file_root = ir.filename.replace(".hep", "");
    Ok(format!("{}-{}.obj", file_root, ir.top_level_symbol))

}

fn convert_byte_code_to_coff_symbol_index(number_of_symbols: usize, symbol_index: SymbolIndex) -> u32 {
    (6 + number_of_symbols) as u32 - *symbol_index
}

type ResgisterResult = Result<u8, BackendError>;

fn get_register(register: ByteCodeRegister) -> ResgisterResult {
    match register {
        ByteCodeRegister::Standard(number) => match number {
            0 => Ok(REG_AX),
            1 => Ok(REG_CX),
            2 => Ok(REG_DX),
            3 => Ok(REG_R8),
            4 => Ok(REG_R9),
            5 => Ok(REG_R10),
            6 => Ok(REG_R11),
            7 => Ok(REG_R12),
            8 => Ok(REG_R13),
            9 => Ok(REG_R14),
            10 => Ok(REG_R15),
            r => Err(register_not_available_error(r))
        },
        ByteCodeRegister::CallArg(number) => match number {
            0 => Ok(REG_CX),
            1 => Ok(REG_DX),
            2 => Ok(REG_R8),
            3 => Ok(REG_R9),
            4 => Ok(REG_R10),
            5 => Ok(REG_R11),
            6 => Ok(REG_R12),
            7 => Ok(REG_R13),
            8 => Ok(REG_R14),
            9 => Ok(REG_R15),
            r => Err(register_not_available_error(r))
        },
        ByteCodeRegister::CallReturnArg(number) => match number {
            0 => Ok(REG_AX),
            _ => Err(todo_error("more call return arg registers"))
        },
        ByteCodeRegister::StackPointer => Ok(REG_SP),
        ByteCodeRegister::BasePointer => Ok(REG_BP)
    }
}