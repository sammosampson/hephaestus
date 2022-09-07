
use crate::typing::*;
use crate::parsing::*;
use crate::tests::parsing::*;

#[test]
fn typing_procedure_body_waits_for_external_procedure() {
    let mut units = crate::tests::parsing::run_parse_file_return_only_units("SomeProcedure :: () {
    SomeExternalProcedure(1);
}");

    let external_proc_type = crate::tests::typing::create_procedure_definition_type(
        "SomeExternalProcedure",
        vec!(
            ResolvedTypeId::BuiltInType(BuiltInType::Int32)
        ),
        vec!()
    );
    
    let external_proc_type_id = external_proc_type.id.clone();

    let typing_repository = crate::tests::typing::start_type_repository_actor();
        
    crate::tests::typing::add_resolved_type(
        &typing_repository, 
        crate::tests::typing::create_procedure_definition_type_with_no_args("SomeExternalProcedure")
    );

    crate::tests::typing::add_resolved_type(&typing_repository, external_proc_type);
    
    crate::tests::typing::add_resolved_type(
        &typing_repository, 
        crate::tests::typing::create_procedure_definition_type_with_no_args("SomeOtherExternalProcedure")
    );

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();

    let (types, unit) = crate::tests::typing::run_typing_on_unit(
        typing_repository, 
        proc_body
    );

    assert_eq!(types.len(), 0);
    assert_eq!(
        unit.tree, 
        node(
            position(20, 1, 21),
            procedure_body_item(
                vec!(),
                vec!(),
                vec!(                       
                    node(                    
                        position(26, 2, 5),
                        procedure_call_item(
                            string("SomeExternalProcedure"),
                            vec!(
                                node(
                                    position(48, 2, 27),                            
                                    arg_item( 
                                        node(
                                            position(48, 2, 27),
                                            literal_item(int_literal(1))
                                        ),
                                        resolved_resolvable_type(built_in_type_resolved_type_id(int_32_built_in_type())) 
                                    )
                                ),
                            ),
                            resolved_resolvable_type(external_proc_type_id),
                            vec!()
                        )
                    )
                )
            )
        )
    )
}