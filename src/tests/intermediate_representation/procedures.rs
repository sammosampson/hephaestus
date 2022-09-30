
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

    let proc_body_ir = get_first_ir_with_byte_code(&irs);
    
    assert_eq!(proc_body_ir.filename, string("test.hep"));
    assert_eq!(proc_body_ir.top_level_symbol, string("SomeProcedure"));
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
        "SomeProcedure :: (a: int, b: *void) {
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let proc_body_ir = get_first_ir_with_byte_code(&irs);
    
    assert_eq!(proc_body_ir.filename, string("test.hep"));
    assert_eq!(proc_body_ir.top_level_symbol, string("SomeProcedure"));
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
    print(\"hello world!\r\n\0\", 15);
}"
    );   
    
    assert_eq!(irs.len(), 4);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    let print_body_ir = get_first_ir_with_byte_code_named(&irs, "print");
    
    assert_eq!(main_body_ir.byte_code, vec!(
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        ret_instruction()
    ));
    
    assert_eq!(print_body_ir.byte_code, vec!(
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(0), base_pointer_register(), 16),
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(1), base_pointer_register(), 24),
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        ret_instruction()
    ));
}

