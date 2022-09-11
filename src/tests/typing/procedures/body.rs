use crate::typing::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;

#[test]
fn typing_procedure_body_with_args_from_header_used_in_expression_gets_typed_correctly() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: (a: int, b: float) -> float, int {
        x := a;
        y := b;
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
            position(50, 1, 51),
            procedure_body_item(
                vec!(
                    node(
                        position(18, 1, 19),
                        arg_declaration_item(
                            string("a"),
                            resolved_resolvable_type(built_in_type_runtime_type_id(int_32_built_in_type()))
                        )
                    ),
                    node(
                        position(26, 1, 27),
                        arg_declaration_item(
                            string("b"),
                            resolved_resolvable_type(built_in_type_runtime_type_id(float_32_built_in_type()))
                        )
                    )
                ),
                vec!(
                    node(
                        position(39, 1, 40),
                        type_item(
                            resolved_resolvable_type(built_in_type_runtime_type_id(float_32_built_in_type()))
                        )
                    ),
                    node(
                        position(46, 1, 47),
                        type_item(
                            resolved_resolvable_type(built_in_type_runtime_type_id(int_32_built_in_type()))
                        )
                    )
                ),
                vec!(
                    node(
                        position(60, 2, 9),
                        assignment_item(
                            string("x"), 
                            node(
                                position(65, 2, 14),
                                identifier_item(string("a"))
                            ),
                            resolved_resolvable_type(built_in_type_runtime_type_id(int_32_built_in_type()))
                        )
                    ),
                    node(
                        position(76, 3, 9),
                        assignment_item(
                            string("y"), 
                            node(
                                position(81, 3, 14),
                                identifier_item(string("b"))
                            ),
                            resolved_resolvable_type(built_in_type_runtime_type_id(float_32_built_in_type()))
                        )
                    ),
                    node( 
                        position(92, 4, 9),
                        return_item( 
                            vec!(
                                node(
                                    position(99, 4, 16),
                                    arg_item(
                                        node(
                                            position(99, 4, 16),
                                            literal_item(float_literal(1.0))
                                        ),
                                        resolved_resolvable_type(built_in_type_runtime_type_id(float_32_built_in_type()))
                                    )
                                ),
                                node(
                                    position(104, 4, 21),
                                    arg_item(
                                        node(
                                            position(104, 4, 21),
                                            literal_item(unsigned_int_literal(2))
                                        ),
                                        resolved_resolvable_type(built_in_type_runtime_type_id(int_32_built_in_type()))
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

