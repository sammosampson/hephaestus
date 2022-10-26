
use crate::{
    tests::intermediate_representation::*, 
    utilities::*
};

#[test]
fn byte_code_for_known_type_signed_number_constant_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "STD_OUTPUT_HANDLE : s32 : -11;"
    );   
    
    assert_eq!(irs.len(), 1);

    let foreign_library_const_ir = get_first_ir_named(&irs, "STD_OUTPUT_HANDLE");
    
    assert_eq!(foreign_library_const_ir.symbols.len(), 1);
    assert_eq!(foreign_library_const_ir.symbols, vec!(absolute_external_32(string("STD_OUTPUT_HANDLE"), 0xFFFFFFF5)));    
    assert_eq!(foreign_library_const_ir.data.items.len(), 0);
    assert_eq!(foreign_library_const_ir.byte_code.len(), 0);
    assert_eq!(foreign_library_const_ir.foreign_libraries.len(), 0);
}

#[test]
fn byte_code_for_inferred_type_signed_number_constant_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "STD_OUTPUT_HANDLE :: -11;"
    );   
    
    assert_eq!(irs.len(), 1);

    let foreign_library_const_ir = get_first_ir_named(&irs, "STD_OUTPUT_HANDLE");
    
    assert_eq!(foreign_library_const_ir.symbols.len(), 1);
    assert_eq!(foreign_library_const_ir.symbols, vec!(absolute_external_64(string("STD_OUTPUT_HANDLE"), 0xFFFFFFFFFFFFFFF5)));    
    assert_eq!(foreign_library_const_ir.data.items.len(), 0);
    assert_eq!(foreign_library_const_ir.byte_code.len(), 0);
    assert_eq!(foreign_library_const_ir.foreign_libraries.len(), 0);
}

#[test]
fn byte_code_for_foreign_system_library_constant_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "Kernel32 :: #foreign_system_library \"kernel32\";"
    );   
    
    assert_eq!(irs.len(), 1);

    let foreign_library_const_ir = get_first_ir_named(&irs, "Kernel32");
    
    assert_eq!(foreign_library_const_ir.symbols.len(), 0);
    assert_eq!(foreign_library_const_ir.data.items.len(), 0);
    assert_eq!(foreign_library_const_ir.byte_code.len(), 0);
    assert_eq!(foreign_library_const_ir.foreign_libraries.len(), 1);
    assert_eq!(foreign_library_const_ir.foreign_libraries, vec!("kernel32"));
}
