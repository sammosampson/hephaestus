use crate::typing::*;
use crate::tests::parsing::*;
use crate::tests::typing::
*;
#[test]
fn typing_procedure_header_returns_correct_types() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {}");

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;
    
    let (types, _unit) = run_typing_on_unit(
        start_type_repository_actor(), 
        proc_header
    );
    
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, user_defined_resolved_type_id(proc_header_id));
    assert_eq!(types[0].name, "SomeProcedure".to_string());
    assert_eq!(types[0].item, procedure_definition_type_item(vec!(), vec!()));
    assert_eq!(types[0].size, unresolved_type_size());
}


#[test]
fn typing_procedure_header_with_args_returns_correct_types() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: (a: int, b: float) -> float, int {
    }");

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;

    let (types, _unit) = run_typing_on_unit(
        start_type_repository_actor(), 
        proc_header
    );

    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, user_defined_resolved_type_id(proc_header_id));
    assert_eq!(types[0].name, string("SomeProcedure"));
    assert_eq!(types[0].item, procedure_definition_type_item( 
        vec!(
            built_in_type_resolved_type_id(int_32_built_in_type()),
            built_in_type_resolved_type_id(float_32_built_in_type()),
        ), 
        vec!(
            built_in_type_resolved_type_id(float_32_built_in_type()),
            built_in_type_resolved_type_id(int_32_built_in_type()),
        )
    ));
    assert_eq!(types[0].size, unresolved_type_size());
}