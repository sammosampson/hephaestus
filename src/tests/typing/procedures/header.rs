use crate::threading::create_shareable;
use crate::types::*;
use crate::utilities::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;

#[test]
fn typing_procedure_header_returns_correct_types() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {}");

    assert_eq!(units.len(), 2);

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;
    
    let (types, _unit) = run_typing_on_unit(
        start_type_repository_actor(), 
        proc_header
    );
    
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, user_defined_runtime_type_id(proc_header_id));
    assert_eq!(types[0].name, "SomeProcedure".to_string());
    assert_eq!(types[0].item, procedure_definition_type_item(vec!(), vec!()));
    assert_eq!(types[0].size, not_required_type_size());
}


#[test]
fn typing_procedure_header_with_args_returns_correct_types() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: (a: int, b: float) -> float, int {
    }");

    assert_eq!(units.len(), 2);

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;

    let (types, _unit) = run_typing_on_unit(
        start_type_repository_actor(), 
        proc_header
    );

    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, user_defined_runtime_type_id(proc_header_id));
    assert_eq!(types[0].name, string("SomeProcedure"));
    assert_eq!(types[0].item, procedure_definition_type_item( 
        vec!(
            create_shareable(signed_int_64_runtime_type()),
            create_shareable(float_32_runtime_type()),
        ), 
        vec!(
            create_shareable(float_32_runtime_type()),
            create_shareable(signed_int_64_runtime_type()),
        )
    ));
    assert_eq!(types[0].size, not_required_type_size());
}

#[test]
fn typing_procedure_header_with_pointer_args_returns_correct_types() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: (a: *int, b: *float) -> *float, *int {
    }");

    assert_eq!(units.len(), 2);

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;

    let (types, _unit) = run_typing_on_unit(
        start_type_repository_actor(), 
        proc_header
    );

    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, user_defined_runtime_type_id(proc_header_id));
    assert_eq!(types[0].name, string("SomeProcedure"));
    assert_eq!(types[0].item, procedure_definition_type_item( 
        vec!(
            create_shareable(signed_int_64_pointer_runtime_type()),
            create_shareable(float_32_pointer_runtime_type()),
        ), 
        vec!(
            create_shareable(float_32_pointer_runtime_type()),
            create_shareable(signed_int_64_pointer_runtime_type()),
        )
    ));
    assert_eq!(types[0].size, not_required_type_size());
}

#[test]
fn typing_foreign_library_procedure_header_returns_correct_types() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: (a: *int, b: *float) -> *float, *int #foreign Kernel32");

    assert_eq!(units.len(), 1);
    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;

    let (types, _unit) = run_typing_on_unit(
        start_type_repository_actor(), 
        proc_header
    );

    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, user_defined_runtime_type_id(proc_header_id));
    assert_eq!(types[0].name, string("SomeProcedure"));
    assert_eq!(types[0].item, procedure_definition_type_item( 
        vec!(
            create_shareable(signed_int_64_pointer_runtime_type()),
            create_shareable(float_32_pointer_runtime_type()),
        ), 
        vec!(
            create_shareable(float_32_pointer_runtime_type()),
            create_shareable(signed_int_64_pointer_runtime_type()),
        )
    ));
    assert_eq!(types[0].size, not_required_type_size());
}