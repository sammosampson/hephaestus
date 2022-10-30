use crate::parsing::*;
use crate::strings::*;
use crate::threading::*;
use crate::typing::*;
use crate::utilities::*;
use crate::tests::parsing::*;

#[test]
fn parse_procedure_body_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeProcedure :: () -> float, int {
    a := 1;
    SomeOtherProcedure(a);
    SomeOtherStringProcedure(\"a\");
    return 1.0, 2;
}"
    );

    assert_eq!(
        units[0].tree, 
        node(
            position(34, 1, 35),
            procedure_body_item(
                string("SomeProcedure"),
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
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                        )
                    ),
                ),
                vec!(
                    node( 
                        position(40, 2, 5),
                        variable_declaration_item(
                            string("a"),
                            node(
                                position(45, 2, 10),
                                literal_item(unresolved_resolvable_literal(unresolved_int_literal(string("1"))))
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
                                            unknown_scope_identifier_item(string("a"))
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
                        procedure_call_item(
                            string("SomeOtherStringProcedure"),
                            vec!(
                                node(
                                    position(104, 4, 30),
                                    arg_item(
                                        node(
                                            position(104, 4, 30),
                                            literal_item(unresolved_resolvable_literal(unresolved_string_literal(to_byte_string("a"))))
                                        ),
                                        unresolved_resolvable_type()
                                    )
                                )
                            ),
                            unresolved_resolvable_type()
                        )
                    ),
                    node( 
                        position(114, 5, 5),
                        return_item( 
                            vec!(
                                node(
                                    position(121, 5, 12),
                                    arg_item(
                                        node(
                                            position(121, 5, 12),
                                            literal_item(unresolved_resolvable_literal(unresolved_float_literal(string("1.0"))))
                                        ),
                                        unresolved_resolvable_type()
                                    )
                                ),
                                node(
                                    position(126, 5, 17),
                                    arg_item(
                                        node(
                                            position(126, 5, 17),
                                            literal_item(unresolved_resolvable_literal(unresolved_int_literal(string("2"))))
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