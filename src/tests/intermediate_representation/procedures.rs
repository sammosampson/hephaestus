
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
    let irs = compile_source_and_get_intemediate_representation("print :: (to_print: string, length: int) {
}
    
main :: () {
    print(\"hello world!\r\0\", 14);
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
        string_data_item(string("hello world!\r\0"))
    ));
    
    assert_eq!(main_body_ir.byte_code, vec!(
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        sub_value_from_reg_8_instruction(32, stack_pointer_register()),
        load_data_section_address_to_reg_64(0, call_arg_register(0)),
        move_value_to_reg_32_instruction(14, call_arg_register(1)),
        call_to_symbol_instruction(2),
        add_value_to_reg_8_instruction(32, stack_pointer_register()),
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_foreign_procedure_header_with_args_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "Kernel32 :: #foreign_system_library \"kernel32\";"
    );   
    
    assert_eq!(irs.len(), 1);

    let foreign_library_const_ir = get_first_ir_with_byte_code_named(&irs, "Kernel32");
    
    assert_eq!(foreign_library_const_ir.symbols.len(), 0);
    assert_eq!(foreign_library_const_ir.data.len(), 0);
    assert_eq!(foreign_library_const_ir.byte_code.len(), 0);
    assert_eq!(foreign_library_const_ir.foreign_libraries.len(), 1);
    assert_eq!(foreign_library_const_ir.foreign_libraries, vec!("kernel32"));
}
