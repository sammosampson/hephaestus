
use crate::{
    tests::intermediate_representation::*,
    strings::*
};

#[test]
fn byte_code_for_known_type_signed_number_assignment_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("main :: () {
    x : u32 = 1;
    y : u64 = 2;
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols.len(), 1);
    assert_eq!(main_body_ir.data.items.len(), 0);
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        
        //reserve space for 2 local assignments
        sub_value_from_reg_8_instruction(12, stack_pointer_register()),
        //store x
        move_value_to_reg_plus_offset_32_instruction(1, base_pointer_register(), -4i8 as u8),
        //store y
        move_value_to_reg_plus_offset_64_instruction(2, base_pointer_register(), -12i8 as u8),

        //epilogue
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        
        ret_instruction()
    ));
    assert_eq!(main_body_ir.foreign_libraries.len(), 0);
}

#[test]
fn byte_code_for_cast_type_assignment_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("main :: () {
    x := cast(u32) 1;
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols.len(), 1);
    assert_eq!(main_body_ir.data.items.len(), 0);
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        
        //reserve space for 1 local assignments
        sub_value_from_reg_8_instruction(4, stack_pointer_register()),
        //store x
        move_value_to_reg_plus_offset_32_instruction(1, base_pointer_register(), -4i8 as u8),
        
        //epilogue
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        
        ret_instruction()
    ));
    assert_eq!(main_body_ir.foreign_libraries.len(), 0);
}

#[test]
fn byte_code_for_string_assignment_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("main :: () {
    x := \"test\";
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let some_proc_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(some_proc_body_ir.symbols.len(), 2);

    assert_eq!(some_proc_body_ir.data.items, vec!(
        string_data_item(to_byte_string("test"))
    ));

    assert_eq!(some_proc_body_ir.byte_code, vec!(
        //prologue
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        
        //reserve space for 1 local of string struct { count: int (8 bytes) + data *u8 (8 bytes) }
        sub_value_from_reg_8_instruction(16, stack_pointer_register()),
        //store x
        move_value_to_reg_plus_offset_64_instruction(4, base_pointer_register(), -16i8 as u8),
        load_data_section_address_to_reg_64(0, call_arg_register(0)),
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(0), base_pointer_register(), -8i8 as u8),
        
        //epilogue
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        
        ret_instruction()
    ));

    assert_eq!(some_proc_body_ir.foreign_libraries.len(), 0);
}

#[test]
fn byte_code_for_string_field_assignment_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("some_proc :: (s: string) {
    x := s.count;
    y := cast(*void) s.data;
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let some_proc_body_ir = get_first_ir_with_byte_code_named(&irs, "some_proc");
    
    assert_eq!(some_proc_body_ir.symbols.len(), 1);

    assert_eq!(some_proc_body_ir.data.items.len(), 0);

    assert_eq!(some_proc_body_ir.byte_code, vec!(
        //prologue
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        
        // store single proc arg in shadow
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(0), base_pointer_register(), 16),
        
        //reserve space for 2 locals
        sub_value_from_reg_8_instruction(16, stack_pointer_register()),
        
        //store x
        // get string pointer from shadow
        move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), 16, standard_register(0)),
        // get count value from string
        move_reg_plus_offset_to_reg_64_instruction(standard_register(0), 0, standard_register(1)),
        // store count value in x space
        move_reg_to_reg_plus_offset_64_instruction(standard_register(1), base_pointer_register(), -8i8 as u8),
        
        //store y
        // get string pointer from shadow
        move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), 16, standard_register(0)),
        // get data pointer value from string
        move_reg_plus_offset_to_reg_64_instruction(standard_register(0), 8, standard_register(1)),
        // store data pointer value in y space
        move_reg_to_reg_plus_offset_64_instruction(standard_register(1), base_pointer_register(), -16i8 as u8),
        
        //epilogue
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        
        ret_instruction()
    ));

    assert_eq!(some_proc_body_ir.foreign_libraries.len(), 0);
}