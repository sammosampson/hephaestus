use rust_hephaestus::*;

#[test]
fn typing_procedure_body_waits_for_external_procedure_with_int_arg() {
    let units_and_types = compile_source_and_get_units_and_types("SomeProcedure :: () {
    SomeExternalProcedure(1);
}

SomeExternalProcedure :: () {
}

SomeOtherExternalProcedure :: () {
}

SomeExternalProcedure :: (arg: int) {
}");    
    let (_unit, external_proc_types) = get_first_typed_procedure_header_unit_named_with_arg_count(
        &units_and_types, "SomeExternalProcedure", 
        1
    );
    let external_proc_type = external_proc_types[0].clone();

    let (unit, types) = get_first_typed_procedure_body_unit_named(&units_and_types, "SomeProcedure");
    assert_eq!(types.len(), 0);
    
    let result = node(
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
                        resolved_resolvable_type(external_proc_type)
                    )
                )
            )
        )
    );
    assert_eq!(unit.tree, result);
}


#[test]
fn typing_procedure_body_does_not_wait_for_external_procedure_that_does_not_exist() {
    let errors = compile_source_and_get_errors("SomeProcedure :: () {
    SomeExternalProcedure(1);
}

SomeExternalProcedure :: () {
}

SomeOtherExternalProcedure :: () {
}");    
    assert_eq!(errors.len(), 1);
}

#[test]
fn typing_procedure_body_waits_for_external_procedure_with_string_arg() {
    let units_and_types = compile_source_and_get_units_and_types("SomeProcedure :: () {
    SomeExternalProcedure(\"Hello\");
}

SomeExternalProcedure :: () {
}

SomeOtherExternalProcedure :: () {
}

SomeExternalProcedure :: (arg: string) {
}");

    let (_unit, external_proc_types) = get_first_typed_procedure_header_unit_named_with_arg_count(
        &units_and_types, "SomeExternalProcedure", 
        1
    );
    let external_proc_type = external_proc_types[0].clone();

    let (unit, types) = get_first_typed_procedure_body_unit_named(&units_and_types, "SomeProcedure");    
    assert_eq!(types.len(), 0);

    let result = node(
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
                                        literal_item(resolved_resolvable_literal(resolved_string_literal(to_byte_string("Hello"))))
                                    ),
                                    resolved_resolvable_type(create_shareable(string_runtime_type())) 
                                )
                            ),
                        ),
                        resolved_resolvable_type(external_proc_type)
                    )
                )
            )
        )
    );

    assert_eq!(unit.tree, result);    
}

#[test]
fn typing_procedure_body_waits_for_external_procedure_with_return_arg() {
    let units_and_types = compile_source_and_get_units_and_types("SomeProcedure :: () {
    x := SomeExternalProcedure(1);
}

SomeExternalProcedure :: () {
}

SomeOtherExternalProcedure :: () {
}

SomeExternalProcedure :: (arg: int) -> float {
}");
    let (_unit, external_proc_types) = get_first_typed_procedure_header_unit_named_with_arg_count(
        &units_and_types, "SomeExternalProcedure", 
        1
    );
    let external_proc_type = external_proc_types[0].clone();

    let (unit, types) = get_first_typed_procedure_body_unit_named(&units_and_types, "SomeProcedure");
    assert_eq!(types.len(), 0);
    
    let result = node(
        position(20, 1, 21),
        procedure_body_item(
            string("SomeProcedure"),
            vec!(),
            vec!(),
            vec!(                       
                node(
                    position(26, 2, 5),
                    variable_declaration_item(
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
                                resolved_resolvable_type(external_proc_type)
                            )
                        ),
                        resolved_resolvable_type(create_shareable(float_32_runtime_type())) 
                    )
                )
            )
        )
    );

    assert_eq!(unit.tree, result);
}

#[test]
fn typing_procedure_body_waits_for_external_procedure_with_arg_from_prior_expression() {
    let units_and_types = compile_source_and_get_units_and_types("SomeProcedure :: () {
    a := 1; 
    SomeExternalProcedure(a);
}

SomeExternalProcedure :: () {
}

SomeOtherExternalProcedure :: () {
}

SomeExternalProcedure :: (arg: int) {
}");
    
    let (_unit, external_proc_types) = get_first_typed_procedure_header_unit_named_with_arg_count(
        &units_and_types, "SomeExternalProcedure", 
        1
    );
    let external_proc_type = external_proc_types[0].clone();

    let (unit, types) = get_first_typed_procedure_body_unit_named(&units_and_types, "SomeProcedure");
    assert_eq!(types.len(), 0);

    let result = node(
        position(20, 1, 21),
        procedure_body_item(
            string("SomeProcedure"),
            vec!(),
            vec!(),
            vec!(                       
                node(
                    position(26, 2, 5),
                    variable_declaration_item(
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
                                        identifier_item(string("a"), local_scope())
                                    ),
                                    resolved_resolvable_type(create_shareable(signed_int_64_runtime_type())) 
                                )
                            ),
                        ),
                        resolved_resolvable_type(external_proc_type)                               
                    )
                )
            )
        )
    );

    assert_eq!(unit.tree, result);
}