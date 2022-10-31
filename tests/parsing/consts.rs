use rust_hephaestus::*;

#[test]
fn parse_const_declaration_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeValue :: 1");
       
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            constant_item(
                string("SomeValue"),
                node(
                    position(13, 1, 14),
                    literal_item(unresolved_resolvable_literal(unresolved_int_literal(string("1")))),
                ),
                unresolved_resolvable_type()
            )
        )
    );

    
}

#[test]
fn parse_const_declaration_with_type_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeValue : s32 : -11");
       
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            constant_item(
                string("SomeValue"),
                node(
                    position(19, 1, 20),
                    literal_item(unresolved_resolvable_literal(unresolved_int_literal(string("-11")))),
                ),
                resolved_resolvable_type(create_shareable(signed_int_32_runtime_type()))
            )    
        )
    );
}