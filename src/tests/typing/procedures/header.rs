use crate::typing::*;

#[test]
fn typing_procedure_header_returns_correct_types() {
    let mut units = crate::tests::parsing::run_parse_file_return_only_units("SomeProcedure :: () {}");

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;
    
    let (types, _unit) = crate::tests::typing::run_typing_on_unit(
        crate::tests::typing::start_type_repository_actor(), 
        proc_header
    );
    
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, ResolvedTypeId::UserDefined(proc_header_id));
    assert_eq!(types[0].name, "SomeProcedure".to_string());
    assert_eq!(types[0].item, TypeItem::ProcedureDefinition { arg_types: vec!(), return_types: vec!()});
    assert_eq!(types[0].size, TypeSize::Unresolved);
}


#[test]
fn typing_procedure_header_with_args_returns_correct_types() {
    let mut units = crate::tests::parsing::run_parse_file_return_only_units("SomeProcedure :: (a: int, b: float) -> float, int {}");

    let proc_header = units.pop().unwrap();
    let proc_header_id = proc_header.id;

    let (types, _unit) = crate::tests::typing::run_typing_on_unit(
        crate::tests::typing::start_type_repository_actor(), 
        proc_header
    );

    assert_eq!(types.len(), 1);
    assert_eq!(types[0].id, ResolvedTypeId::UserDefined(proc_header_id));
    assert_eq!(types[0].name, "SomeProcedure".to_string());
    assert_eq!(types[0].item, TypeItem::ProcedureDefinition { 
        arg_types: vec!(
            ResolvedTypeId::BuiltInType(BuiltInType::Int32),
            ResolvedTypeId::BuiltInType(BuiltInType::Float32),
        ), 
        return_types: vec!(
            ResolvedTypeId::BuiltInType(BuiltInType::Float32),
            ResolvedTypeId::BuiltInType(BuiltInType::Int32),
        )
    });
    assert_eq!(types[0].size, TypeSize::Unresolved);
}

