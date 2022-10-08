
use crate::threading::*;
use crate::typing::*;
use crate::parsing::*;
use crate::utilities::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;

#[test]
fn typing_procedure_body_assignment_types_variable_int_literal_assignment() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {
    x := 1;
}");

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();

    let typing_repository = start_type_repository_actor();
    let (types, unit) = run_typing_on_unit(typing_repository, proc_body);

    assert_eq!(types.len(), 0);
    assert_eq!(
        unit.tree, 
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
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {
    x : u8 = 1;
}");

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();

    let typing_repository = start_type_repository_actor();
    let (types, unit) = run_typing_on_unit(typing_repository, proc_body);

    assert_eq!(types.len(), 0);
    assert_eq!(
        unit.tree, 
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
                                position(35, 2, 14),
                                literal_item(resolved_resolvable_literal(resolved_unsigned_int_8_literal(1))),
                            ),
                            resolved_resolvable_type(create_shareable(unsigned_int_8_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}


#[test]
fn typing_known_type_procedure_body_assignment_for_assignment_to_global_const() {
    let mut units = run_parse_file_return_only_units("GLOBAL :: -11;
SomeProcedure :: () {
    x := GLOBAL;
}");

    let _proc_header = units.pop().unwrap();
    let proc_body = units.pop().unwrap();

    let typing_repository = start_type_repository_actor();
    let (types, unit) = run_typing_on_unit(typing_repository, proc_body);

    assert_eq!(types.len(), 0);
    assert_eq!(
        unit.tree, 
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
                                position(35, 2, 14),
                                literal_item(resolved_resolvable_literal(resolved_unsigned_int_8_literal(1))),
                            ),
                            resolved_resolvable_type(create_shareable(unsigned_int_8_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}
