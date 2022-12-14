use rust_hephaestus::*;

#[test]
fn byte_code_for_procedure_call_with_args_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("some_procedure :: (x: s64, y: s64) -> *void {
}
    
main :: () {
    x := some_procedure(1, 2);
}"
    );   
    
    assert_eq!(irs.len(), 4);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols, vec!(
        external_code_label(string("main"), 0),
        foreign_external(string("some_procedure"))
    ));

    assert_eq!(main_body_ir.data.items.len(), 0);
    
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_instruction(register_size_64(), base_pointer_register()),
        move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),

        //reserve space for 1 local assignments
        sub_value_from_reg_instruction(instruction_value_8(8), stack_pointer_register()),
        
        // reserve shadow space for proc call
        sub_value_from_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        // set call arg registers
        move_value_to_reg_instruction(instruction_value_64(1), call_arg_register(0)),
        move_value_to_reg_instruction(instruction_value_64(2), call_arg_register(1)),
        // proc call
        call_to_symbol_instruction(symbol_index(1)),
        // store proc call return value
        move_reg_to_reg_plus_offset_instruction(register_size_32(), call_return_arg_register(0), base_pointer_register(), negative_address_offset(8)),
        // release shadow space for proc call
        add_value_to_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_instruction(register_size_64(), base_pointer_register(), stack_pointer_register()),
        pop_reg_instruction(register_size_64(), base_pointer_register()),
        
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_procedure_call_with_variable_arg_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("some_procedure :: (x: u32) {
}
    
main :: () {
    x: u32 = 14;
    some_procedure(x);
}"
    );   
    
    assert_eq!(irs.len(), 4);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols, vec!(
        external_code_label(string("main"), 0),
        foreign_external(string("some_procedure"))
    ));

    assert_eq!(main_body_ir.data.items.len(), 0);
    
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_instruction(register_size_64(), base_pointer_register()),
        move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),
        
        //reserve space for 1 local assignment
        sub_value_from_reg_instruction(instruction_value_8(4), stack_pointer_register()),
        
        //store x
        move_value_to_reg_plus_offset_instruction(instruction_value_32(14), base_pointer_register(), negative_address_offset(4)),
        
        // reserve shadow space for proc call
        sub_value_from_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        // set call arg registers
        move_reg_plus_offset_to_reg_instruction(register_size_32(), base_pointer_register(), negative_address_offset(4), call_arg_register(0)),
        // proc call
        call_to_symbol_instruction(symbol_index(1)),
        // release shadow space for proc call
        add_value_to_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_instruction(register_size_64(), base_pointer_register(), stack_pointer_register()),
        pop_reg_instruction(register_size_64(), base_pointer_register()),
        
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_procedure_call_with_null_void_pointer_variable_arg_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("some_procedure :: (x: *void) {
}
    
main :: () {
    x: *void = null;
    some_procedure(x);
}"
    );   
    
    assert_eq!(irs.len(), 4);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols, vec!(
        external_code_label(string("main"), 0),
        foreign_external(string("some_procedure"))
    ));

    assert_eq!(main_body_ir.data.items.len(), 0);
    
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_instruction(register_size_64(), base_pointer_register()),
        move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),
        
        //reserve space for 1 local assignment
        sub_value_from_reg_instruction(instruction_value_8(8), stack_pointer_register()),
        
        //store x
        move_value_to_reg_plus_offset_instruction(instruction_value_64(0), base_pointer_register(), negative_address_offset(8)),
        
        // reserve shadow space for proc call
        sub_value_from_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        // set call arg registers
        move_reg_plus_offset_to_reg_instruction(register_size_64(), base_pointer_register(), negative_address_offset(8), call_arg_register(0)),
        // proc call
        call_to_symbol_instruction(symbol_index(1)),
        // release shadow space for proc call
        add_value_to_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_instruction(register_size_64(), base_pointer_register(), stack_pointer_register()),
        pop_reg_instruction(register_size_64(), base_pointer_register()),
        
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_procedure_call_with_string_arg_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("some_procedure :: (x: string) {
}
    
main :: () {
    some_procedure(\"test\");
}"
    );   
    
    assert_eq!(irs.len(), 4);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols, vec!(
        external_code_label(string("main"), 0),
        data_section_item(string("ds0"), 0),
        data_section_item(string("ds4"), 4),
        foreign_external(string("some_procedure"))
    ));

    assert_eq!(main_body_ir.data.items, vec!(
        string_data_item(to_byte_string("test")),
        quad_word_data_item(4),
        pointer_data_item(0)
    ));
    
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_instruction(register_size_64(), base_pointer_register()),
        move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),
        
        // reserve shadow space for proc call
        sub_value_from_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        // set call arg register to point at
        load_data_section_address_to_reg(register_size_64(), data_section_offset(4), call_arg_register(0)),
        
        // proc call
        call_to_symbol_instruction(symbol_index(3)),
        
        // release shadow space for proc call
        add_value_to_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_instruction(register_size_64(), base_pointer_register(), stack_pointer_register()),
        pop_reg_instruction(register_size_64(), base_pointer_register()),
        
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_procedure_call_with_global_const_arg_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("GLOBAL : s32 : -11;
some_procedure :: (x: s32) {
}
    
main :: () {
    some_procedure(GLOBAL);
}");   
    
    assert_eq!(irs.len(), 5);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols, vec!(
        external_code_label(string("main"), 0),
        foreign_external(string("GLOBAL")),
        foreign_external(string("some_procedure"))
    ));

    assert_eq!(main_body_ir.data.items.len(), 0);
    
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_instruction(register_size_64(), base_pointer_register()),
        move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),
        
        // reserve shadow space for proc call
        sub_value_from_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        // set call arg registers
        move_symbol_to_reg_instruction(register_size_32(), symbol_index(1), call_arg_register(0)),
        // proc call
        call_to_symbol_instruction(symbol_index(2)),
        // release shadow space for proc call
        add_value_to_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_instruction(register_size_64(), base_pointer_register(), stack_pointer_register()),
        pop_reg_instruction(register_size_64(), base_pointer_register()),
        
        ret_instruction()
    ));
}

#[test]
fn byte_code_for_procedure_call_with_arg_passed_from_body_args_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("some_procedure :: (x: s32) {
}
    
main :: (x: s32) {
    some_procedure(x);
}");   
    
    assert_eq!(irs.len(), 4);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols, vec!(
        external_code_label(string("main"), 0),
        foreign_external(string("some_procedure"))
    ));

    assert_eq!(main_body_ir.data.items.len(), 0);
    
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_instruction(register_size_64(), base_pointer_register()),
        move_reg_to_reg_instruction(register_size_64(), stack_pointer_register(), base_pointer_register()),
        
        move_reg_to_reg_plus_offset_instruction(register_size_64(), call_arg_register(0), base_pointer_register(), address_offset(16)),
        
        // reserve shadow space for proc call
        sub_value_from_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        // set call arg registers
        move_reg_plus_offset_to_reg_instruction(register_size_32(), base_pointer_register(), address_offset(16), call_arg_register(0)),
        // proc call
        call_to_symbol_instruction(symbol_index(1)),
        // release shadow space for proc call
        add_value_to_reg_instruction(instruction_value_8(32), stack_pointer_register()),
        
        //epilogue
        move_reg_to_reg_instruction(register_size_64(), base_pointer_register(), stack_pointer_register()),
        pop_reg_instruction(register_size_64(), base_pointer_register()),
        
        ret_instruction()
    ));
}
