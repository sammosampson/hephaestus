
use crate::threading::*;
use crate::types::*;
use crate::parsing::*;
use crate::utilities::*;
use crate::tests::parsing::*;
use crate::tests::typing::*;

#[test]
fn typing_const_types_int_literal_assignment() {
    let mut units = run_parse_file_return_only_units("STD_OUTPUT_HANDLE :: -11;");

    let constant = units.pop().unwrap();

    let typing_repository = start_type_repository_actor();
    let (types, unit) = run_typing_on_unit(typing_repository, constant);

    assert_eq!(types.len(), 1);
    assert_eq!(
        unit.tree, 
        node(
            position(0, 1, 1),
            constant_item(
                string("STD_OUTPUT_HANDLE"),
                node(
                    position(22, 1, 23),
                    literal_item(resolved_resolvable_literal(resolved_signed_int_64_literal(-11))),
                ),
                resolved_resolvable_type(create_shareable(signed_int_64_runtime_type()))
            )
        )
    )
}

#[test]
fn typing_known_type_const_types_int_literal_assignment() {
    let mut units = run_parse_file_return_only_units("STD_OUTPUT_HANDLE : s32 : -11;");

    let constant = units.pop().unwrap();

    let typing_repository = start_type_repository_actor();
    let (types, unit) = run_typing_on_unit(typing_repository, constant);

    assert_eq!(types.len(), 1);
    assert_eq!(
        unit.tree, 
        node(
            position(0, 1, 1),
            constant_item(
                string("STD_OUTPUT_HANDLE"),
                node(
                    position(27, 1, 28),
                    literal_item(resolved_resolvable_literal(resolved_signed_int_32_literal(-11))),
                ),
                resolved_resolvable_type(create_shareable(signed_int_32_runtime_type()))
            )
        )
    )
}