
use crate::typing::*;
use crate::parsing::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;


fn create_external_procedure(args: ResolvedTypeIds, returns: ResolvedTypeIds) -> ResolvedType {
    create_procedure_definition_type("SomeExternalProcedure", args, returns)
}

fn create_external_procedure_with_no_args_type() -> ResolvedType {
    create_procedure_definition_type_with_no_args("SomeExternalProcedure")
}

fn create_external_procedure_with_int_arg_type() -> ResolvedType {
    create_external_procedure(
        vec!(built_in_type_resolved_type_id(int_32_built_in_type())),
        vec!()
    )
}

fn create_external_procedure_with_int_arg_and_float_return_type() -> ResolvedType {
    create_external_procedure(
        vec!(built_in_type_resolved_type_id(int_32_built_in_type())),        
        vec!(built_in_type_resolved_type_id(float_32_built_in_type()))
    )
}

fn create_some_other_external_procedure_with_no_args_type() -> ResolvedType {
    create_procedure_definition_type_with_no_args("SomeOtherExternalProcedure")
}

#[test]
fn typing_procedure_body_waits_for_external_procedure() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {
    SomeExternalProcedure(1);
}");

    let typing_repository = start_type_repository_actor();

    let external_proc_type = create_external_procedure_with_int_arg_type();
    let external_proc_type_id = external_proc_type.id.clone();
    
    add_resolved_type(&typing_repository, create_external_procedure_with_no_args_type());
    add_resolved_type(&typing_repository, external_proc_type);
    add_resolved_type(&typing_repository, create_some_other_external_procedure_with_no_args_type());

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();

    let (types, unit) = run_typing_on_unit(typing_repository, proc_body);

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
                                            literal_item(unsigned_int_literal(1))
                                        ),
                                        resolved_resolvable_type(built_in_type_resolved_type_id(int_32_built_in_type())) 
                                    )
                                ),
                            ),
                            resolved_resolvable_type(external_proc_type_id)
                        )
                    )
                )
            )
        )
    )
}

#[test]
fn typing_procedure_body_waits_for_external_procedure_with_return_arg() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {
    x := SomeExternalProcedure(1);
}");
    
    let typing_repository = start_type_repository_actor();
        
    let external_proc_type = create_external_procedure_with_int_arg_and_float_return_type();
    let external_proc_type_id = external_proc_type.id.clone();

    add_resolved_type(&typing_repository, create_external_procedure_with_no_args_type());
    add_resolved_type(&typing_repository, external_proc_type);
    add_resolved_type(&typing_repository, create_some_other_external_procedure_with_no_args_type());


    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();

    let (types, unit) = run_typing_on_unit(typing_repository, proc_body);

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
                        assignment_item(
                            string("x"),                     
                            node(                    
                                position(31, 2, 10),
                                procedure_call_item(
                                    string("SomeExternalProcedure"),
                                    vec!(
                                        node(
                                            position(53, 2, 32),                            
                                            arg_item( 
                                                node(
                                                    position(53, 2, 32),
                                                    literal_item(unsigned_int_literal(1))
                                                ),
                                                resolved_resolvable_type(built_in_type_resolved_type_id(int_32_built_in_type())) 
                                            )
                                        ),
                                    ),
                                    resolved_resolvable_type(external_proc_type_id)
                                )
                            ),
                            resolved_resolvable_type(built_in_type_resolved_type_id(float_32_built_in_type()))
                        )
                    )
                )
            )
        )
    )
}

#[test]
fn typing_procedure_body_waits_for_external_procedure_with_arg_from_prior_expression() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {
    a := 1; 
    SomeExternalProcedure(a);
}");
    
    let typing_repository = start_type_repository_actor();
        
    let external_proc_type = create_external_procedure_with_int_arg_type();
    let external_proc_type_id = external_proc_type.id.clone();

    add_resolved_type(&typing_repository, create_external_procedure_with_no_args_type());
    add_resolved_type(&typing_repository, external_proc_type);
    add_resolved_type(&typing_repository, create_some_other_external_procedure_with_no_args_type());

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();

    let (types, unit) = run_typing_on_unit(typing_repository, proc_body);

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
                        assignment_item(
                            string("a"),
                            node(
                                position(31, 2, 10),
                                literal_item(unsigned_int_literal(1))
                            ),
                            resolved_resolvable_type(built_in_type_resolved_type_id(int_32_built_in_type())) 
                        )
                    ),
                    node(
                        position(39, 3, 5),
                        procedure_call_item(
                            string("SomeExternalProcedure"),
                            vec!(
                                node(
                                    position(61, 3, 27),                            
                                    arg_item( 
                                        node(
                                            position(61, 3, 27),  
                                            identifier_item(string("a"))
                                        ),
                                        resolved_resolvable_type(built_in_type_resolved_type_id(int_32_built_in_type())) 
                                    )
                                ),
                            ),
                            resolved_resolvable_type(external_proc_type_id)                                
                        )
                    )
                )
            )
        )
    )
}