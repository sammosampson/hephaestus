use crate::parsing::*;
use crate::typing::*;
use crate::utilities::*;
use crate::tests::parsing::*;

#[test]
fn parse_run_directive_parses_correctly() {        
    let units = run_parse_file_return_only_units("#run 1 + 2");
    
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            run_directive_item(
                node(
                    position(5, 1, 6),
                    binary_expression_item(
                        node(
                            position(7, 1, 8),
                            operator_item(add_operator())
                        ),
                        node(
                            position(5, 1, 6),
                            literal_item(unresolved_resolvable_literal(unresolved_int_literal(1, false))),
                        ),
                        node(
                            position(9, 1, 10),
                            literal_item(unresolved_resolvable_literal(unresolved_int_literal(2, false))),
                        ),
                        unresolved_resolvable_type()
                    )
                )  
            )
        )
    )
}

#[test]
fn parse_load_directive_parses_correctly() {
    let units = run_parse_file_return_only_units("#load \"test.jai\"");
       
    assert_eq!(units.len(), 1);
    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            load_directive_item(
                node(
                    position(6, 1, 7),
                    literal_item(unresolved_resolvable_literal(unresolved_string_literal(string("test.jai")))),
                )
            )
        )
    );
}


#[test]
fn parse_foreign_system_library_directive_parses_correctly() {
    let units = run_parse_file_return_only_units("Kernel32 :: #foreign_system_library \"kernel32\";");
       
    assert_eq!(units.len(), 1);

    assert_eq!(
        units[0].tree, 
        node(
            position(0, 1, 1),
            constant_item(
                string("Kernel32"),
                node(
                    position(12, 1, 13),
                    foreign_system_library_directive_item(
                        node(
                            position(36, 1, 37),
                            literal_item(unresolved_resolvable_literal(unresolved_string_literal(string("kernel32")))),
                        )       
                    ),
                )
            )    
        )
    );
}
