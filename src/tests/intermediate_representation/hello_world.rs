
use crate::{
    tests::intermediate_representation::*
};

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
    print(\"hello world!\r\0\");
}"
    );   
    
    assert_eq!(irs.len(), 8);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    let print_body_ir = get_first_ir_with_byte_code_named(&irs, "print");
    
    assert_eq!(main_body_ir.symbols.len(), 3);
    assert_eq!(main_body_ir.data.items.len(), 1);
    assert_eq!(main_body_ir.byte_code.len(), 9);
    assert_eq!(main_body_ir.foreign_libraries.len(), 0);

    assert_eq!(print_body_ir.symbols.len(), 4);
    assert_eq!(print_body_ir.data.items.len(), 0);
    assert_eq!(print_body_ir.byte_code, vec!(
        //prologue
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),

        // move call args to shadow
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(0), base_pointer_register(), 16),
        
        // make storage for 4 local assignments in statement body
        sub_value_from_reg_8_instruction(40, stack_pointer_register()),
        
        // reserve shadow space for GetStdHandle proc call
        sub_value_from_reg_8_instruction(32, stack_pointer_register()),
        // set call arg registers for GetStdHandle proc call
        move_symbol_to_reg_32_instruction(1, call_arg_register(0)),
        // call GetStdHandle
        call_to_symbol_instruction(2),
        // store returned handle value
        move_reg_to_reg_plus_offset_32_instruction(call_return_arg_register(0), base_pointer_register(), -8i8 as u8),
        // release shadow space for GetStdHandle proc call
        add_value_to_reg_8_instruction(32, stack_pointer_register()),
        
        // store bytes_written
        move_value_to_reg_plus_offset_64_instruction(0, base_pointer_register(), -16i8 as u8),
        // store overlapped
        move_value_to_reg_plus_offset_64_instruction(0, base_pointer_register(), -24i8 as u8),
        //////// to_write: *void = to_print; here \\\\\\\\
        
        // reserve shadow space for WriteFile proc call
        sub_value_from_reg_8_instruction(32, stack_pointer_register()),
        move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), -8i8 as u8, call_arg_register(0)),
        move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), -32i8 as u8, call_arg_register(1)),
        move_reg_plus_offset_to_reg_32_instruction(base_pointer_register(), 16, call_arg_register(2)),
        move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), -16i8 as u8, call_arg_register(3)),
        move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), -24i8 as u8, call_arg_register(4)),
        call_to_symbol_instruction(3),
        // release shadow space for WriteFile proc call
        add_value_to_reg_8_instruction(32, stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        
        ret_instruction()
        
    ));
    assert_eq!(print_body_ir.foreign_libraries.len(), 0);

}