
use crate::{
    tests::intermediate_representation::*
};

#[test]
fn byte_code_for_known_type_signed_number_assignment_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "main :: () {
    x : u32 = 1;
    y : u64 = 2;
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols.len(), 1);
    assert_eq!(main_body_ir.data.len(), 0);
    assert_eq!(main_body_ir.byte_code, vec!(
        //prologue
        push_reg_64_instruction(base_pointer_register()),
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register()),
        
        //reserve space for 2 local assignments
        sub_value_from_reg_8_instruction(16, stack_pointer_register()),
        //store x
        move_value_to_reg_plus_offset_32_instruction(1, base_pointer_register(), -8i8 as u8),
        //store y
        move_value_to_reg_plus_offset_64_instruction(2, base_pointer_register(), -16i8 as u8),

        //epilogue
        move_reg_to_reg_64_instruction(base_pointer_register(), stack_pointer_register()),
        pop_reg_64_instruction(base_pointer_register()),
        
        ret_instruction()
    ));
    assert_eq!(main_body_ir.foreign_libraries.len(), 0);
}
