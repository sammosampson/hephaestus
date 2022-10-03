
use crate::threading::*;
use crate::typing::*;
use crate::parsing::*;
use crate::utilities::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;


fn create_external_procedure(args: RuntimeTypePointers, returns: RuntimeTypePointers) -> RuntimeType {
    create_procedure_definition_type("SomeExternalProcedure", args, returns)
}

fn create_external_procedure_with_no_args_type() -> RuntimeType {
    create_procedure_definition_type_with_no_args("SomeExternalProcedure")
}

fn create_external_procedure_with_int_arg_type() -> RuntimeType {
    create_external_procedure(
        vec!(create_shareable(signed_int_64_runtime_type())),
        vec!()
    )
}

fn create_external_procedure_with_string_arg_type() -> RuntimeType {
    create_external_procedure(
        vec!(create_shareable(string_runtime_type())),
        vec!()
    )
}

fn create_external_procedure_with_int_arg_and_float_return_type() -> RuntimeType {
    create_external_procedure(
        vec!(create_shareable(signed_int_64_runtime_type())),        
        vec!(create_shareable(float_32_runtime_type()))
    )
}

fn create_some_other_external_procedure_with_no_args_type() -> RuntimeType {
    create_procedure_definition_type_with_no_args("SomeOtherExternalProcedure")
}

#[test]
fn typing_procedure_body_waits_for_external_procedure_with_int_arg() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {
    SomeExternalProcedure(1);
}");

    let typing_repository = start_type_repository_actor();

    let external_proc_type = create_external_procedure_with_int_arg_type();
    
    add_resolved_type(&typing_repository, create_external_procedure_with_no_args_type());
    add_resolved_type(&typing_repository, external_proc_type.clone());
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
                string("SomeProcedure"),
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
                                            literal_item(resolved_resolvable_literal(resolved_signed_int_64_literal(1)))
                                        ),
                                        resolved_resolvable_type(create_shareable(signed_int_64_runtime_type())) 
                                    )
                                ),
                            ),
                            resolved_resolvable_type(create_shareable(external_proc_type))
                        )
                    )
                )
            )
        )
    )
}


#[test]
fn typing_procedure_body_waits_for_external_procedure_with_string_arg() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {
    SomeExternalProcedure(\"Hello\");
}");

    let typing_repository = start_type_repository_actor();

    let external_proc_type = create_external_procedure_with_string_arg_type();
    
    add_resolved_type(&typing_repository, create_external_procedure_with_no_args_type());
    add_resolved_type(&typing_repository, external_proc_type.clone());
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
                string("SomeProcedure"),
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
                                            literal_item(resolved_resolvable_literal(resolved_string_literal(string("Hello"))))
                                        ),
                                        resolved_resolvable_type(create_shareable(string_runtime_type())) 
                                    )
                                ),
                            ),
                            resolved_resolvable_type(create_shareable(external_proc_type))
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
    
    add_resolved_type(&typing_repository, create_external_procedure_with_no_args_type());
    add_resolved_type(&typing_repository, external_proc_type.clone());
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
                string("SomeProcedure"),
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
                                                    literal_item(resolved_resolvable_literal(resolved_signed_int_64_literal(1)))
                                                ),
                                                resolved_resolvable_type(create_shareable(signed_int_64_runtime_type())) 
                                            )
                                        ),
                                    ),
                                    resolved_resolvable_type(create_shareable(external_proc_type))
                                )
                            ),
                            resolved_resolvable_type(create_shareable(float_32_runtime_type())) 
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

    add_resolved_type(&typing_repository, create_external_procedure_with_no_args_type());
    add_resolved_type(&typing_repository, external_proc_type.clone());
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
                string("SomeProcedure"),
                vec!(),
                vec!(),
                vec!(                       
                    node(
                        position(26, 2, 5),
                        assignment_item(
                            string("a"),
                            node(
                                position(31, 2, 10),
                                literal_item(resolved_resolvable_literal(resolved_signed_int_64_literal(1)))
                            ),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
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
                                        resolved_resolvable_type(create_shareable(signed_int_64_runtime_type())) 
                                    )
                                ),
                            ),
                            resolved_resolvable_type(create_shareable(external_proc_type))                               
                        )
                    )
                )
            )
        )
    )
}