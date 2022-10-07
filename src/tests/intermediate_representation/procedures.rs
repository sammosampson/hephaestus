
use crate::{
    tests::intermediate_representation::*, 
    utilities::*
};

#[test]
fn byte_code_for_procedure_body_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "SomeProcedure :: () {
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let proc_body_ir = get_first_ir_with_byte_code_named(&irs, "SomeProcedure");
    
    assert_eq!(proc_body_ir.filename, string("test.hep"));
    
    assert_eq!(proc_body_ir.symbols, vec!(
        external_code_label(string("SomeProcedure"), 0),
    ));

    assert_eq!(proc_body_ir.byte_code, vec!(
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_procedure_body_with_args_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "print :: (to_print: string, length: int) {
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let proc_body_ir = get_first_ir_with_byte_code_named(&irs, "print");

    assert_eq!(proc_body_ir.symbols, vec!(
        external_code_label(string("print"), 0),
    ));
    
    assert_eq!(proc_body_ir.byte_code, vec!(
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(0), base_pointer_register(), 16),
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(1), base_pointer_register(), 24),
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_procedure_call_with_args_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("print :: (to_print: string, length: u32) -> *void {
}
    
main :: () {
    len: u32 = 14;
    x := print(\"test\", len);
}"
    );   
    
    assert_eq!(irs.len(), 4);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols, vec!(
        external_code_label(string("main"), 0),
        data_section_item(string("ds0"), 0),
        foreign_external(string("print"))
    ));

    assert_eq!(main_body_ir.data, vec!(
        string_data_item(string("test"))
    ));
    
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        
        //reserve space for 2 local assignments
        sub_value_from_reg_8_instruction(16, stack_pointer_register()),
        
        //store len
        move_value_to_reg_plus_offset_32_instruction(14, base_pointer_register(), -8i8 as u8),
        
        // reserve shadow space for proc call
        sub_value_from_reg_8_instruction(32, stack_pointer_register()),
        // set call arg registers
        load_data_section_address_to_reg_64(0, call_arg_register(0)),
        move_reg_plus_offset_to_reg_32_instruction(base_pointer_register(), -8i8 as u8, call_arg_register(1)),
        // proc call
        call_to_symbol_instruction(2),
        // store proc call return value
        move_reg_to_reg_plus_offset_32_instruction(call_return_arg_register(0), base_pointer_register(), -16i8 as u8),
        // release shadow space for proc call
        add_value_to_reg_8_instruction(32, stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_foreign_procedure_header_with_args_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "WriteFile :: (handle: *void, to_write: *void, bytes_to_write: int, bytes_written: *void, overlapped: *void) -> bool #foreign Kernel32"
    );   
    
    assert_eq!(irs.len(), 1);

    let foreign_library_const_ir = get_first_ir_named(&irs, "WriteFile");
    
    assert_eq!(foreign_library_const_ir.symbols.len(), 0);
    assert_eq!(foreign_library_const_ir.data.len(), 0);
    assert_eq!(foreign_library_const_ir.byte_code.len(), 0);
    assert_eq!(foreign_library_const_ir.foreign_libraries.len(), 0);
}
