use rust_hephaestus::*;

#[test]
fn byte_code_for_foreign_procedure_header_with_args_generates_correctly() {
    let irs = compile_source_and_get_intemediate_representation(
        "WriteFile :: (handle: *void, to_write: *void, bytes_to_write: int, bytes_written: *void, overlapped: *void) -> bool #foreign Kernel32"
    );   
    
    assert_eq!(irs.len(), 1);

    let foreign_library_const_ir = get_first_ir_named(&irs, "WriteFile");
    
    assert_eq!(foreign_library_const_ir.symbols.len(), 0);
    assert_eq!(foreign_library_const_ir.data.items.len(), 0);
    assert_eq!(foreign_library_const_ir.byte_code.len(), 0);
    assert_eq!(foreign_library_const_ir.foreign_libraries.len(), 0);
}
