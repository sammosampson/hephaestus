use rust_hephaestus::*;

#[test]
fn typing_procedure_body_with_args_from_header_used_in_expression_gets_typed_correctly() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: (a: int, b: float, c: string) -> float, int {
        x := a;
        y := b;
        z := c;
        return 1.0, 2;
    }");

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();
    
    let (types, unit) = run_typing_on_unit(
        start_type_repository_actor(), 
        proc_body
    );
    
    assert_eq!(types.len(), 0);
    assert_eq!(
        unit.tree,
        node(
            position(61, 1, 62),
            procedure_body_item(
                string("SomeProcedure"),
                vec!(
                    node(
                        position(18, 1, 19),
                        member_declaration_item(
                            string("a"),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                        )
                    ),
                    node(
                        position(26, 1, 27),
                        member_declaration_item(
                            string("b"),
                            resolved_resolvable_type(create_shareable(float_32_runtime_type()))
                        )
                    ),
                    node(
                        position(36, 1, 37),
                        member_declaration_item(
                            string("c"),
                            resolved_resolvable_type(create_shareable(string_runtime_type()))
                        )
                    )
                ),
                vec!(
                    node(
                        position(50, 1, 51),
                        type_item(
                            resolved_resolvable_type(create_shareable(float_32_runtime_type()))
                        )
                    ),
                    node(
                        position(57, 1, 58),
                        type_item(
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                        )
                    )
                ),
                vec!(
                    node(
                        position(71, 2, 9),
                        variable_declaration_item(
                            string("x"), 
                            node(
                                position(76, 2, 14),
                                identifier_item(string("a"), local_scope())
                            ),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                        )
                    ),
                    node(
                        position(87, 3, 9),
                        variable_declaration_item(
                            string("y"), 
                            node(
                                position(92, 3, 14),
                                identifier_item(string("b"), local_scope())
                            ),
                            resolved_resolvable_type(create_shareable(float_32_runtime_type()))
                        )
                    ),
                    node(
                        position(103, 4, 9),
                        variable_declaration_item(
                            string("z"), 
                            node(
                                position(108, 4, 14),
                                identifier_item(string("c"), local_scope())
                            ),
                            resolved_resolvable_type(create_shareable(string_runtime_type()))
                        )
                    ),
                    node( 
                        position(119, 5, 9),
                        return_item( 
                            vec!(
                                node(
                                    position(126, 5, 16),
                                    arg_item(
                                        node(
                                            position(126, 5, 16),
                                            literal_item(resolved_resolvable_literal(resolved_float_32_literal(1.0)))
                                        ),
                                        resolved_resolvable_type(create_shareable(float_32_runtime_type()))
                                    )
                                ),
                                node(
                                    position(131, 5, 21),
                                    arg_item(
                                        node(
                                            position(131, 5, 21),
                                            literal_item(resolved_resolvable_literal(resolved_signed_int_64_literal(2)))
                                        ),
                                        resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                                    )
                                )
                            )
                        )
                    ),   
                )
            )
        )
    );
}


#[test]
fn typing_procedure_body_with_pointer_args_from_header_get_typed_correctly() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: (a: *int, b: *float) -> *float, *int {
    }");

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();
    
    let (types, unit) = run_typing_on_unit(
        start_type_repository_actor(), 
        proc_body
    );

    assert_eq!(types.len(), 0);
    assert_eq!(
        unit.tree,
        node(
            position(54, 1, 55),
            procedure_body_item(
                string("SomeProcedure"),
                vec!(
                    node(
                        position(18, 1, 19),
                        member_declaration_item(
                            string("a"),
                            resolved_resolvable_type(create_shareable(signed_int_64_pointer_runtime_type()))
                        )
                    ),
                    node(
                        position(27, 1, 28),
                        member_declaration_item(
                            string("b"),
                            resolved_resolvable_type(create_shareable(float_32_pointer_runtime_type()))
                        )
                    )
                ),
                vec!(
                    node(
                        position(42, 1, 43),
                        type_item(
                            resolved_resolvable_type(create_shareable(float_32_pointer_runtime_type()))
                        )
                    ),
                    node(
                        position(50, 1, 51),
                        type_item(
                            resolved_resolvable_type(create_shareable(signed_int_64_pointer_runtime_type()))
                        )
                    )
                ),
                vec!()
            )
        )
    );
}

