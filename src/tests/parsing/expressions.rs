use crate::{
    parsing::*,
    types::*,
    tests::parsing::*,
    threading::*, 
    utilities::*
};

#[test]
fn parse_literal_expression_parses_correctly() {        
    let units = run_parse_file_return_only_units("main :: () {
        x := 1 + 2;
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
                                position(26, 2, 14),
                                binary_expression_item(
                                    node(
                                        position(28, 2, 16),
                                        operator_item(add_operator())
                                    ),
                                    node(
                                        position(26, 2, 14),
                                        literal_item(unresolved_resolvable_literal(unresolved_int_literal(string("1")))),
                                    ),
                                    node(
                                        position(30, 2, 18),
                                        literal_item(unresolved_resolvable_literal(unresolved_int_literal(string("2")))),
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

#[test]
fn parse_variable_expression_parses_correctly() {        
    let units = run_parse_file_return_only_units("main :: (a: int, b: int) {
    x := a + b;
}");    
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[0].tree, 
        node(
            position(25, 1, 26),
            procedure_body_item(
                string("main"),
                vec!(
                    node(
                        position(9, 1, 10),
                        member_declaration_item( 
                            string("a"),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type())),
                        )
                    ),
                    node(
                        position(17, 1, 18),
                        member_declaration_item( 
                            string("b"),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type())),
                        )
                    )
                ),
                vec!(),
                vec!(
                    node(
                        position(31, 2, 5),
                        variable_declaration_item(            
                            string("x"), 
                            node(
                                position(36, 2, 10),
                                binary_expression_item(
                                    node(
                                        position(38, 2, 12),
                                        operator_item(add_operator())
                                    ),
                                    node(
                                        position(36, 2, 10),
                                        unknown_scope_identifier_item(string("a")),
                                    ),
                                    node(
                                        position(40, 2, 14),
                                        unknown_scope_identifier_item(string("b")),
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