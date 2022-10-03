
use crate::threading::*;
use crate::typing::*;
use crate::parsing::*;
use crate::utilities::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;

#[test]
fn typing_procedure_body_types_variable_int_literal_assignment() {
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
fn typing_procedure_body_types_variable_number_expression_assignment() {
    let mut units = run_parse_file_return_only_units("SomeProcedure :: () {
    x := 1 + 2;
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
                                binary_expression_item(
                                    node(
                                        position(33, 2, 12),
                                        operator_item(add_operator())
                                    ),
                                    node(
                                        position(31, 2, 10),
                                        literal_item(resolved_resolvable_literal(resolved_signed_int_64_literal(1))),
                                    ),
                                    node(
                                        position(35, 2, 14),
                                        literal_item(resolved_resolvable_literal(resolved_signed_int_64_literal(2))),
                                    ),
                                    resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                                )
                            ),
                            resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
                        )
                    )
                )
            )
        )
    )
}