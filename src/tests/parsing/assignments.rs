use crate::{
    parsing::*,
    typing::*,
    tests::parsing::*,
    threading::*, 
    utilities::*
};

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
                        assignment_item(            
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
                        assignment_item(            
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
