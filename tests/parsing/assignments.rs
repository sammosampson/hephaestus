use rust_hephaestus::*;

#[test]
fn parse_typed_assigment_parses_correctly() {        
    let units_and_types = compile_source_and_get_units_and_types("main :: () {
        x : s32 = 1;
    }");

    assert_eq!(units_and_types.len(), 2);
    let (proc_body_unit, _,) = get_first_typed_procedure_body_unit_named(&units_and_types, "main");
    assert_eq!(
        proc_body_unit.tree, 
        node(
            position(11, 1, 12),
            procedure_body_item(
                string("main"),
                vec!(),
                vec!(),
                vec!(
                    node(
                        position(21, 2, 9),
                        variable_declaration_item(            
                            string("x"),                     
                            node(
                                position(31, 2, 19),
                                literal_item(resolved_resolvable_literal(resolved_signed_int_32_literal(1))),
                            ),
                            resolved_resolvable_type(create_shareable(signed_int_32_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}                       

#[test]
fn parse_invalid_assigment_parses_correctly() {        
    let (units, errors) = compile_source_and_get_parsed_units_and_errors("main :: () {
    x :! s32 = 1;
}");

    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(20, 2, 8),
            error_item()
        )
    );

    assert_eq!(errors.len(), 1);
    let file_error = &errors[0];
    assert_eq!(
        file_error.items[0], 
        compilation_error(
            parser_error(expected_type_error()),
            position(20, 2, 8)
        )
    )
}

#[test]
fn parse_casted_assigment_parses_correctly() {        
    let units = run_parse_file_return_only_units("main :: () {
    x := cast(u32) 1;
}");

    assert_eq!(units.len(), 2);
    assert_eq!(
        units[0].tree, 
        node(
            position(11, 1, 12),
            procedure_body_item(
                string("main"),
                vec!(),
                vec!(),
                vec!(
                    node(
                        position(17, 2, 5),
                        variable_declaration_item(            
                            string("x"),                     
                            node(
                                position(22, 2, 10),
                                cast_item(
                                    resolved_resolvable_type(create_shareable(unsigned_int_32_runtime_type())),
                                    node(
                                        position(32, 2, 20),
                                        literal_item(unresolved_resolvable_literal(unresolved_int_literal(string("1")))),
                                    )
                                )
                            ),
                            unresolved_resolvable_type()
                        )
                    )
                )
            )
        )
    )
}                       


#[test]
fn parse_struct_member_access_assignment_parses_correctly() {        
    let units = run_parse_file_return_only_units("proc :: (s: string) {
    x := s.len;
}");

    assert_eq!(units.len(), 2);
    assert_eq!(
        units[0].tree, 
        node(
            position(20, 1, 21),
            procedure_body_item(
                string("proc"),
                vec!(
                    node(
                        position(9, 1, 10),
                        member_declaration_item(
                            string("s"),
                            resolved_resolvable_type(create_shareable(string_runtime_type()))
                        )
                    )
                ),
                vec!(),
                vec!(
                    node(
                        position(26, 2, 5),
                        variable_declaration_item(            
                            string("x"),                     
                            node(
                                position(31, 2, 10),
                                member_expr_item(
                                    node(
                                        position(31, 2, 10),
                                        instance_item(
                                            string("s"),
                                            unresolved_resolvable_type(),
                                            unknown_scope()
                                        )        
                                    ),
                                    node(
                                        position(33, 2, 12),
                                        member_item(
                                            string("len"),
                                            unresolved_resolvable_type()
                                        )        
                                    ),                                
                                    unresolved_resolvable_type()
                                )
                            ),
                            unresolved_resolvable_type()
                        )
                    )
                )
            )
        )
    )
}