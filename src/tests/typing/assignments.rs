
use crate::threading::*;
use crate::typing::*;
use crate::parsing::*;
use crate::utilities::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;

#[test]
fn typing_procedure_body_assignment_types_variable_int_literal_assignment() {
    let units_and_types = compile_source_and_get_types_and_unit("SomeProcedure :: () {
    x := 1;
}");

    
    assert_eq!(units_and_types.len(), 2);
    let (proc_body_unit, proc_body_types) = get_first_typed_procedure_body_unit(&units_and_types);

    assert_eq!(proc_body_types.len(), 0);
    assert_eq!(
        proc_body_unit.tree, 
        node(
            position(20, 1, 21),
            procedure_body_item(
                string("SomeProcedure"),
                vec!(),
                vec!(),
                vec!(                       
                    node(
                        position(26, 2, 5),
                        assignment_item(
                            string("x"),                     
                            node(                    
                                position(31, 2, 10),
                                literal_item(resolved_resolvable_literal(resolved_signed_int_64_literal(1))),
                            ),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}


#[test]
fn typing_known_type_procedure_body_assignment_types_variable_int_literal_assignment() {
    let units_and_types = compile_source_and_get_types_and_unit("SomeProcedure :: () {
    x : u32 = 1;
}");

    assert_eq!(units_and_types.len(), 2);
    let (proc_body_unit, proc_body_types) = get_first_typed_procedure_body_unit(&units_and_types);

    assert_eq!(proc_body_types.len(), 0);
    assert_eq!(
        proc_body_unit.tree, 
        node(
            position(20, 1, 21),
            procedure_body_item(
                string("SomeProcedure"),
                vec!(),
                vec!(),
                vec!(                       
                    node(
                        position(26, 2, 5),
                        assignment_item(
                            string("x"),                     
                            node(                    
                                position(36, 2, 15),
                                literal_item(resolved_resolvable_literal(resolved_unsigned_int_32_literal(1))),
                            ),
                            resolved_resolvable_type(create_shareable(unsigned_int_32_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}


#[test]
fn typing_known_type_procedure_body_assignment_for_assignment_to_global_const() {
    let units_and_types = compile_source_and_get_types_and_unit("GLOBAL :: -11;
SomeProcedure :: () {
    x := GLOBAL;
}");
    
    assert_eq!(units_and_types.len(), 3);
    let (_, global_const_types) = get_first_typed_const_unit(&units_and_types);
    let (proc_body_unit, proc_body_types) = get_first_typed_procedure_body_unit(&units_and_types);
    
    assert_eq!(global_const_types.len(), 1);
    assert_eq!(proc_body_types.len(), 0);
    assert_eq!(
        proc_body_unit.tree, 
        node(
            position(35, 2, 21),
            procedure_body_item(
                string("SomeProcedure"),
                vec!(),
                vec!(),
                vec!(                       
                    node(
                        position(41, 3, 5),
                        assignment_item(
                            string("x"),                     
                            node(                    
                                position(46, 3, 10),
                                identifier_item(string("GLOBAL")),
                            ),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}