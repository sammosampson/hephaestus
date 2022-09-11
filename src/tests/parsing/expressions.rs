use crate::{
    parsing::*,
    typing::*,
    tests::parsing::*,
    threading::*, 
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
                vec!(),
                vec!(),
                vec!(
                    node(
                        position(21, 2, 9),
                        assignment_item(            
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
                                        literal_item(unsigned_int_literal(1)),
                                    ),
                                    node(
                                        position(30, 2, 18),
                                        literal_item(unsigned_int_literal(2)),
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
                vec!(
                    node(
                        position(9, 1, 10),
                        arg_declaration_item( 
                            string("a"),
                            resolved_resolvable_type(create_shareable(int_32_runtime_type())),
                        )
                    ),
                    node(
                        position(17, 1, 18),
                        arg_declaration_item( 
                            string("b"),
                            resolved_resolvable_type(create_shareable(int_32_runtime_type())),
                        )
                    )
                ),
                vec!(),
                vec!(
                    node(
                        position(31, 2, 5),
                        assignment_item(            
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
                                        identifier_item(string("a")),
                                    ),
                                    node(
                                        position(40, 2, 14),
                                        identifier_item(string("b")),
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