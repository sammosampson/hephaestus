
use crate::{
    tests::intermediate_representation::*
};

#[test]
fn byte_code_for_known_type_signed_number_assignment_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation("STD_OUTPUT_HANDLE : s32 : -11;

Kernel32 :: #foreign_system_library \"kernel32\";
WriteFile :: (handle: *void, to_write: *void, bytes_to_write: u32, bytes_written: *void, overlapped: *void) -> bool #foreign Kernel32;
GetStdHandle :: (handle_type: s32) -> *void #foreign Kernel32;

print :: (to_print: *void, length: u32) {
    handle := GetStdHandle(STD_OUTPUT_HANDLE);
    bytes_written: *void = null;
    overlapped: *void = null;
    WriteFile(handle, to_print, length, bytes_written, overlapped);
}

main :: () {
    length: u32 = 15;
    print(\"hello world!\r\0\", length);
}"
    );   
    
    assert_eq!(irs.len(), 2);

    let main_body_ir = get_first_ir_with_byte_code_named(&irs, "main");
    
    assert_eq!(main_body_ir.symbols.len(), 1);
    assert_eq!(main_body_ir.data.len(), 0);
    assert_eq!(main_body_ir.byte_code.len(), 5);
    assert_eq!(main_body_ir.foreign_libraries.len(), 0);
}