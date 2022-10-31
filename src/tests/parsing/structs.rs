use crate::{
    parsing::*,
    types::*,
    tests::parsing::*,
    utilities::*, threading::create_shareable
};

#[test]
fn parse_struct_parses_correctly() {        
    let units = run_parse_file_return_only_units("SomeStruct :: struct {
    x: float;
    y: int;
}");

    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            struct_item(
                string("SomeStruct"),
                vec!(
                    node(
                        position(27, 2, 5),
                        member_declaration_item(            
                            string("x"),
                            resolved_resolvable_type(create_shareable(float_32_runtime_type()))
                        )
                    ),
                    node(
                        position(41, 3, 5),
                        member_declaration_item(            
                            string("y"),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}
