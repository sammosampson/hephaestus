
use crate::threading::*;
use crate::typing::*;
use crate::parsing::*;
use crate::utilities::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;

#[test]
fn typing_string_struct_types_sucessfully() {
    let units_and_types = compile_source_and_get_types_and_unit("string :: struct {
    len: int;
    data: *u8;
}

main :: () {
    s := \"hello\"
}");

    
    assert_eq!(units_and_types.len(), 3);
    let (proc_body_unit, proc_body_types) = get_first_typed_procedure_body_unit(&units_and_types);

    assert_eq!(proc_body_types.len(), 0);
    assert_eq!(
        proc_body_unit.tree, 
        node(
            position(62, 6, 12),
            procedure_body_item(
                string("main"),
                vec!(),
                vec!(),
                vec!(                       
                    node(
                        position(68, 7, 5),
                        assignment_item(
                            string("s"),                     
                            node(                    
                                position(73, 7, 10),
                                literal_item(resolved_resolvable_literal(resolved_string_literal(string("hello")))),
                            ),
                            resolved_resolvable_type(create_shareable(string_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}
