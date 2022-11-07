use rust_hephaestus::*;

#[test]
fn parse_typed_assigment_parses_correctly() {        
    let units = run_parse_file_return_only_units("main :: () {
        x : s32 = 1;
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
                        position(21, 2, 9),
                        variable_declaration_item(            
                            string("x"),                     
                            node(
                                position(31, 2, 19),
                                literal_item(unresolved_resolvable_literal(unresolved_int_literal(string("1")))),
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
    let units = run_parse_file_return_only_units("main :: () {
        x :! s32 = 1;
    }");

    assert_eq!(units.len(), 0);
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