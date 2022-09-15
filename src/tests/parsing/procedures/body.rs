use crate::parsing::*;
use crate::threading::*;
use crate::typing::*;
use crate::tests::parsing::*;

#[test]
fn parse_procedure_body_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeProcedure :: () -> float, int {
    a := 1;
    SomeOtherProcedure(a);
    return 1.0, 2;
}"
    );

    assert_eq!(
        units[0].tree, 
        node(
            position(34, 1, 35),
            procedure_body_item(
                vec!(),
                vec!(
                    node( 
                        position(23, 1, 24),
                        type_item(
                            resolved_resolvable_type(create_shareable(float_32_runtime_type()))
                        )
                    ),
                    node( 
                        position(30, 1, 31),
                        type_item(
                            resolved_resolvable_type(create_shareable(signed_int_32_runtime_type()))
                        )
                    ),
                ),
                vec!(
                    node( 
                        position(40, 2, 5),
                        assignment_item( 
                            string("a"),
                            node(
                                position(45, 2, 10),
                                literal_item(unsigned_int_literal(1))
                            ),
                            unresolved_resolvable_type()
                        )
                    ),
                    node(
                        position(52, 3, 5),
                        procedure_call_item(
                            string("SomeOtherProcedure"),
                            vec!(
                                node(
                                    position(71, 3, 24),
                                    arg_item(
                                        node(
                                            position(71, 3, 24),
                                            identifier_item(string("a"))
                                        ),
                                        unresolved_resolvable_type()
                                    )
                                )
                            ),
                            unresolved_resolvable_type()
                        )
                    ),
                    node( 
                        position(79, 4, 5),
                        return_item( 
                            vec!(
                                node(
                                    position(86, 4, 12),
                                    arg_item(
                                        node(
                                            position(86, 4, 12),
                                            literal_item(float_literal(1.0))
                                        ),
                                        unresolved_resolvable_type()
                                    )
                                ),
                                node(
                                    position(91, 4, 17),
                                    arg_item(
                                        node(
                                            position(91, 4, 17),
                                            literal_item(unsigned_int_literal(2))
                                        ),
                                        unresolved_resolvable_type()
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