
use crate::parsing::*;
use crate::typing::*;
use crate::tests::parsing::*;

#[test]
fn parse_procedure_header_parses_correctly() {
    let content = "SomeProcedure :: () {
}";
    let units = crate::tests::parsing::run_parse_file_return_only_units(content);
    
    assert_eq!(units.len(), 2);
    assert_eq!(
        units[1].tree, 
        node(
            position(0, 1, 1),
            procedure_header_item(
                string("SomeProcedure"),
                vec!(),
                vec!(), 
                units[0].id,
            )
        )
    );
}

#[test]
fn parse_procedure_header_with_return_type_parses_correctly() {
    let content = "SomeProcedure :: () -> void {
}";   
    let units = crate::tests::parsing::run_parse_file_return_only_units(content);
    
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
                        type_item(resolved_resolvable_type(built_in_type_resolved_type_id(void_built_in_type()))),
                    )
                ),
                units[0].id,
            ),
        )
    );
}

#[test]
fn parse_procedure_header_with_return_types_parses_correctly() {
    let content = "SomeProcedure :: () -> SomeType, int {
}";
    let units = crate::tests::parsing::run_parse_file_return_only_units(content);
    
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
                        type_item(resolved_resolvable_type(built_in_type_resolved_type_id(int_32_built_in_type()))),
                    )
                ),
                units[0].id,
            ),
        )
    );
}

#[test]
fn parse_procedure_header_with_arg_parses_correctly() {
    let content = "SomeProcedure :: (x: int) {
}";
    let units = crate::tests::parsing::run_parse_file_return_only_units(content);
    
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
                            resolved_resolvable_type(built_in_type_resolved_type_id(int_32_built_in_type())),
                        )
                    )
                ),
                vec!(),
                units[0].id,
            )
        )
    );
}

#[test]
fn parse_procedure_header_with_args_and_return_type_parses_correctly() {
    let content = "SomeProcedure :: (x: float, y: SomeType) -> void {
}";
    let units= crate::tests::parsing::run_parse_file_return_only_units(content);

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
                        arg_declaration_item(string("x"), resolved_resolvable_type(built_in_type_resolved_type_id(float_32_built_in_type()))),
                    ),
                    node(
                        position(28, 1, 29),
                        arg_declaration_item(string("y"), unresolved_named_resolvable_type(string("SomeType"))),
                    )
                ),
                vec!(
                    node(
                        position(44, 1, 45),
                        type_item(resolved_resolvable_type(built_in_type_resolved_type_id(void_built_in_type()))),
                    ),
                ),
                units[0].id
            )
        )
    );
}
