
use crate::parsing::*;
use crate::threading::*;
use crate::typing::*;
use crate::tests::parsing::*;

#[test]
fn parse_procedure_header_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeProcedure :: () {
}");
    
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        node(
            position(0, 1, 1),
            procedure_header_item(
                string("SomeProcedure"),
                vec!(),
                vec!(), 
                local_procedure_body_reference(units[0].id),
            )
        )
    );
}

#[test]
fn parse_procedure_header_with_return_type_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeProcedure :: () -> void {
}");
    
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree,  
        node(
            position(0, 1, 1),
            procedure_header_item(
                string("SomeProcedure"),
                vec!(),
                vec!(
                    node(
                        position(23, 1, 24),
                        type_item(resolved_resolvable_type(create_shareable(void_runtime_type()))),
                    )
                ),
                local_procedure_body_reference(units[0].id),
            ),
        )
    );
}

#[test]
fn parse_procedure_header_with_return_types_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeProcedure :: () -> SomeType, int {
}");
    
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        node(
            position(0, 1, 1),
            procedure_header_item(
                string("SomeProcedure"),
                vec!(),
                vec!(
                    node(
                        position(23, 1, 24),
                        type_item(unresolved_named_resolvable_type(string("SomeType"))),
                    ),
                    node(
                        position(33, 1, 34),
                        type_item(resolved_resolvable_type(create_shareable(signed_int_32_runtime_type()))),
                    )
                ),
                local_procedure_body_reference(units[0].id),
            ),
        )
    );
}

#[test]
fn parse_procedure_header_with_arg_parses_correctly() {
    let units = run_parse_file_return_only_units("SomeProcedure :: (x: int) {
}");
    
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        node(
            position(0, 1, 1),
            procedure_header_item(
                string("SomeProcedure"),
                vec!(
                    node(
                        position(18, 1, 19),
                        arg_declaration_item( 
                            string("x"),
                            resolved_resolvable_type(create_shareable(signed_int_32_runtime_type())),
                        )
                    )
                ),
                vec!(),
                local_procedure_body_reference(units[0].id),
            )
        )
    );
}

#[test]
fn parse_procedure_header_with_args_and_return_type_parses_correctly() {
    let units= run_parse_file_return_only_units("SomeProcedure :: (x: float, y: SomeType) -> void {
}");

    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        node(
            position(0, 1, 1),
            procedure_header_item(
                string("SomeProcedure"),
                vec!(
                    node(
                        position(18, 1, 19),
                        arg_declaration_item(string("x"), resolved_resolvable_type(create_shareable(float_32_runtime_type()))),
                    ),
                    node(
                        position(28, 1, 29),
                        arg_declaration_item(string("y"), unresolved_named_resolvable_type(string("SomeType"))),
                    )
                ),
                vec!(
                    node(
                        position(44, 1, 45),
                        type_item(resolved_resolvable_type(create_shareable(void_runtime_type()))),
                    ),
                ),
                local_procedure_body_reference(units[0].id)
            )
        )
    );
}

#[test]
fn parse_procedure_header_with_pointer_args_and_return_type_parses_correctly() {
    let units= run_parse_file_return_only_units("SomeProcedure :: (x: *float, y: *int) -> *void {
}");

    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        node(
            position(0, 1, 1),
            procedure_header_item(
                string("SomeProcedure"),
                vec!(
                    node(
                        position(18, 1, 19),
                        arg_declaration_item(string("x"), resolved_resolvable_type(create_shareable(float_32_pointer_runtime_type()))),
                    ),
                    node(
                        position(29, 1, 30),
                        arg_declaration_item(string("y"), resolved_resolvable_type(create_shareable(signed_int_32_pointer_runtime_type()))),
                    )
                ),
                vec!(
                    node(
                        position(42, 1, 43),
                        type_item(resolved_resolvable_type(create_shareable(void_pointer_runtime_type()))),
                    ),
                ),
                local_procedure_body_reference(units[0].id)
            )
        )
    );
}


#[test]

fn parse_foreign_system_library_procedure_header_with_pointer_args_and_return_type_parses_correctly() {
    let units= run_parse_file_return_only_units("SomeProcedure :: (x: *float, y: *int) -> *void #foreign Kernel32");

    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            procedure_header_item(
                string("SomeProcedure"),
                vec!(
                    node(
                        position(18, 1, 19),
                        arg_declaration_item(string("x"), resolved_resolvable_type(create_shareable(float_32_pointer_runtime_type()))),
                    ),
                    node(
                        position(29, 1, 30),
                        arg_declaration_item(string("y"), resolved_resolvable_type(create_shareable(signed_int_32_pointer_runtime_type()))),
                    )
                ),
                vec!(
                    node(
                        position(42, 1, 43),
                        type_item(resolved_resolvable_type(create_shareable(void_pointer_runtime_type()))),
                    ),
                ),
                foreign_procedure_body_reference(
                    node(
                        position(56, 1, 57),
                        identifier_item(string("Kernel32"))
                    )
                )
            )
        )
    );
}
