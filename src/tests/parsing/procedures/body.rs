use crate::parsing::*;
use crate::typing::*;
use crate::tests::parsing::*;

#[test]
fn parse_procedure_body_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeProcedure :: () {
    a := 1;
    SomeOtherProcedure(a);
}"
    );
       
    assert_eq!(
        units[0].tree, 
        node(
            position(20, 1, 21),
            procedure_body_item(
                vec!(),
                vec!(),
                vec!(
                    node( 
                        position(26, 2, 5),
                        assignment_item( 
                            string("a"),
                            node(
                                position(31, 2, 10),
                                literal_item(int_literal(1))
                            ),
                            unresolved_resolvable_type()
                        )
                    ),
                    node(
                        position(38, 3, 5),
                        procedure_call_item(
                            string("SomeOtherProcedure"),
                            vec!(
                                node(
                                    position(57, 3, 24),
                                    arg_item(
                                        node(
                                            position(57, 3, 24),
                                            identifier_item(string("a"))
                                        ),
                                        unresolved_resolvable_type()
                                    )
                                )
                            ),
                            unresolved_resolvable_type()
                        )
                    )                        
                )
            )
        )
    );
}