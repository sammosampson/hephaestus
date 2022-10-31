use rust_hephaestus::*;

#[test]
fn byte_code_for_known_type_signed_number_assignment_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("STD_OUTPUT_HANDLE : s32 : -11;

Kernel32 :: #foreign_system_library \"kernel32\";
WriteFile :: (handle: *void, to_write: *void, bytes_to_write: u32, bytes_written: *void, overlapped: *void) -> bool #foreign Kernel32;
GetStdHandle :: (handle_type: s32) -> *void #foreign Kernel32;

print :: (to_print: string) {
    handle := GetStdHandle(STD_OUTPUT_HANDLE);
    to_write := cast(*void) to_print.data;
    length := cast(u32) to_print.count
    bytes_written: *void = null;
    overlapped: *void = null;
    WriteFile(handle, to_write, length, bytes_written, overlapped);
}

main :: () {
    print(\"hello world!\\r\\n\\0\");
}"
    );   
    assert_eq!(irs.len(), 8);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    let print_body_ir = get_first_ir_with_byte_code_named(&irs, "print");
    
    assert_eq!(main_body_ir.symbols.len(), 4);
    assert_eq!(main_body_ir.data.items, vec!(
        // string lit
        string_data_item(to_byte_string("hello world!\\r\\n\\0")),
        // string instance
        // length
        quad_word_data_item(15),
        // data (ptr to item 0)
        pointer_data_item(0),
    ));
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_instruction(register_size_64(), base_pointer_register()),
        move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),

        // reserve shadow space for print proc call
        sub_value_from_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        // set call arg registers for GetStdHandle proc call
        load_data_section_address_to_reg(register_size_64(), data_section_offset(15), call_arg_register(0)),
        // call print
        call_to_symbol_instruction(symbol_index(3)),
        // release shadow space for print proc call
        add_value_to_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_instruction(register_size_64(), base_pointer_register(), stack_pointer_register()),
        pop_reg_instruction(register_size_64(), base_pointer_register()),
        
        ret_instruction()        
    ));
    assert_eq!(main_body_ir.foreign_libraries.len(), 0);

    assert_eq!(print_body_ir.symbols.len(), 4);
    assert_eq!(print_body_ir.data.items.len(), 0);
    assert_eq!(print_body_ir.byte_code, vec!(
        //prologue
        push_reg_instruction(register_size_64(), base_pointer_register()),
        move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),

        // move call arg to shadow
        move_reg_to_reg_plus_offset_instruction(register_size_64(), call_arg_register(0), base_pointer_register(), address_offset(16)),
        
        // make storage for 4 * 8 byte and 1 * 4 byte local assignments in statement body
        sub_value_from_reg_instruction(instruction_value_8(36), stack_pointer_register()),
        
        // reserve shadow space for GetStdHandle proc call
        sub_value_from_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        // set call arg registers for GetStdHandle proc call
        move_symbol_to_reg_instruction(register_size_32(), symbol_index(1), call_arg_register(0)),
        // call GetStdHandle
        call_to_symbol_instruction(symbol_index(2)),
        // store returned handle value
        move_reg_to_reg_plus_offset_instruction(register_size_32(), call_return_arg_register(0), base_pointer_register(), negative_address_offset(8)),
        // release shadow space for GetStdHandle proc call
        add_value_to_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        
        // to_write := cast(*void) to_print.data;
        // get string instance pointer
        move_reg_plus_offset_to_reg_instruction(register_size_64(), base_pointer_register(), address_offset(16), standard_register(0)),
        // to_print.data
        move_reg_plus_offset_to_reg_instruction(register_size_64(), standard_register(0), address_offset(8), standard_register(1)),
        // into to_write
        move_reg_to_reg_plus_offset_instruction(register_size_64(), standard_register(1), base_pointer_register(), negative_address_offset(16)),
        
        //length := cast(u32) to_print.count
        // get string instance pointer
        move_reg_plus_offset_to_reg_instruction(register_size_64(), base_pointer_register(), address_offset(16), standard_register(0)),
        // to_print.count
        move_reg_plus_offset_to_reg_instruction(register_size_64(), standard_register(0), address_offset(0), standard_register(1)),
        // into length
        move_reg_to_reg_plus_offset_instruction(register_size_32(), standard_register(1), base_pointer_register(), negative_address_offset(20)),
        
        // store bytes_written
        move_value_to_reg_plus_offset_instruction(instruction_value_64(0), base_pointer_register(), negative_address_offset(28)),
        // store overlapped
        move_value_to_reg_plus_offset_instruction(instruction_value_64(0), base_pointer_register(), negative_address_offset(36)),
        
        // reserve shadow space for WriteFile proc call
        sub_value_from_reg_instruction(instruction_value_8(40), stack_pointer_register()),
        move_reg_plus_offset_to_reg_instruction(register_size_64(), base_pointer_register(), negative_address_offset(8), call_arg_register(0)),
        move_reg_plus_offset_to_reg_instruction(register_size_64(), base_pointer_register(), negative_address_offset(16), call_arg_register(1)),
        move_reg_plus_offset_to_reg_instruction(register_size_32(), base_pointer_register(), negative_address_offset(20), call_arg_register(2)),
        move_reg_plus_offset_to_reg_instruction(register_size_64(), base_pointer_register(), negative_address_offset(28), call_arg_register(3)),
        move_reg_plus_offset_to_reg_instruction(register_size_64(), base_pointer_register(), negative_address_offset(36), call_arg_register(4)),
        move_reg_to_reg_plus_offset_instruction(register_size_64(), call_arg_register(4), stack_pointer_register(), address_offset(32)),
        call_to_symbol_instruction(symbol_index(3)),
        // release shadow space for WriteFile proc call
        add_value_to_reg_instruction(instruction_value_8(40), stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_instruction(register_size_64(), base_pointer_register(), stack_pointer_register()),
        pop_reg_instruction(register_size_64(), base_pointer_register()),
        
        ret_instruction()
        
    ));
    assert_eq!(print_body_ir.foreign_libraries.len(), 0);

}